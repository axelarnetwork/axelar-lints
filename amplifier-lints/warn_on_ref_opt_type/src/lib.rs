#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_lexer;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_parse;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;

use std::option;

use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_hir::{Ty, AmbigArg, TyKind, QPath, def::Res};
use rustc_middle::ty;
use rustc_span::sym;

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
