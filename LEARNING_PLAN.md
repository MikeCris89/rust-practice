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

- Lessons 1 through 5 implementation exercises are complete. Rust and Git are initialized; there are no third-party dependencies.
- `src/main.rs` defines `Detections` with `species: String`, `confidence: u8`, and `status: ReviewStatus`. The enum has Pending, Confirmed, and Dismissed variants. `main` constructs one pending detection, calls `is_high_confidence(detection.confidence)`, matches every status variant to a string label, and prints the fields and label. This source has been reviewed; confidence remains 92. The usual name for a type representing one detection would be singular `Detection`; no rename was made by the tutor.
- You reported all lesson 3 checks passing, including formatting after the earlier failure. You said lesson 4 is done, but have not supplied its run/formatting results or answers to its two code-reading questions. The tutor reviewed the source without running it. Struct implementation is demonstrated; the lesson 4 verification and explanation remain to be confirmed.
- You identified the return type and explained that a semicolon would cause the body to return `()` instead of the comparison value. Local type inference follows the initializer's type: `is_high` is a `bool` because the called function declares `-> bool`, regardless of whether it returns true or false.
- You said lesson 5 is done. The tutor reviewed its implementation and ran `cargo check --bin fieldwatch` and `cargo fmt --check` successfully before committing. The compiler warns that Confirmed and Dismissed are never constructed; all match arms are present. Runtime/status-case results remain unreported and were not rerun by the tutor.
- You correctly predicted a compile-time error for a missing match arm and identified the incompatible types when one arm returns an integer instead of a string. We clarified that `ReviewStatus::Pending` is already a complete enum value, while `match` computes a separate label without changing the status. You reported understanding this explanation. `Display`, `Debug`, `#[derive(Debug)]`, and formatting versus value conversion were explained; no formatting traits have been implemented or practiced.
- Next lesson: [lesson 6: collections and borrowed iteration](lessons/06-collections.md). Its document is ready; no collection or loop has been implemented. Source enums remain a later step.

## Handoff for the next chat

- Resume at lesson 6's first exercise: let the user place the existing detection plus two new values in a vector and print its length. Review that step before introducing borrowed loops. Explain moves and shared references using the lesson's examples before asking the user to apply them. Do not implement the exercise or add later milestones.
- The user requests harder, Rust-specific questions rather than conventional coding/design questions. Favor reasoning about compiler checks, expression types, exhaustive matches, and later ownership/borrowing; keep questions within explained concepts. Lesson 4's verification results and code-reading answers remain unreported; do not turn them into a generic recap that blocks the current question.
- Use TypeScript/Next.js/React/Express comparisons by default. The user asks short conceptual questions as they work; explain unfamiliar syntax before expecting them to use it.
- Concepts already discussed: entry points, Cargo versus npm, macros (`println!`, `vec!`), `format!` versus JS template literals, immutable bindings, type annotations, semicolons and final expressions, and a high-level introduction to ownership versus garbage collection. Exposure does not imply mastery; ownership has not been practiced.
- Structs versus TypeScript types/classes, `String` versus `&str`, `impl`/`&self` methods, and top-level definitions versus modules were explained. Struct construction and field access are demonstrated in source; methods, modules, moves, and explicit borrows have not been practiced.
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
- [x] Introduce a function with typed parameters and a return value (lesson 3); implementation and explanation reviewed, passing checks reported by the user.
- [x] Group a detection's fields in a struct (lesson 4); implementation reviewed, check results and code-reading answers pending.
- [ ] Add source/status enums and three mock detections in a `Vec`.
- [x] Use `match` to describe review status (lesson 5); implementation reviewed, enum-value versus label explanation acknowledged, compile and formatting checks observed; runtime/status-case checks unreported.
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
