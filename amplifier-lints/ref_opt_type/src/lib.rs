#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{AmbigArg, GenericArg, Path, QPath, Ty, TyKind, def::Res};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::Span;

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
            && Some(def_id) == cx.tcx.lang_items().option_type()
        {
            let inner_snippet = get_snippet_or_default(cx, path);

            cx.span_lint(REF_OPT_TYPE, ty.span, |diag| {
                diag.primary_message(format!("use `Option<&{}>` instead", inner_snippet));
            });
        }
    }
}

fn get_snippet_or_default(cx: &LateContext<'_>, path: &Path) -> String {
    if let Some(seg) = path.segments.last()
        && let Some(gen_args) = seg.args
        && let Some(GenericArg::Type(opt_inner_ty)) = gen_args.args.last()
    {
        return get_snippet(cx, opt_inner_ty.span).unwrap_or_else(|| "T".to_string());
    }

    "T".to_string()
}

fn get_snippet(cx: &LateContext<'_>, span: Span) -> Option<String> {
    let sess = cx.tcx.sess;
    sess.source_map().span_to_snippet(span).ok()
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui_tests");
}
