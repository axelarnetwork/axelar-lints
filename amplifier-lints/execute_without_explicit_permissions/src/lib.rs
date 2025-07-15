#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{
    intravisit::FnKind,
    intravisit::{self, Visitor},
    Body, ExprKind, FnDecl,
};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{def_id::LocalDefId, FileName, RealFileName, Span};

dylint_linting::declare_late_lint! {
    pub EXECUTE_WITHOUT_EXPLICIT_PERMISSIONS,
    Warn,
    "All contract.rs execute entry points should have explicit permissions checks"
}

impl<'tcx> LateLintPass<'tcx> for ExecuteWithoutExplicitPermissions {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fn_kind: FnKind<'tcx>,
        _fn_decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        _local_def_id: LocalDefId,
    ) {
        if let FnKind::ItemFn(ident, ..) = fn_kind {
            if ident.name.as_str() != "execute" {
                return;
            }

            let is_contract_rs_execute = is_contract_rs_execute(cx, span);

            if !is_contract_rs_execute {
                return;
            }

            let has_permissions_match = has_match_on_permissions(cx, body);

            if !has_permissions_match {
                cx.span_lint(EXECUTE_WITHOUT_EXPLICIT_PERMISSIONS, span, |diag| {
                    diag.primary_message("No permissions check found in execute entry point");
                });
            }
        }
    }
}

fn is_contract_rs_execute(cx: &LateContext<'_>, span: Span) -> bool {
    let source_map = cx.tcx.sess.source_map();
    let file_name = source_map.span_to_filename(span);

    if let FileName::Real(RealFileName::LocalPath(path)) = file_name {
        if path.ends_with("contract.rs") {
            return true;
        }
    }

    false
}

struct MatchVisitor {
    has_match: bool,
}

impl<'tcx> Visitor<'tcx> for MatchVisitor {
    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        if let ExprKind::Match(match_inner, _, _) = expr.kind {
            if let ExprKind::Call(_, args) = match_inner.kind {
                for arg in args {
                    if let ExprKind::MethodCall(path_segment, _, _, _) = arg.kind {
                        if path_segment.ident.name.as_str() == "ensure_permissions" {
                            self.has_match = true;
                            return;
                        }
                    }
                }
            }
        }

        intravisit::walk_expr(self, expr);
    }
}

fn has_match_on_permissions(_cx: &LateContext<'_>, body: &Body<'_>) -> bool {
    let mut mv = MatchVisitor { has_match: false };

    intravisit::walk_body(&mut mv, body);

    if mv.has_match {
        return true;
    }

    false
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
