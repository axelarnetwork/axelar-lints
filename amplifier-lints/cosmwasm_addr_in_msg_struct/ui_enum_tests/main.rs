mod cosmwasm_std {
    pub struct Addr;
}

use cosmwasm_std::Addr;

// expected result: linted
// - is an enum with a cosmwasm::Addr variant
enum InstantiateMsg {
    Addr(Addr),
}

// expected result: not linted
// - is an enum with no cosmwasm::Addr variant
enum MigrateMsg {}

fn main() {}
