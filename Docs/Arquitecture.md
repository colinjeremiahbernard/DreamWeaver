# DREAMWEAVER – SYSTEM ARCHITECTURE

## 1. Overview

DreamWeaver uses a modular architecture that separates:

- User Interface (creative studio experience)
- Backend API (orchestration layer)
- AI Agent System (creative reasoning layer)
- Persistence Layer (projects and creative memory)

The system is designed for real-time collaboration between user and AI agents.

---

## 2. High-Level Architecture

Frontend (Studio UI)
↓
Backend API (Rust / Axum)
↓
AI Orchestrator
↓
Agent Execution Layer
↓
Shared Creative Memory
↓
Database + Storage

---

## 3. Frontend (Studio Interface)

**Technology:**

- Next.js (recommended)
- React

**Responsibilities:**

- Idea input
- Chat + interview UI
- Live agent activity dashboard
- Universe visualization panels
- Real-time streaming updates

**Key Concept:**
The UI behaves like a “live studio”, not a static chat app.

---

## 4. Backend API (Rust / Axum)

**Responsibilities:**

- Receives user input
- Manages sessions and projects
- Routes requests to AI Orchestrator
- Streams agent updates to frontend
- Handles persistence layer communication

**Key Design Principle:**
Backend does NOT generate creative content directly.
It only orchestrates.

---

## 5. AI Orchestrator Layer

This is the brain of DreamWeaver.

### Responsibilities:

- Receives creative request
- Initializes Creative Director Agent
- Dispatches tasks to other agents
- Manages execution order (parallel + sequential)
- Collects outputs
- Sends data to Critic Agent
- Finalizes universe output

---

## 6. Agent Execution Model

Agents operate in two modes:

### Parallel Execution

Used for:

- Story Agent
- World Agent
- Character Agent
- Visual Agent

These run simultaneously after receiving the creative brief.

### Sequential Execution

Used for:

- Creative Director (first)
- Critic Agent (final validation step)

---

## 7. Shared Creative Memory

A centralized context store used by all agents.

### Stores:

- Theme
- Tone
- World rules
- Characters
- Story state
- Visual direction

### Purpose:

Ensures all agents remain consistent with the same universe definition.

---

## 8. Data Layer

### Database (PostgreSQL)

Stores:

- Users
- Projects
- Universes
- Agent outputs
- Creative Memory snapshots

### Object Storage

Stores:

- Images
- Visual assets
- Exported documents (PDF, markdown)

---

## 9. Real-Time Communication

### Method:

- WebSockets or streaming API

### Purpose:

- Show live agent activity
- Stream partial outputs
- Update UI as agents work

Example events:

- "World Agent started"
- "Story Agent completed Act 1"
- "Critic Agent requested revision"

---

## 10. Key System Principle

> The system must feel alive.

Users should see AI agents working, not waiting for a single response.

---

## 11. Key Differentiator

Instead of generating a single output, DreamWeaver orchestrates a multi-agent system that collaboratively builds a structured creative universe in real time.
