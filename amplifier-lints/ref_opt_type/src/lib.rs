#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use rustc_hir::{def::Res, AmbigArg, QPath, Ty, TyKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};

dylint_linting::declare_late_lint! {
    pub REF_OPT_TYPE,
    Warn,
    "avoid using `&Option<T>`; use `Option<&T>` instead"
}

impl<'tcx> LateLintPass<'tcx> for RefOptType {
    fn check_ty(&mut self, cx: &LateContext<'tcx>, ty: &'tcx Ty<'tcx, AmbigArg>) {
        if let TyKind::Ref(_, inner_ty) = ty.kind
            && let TyKind::Path(QPath::Resolved(_, path)) = inner_ty.ty.kind
            && let Res::Def(_, def_id) = path.res
        {
            if Some(def_id) == cx.tcx.lang_items().option_type() {
                cx.span_lint(REF_OPT_TYPE, ty.span, |diag| {
                    diag.primary_message("use `Option<&T>` instead");
                });
            }
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
