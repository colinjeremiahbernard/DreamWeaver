const BACKEND_URL = "http://127.0.0.1:3000";
let activeAbortController = null;

document.addEventListener("DOMContentLoaded", () => {
  // --- 1. Query All Elements Safely ---
  const createBtn = document.getElementById("create-btn");
  const cancelBtn = document.getElementById("cancel-btn");
  const toggleSidebarBtn = document.getElementById("toggle-sidebar-btn");
  const closeSidebarBtn = document.getElementById("close-sidebar-btn");
  const sidebar = document.getElementById("sidebar");

  const toggleSettingsBtn = document.getElementById("toggle-settings-btn");
  const closeSettingsBtn = document.getElementById("close-settings-btn");
  const settingsModal = document.getElementById("settings-modal");

  // --- 2. Attach Event Listeners (With Safe Checks) ---
  if (createBtn) {
    createBtn.addEventListener("click", createWorkflow);
  }

  if (cancelBtn) {
    cancelBtn.addEventListener("click", () => {
      if (activeAbortController) {
        activeAbortController.abort();
        console.log("🛑 User cancelled the agent orchestration.");
        toggleLoadingState(false);
      }
    });
  }

  if (toggleSidebarBtn && sidebar) {
    toggleSidebarBtn.addEventListener("click", () => sidebar.classList.add("open"));
  }
  if (closeSidebarBtn && sidebar) {
    closeSidebarBtn.addEventListener("click", () => sidebar.classList.remove("open"));
  }

  if (toggleSettingsBtn && settingsModal) {
    toggleSettingsBtn.addEventListener("click", () => settingsModal.classList.remove("hidden"));
  }
  if (closeSettingsBtn && settingsModal) {
    closeSettingsBtn.addEventListener("click", () => settingsModal.classList.add("hidden"));
  }

  // Initial load
  loadHistory();
});

// --- 3. Core Workflow Execution Engine ---
async function createWorkflow() {
  const ideaInput = document.getElementById("idea");
  if (!ideaInput) return;

  const idea = ideaInput.value.trim();
  if (!idea) {
    alert("Please write a project idea premise first!");
    return;
  }

  const modelSelect = document.getElementById("model-select");
  const selectedModel = modelSelect ? modelSelect.value : "qwen2.5";

  toggleLoadingState(true);

  activeAbortController = new AbortController();
  const workflowSignal = activeAbortController.signal;

  try {
    const response = await fetch(`${BACKEND_URL}/run_agents`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        project_id: idea,
        model: selectedModel
      }),
      signal: workflowSignal
    });

    if (!response.ok) {
      throw new Error(`Server status error: ${response.status}`);
    }

    const data = await response.json();
    if (data && data.artifacts) {
      renderArtifacts(data.artifacts);
      loadHistory(); // Refresh history sidebar with the new item
    }
  } catch (error) {
    if (error.name === 'AbortError') {
      console.log("Fetch successfully cancelled by the user.");
    } else {
      console.error("Workflow failed:", error);
      alert("An execution error occurred. Check the backend log console.");
    }
  } finally {
    toggleLoadingState(false);
    activeAbortController = null;
  }
}

// --- 4. Render Layout Cards & Actions ---
function renderArtifacts(artifacts) {
  const results = document.getElementById("results");
  if (!results) return;

  results.innerHTML = "";

  // Global Export button at top
  const globalActions = document.createElement("div");
  globalActions.className = "global-actions";
  globalActions.innerHTML = `
    <button id="export-all-btn" class="sidebar-toggle-btn" style="background: #2563eb; border-color: #3b82f6;">
      📥 Export Entire Suite (.md)
    </button>
  `;
  results.appendChild(globalActions);

  document.getElementById("export-all-btn").addEventListener("click", () => {
    exportEntireSuite(artifacts);
  });

  const sections = [
    { id: "research", title: "🔍 Researcher", content: artifacts.research },
    { id: "story", title: "✍️ Screenwriter", content: artifacts.story },
    { id: "visuals", title: "🎨 Visual Designer", content: artifacts.visuals },
    { id: "critique", title: "⚖️ Creative Critic", content: artifacts.critique }
  ];

  sections.forEach(section => {
    const card = document.createElement("div");
    card.className = "card";
    card.innerHTML = `
      <div class="card-header">
        <h3>${section.title}</h3>
        <div class="card-utilities">
          <button class="util-btn copy-btn" data-id="${section.id}">📋 Copy</button>
          <button class="util-btn download-btn" data-id="${section.id}" data-title="${section.title}">💾 Download</button>
        </div>
      </div>
      <div class="card-content">${section.content || "No output generated."}</div>
    `;
    results.appendChild(card);
  });

  // Attach button utility clicks
  results.querySelectorAll(".copy-btn").forEach(btn => {
    btn.addEventListener("click", (e) => {
      const id = e.target.getAttribute("data-id");
      copyToClipboard(artifacts[id], e.target);
    });
  });

  results.querySelectorAll(".download-btn").forEach(btn => {
    btn.addEventListener("click", (e) => {
      const id = e.target.getAttribute("data-id");
      const label = e.target.getAttribute("data-title");
      const text = artifacts[id];
      const ideaInput = document.getElementById("idea");
      const proj = ideaInput ? ideaInput.value.trim().substring(0, 20) : "project";
      const filename = `${proj.toLowerCase().replace(/[^a-z0-9]/gi, '_')}_${id}.md`;
      downloadAsFile(filename, `# ${label}\n\n${text}`);
    });
  });
}

