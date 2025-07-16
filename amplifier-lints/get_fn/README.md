### What it does

Warns on any functions that start with `get_`.

### Why is this bad?

It's idiomatic in Rust to omit the `get_` prefix for getter functions.

### Known problems

### Example

```rust
fn get_some_val() {
    // get some val
}
```

Use instead:

```rust
fn some_val() {
    // get some val
}
```
