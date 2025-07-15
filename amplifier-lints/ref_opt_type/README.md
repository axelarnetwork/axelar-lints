### What it does

Warns on any `&Option<T>`.

### Why is this bad?

`Option<&T>` is clearer and can be used easier.

### Known problems

### Example

```rust
fn opt_test(x: &Option<u8>) {}
```

Use instead:

```rust
fn opt_test(x: Option<&u8>) {}
```
