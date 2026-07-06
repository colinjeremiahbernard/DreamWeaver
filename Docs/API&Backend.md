DreamWeaver exposes a minimal but powerful backend API responsible for orchestrating AI agents, managing creative projects, and streaming real-time updates to the frontend. The backend is intentionally thin: it does not generate creative content itself, but instead coordinates agent execution and maintains system state.

At the center of the system is a Rust (Axum-based) API layer that receives user requests and routes them into an AI Orchestrator. The orchestrator manages the lifecycle of a creative session, from initial idea input to final universe generation. All AI agent execution is triggered through this layer, ensuring full control over sequencing, memory updates, and consistency.

The system revolves around a small set of core API endpoints. A user begins by creating a project, which represents a single creative idea. This project is then expanded into a universe through iterative AI agent execution.

The primary endpoint is project creation:

POST /project/create

Request:
{
user_id: string,
title: string,
idea: string
}

This initializes a new project and triggers the Creative Director agent. The Creative Director analyzes the idea, asks clarifying questions, and produces a CreativeBrief. This brief becomes the foundation for all downstream agent execution.

Once the CreativeBrief is ready, the frontend initiates the generation pipeline:

POST /project/{project_id}/generate

This endpoint triggers the AI Orchestrator, which executes the full agent workflow:
Creative Director → Story Agent → World Agent → Character Agent → Visual Agent → Critic Agent.

Each agent execution is treated as a structured event and stored as an AgentRun.

To support real-time interaction, the backend exposes a streaming endpoint:

WS /project/{project_id}/stream

This WebSocket connection streams live updates from the AI Orchestrator to the frontend. Events include:

- Agent started
- Agent completed
- Partial output updates
- Critic feedback
- Memory updates
- Final universe completion

This streaming layer is critical to the DreamWeaver experience because it allows users to observe the “creative studio” in action rather than waiting for a single static response.

The backend also exposes project retrieval:

GET /project/{project_id}

This returns:

- Project metadata
- Latest Universe
- CreativeMemory state
- Agent history (AgentRuns)

Each project can evolve into multiple universes over time. A universe is generated result of a full agent cycle and contains structured creative output such as story, world-building, characters, and visual direction.

Internally, the AI Orchestrator is responsible for managing execution flow. It ensures that the Creative Director runs first, followed by parallel execution of production agents, and finally the Critic Agent. If the Critic Agent detects inconsistencies, it can trigger a revision loop, causing specific agents to re-run with updated context.

The system maintains strict separation of concerns: the backend handles orchestration and state, while AI agents handle reasoning and generation. This ensures that logic, memory, and execution remain deterministic from a system perspective, even though AI outputs are probabilistic.

A simplified flow of execution is:

User Input → Project Creation → Creative Director → Creative Brief → Parallel Agent Execution → Critic Review → Memory Update → Universe Finalization → Streaming to Frontend

Each step is recorded as an AgentRun, ensuring full traceability of how any creative output was produced.

The key principle of the API design is minimal surface area with maximum orchestration power. Only a few endpoints exist, but each one triggers a complex, stateful AI workflow behind the scenes.

This makes DreamWeaver feel like a responsive creative system rather than a traditional REST-based application.

The key differentiator is that API calls do not return static responses. Instead, they initiate living creative processes that evolve over time and stream their progress back to the user in real time.
