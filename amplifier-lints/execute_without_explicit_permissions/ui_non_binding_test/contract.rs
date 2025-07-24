struct Msg {}

impl Msg {
    pub fn ensure_permissions(&self) {}
}

// expected result: linted
// - last parameter is expected to be used for permission checks, must be bound
fn execute(_: Msg) {}

fn main() {}
