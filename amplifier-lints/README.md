# Amplifier Lints

Lints for the `axelar-amplifier` repository.

## Current Lints

| Lint | Description |
| --------------------------------------------------------- | ----------------------------------------------------------------- |
| [`warn_on_unwraps`](./amplifier-lints/warn_on_unwraps) | Warns on any unwraps used in production code. |
| [`ensure_msg_has_permissions`](./ensure_msg_has_permissions) | Warns on any ExecuteMsg that doesn't derive Permissions. |
| [`restrict_cosmwasm_addr_in_msg_struct`](./restrict_cosmwasm_addr_in_msg_struct/) | Warns on any msg type with a field of type `cosmwasm_std::Addr` |

- TODO: add tests to lints
