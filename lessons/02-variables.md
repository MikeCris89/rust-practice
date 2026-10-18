Goal: represent one mock detection with simple variables and print its details. Keep working in `src/main.rs`; structs come after this exercise.

## Review of lesson 1

`main` is the entry function for this executable. It currently prints text and exits. Later it can start an HTTP server, just as a Node.js startup script can call Express's `app.listen`. Cargo's default binary layout puts it in `src/main.rs`; that path is a convention and can be configured differently.

The `!` in `println!` marks a macro invocation. A macro expands into Rust code during compilation. A regular function call, such as `main()`, has no `!`. You only need to recognize and use macros for now; writing them is outside this exercise.

Formatting is a feature of `println!`, not the meaning of `!` itself. For comparison:

```ts
const site = "North Marsh";
console.log(`Site: ${site}`);
```

```rust
let site = "North Marsh";
println!("Site: {site}");
```

Rust uses `{site}` inside this formatting macro where JavaScript uses `${site}` in a template literal. An ordinary Rust string containing `{site}` does not interpolate on its own.

Cargo combines dependency management and built-in project commands, so npm is a useful comparison. `Cargo.toml` plays a role similar to `package.json`; `Cargo.lock` records resolved dependency versions, like an npm or pnpm lockfile. `rustc` is the compiler Cargo invokes.

## Variables and types

Rust's `let` creates an immutable binding by default. For reassignment, this is closer to TypeScript's `const` than its `let`; mutable Rust bindings require `let mut`. Ownership and mutation of more complex values need their own explanation later.

Both languages can infer types. Rust also lets you annotate them with a colon:

```ts
const imageCount: number = 3;
```

```rust
let image_count: u32 = 3;
```

Rust commonly uses `snake_case` for variables. It also distinguishes numeric types: `u8` is an unsigned 8-bit integer, with values from 0 through 255. Our confidence rule of 0 through 100 will need separate validation later; the type alone does not enforce it.

For now, a quoted species name can be assigned directly to a variable. We will unpack its inferred `&str` type when we reach borrowing.

## Your task

Keep your two startup lines. Inside `main`, add three immutable variables:

- `species`: the string `Japanese knotweed`; let Rust infer its type.
- `confidence`: explicitly typed as `u8`, with the value 92.
- `confirmed`: explicitly typed as `bool`, with the value `false`.

Use those variables in formatted output to add these lines:

```text
Species: Japanese knotweed
Confidence: 92%
Confirmed: false
```

Do not hardcode the values again inside the print statements. No functions, structs, dependencies, or server are needed yet.

Verify with `cargo run` and `cargo fmt --check`. Share your result, plus your prediction: could you assign 300 to `confidence` without changing its type? Why?

## Progress

Implementation complete: the variables and formatted output, including the `%` suffix, have been reviewed in the source. You reported that the output is good. Continue with [lesson 3: functions and return values](03-functions.md); verification details and concepts to revisit are recorded in [the learning plan](../LEARNING_PLAN.md).
