### What it does

Warns on any `&Option<T>`.

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
