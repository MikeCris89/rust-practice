# Fieldwatch

A small Rust practice app for reviewing mock invasive-species detections.

The goal is to understand and read professional Rust: follow a request through a backend, recognize common types and patterns, explain ownership and errors, and make a small change confidently. Building the app gives those concepts a concrete purpose.

## The idea

A fictional conservation team receives possible invasive-species detections from drones and satellites. A reviewer lists the detections, filters by confidence, confirms or dismisses them, and checks a summary of the review queue.

Example session: find pending detections with confidence of at least 80, confirm detection 1, then see the confirmed count increase.

[INVERSA's Origin overview](https://inversa.com/origin) describes geospatial detection, field operations, and reporting. Fieldwatch borrows that general workflow for learning. It is an independent exercise; we are not reproducing Origin's architecture. The Rust/React stack is context supplied by you, not something verified from that page.

## Keep version one small

- Start with three hardcoded, fictional detections; no imagery or AI integration.
- List detections and filter by minimum confidence and review status.
- Confirm or dismiss a detection by ID.
- Report total, pending, confirmed, and dismissed counts.
- First exercise the logic in a terminal; later expose it through a local JSON API.

Version one stores everything in memory. Restarting resets the data. A high confidence score never automatically confirms a detection. Scores are fictional inputs, not validated probabilities or ecological guidance.

Accounts, deployment, maps, uploads, actual model inference, mission assignment, offline sync, and a database are outside version one.

## Technical direction

Start with one Rust binary package in this folder and the standard library. Start in `src/main.rs`; split modules only when the code needs it. Reuse the review/filter/summary functions when we add HTTP.

For the HTTP milestone, the proposed tools are [Axum](https://docs.rs/axum/latest/axum/) for routing, Tokio for running async work, and Serde for converting Rust values to and from JSON. We will discuss and approve dependencies before adding them, and check current APIs then.

React and Rust work well as separate programs communicating over HTTP/JSON. A frontend is optional here: terminal output and later `curl` are enough to learn the backend. If a visual client becomes useful, add a small React app under `web/` in this same repository. That makes a simple monorepo without requiring workspace tooling or multiple Rust packages. Your existing Expo app could consume the same API without moving it here.

## Data model to grow into

Introduce these fields over several lessons, rather than typing out the whole model immediately.

| Field | Planned Rust type | Purpose |
| --- | --- | --- |
| `id` | `u32` | Small, unique identifier |
| `species` | `String` | Suspected species name |
| `latitude`, `longitude` | `f64` | Fictional detection location |
| `confidence` | `u8` | Integer score from 0 through 100 |
| `source` | `DetectionSource` enum | Drone or satellite |
| `status` | `ReviewStatus` enum | Pending, confirmed, or dismissed |
| `note` | `Option<String>` | Optional reviewer note |

For example, use three fictional detections with confidence scores of 92, 80, and 55, initially all pending. Confidence filtering includes the threshold itself. Latitude must be between -90 and 90, and longitude between -180 and 180; numeric locations must also be finite. A `u8` still permits values above 100, so application validation matters.

## Proposed HTTP surface, for later

| Request | Purpose |
| --- | --- |
| `GET /detections?min_confidence=80&status=pending` | List matching detections; filters are optional |
| `GET /detections/{id}` | Fetch one detection |
| `PATCH /detections/{id}/review` | Set confirmed/dismissed status and an optional note |
| `GET /summary` | Count all detections by status |

Unknown IDs should produce 404 responses; invalid input should produce a documented client error, not a panic. Define the exact JSON contract in the HTTP lesson. Seeded data is enough; creating detections through HTTP is unnecessary for this version.

## How we will work

You write the application code. I explain one concept, give a small task, review your attempt, and help interpret compiler feedback. Hints come before complete solutions. Each milestone ends with both working behavior and a short code-reading exercise.

Your starting point is TypeScript, Next.js, React, and Express. Lessons will build on that knowledge: typed objects and unions for data modeling, array methods for filtering, and Express handlers or Next.js Route Handlers for HTTP. Each comparison will also explain the Rust-specific difference, especially ownership, borrowing, explicit error values, and compilation. We will spend time on those differences rather than reteach general programming basics.

Lesson 6's vector and borrowed loops have been reviewed and checked; continue with [lesson 7: ownership practice](lessons/07-ownership-practice.md), one small exercise at a time. Ownership concepts will be reinforced through repeated use. [The learning plan](LEARNING_PLAN.md) records current progress, verification, and the handoff for a new chat. [AGENTS.md](AGENTS.md) preserves the tutoring approach. For initial setup reference, see [lesson 1](lessons/01-setup.md).

Version one is complete when you can list/filter/review mock detections through HTTP, get consistent summary counts, handle invalid input without crashing, and explain the path from a request to its response.
