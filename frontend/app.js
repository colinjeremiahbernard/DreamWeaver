async function createWorkflow() {
  const idea = document.getElementById("idea").value;
  const results = document.getElementById("results");

  // Show loading state
  results.innerHTML = "Creating...";

  try {
    const response = await fetch("http://127.0.0.1:3000/workflow", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        idea: idea
      })
    });

    // Check if the server responded with an error status (like 500 or 404)
    if (!response.ok) {
      throw new Error(`Server returned status: ${response.status}`);
    }

    const data = await response.json();

    // Clear loading state
    results.innerHTML = "";

    // Safely check if outputs exists and is an array before looping
    if (data && Array.isArray(data.outputs)) {
      data.outputs.forEach((output, index) => {
        const card = document.createElement("div");
        card.className = "card";
        card.innerHTML = `
            <h3>Agent ${index + 1}</h3>
            <p>${output}</p>
        `;
        results.appendChild(card);
      });
    } else {
      results.innerHTML = "<p class='error'>Error: Invalid response format from server.</p>";
    }

  } catch (error) {
    // Handle network errors, CORS blocks, or server-down scenarios
    console.error("Workflow creation failed:", error);
    results.innerHTML = `
      <div class="error-box">
        <p><strong>Failed to connect to backend server.</strong></p>
        <p>Make sure your backend is running at <code>http://127.0.0.1:3000</code> and CORS is enabled.</p>
      </div>
    `;
  }
}