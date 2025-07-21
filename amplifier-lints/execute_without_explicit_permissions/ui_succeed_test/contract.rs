struct Msg {}

impl Msg {
    pub fn ensure_permissions(&self) {}
}

fn execute(msg: Msg) {
    match msg.ensure_permissions() {
        _ => {}
    }
}

fn main() {}
