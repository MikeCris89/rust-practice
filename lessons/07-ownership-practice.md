# Lesson 7: ownership practice, one case at a time

Goal: explain who owns a value and what access a reference permits, using the vector you just built. This lesson also finishes lesson 6's borrowed-loop exercise. No new application features are needed.

Your starting code prints one detection, moves it into a vector with two more, and prints `Length: 3`. The loops have not been implemented. Work through one checkpoint at a time with the tutor; do not implement the whole document in one pass.

## The three distinctions to practice

| Operation | Meaning |
| --- | --- |
| Move an owned value | Transfer ownership; the previous binding cannot keep using that moved value |
| Copy a `Copy` value | Produce another usable value without invalidating the original |
| Borrow with `&` | Create a shared reference that permits reading without transferring ownership of the referenced value |

For example, assigning a `String` to another binding moves it. Assigning a `u8` copies it. A shared reference such as `&String` is itself a value and is also `Copy`; copying that reference does not copy or transfer the owned text.

In TypeScript, assigning an object to another variable normally gives both variables access to the same object, with garbage collection managing its lifetime. Rust moves and borrows impose different permissions, checked during compilation. The rules do not depend on whether a JavaScript-style garbage collector could keep the data alive.

A move does not necessarily mean physically relocating bytes or allocating again. For now, reason about which binding may use the value; exact memory layout is not needed.

## Checkpoint 1: a copied field and a moved struct

Your existing `vec![detection, ...]` transfers the original detection into the vector. The vector owns that element, including its species `String`.

First predict this experiment in words, before changing code:

1. Immediately before vector construction, save `detection.confidence` in a local `saved_confidence` binding.
2. After vector construction, print `saved_confidence`.
3. Separately try printing `detection.confidence` at that same point.

Which print should compile? Why does the field's type matter when saving it, while moving the whole struct affects the original binding?

Write steps 1 and 2, run `cargo run --bin fieldwatch`, and share the result and explanation. Then, after review, try step 3 with `cargo check --bin fieldwatch`. That is an intentional compiler-error experiment, not a broken implementation to work around. Explain the diagnostic and remove only that experimental invalid print afterward. Keep the valid program as the baseline for the next checkpoint.

Do not add `.clone()`, derive `Copy`, or rebuild the detection to bypass the ownership question.

## Checkpoint 2: a reference is a value too

`let view = &detections` creates a binding containing a shared reference. The inferred type is `&Vec<Detections>`. The vector still belongs to `detections`; `view` gives borrowed access to it.

After vector construction, create that shared reference. Print the collection's length through `view`, then through `detections`. Also create a second shared reference and read through both references. All lengths should be 3.

Using `view.len()` is normal method syntax: Rust can reach the vector through the reference. No explicit dereference is needed for this call. These shared references permit reading, not changing the vector through them.

Before running, explain which binding owns the vector and what value each view binding contains. Run the program and share the relevant lines. Success includes explaining why the original vector binding remains usable; output alone does not establish understanding.

## Checkpoint 3: borrow the collection twice

Now finish lesson 6's loop step. Keep the startup messages outside the loops.

Move your per-detection `is_high` and `status_label` calculations and four detection print statements into a loop after vector construction. Use `for detection in &detections`. Each loop variable has type `&Detections`; the original binding used to construct the first element and this loop variable are separate bindings, even though they have the same name.

Inside the loop, read the fields with dot syntax. Passing `detection.confidence` to your existing helper copies a `u8`. Formatting `detection.species` in `println!` borrows the text. Your match only inspects status variants without extracting owned data, so it can remain as written.

Add a second borrowed loop that prints each species, then print the vector's length after both loops. Expect all three detections at scores 92, 80, and 55, with high-confidence results true, true, and false, and pending status throughout. Expect all three species again and a final length of 3.

After the working version is reviewed, predict what happens if only the first loop uses `detections` instead of `&detections`. Try that single change with `cargo check --bin fieldwatch`, identify which later uses fail, and explain what was consumed. Restore the borrowed loop after the experiment.

Two further readings to discuss: why would `let species = detection.species` fail inside the borrowed loop, and how would `let species = &detection.species` change the type and permissions? Predict first; the tutor can guide one experiment at a time.

## Checkpoint 4: borrow into a function

After the loops are understood, write a small helper with this signature:

```rust
fn print_species(detection: &Detections)
```

Give it a body that prints the species. Use it in the second borrowed loop instead of that loop's inline print. The loop variable is already `&Detections`, so it can be passed directly. Keep the first loop's detailed output intact.

The helper receives borrowed access rather than ownership of the detection. Call the helper twice on the same borrowed detection during one iteration to check that repeated read access works. Then leave one call per iteration in the final version.

Contrast that signature with `fn print_species(detection: Detections)`: the latter requests ownership of a detection. It cannot accept the loop's shared reference as though it were an owned value. Discuss how a call with an owned local detection would differ; do not change the working helper to consume data.

## Verify and hand back

At each checkpoint, use only the small run or compile check requested. For intentional failing experiments, share the diagnostic, explain it, and restore the valid baseline before continuing. Ask for a hint if the explanation is unclear; do not solve it by cloning.

At the end, run `cargo run --bin fieldwatch` and `cargo fmt --check` (use `cargo fmt` first if needed). The same collection must remain readable after both loops and the helper calls. The existing unused-status-variant warning is expected.

For the final review, point to one move, one copied value, one shared reference, and one borrowed function argument in your own code. Name their types and explain what the original binding can do afterward.

Mutable references, borrow conflicts, owned versus borrowed text parameters (`String` and `&str`), and returning references will get separate exercises when introduced. Common shared-borrow and move cases will recur in later lessons; finishing this document does not imply mastery of every ownership scenario.
