### What it does

Checks whether an `ExecuteMsg` enum is annotated with `#[derive(Permissions)]`.

## How it works

Checks all `enum` definitions, checking if it's named `ExecuteMsg`. If so, search for the `impl` function `ensure_permissions`, since the `#[derive(Permissions)]` macro expands to that function.

### Why is this bad?

Without deriving `Permissions`, permission checks can be skipped for `ExecuteMsg` -- unsafe.

### Known problems

### Example

```rust
#[cw_serde]
enum ExecuteMsg {
    ...
}
```

Use instead:

```rust
#[cw_serde]
#[derive(Permissions)]
enum ExecuteMsg {
    ...
}
```
