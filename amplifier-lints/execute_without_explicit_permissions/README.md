### What it does

Checks whether an `execute` entry point calls a match on `msg.ensure_permissions` within the body.

### How it works

Checks all function declarations. If the function is named `execute`, and it resides in a `contract.rs` file, we begin a set of checks.
- If the function has no parameters, then do not warn (trivial pass).
- Otherwise, ensure that the last parameter (expected to be the one which we call `ensure_permissions` on) is a binding such that we can call `ensure_permissions` on it in the function body.
- Checks the body by walking the HIR, searching for the `ensure_permissions` `MethodCall` that is called on the last parameter. If not found, warn on the function.

### Why is this bad?

Without checking permissions, it is unsafe.

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
