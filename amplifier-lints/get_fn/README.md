### What it does

Warns on any functions that start with `get_`.

### How it works

For free functions:
- Checks the name of the free function to see if it starts with `get_`.

For trait functions:
- Checks the name of the trait function to see if it starts with `get_`.

For impl functions:
- Checks first to see if the parent of the function (i.e. the impl block) is the implementation of a trait. If so, there are two cases: the trait function check will have detected it if we defined the trait, or it is an externally defined trait. Either/or, there is no need to lint the function. Otherwise, proceed as normal and check the name of the impl function to see if it starts with `get_`.


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
