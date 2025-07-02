#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use rustc_hir::def::Res;
use rustc_hir::{FieldDef, Item, ItemKind, QPath, TyKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};

dylint_linting::declare_late_lint! {
    pub RESTRICT_COSMWASM_ADDR_IN_MSG_STRUCT,
    Warn,
    "avoid using cosmwasm::Addr in msg typesi"
}

impl<'tcx> LateLintPass<'tcx> for RestrictCosmwasmAddrInMsgStruct {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        match item.kind {
            ItemKind::Struct(ident, variant_data, _) => {
                if ident.name.as_str() != "ExecuteMsg"
                    && ident.name.as_str() != "InstantiateMsg"
                    && ident.name.as_str() != "QueryMsg"
                    && ident.name.as_str() != "MigrateMsg"
                {
                    return;
                }

                let fields = variant_data.fields();

                check_cosmwasm_addr_in_field(cx, fields).then(|| {
                    cx.span_lint(RESTRICT_COSMWASM_ADDR_IN_MSG_STRUCT, item.span, |diag| {
                        diag.primary_message("no msg types should use cosmwasm::Addr");
                    });
                });
            }
            ItemKind::Enum(ident, enum_def, _) => {
                if ident.name.as_str() != "ExecuteMsg"
                    && ident.name.as_str() != "InstantiateMsg"
                    && ident.name.as_str() != "QueryMsg"
                    && ident.name.as_str() != "MigrateMsg"
                {
                    return;
                }

                let variants = enum_def.variants;

                for variant in variants {
                    let fields = variant.data.fields();

                    check_cosmwasm_addr_in_field(cx, fields).then(|| {
                        cx.span_lint(RESTRICT_COSMWASM_ADDR_IN_MSG_STRUCT, item.span, |diag| {
                            diag.primary_message("no msg types should use cosmwasm::Addr");
                        });
                    });
                }
            }
            _ => {}
        }
    }
}

fn check_cosmwasm_addr_in_field(cx: &LateContext, fields: &[FieldDef]) -> bool {
    for field in fields {
        if let TyKind::Path(QPath::Resolved(_, path)) = field.ty.kind {
            if let Res::Def(_, def_id) = path.res {
                let path_str = cx.tcx.def_path_str(def_id);
                if path_str != "cosmwasm_std::Addr" {
                    continue;
                }

                return true;
            }
        }
    }
    false
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
