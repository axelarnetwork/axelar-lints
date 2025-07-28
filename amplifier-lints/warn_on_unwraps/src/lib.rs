#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty;
use rustc_span::sym;

dylint_linting::declare_late_lint! {
    pub UNWRAPS_OUTSIDE_TESTS,
    Warn,
    "warns if `Option::unwrap` or `Result::unwrap` is called"
}

impl<'tcx> LateLintPass<'tcx> for UnwrapsOutsideTests {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::MethodCall(name, recv, _args, span) = expr.kind {
            if name.ident.as_str() != "unwrap" {
                return;
            }
            let caller_ty = cx.typeck_results().expr_ty(recv);
            let is_option_or_result = match caller_ty.kind() {
                ty::Adt(adt, _) => {
                    cx.tcx.is_diagnostic_item(sym::Option, adt.did())
                        || cx.tcx.is_diagnostic_item(sym::Result, adt.did())
                }
                _ => false,
            };
            if !is_option_or_result {
                return;
            }

            let mut current_id = expr.hir_id;
            let mut automatically_derived = cx
                .tcx
                .is_automatically_derived(current_id.owner.to_def_id());

            while let Some(parent) = cx.tcx.hir_get_enclosing_scope(current_id) {
                current_id = parent;
                if cx
                    .tcx
                    .is_automatically_derived(current_id.owner.to_def_id())
                {
                    automatically_derived = true;
                }
            }

            if automatically_derived {
                return;
            }

            cx.span_lint(UNWRAPS_OUTSIDE_TESTS, span, |diag| {
                diag.primary_message("avoid using `unwrap` if possible");
            });
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui_tests");
}
