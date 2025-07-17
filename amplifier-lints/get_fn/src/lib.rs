#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{ImplItem, ImplItemKind, Item, ItemKind, TraitItem, TraitItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::Ident;

dylint_linting::declare_late_lint! {
    pub GET_FN,
    Warn,
    "no functions should start with `get_`"
}

impl<'tcx> LateLintPass<'tcx> for GetFn {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if let ItemKind::Fn { ident, .. } = item.kind {
            check_fn_name(cx, ident);
        }
    }

    fn check_impl_item(&mut self, cx: &LateContext<'tcx>, impl_item: &'tcx ImplItem<'tcx>) {
        let parent_id = cx.tcx.hir_get_parent_item(impl_item.hir_id());
        let parent_item = cx.tcx.hir_expect_item(parent_id.def_id);
        if let ItemKind::Impl(parent_impl_item) = parent_item.kind
            && parent_impl_item.of_trait.is_none()
            && matches!(impl_item.kind, ImplItemKind::Fn(..))
        {
            check_fn_name(cx, impl_item.ident);
        }
    }

    fn check_trait_item(&mut self, cx: &LateContext<'tcx>, trait_item: &'tcx TraitItem<'tcx>) {
        if matches!(trait_item.kind, TraitItemKind::Fn(..)) {
            check_fn_name(cx, trait_item.ident);
        }
    }
}

fn check_fn_name(cx: &LateContext<'_>, ident: Ident) {
    let name = ident.name.as_str();
    if name.starts_with("get_") {
        cx.span_lint(GET_FN, ident.span, |diag| {
            diag.primary_message("getter functions should omit the `get_` prefix");
        });
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
