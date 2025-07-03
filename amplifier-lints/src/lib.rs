#![feature(rustc_private)]
#![warn(unused_extern_crates)]

dylint_linting::dylint_library!();

extern crate rustc_lint;
extern crate rustc_session;

#[expect(clippy::no_mangle_with_rust_abi)]
#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    warn_on_unwraps::register_lints(sess, lint_store);
    msg_without_explicit_permissions::register_lints(sess, lint_store);
    cosmwasm_addr_in_msg_struct::register_lints(sess, lint_store);
    warn_on_get_fn::register_lints(sess, lint_store);
}
