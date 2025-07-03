### What it does

Warns on any functions that start with `get_`.

### Why is this bad?

It's convention to install call the function what you're trying to get directly.

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