// --- 5. Utilities & Sidebars Fetching ---
function toggleLoadingState(isLoading) {
  const loader = document.getElementById("loader");
  const createBtn = document.getElementById("create-btn");
  const cancelBtn = document.getElementById("cancel-btn");

  if (loader) loader.className = isLoading ? "" : "hidden";
  if (createBtn) createBtn.disabled = isLoading;
  if (cancelBtn) cancelBtn.className = isLoading ? "sidebar-toggle-btn btn-danger" : "sidebar-toggle-btn btn-danger hidden";
}

function copyToClipboard(text, btn) {
  if (!text) return;
  navigator.clipboard.writeText(text).then(() => {
    const orig = btn.innerText;
    btn.innerText = "✅ Copied!";
    setTimeout(() => { btn.innerText = orig; }, 2000);
  });
}

function downloadAsFile(filename, text) {
  const element = document.createElement('a');
  element.setAttribute('href', 'data:text/markdown;charset=utf-8,' + encodeURIComponent(text));
  element.setAttribute('download', filename);
  element.style.display = 'none';
  document.body.appendChild(element);
  element.click();
  document.body.removeChild(element);
}

function exportEntireSuite(artifacts) {
  const ideaInput = document.getElementById("idea");
  const name = ideaInput ? ideaInput.value.trim() : "Project";
  const masterMarkdown = `# DreamWeaver Master Suite: ${name}\n\n## 🔍 Research\n${artifacts.research}\n\n## ✍️ Story\n${artifacts.story}\n\n## 🎨 Visuals\n${artifacts.visuals}\n\n## ⚖️ Critique\n${artifacts.critique}`;
  downloadAsFile(`${name.toLowerCase().replace(/[^a-z0-9]/gi, '_')}_suite.md`, masterMarkdown);
}

async function loadHistory() {
  const historyList = document.getElementById("history-list");
  if (!historyList) return;

  try {
    const res = await fetch(`${BACKEND_URL}/history`);
    if (!res.ok) return;
    const items = await res.json();

    // Explicitly binding via addEventListener inside the loop to ensure clean execution context
    historyList.innerHTML = "";
    items.forEach(item => {
      const div = document.createElement("div");
      div.className = "history-item";
      div.innerHTML = `
        <strong>${item.project_id.substring(0, 25)}...</strong>
        <small>${item.created_at}</small>
      `;
      div.addEventListener("click", () => loadHistoryItem(item.id));
      historyList.appendChild(div);
    });
  } catch (e) {
    console.error("Failed to load history list:", e);
  }
}

async function loadHistoryItem(id) {
  try {
    const res = await fetch(`${BACKEND_URL}/history/${id}`);
    if (!res.ok) return;
    const record = await res.json();

    // Reconstruct artifacts mapping format expected by renderArtifacts
    const mappedArtifacts = {
      research: record.research,
      story: record.story,
      visuals: record.visuals,
      critique: record.critique
    };

    renderArtifacts(mappedArtifacts);

    const ideaInput = document.getElementById("idea");
    if (ideaInput) ideaInput.value = record.project_id;

    const sidebar = document.getElementById("sidebar");
    if (sidebar) sidebar.classList.remove("open");
  } catch (e) {
    console.error("Failed loading history item details:", e);
  }
}