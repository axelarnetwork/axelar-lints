struct Msg {}

impl Msg {
    pub fn ensure_permissions(&self) {}
}

fn execute(msg: Msg) {
    let _ = msg.ensure_permissions();
}

fn main() {}
