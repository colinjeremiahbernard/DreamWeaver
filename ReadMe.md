# DreamWeaver 🚀
> An open-source, local multi-agent creative workflow orchestrator powered by Rust (Axum) and Ollama.

DreamWeaver coordinates a sequenced pipeline of localized AI specialized agents (**Researcher 🔍**, **Screenwriter ✍️**, **Visual Designer 🎨**, and **Creative Critic ⚖️**) to systematically turn high-level premise ideas into deep, fully realized master suite dossiers—all stored locally in SQLite with built-in export suites.

---

## 🏗️ Architecture Overview

The application runs on a twin-engine split model:
*   **Backend (`:3000`):** An asynchronous Rust binary powered by **Axum**, **Tokio**, and **SQLx** that manages agent execution loops, interacts with local SQLite storage, and proxies requests to Ollama.
*   **Frontend (`:8000` or Static):** A vanilla HTML5/CSS3/JavaScript interface providing custom loading indicators, markdown exports, real-time context cancellation, a session history sidebar, and model switching configurations.

---

## 🛠️ Prerequisites & Installation

Before spinning up the application, make sure your machine has the following dependencies configured.

### 1. Local LLM Runtime (Ollama)
DreamWeaver runs entirely on local hardware. You must have Ollama installed and running.
*   **Download:** [ollama.com/download](https://ollama.com/download)
*   Ensure the background daemon app is active (look for the llama icon in your system menu bar/tray).
*   Download your preferred baseline model(s) via your terminal:
    ```bash
    ollama pull qwen2.5
    ollama pull llama3
    ```

### 2. Rust Toolchain
Ensure you have the Rust compiler and cargo installer configured:
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh