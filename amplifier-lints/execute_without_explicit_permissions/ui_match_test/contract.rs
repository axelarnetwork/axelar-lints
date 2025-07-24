struct Msg {}

impl Msg {
    pub fn ensure_permissions(&self) {}
}

// expected result: not linted
// - msg.ensure_permissions() called by a match
fn execute(msg: Msg) {
    match msg.ensure_permissions() {
        _ => {}
    }
}

fn main() {}
