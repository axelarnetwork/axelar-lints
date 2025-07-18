### What it does

Checks whether an `execute` entry point (currently any `execute` function in `contract.rs`) matches on `ensure_permissions` within the body.

### Why is this bad?

Without checking permissions, it is unsafe.

### Known problems

### Example

```rust
// in a contract.rs file
pub fn execute(..) {
    match msg {
        ..
    }
}
```

Use instead:

```rust
// in a contract.rs file
pub fn execute(..) {
    match msg.ensure_permissions(..)? {
        ..
    }
}```
