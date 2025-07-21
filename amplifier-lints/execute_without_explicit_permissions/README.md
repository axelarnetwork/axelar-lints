### What it does

Checks whether an `execute` entry point (currently any `execute` function in `contract.rs`) calls a match on `msg.ensure_permissions` within the body.

### Why is this bad?

Without checking permissions, it is unsafe.

### Known problems

This lint works upon these assumptions:
- The last parameter of the `execute` function is the `msg` object upon which `ensure_permissions` acts on.

This lint may generate false positives otherwise.

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
