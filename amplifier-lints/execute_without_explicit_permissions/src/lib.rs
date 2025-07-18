#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{
    Body, ExprKind, FnDecl,
    intravisit::FnKind,
    intravisit::{self, Visitor},
};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{FileName, RealFileName, Span, def_id::LocalDefId};

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
        if let FnKind::ItemFn(ident, ..) = fn_kind
            && ident.name.as_str() == "execute"
            && is_contract_rs_execute(cx, span)
            && !has_match_on_permissions(cx, body)
        {
            cx.span_lint(EXECUTE_WITHOUT_EXPLICIT_PERMISSIONS, span, |diag| {
                diag.primary_message("no permissions check found in execute entry point");
            });
        }
    }
}

fn is_contract_rs_execute(cx: &LateContext<'_>, span: Span) -> bool {
    let source_map = cx.tcx.sess.source_map();
    let file_name = source_map.span_to_filename(span);

    if let FileName::Real(RealFileName::LocalPath(path)) = file_name
        && path.ends_with("contract.rs")
    {
        return true;
    }

    false
}

struct PermissionMatchVisitor {
    has_match: bool,
}

impl<'tcx> Visitor<'tcx> for PermissionMatchVisitor {
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
    let mut mv = PermissionMatchVisitor { has_match: false };

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
