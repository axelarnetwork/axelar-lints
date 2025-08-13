# Amplifier Lints

Lints for the `axelar-amplifier` repository.

## Current Lints

| Lint | Description |
|------|-------------|
| [`cosmwasm_addr_in_msg_struct`](./cosmwasm_addr_in_msg_struct) | Warns on any msg type with a field of type `cosmwasm_std::Addr` |
| [`execute_without_explicit_permissions`](./execute_without_explicit_permissions/) | Warns on any `execute` entry point that doesn't call `ensure_permissions`. |
| [`get_fn`](./amplifier-lints/get_fn) | Warns on any function beginning with `get_`. |
| [`msg_without_explicit_permissions`](./msg_without_explicit_permissions) | Warns on any ExecuteMsg that doesn't derive Permissions. |
| [`ref_opt_type`](./amplifier-lints/ref_opt_type) | Warns on any `&Option<T>`; use `Option<&T>` instead. |
| [`unwraps_outside_tests`](./amplifier-lints/unwraps_outside_tests) | Warns on any unwraps used in production code. |

- TODO: add tests to lints
