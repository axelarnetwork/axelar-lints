### What it does
    
Emits a warning whenever `unwrap()` is called on an `Option` or `Result` type.

### Why is this bad?

Using `unwrap()` can cause a panic at runtime if the value is `None` or `Err`.

### Known problems

### Example

```rust
let value: Option<i32> = None;
let x = value.unwrap();
```

Use instead:

```rust
// still panics, but with context
let value: Option<i32> = None;
let x = value.expect("Expected a value"); 

// can use safer alternative:
let x = value.unwrap_or(0); 

// or use pattern matching if let
```