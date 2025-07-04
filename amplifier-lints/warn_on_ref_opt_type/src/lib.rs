#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use rustc_hir::{def::Res, AmbigArg, QPath, Ty, TyKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};

dylint_linting::declare_late_lint! {
    pub WARN_ON_REF_OPT_TYPE,
    Warn,
    "description goes here"
}

impl<'tcx> LateLintPass<'tcx> for WarnOnRefOptType {
    fn check_ty(&mut self, cx: &LateContext<'tcx>, ty: &'tcx Ty<'tcx, AmbigArg>) {
        if let TyKind::Ref(_, inner_ty) = ty.kind {
            if let TyKind::Path(QPath::Resolved(_, path)) = inner_ty.ty.kind {
                if let Res::Def(_, def_id) = path.res {
                    let path_str = cx.tcx.def_path_str(def_id);

                    if path_str == "std::option::Option" {
                        cx.span_lint(WARN_ON_REF_OPT_TYPE, ty.span, |diag| {
                            diag.primary_message("use `Option<&T>` instead");
                        });
                    }
                }
            }
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
