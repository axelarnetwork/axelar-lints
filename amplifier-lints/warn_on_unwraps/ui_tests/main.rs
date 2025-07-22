// expected result: linted
// - option unwraps should be linted
fn test_option_unwrap() {
    let _ = Some(42).unwrap();
}

// expected result: linted
// - result unwraps should be linted
fn test_result_unwrap() {
    let _ = Result::<i32, ()>::Ok(42).unwrap();
}

// expected result: not linted
// - unwrap_or_else should not be linted
fn test_unwrap_or_else() {
    let _ = Some(42).unwrap_or_else(|| 0);
}

// expected result: not linted
// - expect should not be linted
fn test_expect() {
    let _ = Some(42).expect("Expected a value");
}

fn main() {}
