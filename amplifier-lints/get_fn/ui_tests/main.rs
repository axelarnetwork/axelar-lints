// expected result: linted
// - is a free function, so should be linted
fn get_some_val() {}

struct Foo {
    val: i32,
}

// expected result: linted
// - is an implementation of a struct, so should be linted
impl Foo {
    fn get_foo(&self) {}
}

// expected result: linted
// - is a defined trait method, so should be linted
trait FooTrait {
    fn get_foo_trait(&self);
}

struct Baz;

// expected result: not linted
// - is an implementation of a trait, so should not be linted
impl FooTrait for Baz {
    fn get_foo_trait(&self) {}
}

fn main() {}
