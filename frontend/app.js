const BACKEND_URL = "http://127.0.0.1:3000";
let activeAbortController = null;

// Track buffer contexts for incoming text streams globally
let streamBuffers = { research: "", story: "", visuals: "", critique: "" };

document.addEventListener("DOMContentLoaded", () => {
  const createBtn = document.getElementById("create-btn");
  if (createBtn) createBtn.addEventListener("click", createWorkflow);
  loadHistory();
});

async function createWorkflow() {
  const ideaInput = document.getElementById("idea");
  if (!ideaInput || !ideaInput.value.trim()) {
    alert("Please write a project idea premise first!");
    return;
  }

  const modelSelect = document.getElementById("model-select");
  const selectedModel = modelSelect ? modelSelect.value : "qwen2.5";

  // Clear layout buffers
  streamBuffers = { research: "", story: "", visuals: "", critique: "" };
  prepareBlankWorkspaceCards();
  toggleLoadingState(true);

  activeAbortController = new AbortController();

  try {
    // SSE requests use POST bodies via fetch stream readers
    const response = await fetch(`${BACKEND_URL}/run_agents`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ project_id: ideaInput.value.trim(), model: selectedModel }),
      signal: activeAbortController.signal
    });

    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    let partialLine = "";

    while (true) {
      const { value, done } = await reader.read();
      if (done) break;

      // Decode bytes chunk to plain text stream segments
      const chunk = decoder.decode(value, { stream: true });
      const lines = (partialLine + chunk).split("\n");
      partialLine = lines.pop(); // Hold incomplete trailing text segment

      for (const line of lines) {
        if (!line.trim()) continue;

        // Parse basic Server Sent Events schema (event: Name \n data: Payload)
        if (line.startsWith("event:")) {
          currentEvent = line.replace("event:", "").trim();
        } else if (line.startsWith("data:")) {
          const dataPayload = line.replace("data:", "").trim();
          handleStreamEvent(currentEvent, dataPayload);
        }
      }
    }
  } catch (error) {
    if (error.name !== 'AbortError') {
      console.error("Streaming pipeline crashed:", error);
    }
  } finally {
    toggleLoadingState(false);
    loadHistory();
  }
}

function prepareBlankWorkspaceCards() {
  const results = document.getElementById("results");
  if (!results) return;

  results.innerHTML = `
    <div class="global-actions hidden" id="global-export-zone">
      <button id="export-all-btn" class="sidebar-toggle-btn" style="background: #2563eb; border-color: #3b82f6;">📥 Export Entire Suite (.md)</button>
    </div>
  `;

  const sections = [
    { id: "research", title: "🔍 Researcher" },
    { id: "story", title: "✍️ Screenwriter" },
    { id: "visuals", title: "🎨 Visual Designer" },
    { id: "critique", title: "⚖️ Creative Critic" }
  ];

  sections.forEach(sec => {
    const card = document.createElement("div");
    card.className = "card opacity-50"; // Start slightly faded until agent wakes up
    card.id = `card-${sec.id}`;
    card.innerHTML = `
      <div class="card-header">
        <h3>${sec.title} <span class="status-indicator" id="status-${sec.id}">💤 Waiting</span></h3>
      </div>
      <div class="card-content markdown-body" id="content-${sec.id}">*Awaiting previous agent steps...*</div>
    `;
    results.appendChild(card);
  });
}

function handleStreamEvent(event, data) {
  if (event === "agent_start") {
    const card = document.getElementById(`card-${data}`);
    const status = document.getElementById(`status-${data}`);
    if (card) card.classList.remove("opacity-50");
    if (status) status.innerText = "✍️ Writing...";
  }
  else if (event === "token") {
    // Delimiter tracking separating card target id from actual text tokens
    const splitIndex = data.indexOf(":");
    const targetId = data.substring(0, splitIndex);
    const tokenBytes = data.substring(splitIndex + 1);

    streamBuffers[targetId] += tokenBytes;

    const contentArea = document.getElementById(`content-${targetId}`);
    if (contentArea) {
      // 🚀 Markdown Parsing in Real-Time!
      contentArea.innerHTML = typeof marked !== 'undefined' ? marked.parse(streamBuffers[targetId]) : streamBuffers[targetId];
    }
  }
  else if (event === "completed") {
    document.querySelectorAll(".status-indicator").forEach(el => el.innerText = "✅ Done");
    const exportZone = document.getElementById("global-export-zone");
    if (exportZone) exportZone.classList.remove("hidden");

    document.getElementById("export-all-btn")?.addEventListener("click", () => {
      exportEntireSuite(streamBuffers);
    });
  }
}

// --- Utilities & Re-rendering Static Views ---
function renderArtifacts(artifacts) {
  // Save static history records to our runtime buffer so exports remain unified
  streamBuffers = { ...artifacts };
  prepareBlankWorkspaceCards();

  document.getElementById("global-export-zone")?.classList.remove("hidden");
  document.getElementById("export-all-btn")?.addEventListener("click", () => { exportEntireSuite(artifacts); });

  Object.keys(artifacts).forEach(id => {
    const card = document.getElementById(`card-${id}`);
    const status = document.getElementById(`status-${id}`);
    const contentArea = document.getElementById(`content-${id}`);

    if (card) card.classList.remove("opacity-50");
    if (status) status.innerText = "💾 Archived";
    if (contentArea) {
      contentArea.innerHTML = typeof marked !== 'undefined' ? marked.parse(artifacts[id]) : artifacts[id];
    }
  });
}

function toggleLoadingState(isLoading) {
  const loader = document.getElementById("loader");
  if (loader) loader.className = isLoading ? "" : "hidden";
}

function exportEntireSuite(artifacts) {
  const ideaInput = document.getElementById("idea");
  const name = ideaInput ? ideaInput.value.trim() : "Project";
  const masterMarkdown = `# DreamWeaver Master Suite: ${name}\n\n## 🔍 Research\n${artifacts.research}\n\n## ✍️ Story\n${artifacts.story}\n\n## 🎨 Visuals\n${artifacts.visuals}\n\n## ⚖️ Critique\n${artifacts.critique}`;

  const element = document.createElement('a');
  element.setAttribute('href', 'data:text/markdown;charset=utf-8,' + encodeURIComponent(masterMarkdown));
  element.setAttribute('download', `${name.toLowerCase().replace(/[^a-z0-9]/gi, '_')}_suite.md`);
  element.style.display = 'none';
  document.body.appendChild(element);
  element.click();
  document.body.removeChild(element);
}

async function loadHistory() {
  const historyList = document.getElementById("history-list");
  if (!historyList) return;
  try {
    const res = await fetch(`${BACKEND_URL}/history`);
    if (!res.ok) return;
    const items = await res.json();
    historyList.innerHTML = "";
    items.forEach(item => {
      const div = document.createElement("div");
      div.className = "history-item";
      div.innerHTML = `<strong>${item.project_id.substring(0, 25)}...</strong><small>${item.created_at}</small>`;
      div.addEventListener("click", () => loadHistoryItem(item.id));
      historyList.appendChild(div);
    });
  } catch (e) { console.error(e); }
}

async function loadHistoryItem(id) {
  try {
    const res = await fetch(`${BACKEND_URL}/history/${id}`);
    if (!res.ok) return;
    const record = await res.json();
    renderArtifacts({ research: record.research, story: record.story, visuals: record.visuals, critique: record.critique });

    if (document.getElementById("idea")) document.getElementById("idea").value = record.project_id;
    document.getElementById("sidebar")?.classList.remove("open");
  } catch (e) { console.error(e); }
}