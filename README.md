# Axelar Lints

Built using the `dylint` library (https://github.com/trailofbits/dylint).

## Setup

### Installing Dylint

```sh
# If this causes issues, try switching to a more recent version of rust.
cargo install cargo-dylint dylint-link
```

## Using Dylint

### Writing lints

Using Dylint, you can either set up lints as standalone libraries, or as a combined library. Currently, I've set up a library named "amplifier_lints" which will hold lints for `axelar-amplifier`.

You can set up a lint with `cargo dylint new new_lint_name`. Your `new_lint_name/src/lib.rs` file will be where you write your lint. 

Combined libraries can contain a number of your lints. They have [a number of requirements for the organization](https://github.com/trailofbits/dylint/blob/master/examples/general/README.md). Follow these requirements to get one working.

Here are some resources for creating lints:
- Dylint has some [good example lints](https://github.com/trailofbits/dylint/tree/master/examples) that can help you get a good feel for how lints work.
- Starting from [LateLintPass in the rust docs](https://doc.rust-lang.org/1.88.0/nightly-rustc/rustc_lint/trait.LateLintPass.html) and searching for functions in the rust docs can help in finding specific functionality for linting.

### Running Dylint on Target Workspaces 

To run a lint/library of lints from this repository, in the target workspace, add

```sh
[workspace.metadata.dylint]

[[workspace.metadata.dylint.libraries]]
git = "https://github.com/axelarnetwork/axelar-lints",
branch = "..", # Optional, for testings axelar-lints branches
pattern = [
    "amplifier-lints",
    ...
]
```
to the `Cargo.toml` file, and run `cargo dylint --all`.

### Conditional Compilation

From what it appears, test code will not be linted by default (correct if wrong). Otherwise, allow a lint to be ignored as such:

```sh
#[cfg_attr(dylint_lib = "LIBRARY_NAME", allow(LINT_NAME))]
```

(e.g. LIBRARY_NAME="amplifier_lints", LINT_NAME="warn_on_unwrap"), or if it is a pre-expansion lint,

```sh
#[allow(unknown_lints)]
#[allow(PRE_EXPANSION_LINT_NAME)]
```

NOTE: name your lint/library with underscores and not dashes in the `Cargo.toml` file, otherwise [conditional compilation](https://github.com/trailofbits/dylint?tab=readme-ov-file#conditional-compilation) may not function properly (fairly certain this is a rust feature? remove this line if unnecessary).

## Credits

- Credit to [Dylint](https://github.com/trailofbits/dylint)
- Credit to GuillaumeGomez on GitHub for [warn_on_unwrap lint](https://github.com/GuillaumeGomez/rustc-tools-example)
