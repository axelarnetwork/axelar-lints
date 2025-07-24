struct Msg {}

impl Msg {
    pub fn ensure_permissions(&self) {}
}

// expected result: linted
// - `unchecked_msg` never calls `ensure_permissions` in the function body
fn execute(unchecked_msg: Msg) {}

fn main() {}
