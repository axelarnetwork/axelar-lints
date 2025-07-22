mod cosmwasm_std {
    pub struct Addr;
}

use cosmwasm_std::Addr;

// expected result: linted
// - is a struct with a cosmwasm::Addr field
struct ExecuteMsg {
    addr: cosmwasm_std::Addr,
}

// expected result: not linted
// - is a struct with no cosmwasm::Addr field
struct QueryMsg {}

fn main() {}
