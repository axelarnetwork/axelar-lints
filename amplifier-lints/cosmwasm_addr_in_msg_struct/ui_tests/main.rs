mod cosmwasm_std {
    #[derive(Debug, Clone)]
    pub struct Addr(pub String);
}

use cosmwasm_std::Addr;

enum ExecuteMsg {
    Addr(Addr),
    // other variants
}

struct InstantiateMsg {
    addr: cosmwasm_std::Addr,
    // other fields
}

fn main() {}
