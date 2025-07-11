### What it does

Checks whether an `ExecuteMsg`, `QueryMsg`, `InstantiateMsg`, or `MigrateMsg` has a `cosmwasm_std::Addr` type in their fields.

### Why is this bad?

This is a validated type, but when the msg gets deserialized, the field is populated without that validation, so you could potentially introduce data that bypassed expected checks.

### Known problems

### Example

```rust
use cosmwasm_std::Addr;

enum ExecuteMsg {
    Addr(Addr),
    // other variants
}

struct QueryMsg {
    addr: cosmwasm_std::Addr,
    // other fields
}
```

Use instead:

```rust
use cosmwasm_std::Addr;

enum ExecuteMsg {
    // other variants
}

struct QueryMsg {
    // other fields
}
```
