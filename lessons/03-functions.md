# Lesson 3: functions and return values

Goal: move a confidence decision into a small function and call it from `main`. Keep everything in `src/main.rs`. This lesson introduces typed parameters, return types, and returning a final expression.

## Start from TypeScript

You already know how to put a decision in a function:

```ts
function isEven(value: number): boolean {
	return value % 2 === 0;
}

const even = isEven(4);
```

```rust
fn is_even(value: u8) -> bool {
    value % 2 == 0
}

// This call goes inside main.
let even = is_even(4);
```

Here `%` is the remainder operator in both languages. Compare the function syntax:

| TypeScript                   | Rust                             | Meaning                                                             |
| ---------------------------- | -------------------------------- | ------------------------------------------------------------------- |
| `function`                   | `fn`                             | Declare a function                                                  |
| `value: number`              | `value: u8`                      | A named parameter with a type; Rust's numeric type is more specific |
| `: boolean` after parameters | `-> bool`                        | Declare the return type                                             |
| `return expression;`         | A final `expression` without `;` | Return a value from the function body                               |
| `isEven(4)`                  | `is_even(4)`                     | Call a regular function; no macro `!`                               |

Rust also supports `return expression;`, which is useful for early returns. Practice the final-expression style here because it is common in Rust code.

Adding a semicolon after the final comparison discards that value. The body then returns `()`, Rust's unit type, which will not match a promised `bool` return type. Think of `()` as a rough counterpart to `void` when reading a function signature.

Define your new function at the top level, outside `main`, in the same file. It can appear above or below `main`. Function parameter types are required; Rust can still infer the types of local variables from their values.

## Your task

1. Write a function named `is_high_confidence` that takes a `confidence: u8` parameter and returns `bool`.
2. Return whether the score is at least 80. Use a final expression without a semicolon. Rust uses `>=` for "at least," just like TypeScript.
3. In `main`, call the function with your existing `confidence` variable and store the result in a local variable. Let Rust infer that local variable's type.
4. Keep all existing output and add `High confidence: true` using that result in a `println!` call.

The function should return a value rather than print it. This gives `main` a result it can use, much as an Express handler can call a helper before constructing a response.

Keep `confirmed` as `false`: high model confidence and human confirmation are separate concepts. For now, assume the input is a valid percentage; validation comes later. No structs, additional modules, dependencies, or server are needed.

## Verify and explain

Run `cargo run` and `cargo fmt --check`. With the original confidence of 92, the added line should be `High confidence: true`.

Then manually try confidence 80 and 79, running the program for each. The added line should be `true` for 80 and `false` for 79. Restore the original value of 92 afterward. These three manual checks are enough for this exercise.

Share your function, its call in `main`, and your results. Answer these two questions:

1. What does `-> bool` promise to the caller?
2. What would adding `;` to your function's final comparison change?

Next, after review: [lesson 4: structs](04-structs.md), using a TypeScript object type as the reference point.
