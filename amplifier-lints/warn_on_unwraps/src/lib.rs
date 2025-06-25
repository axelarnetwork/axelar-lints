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
    /// ### What it does
    ///
    /// Emits a warning whenever `unwrap()` is called on an `Option` or `Result` type.
    ///
    /// ### Why is this bad?
    ///
    /// Using `unwrap()` can cause a panic at runtime if the value is `None` or `Err`.
    ///
    /// ### Known problems
    ///
    /// ### Example
    ///
    /// ```rust
    /// let value: Option<i32> = None;
    /// let x = value.unwrap();
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// // still panics, but with context
    /// let value: Option<i32> = None;
    /// let x = value.expect("Expected a value"); 
    ///
    /// // can use safer alternative:
    /// let x = value.unwrap_or(0); 
    ///
    /// // or use pattern matching if let
    //// ```
    pub WARN_ON_UNWRAPS,
    Warn,
    "warns if `Option::unwrap` or `Result::unwrap` is called"
}

impl<'tcx> LateLintPass<'tcx> for WarnOnUnwraps {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        match expr.kind {
            ExprKind::MethodCall(name, recv, _args, span) => {
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
                cx.span_lint(WARN_ON_UNWRAPS, span, |diag| {
                    diag.primary_message("avoid using `unwrap` if possible");
                });
            }
            _ => {}
        }
    }
}
