#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use rustc_ast::{NodeId, visit::FnKind};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_span::Span;

dylint_linting::declare_early_lint! {
    pub GET_FN,
    Warn,
    "no functions should start with `get_`"
}

impl EarlyLintPass for GetFn {
    fn check_fn(&mut self, cx: &EarlyContext, fn_kind: FnKind, span: Span, _: NodeId) {
        if let FnKind::Fn(_, _, fn_data) = fn_kind {
            let name = fn_data.ident.name.as_str();
            if name.starts_with("get_") {
                cx.span_lint(GET_FN, span, |diag| {
                    diag.primary_message("getter functions should omit the `get_` prefix");
                });
            }
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
