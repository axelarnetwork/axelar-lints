struct Msg {}

impl Msg {
    pub fn ensure_permissions(&self) {}
}

// expected result: not linted
// - msg.ensure_permissions() called by a let binding
fn execute(msg: Msg) {
    let _ = msg.ensure_permissions();
}

fn main() {}
