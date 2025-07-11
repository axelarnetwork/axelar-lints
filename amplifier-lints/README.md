# Amplifier Lints

Lints for the `axelar-amplifier` repository.

## Current Lints

| Lint | Description |
| - | - |
| [`warn_on_unwraps`](./amplifier-lints/warn_on_unwraps) | Warns on any unwraps used in production code. |
| [`msg_without_explicit_permissions`](./msg_without_explicit_permissions) | Warns on any ExecuteMsg that doesn't derive Permissions. |
| [`cosmwasm_addr_in_msg_struct`](./cosmwasm_addr_in_msg_struct)| Warns on any msg type with a field of type `cosmwasm_std::Addr` |

- TODO: add tests to lints
