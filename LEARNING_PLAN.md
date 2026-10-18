# Fieldwatch learning plan

Progress means you can explain the code as well as run it. Work through one small exercise at a time; these milestones are not a schedule or a requirement to memorize the language.

## Build on your existing stack

Your background is TypeScript, Next.js, React, and Express. Use these comparisons throughout the milestones, with short side-by-side examples when useful:

| Familiar concept | Rust topic | Difference to explore |
| --- | --- | --- |
| Typed objects and interfaces | Structs and `impl` blocks | Rust structs define concrete data types; TypeScript interfaces use structural typing |
| Discriminated unions and narrowing | Enums and `match` | Rust variants can carry data; matches must cover all possible cases |
| Arrays, `.filter()`, `.map()` | `Vec`, iterators, closures | Iteration can borrow or consume values; iterator operations are lazy until consumed |
| `T \| undefined`, null checks | `Option<T>` | Absence is represented by explicit `Some`/`None` variants |
| `throw`, `try`/`catch` | `Result<T, E>` and `?` | Recoverable errors are returned values; `?` propagates an error to the caller |
| Express handlers / Next.js Route Handlers | Axum routes, extractors, handlers | Trace typed input through application logic into an HTTP response |
| Promises and `await` | Futures, `async`/`.await`, Tokio | Rust futures need polling to make progress; async work is not automatically parallel |
| JavaScript references and garbage collection | Ownership, borrowing, lifetimes | Rust checks access and lifetimes at compile time; it has no direct JS equivalent |

React remains a useful reference for the client/server boundary. Backend shared state is not component state, and Rust lifetimes are not React lifecycles.

## Current position

- Lessons 1 and 2 implementation exercises are complete. Rust and Git are initialized; there are no third-party dependencies.
- `src/main.rs` contains the two startup messages, `species`, `confidence: u8 = 92`, and `confirmed: bool = false`, followed by formatted output including the corrected `%` suffix. This source has been reviewed.
- You reported successful output for lesson 2. The lesson 1 run and formatting check were also reported as passing. A separate lesson 2 formatting result was not supplied; commands were not rerun during review.
- Next lesson: [lesson 3: functions and return values](lessons/03-functions.md). Its exercise has not been implemented.

## Handoff for the next chat

- Start at lesson 3, reading the current source before guiding the exercise. Let the user write the function; give hints before solutions. Do not restart setup or implement later milestones.
- Use TypeScript/Next.js/React/Express comparisons by default. The user asks short conceptual questions as they work; explain unfamiliar syntax before expecting them to use it.
- Concepts already discussed: entry points, Cargo versus npm, macros (`println!`, `vec!`), `format!` versus JS template literals, immutable bindings, type annotations, semicolons and final expressions, and a high-level introduction to ownership versus garbage collection. Exposure does not imply mastery; ownership has not been practiced.
- The user predicted that 300 would not fit a `u8` but was unsure whether it could be saved incorrectly. We explained that the literal is rejected at compile time, that `u8` permits 0 through 255, and that the app's 0 through 100 rule needs separate validation. That understanding has not yet been reassessed.
- The manifest/lockfile analogy and macro behavior have been explained; the remaining lesson 1 explanation checkbox can be revisited naturally, without blocking lesson 3 on a full recap.

## Milestones

### 1. Run and change a Rust program

- [x] Install Rust and initialize the package yourself.
- [x] Replace the generated greeting with a Fieldwatch startup message.
- [ ] Explain `fn main`, `println!`, `Cargo.toml`, and `Cargo.lock` in plain language.

Verify: `cargo run` prints your message; `cargo fmt --check` succeeds. No test needed for a greeting.

### 2. Represent a detection

- [x] Represent one detection with variables and formatted output (lesson 2).
- [ ] Introduce a function with typed parameters and a return value (lesson 3).
- [ ] Group a detection's fields in a struct (a later lesson).
- [ ] Add source/status enums and three mock detections in a `Vec`.
- [ ] Use `match` to describe review status.
- [ ] Read `String`, `&str`, `&Detection`, and `&mut Detection`; explain owned data versus borrowed access.

Verify: print all three detections, then print them again without rebuilding or cloning the collection. Explain why iteration does or does not consume it.

### 3. Filter and summarize without consuming the data

- [ ] Write a function that borrows detections and filters by minimum confidence.
- [ ] Add a status filter and summary counts.
- [ ] Write a plain loop first, then read an equivalent iterator chain using a closure and `collect`.
- [ ] Read `Option` by looking up a detection that might not exist.

Verify: with scores 92, 80, and 55, a minimum of 80 returns two detections; 100 returns none. Add focused tests for the threshold boundary and an empty collection. The original collection remains usable.

### 4. Review detections and handle mistakes

- [ ] Update a detection through a mutable borrow; add an optional note.
- [ ] Return a `Result` for an unknown ID or invalid input.
- [ ] Validate confidence and coordinates when constructing input data.
- [ ] Read an `impl` block, a small error enum, and a function using `?`.
- [ ] Move related code into modules when needed; follow `mod`, `use`, and `pub`.

Verify: confirming a detection moves one count from pending to confirmed. Dismissing it afterward updates the counts again; reviewing it with the same status leaves counts unchanged. Unknown IDs return an error and leave data unchanged. Test confidence bounds, coordinate bounds, and non-finite coordinates in the relevant validation function.

### 5. Expose the same behavior through HTTP

- [ ] Discuss the proposed Axum/Tokio/Serde dependencies before adding them.
- [ ] Learn how a route, handler, typed input, and JSON response fit together.
- [ ] Reuse the existing application functions behind the four planned endpoints.
- [ ] Introduce `async`/`.await` and shared in-memory state only as needed.
- [ ] Read `#[derive(...)]`, `Json<T>`, `Result<T, E>`, and a trait implementation.
- [ ] Explain `Arc` and the chosen lock: who owns the data, who can change it, and when a lock is released. Keep lock guards out of unrelated async work.

Verify: manually start the local server for this lesson and use `curl` to filter, review, and fetch updated counts. Exercise an unknown ID and invalid JSON/input. Add focused handler tests for those error paths using the agreed tooling. Explain the complete request path.

### 6. Read the finished backend

- [ ] Pick a handler and trace its inputs, borrowed/owned values, possible errors, and response type.
- [ ] Explain a generic type, a trait bound, a lifetime annotation, and a macro from short examples; advanced implementations can remain unfamiliar.
- [ ] Make one small behavior change and predict which tests should change before editing.
- [ ] Explain why mock confidence is different from human confirmation.
- [ ] Discuss what persistence and a React/Expo client would add without implementing them by default.

Verify: give a short walkthrough of a real function from this app, including one failure path, without relying on a line-by-line explanation from me.

## Session rhythm

1. Read a short explanation tied to the next task, starting from a familiar TypeScript/Next.js/React/Express concept when useful and identifying where Rust differs.
2. Predict what a small piece of code will do.
3. Write or change it yourself.
4. Run the smallest relevant check and share the result.
5. Explain what happened; update only the completed checkboxes.

When you are stuck, bring the exact compiler message and the relevant function. We will unpack the message before changing the code. Passing a check alone does not mean a concept is understood.

## References, used as needed

- [Rust Book: Hello, Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Rust Book: ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Book: error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Axum documentation](https://docs.rs/axum/latest/axum/) for the HTTP milestone

Read the section needed for the exercise. Reading the entire book first is unnecessary.
