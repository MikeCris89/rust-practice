# Lesson 4: group fields in a struct

Goal: represent your detection as one value with named fields, keeping the same behavior and output. Keep everything in `src/main.rs`.

## Start from TypeScript

In TypeScript, you can describe an object and then create one:

type Sensor = {
label: string;
active: boolean;
};

const sensor: Sensor = {
label: "North Marsh",
active: true,
};

console.log(sensor.label);

````

In Rust, a struct defines a type with named fields:

```rust
struct Sensor {
    label: String,
    active: bool,
}

fn main() {
    let sensor = Sensor {
        label: String::from("North Marsh"),
        active: true,
    };

    println!("Label: {}", sensor.label);
}
````

- Define the struct outside `main`. Type names conventionally use `PascalCase`.
- Give every field a name and type, separated by `:`. Fields use commas.
- `Sensor { ... }` constructs a value of that type; no `new` keyword is needed.
- Read a field with a dot, as in `sensor.label`.
- In `println!`, `{}` takes the next argument. To print a field, use `println!("Label: {}", sensor.label)`; `{sensor.label}` is not supported inside the format string.

The analogy has a limit: TypeScript generally accepts objects based on their shape. Rust structs are distinct named types. Two different struct types with identical fields are still different types.

## Why `String` here?

Your existing quoted species name is a `&str`, a borrowed view of text. Use `String` for the new struct's species field so the detection owns its text.

`String::from("North Marsh")` creates an owned `String` from a string literal. Here `::` selects the `from` function associated with `String`; it is not field access.

Unlike JavaScript's garbage collection, Rust uses ownership rules to determine when owned data is released. We will practice moves and borrowing later. For this exercise, construct the text directly in the struct and read its field through `println!`; no cloning or lifetime annotations are needed.

## Your first exercise

Define a struct named `Detection` outside `main`, with these three fields:

| Field        | Type     |
| ------------ | -------- |
| `species`    | `String` |
| `confidence` | `u8`     |
| `confirmed`  | `bool`   |

Leave `main` and `is_high_confidence` as they are for this first step. Share your struct for review before continuing. Success means its fields have the required names and types. If you run the program now, an unused-struct warning is expected until you construct a detection.

## After review: use your struct

Replace the three separate bindings in `main` with one `detection` binding containing a `Detection` value. Keep the same values: Japanese knotweed, 92, and false. Use `String::from(...)` for the species.

Update the existing function call to pass `detection.confidence`. Keep `is_high_confidence` unchanged: it still accepts a `u8`, not an entire detection. Update the print statements to read the fields from `detection`, preserving all existing output.

Do not add IDs, coordinates, enums, methods, extra modules, or dependencies yet. High confidence should still be true while human confirmation stays false.

## Verify and explain

Once the struct is in use, run `cargo run`. All six output lines should match your lesson 3 output.

Then run `cargo fmt` if needed and `cargo fmt --check`. `cargo fmt` rewrites formatting; `--check` only checks it, showing differences and exiting unsuccessfully if changes are needed. This is useful in CI because it can enforce formatting without editing files. Neither command checks program behavior.

Share your struct, its construction, and your check results. Answer:

1. What is the difference between defining `Detection` and constructing `Detection { ... }`?
2. Why does the existing helper receive `detection.confidence` rather than `detection`?

Next, after review: [lesson 5: enums and match](05-enums.md).
