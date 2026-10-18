# Lesson 1: make the project yours

Goal: install the Rust tools, run a generated program, and make one small change yourself. Stop after this lesson so we can review it before adding the data model.

## 1. Install Rust

Use the [official Rust installation page](https://rust-lang.org/install.html). On your Mac, its recommended command is:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This downloads and runs the official installer. Follow its prompts for the default stable toolchain, then open a new terminal so PATH changes take effect.

```sh
rustup --version
rustc --version
cargo --version
```

`rustup` manages Rust installations. `rustc` compiles Rust. `cargo` manages a project, its dependencies, and commands such as running and testing it. You will usually use Cargo rather than invoke the compiler directly.

Coming from TypeScript, think of Cargo as combining several jobs from your package manager and build tools. `rustc` overlaps with `tsc` in checking types, but also checks ownership and compiles this application into a native executable rather than emitting JavaScript for Node.js to run.

If a command fails, share the exact output before proceeding. If compilation later reports a missing linker, the [Rust Book installation guide](https://doc.rust-lang.org/book/ch01-01-installation.html) explains macOS developer tools; we will address that only if needed.

## 2. Initialize the application

From this project's folder:

```sh
cd /Users/mikec/Documents/Code/Projects/rust-practice
cargo init --bin --name fieldwatch --vcs git .
cargo run
```

`init` adds a package to the existing folder. `--bin` requests an executable application, `--name` sets its package name, and `--vcs git` initializes Git. It does not make a commit. Keep the planning documents already here.

`cargo init` only creates the project, similar to the scaffolding step of `create-next-app`. After that, `cargo run` compiles and runs it. Expected application output after `cargo run`:

```text
Hello, world!
```

Unlike a typical `npm run dev` in Next.js, this terminal application prints its message and exits; it does not start a development server.

Open these files:

- `Cargo.toml`: the package manifest; compare its package name with the folder name.
- `src/main.rs`: the application entry point.
- `Cargo.lock`: the resolved dependency versions. Keep this file in version control for this application.
- `.gitignore`: check that it excludes `/target`, where Cargo writes generated build output.

Use these familiar reference points while exploring:

| Rust item       | TypeScript/Next.js reference point        | Difference                                                                                                     |
| --------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `Cargo.toml`    | `package.json`                            | Package metadata and dependencies; Cargo supplies built-in commands rather than an npm-style `scripts` section |
| `Cargo.lock`    | `package-lock.json` / `pnpm-lock.yaml`    | Records resolved dependency versions for reproducible dependency resolution                                    |
| `target/`       | `.next/` / `dist/`                        | Generated output, here including compiled native code                                                          |
| `fn main()`     | A Node.js script's top-level startup code | Rust executables start in an explicit `main` function                                                          |
| `println!(...)` | `console.log(...)`                        | Prints to the terminal, but uses a Rust macro and Rust formatting rules                                        |

## 3. Your first edit

Change the program so it prints:

```text
Fieldwatch: invasive species review
Data source: mock detections
```

Use two `println!` calls. Do not add dependencies, data structures, or a server yet.

Then run:

```sh
cargo run
cargo fmt --check
git status --short
```

If the formatting check fails, read its suggested change, run `cargo fmt`, and check again. `git status` lets you inspect newly created files; there is no need to commit yet.

## 4. Bring back your result

Share your `src/main.rs`, the output of `cargo run`, and any failed command output. Also take a guess at these questions; uncertainty is useful here:

1. Why is `main` special?
2. What does the `!` in `println!` suggest? Hint: it is a macro invocation.
3. What job does Cargo do that `rustc` alone does not handle for you?

Next lesson, after review: represent one mock detection and learn why Rust asks you to be explicit about its types.
