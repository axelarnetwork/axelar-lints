#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use rustc_hir::{Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty::AssocKind;
use rustc_span::symbol::Ident;

dylint_linting::declare_late_lint! {
    pub MSG_WITHOUT_EXPLICIT_PERMISSIONS,
    Warn,
    "warns if any ExecuteMsg has no `#[derive(Permissions)]`"
}

impl<'tcx> LateLintPass<'tcx> for MsgWithoutExplicitPermissions {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if let ItemKind::Enum(..) = item.kind {
            let ident = cx.tcx.item_ident(item.owner_id.into());
            if ident.name.as_str() != "ExecuteMsg" {
                return;
            }

            let impls = cx.tcx.inherent_impls(item.owner_id.to_def_id());

            for impl_id in impls {
                let associated_items = cx.tcx.associated_items(impl_id);
                if associated_items
                    .find_by_name_and_kind(
                        cx.tcx,
                        Ident::from_str("ensure_permissions"),
                        AssocKind::Fn,
                        item.owner_id.to_def_id(),
                    )
                    .is_some()
                {
                    return;
                }
            }

            cx.span_lint(MSG_WITHOUT_EXPLICIT_PERMISSIONS, item.span, |diag| {
                diag.primary_message("all ExecuteMsg enums should derive `Permissions`");
            });
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui_tests");
}
