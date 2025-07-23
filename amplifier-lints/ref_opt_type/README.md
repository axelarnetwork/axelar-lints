### What it does

Warns on any `&Option<T>`.

### How it works

Checks all type definitions, searching for a `Ref` type containing an `Option` type.

### Why is this bad?

`Option<&T>` is more flexible, provides better pattern matching capabilities, and allows more compiler optimizations.

### Known problems

### Example

```rust
fn opt_test(x: &Option<u8>) {}
```

Use instead:

```rust
fn opt_test(x: Option<&u8>) {}
```
