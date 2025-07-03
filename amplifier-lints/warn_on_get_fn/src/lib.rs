#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use rustc_lint::{EarlyLintPass, EarlyContext, LintContext};
use rustc_ast::{visit::FnKind, NodeId};
use rustc_span::{Span};

dylint_linting::declare_early_lint! {
    pub WARN_ON_GET_FN,
    Warn,
    "no functions should start with `get_`"
}

impl EarlyLintPass for WarnOnGetFn {
    fn check_fn(&mut self, cx: &EarlyContext, fn_kind: FnKind, span: Span, _: NodeId) {
        if let FnKind::Fn(_, _, fn_data) = fn_kind {
            let name = fn_data.ident.name.as_str();
            if name.starts_with("get_") {
                cx.span_lint(WARN_ON_GET_FN, span, |diag| {
                    diag.primary_message("functions should not start with `get_`");
                });
            }
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
