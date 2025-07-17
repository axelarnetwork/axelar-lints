### What it does

Warns on any functions that start with `get_`.

### Why is this bad?

It's idiomatic in Rust to omit the `get_` prefix for getter functions.

### Known problems

### Example

```rust
fn get_some_val() {
    // get some val
}

impl Foo {
    fn get_foo(&self) {
        // get some foo val
    }
}

trait FooTrait {
    fn get_foo_trait(&self);
}
```

Use instead:

```rust
fn some_val() {
    // get some val
}

impl Foo {
    fn foo(&self) {
        // get some foo val
    }
}

trait FooTrait {
    fn foo_trait(&self);
}
```
