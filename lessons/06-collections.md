# Lesson 6: collections and borrowed iteration

Goal: store three detections in a `Vec` and read them twice without rebuilding or cloning the collection. Keep everything in `src/main.rs`.

In TypeScript, you might write:

```ts
const names: Array<string> = ["North Marsh", "South Marsh"];

for (const name of names) {
	console.log(name);
}

console.log(names.length);
```

Rust's `Vec<T>` is a growable collection whose elements all have type `T`. The angle brackets play a similar role to `Array<string>`: `Vec<String>` holds owned strings, and `Vec<Detections>` holds values of your existing struct type.

```rust
let names: Vec<String> = vec![
    String::from("North Marsh"),
    String::from("South Marsh"),
];

for name in &names {
    println!("{name}");
}

println!("Count: {}", names.len());
```

`vec![...]` is a macro that constructs a vector from its elements. Rust can infer the vector's element type from those values, so the explicit `: Vec<String>` above is optional. `.len()` returns the number of elements.

## Ownership changes the loop

The TypeScript analogy stops at ownership. A JavaScript `for...of` loop leaves the array available. In Rust, these two forms make different choices:

| Loop                 | What it does                                                                                 |
| -------------------- | -------------------------------------------------------------------------------------------- |
| `for name in &names` | Borrows the vector and yields a shared reference to each element                             |
| `for name in names`  | Consumes the vector and yields owned elements; the original binding cannot be used afterward |

`&names` means shared borrowed access, without transferring ownership. In the borrowed loop, `name` has type `&String`. Once the loop's borrow ends, you can borrow the vector again or call `.len()`.

By contrast, `vec![existing_value]` moves an owned `String` or your detection struct into the vector. The old binding cannot then be used as though it still owned that value. JavaScript assigning another reference to an object is not the same operation: Rust checks ownership at compile time rather than relying on garbage collection.

With `for detection in &detections`, the loop variable has type `&Detections`. Rust lets you access fields through that reference using normal dot syntax. You do not need to write an explicit dereference here.

Reading `detection.confidence` produces a copied `u8`: this small numeric type supports `Copy`, so reading it does not transfer ownership out of the struct. A `String` does not support `Copy`. `println!("{}", detection.species)` borrows the text for formatting; assigning `let species = detection.species` would instead try to move the owned text out. A shared reference does not allow that move.

Your existing status match only checks variants without extracting owned data, so you can keep using it inside the borrowed loop. Enums with data need more care, which we will explore later.

## Your first exercise

Keep your current single-detection code and output for now. After those print statements, but still inside `main`, create a `detections` binding with a vector containing:

| Species           | Confidence | Status  |
| ----------------- | ---------- | ------- |
| Japanese knotweed | 92         | Pending |
| Giant hogweed     | 80         | Pending |
| Himalayan balsam  | 55         | Pending |

Use your existing `detection` value as the first element; construct the other two using your existing `Detections` type. Use `String::from(...)` for their species names. Let Rust infer the vector type.

After constructing the vector, print its `.len()` with the label `Total detections`. Do not use the old `detection` binding after moving it into the vector.

Run `cargo run`. Success for this first step is your previous output followed by `Total detections: 3`. Share the vector construction and result before continuing.

## After review: read the collection twice

Move the `is_high` and `status_label` calculations and the four per-detection print statements into a loop after vector construction. Use `for detection in &detections`. Keep the startup messages outside the loop and keep the existing helper, enum, and struct unchanged.

Each iteration should print species, confidence, high-confidence result, and review-status label for that detection. Calculate the result and label inside the loop so they correspond to the current detection.

After that loop, add a second borrowed loop that prints just each species name. Finally print `Total detections: 3` using `.len()`. Use the same vector throughout, with no `.clone()` or replacement collection. The second loop is a small exercise in repeated access; no printing helper is needed yet.

Do not add source enums, filtering, mutation, new fields, dependencies, or modules in this lesson.

## Verify and explain

Run `cargo run` and check:

- The first loop prints all three detections in the table's order, with confidence scores 92, 80, and 55.
- Their high-confidence results are true, true, and false; all review-status labels are Pending.
- The second loop prints the same three species in the same order.
- The final count is 3, demonstrating that the vector remains available after both loops.

Then run `cargo fmt` if needed and `cargo fmt --check`. No automated test is needed for this printing exercise. You may still see the existing warning that Confirmed and Dismissed are never constructed: matching a variant does not construct a value of it. Leave the variants and warning in place.

Share your loops and check results. Predict these changes before trying them:

1. If you removed `&` from only the first loop, which later uses would fail to compile, and what was transferred?
2. Inside the borrowed loop, why can `let confidence = detection.confidence` compile while `let species = detection.species` cannot? Name the types involved and connect the difference to `Copy` and borrowed access.

Current handoff: the vector-construction step is complete; borrowed loops are still pending. Continue with [lesson 7: ownership practice](07-ownership-practice.md) for smaller ownership exercises and guided completion of those loops.
