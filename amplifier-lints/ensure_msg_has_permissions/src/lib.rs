#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use rustc_hir::{Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::symbol::Ident;
use rustc_middle::ty::{AssocKind};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    ///
    /// Remove if none.
    ///
    /// ### Example
    ///
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub ENSURE_MSG_HAS_PERMISSIONS,
    Warn,
    "warns if any ExecuteMsg has no `#[derive(EnsurePermissions)]`"
}

impl<'tcx> LateLintPass<'tcx> for EnsureMsgHasPermissions {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        match item.kind {
            ItemKind::Enum(..) => {
                let ident = cx.tcx.item_ident(item.owner_id.into());
                if ident.name.as_str() != "ExecuteMsg" {
                    return;
                }

                let impls = cx.tcx.inherent_impls(item.owner_id.to_def_id());

                for impl_id in impls {
                    let associated_items = cx.tcx.associated_items(impl_id);
                    if let Some(_) = associated_items.find_by_name_and_kind(cx.tcx, Ident::from_str("ensure_permissions"), AssocKind::Fn, item.owner_id.to_def_id()) {
                        return;
                    }
                }

                cx.span_lint(ENSURE_MSG_HAS_PERMISSIONS, item.span, |diag| {
                    diag.primary_message("all ExecuteMsg enums should derive `EnsurePermissions`");
                });
            }
            _ => {}
        }
    }
}
