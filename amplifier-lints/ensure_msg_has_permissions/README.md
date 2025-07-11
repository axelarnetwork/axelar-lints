### What it does

Checks whether an `ExecuteMsg` enum is annotated with `#[derive(Permissions)]`.

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
