fn test_option_unwrap() {
    let _ = Some(42).unwrap();
}

fn test_result_unwrap() {
    let _ = Result::<i32, ()>::Ok(42).unwrap();
}

fn test_unwrap_or_else() {
    let _ = Some(42).unwrap_or_else(|| 0);
}

fn test_expect() {
    let _ = Some(42).expect("Expected a value");
}

fn main() {}
