# Amplifier Lints

Lints for the `axelar-amplifier` repository.

## Current Lints

| Lint                                                          | Description                                                       |
| ------------------------------------------------------------- | ----------------------------------------------------------------- |
| [`warn_on_unwraps`](./amplifier-lints/warn_on_unwraps)        | Warns on any unwraps used in production code.                     |
| [`ensure_msg_has_permissions`](./ensure_msg_has_permissions)  | Warns on any ExecuteMsg that doesn't derive EnsurePermissions.    |

- TODO: add tests to lints
