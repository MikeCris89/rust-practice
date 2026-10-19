Goal: replace the confirmation boolean with an explicit review status and use `match` to turn that status into a printed label. Keep everything in `src/main.rs`; you write the application code.

## Why a boolean is no longer enough

`confirmed: false` cannot distinguish a detection waiting for review from one a reviewer dismissed. An enum lets us name all three possibilities, with exactly one selected for each detection.

This status still represents a human decision. A confidence of 92 does not automatically mean confirmed.

## Start from TypeScript

A TypeScript union can describe a fixed set of choices:

```ts
type DeliveryStatus = "queued" | "sent";

const status: DeliveryStatus = "queued";
```

In Rust, define an enum with named variants:

```rust
enum DeliveryStatus {
    Queued,
    Sent,
}
```

Create a value by naming the type and a variant:

```rust
let status = DeliveryStatus::Queued;
```

Here `::` selects a variant belonging to the enum. The type name and variants conventionally use `PascalCase`.

The analogy has a limit: these variants are not strings. They are values of the distinct Rust type `DeliveryStatus`. Rust enums can also carry data, similar to TypeScript discriminated unions of objects, but this lesson uses variants without extra data.

## Turn a variant into text with `match`

In TypeScript, you might use a `switch` to choose a label. Rust's `match` can produce a value directly:

```rust
let label = match status {
    DeliveryStatus::Queued => "Waiting to send",
    DeliveryStatus::Sent => "Sent",
};

println!("Delivery: {label}");
```

- The value after `match` is inspected to select an arm.
- Each arm has a pattern on the left and its result after `=>`.
- Arms use commas; there is no fallthrough and no `break`.
- Every possible variant must be covered. Unlike a typical TypeScript `switch`, Rust enforces this without an extra exhaustiveness-checking pattern.
- The results must have a compatible type. Both results here are string literals, so Rust infers `label` as `&str`.
- The final `;` completes the `let` statement. It does not discard the match result before assigning it to `label`.

We will list every variant explicitly instead of adding a catch-all arm. This lets the compiler point out places that need updating if we later add a variant.

## Your first exercise

Define an enum named `ReviewStatus` outside `main` with three variants:

- `Pending`
- `Confirmed`
- `Dismissed`

Leave your existing struct and `main` unchanged for now. Share the enum for review. Success means the type and all three variants are defined; an unused-enum warning is expected until the next step.

## After review: use the status

In your existing detection struct (currently named `Detections`), replace `confirmed: bool` with `status: ReviewStatus`.

Update the value constructed in `main` to start with `ReviewStatus::Pending`. Keep the species, confidence of 92, and `is_high_confidence` helper unchanged.

Before printing the detection, use a `match` on `detection.status` to assign a local `status_label`. Return the strings `"Pending"`, `"Confirmed"`, and `"Dismissed"` from the corresponding arms, following the delivery example.

Replace the old confirmed output with a line that prints this label:

```text
Review status: Pending
```

Keep the other output. Do not print the enum directly with `{}`: the enum has no display formatting implementation, and choosing readable labels with `match` is the exercise. No traits or derive attributes are needed.

Keep this step to one detection. Source enums, collections, review updates, methods, and additional files come later.

## Verify and explain

Run `cargo run`. With confidence 92 and a pending status, expect both `High Confidence: true` and `Review status: Pending`.

Then manually change the constructed status to `Confirmed` and `Dismissed`, running the program for each. Only the review-status line should change. Restore `Pending` afterward, then run `cargo fmt` if needed and `cargo fmt --check`.

These three manual cases are enough for this lesson. Share your enum, changed struct, match expression, and results. Answer:

1. Why is `ReviewStatus` more useful than `confirmed: bool` for this workflow?
2. What would happen if your match omitted the `Dismissed` arm?

Next, after review: [lesson 6: collections and borrowed iteration](06-collections.md).
