#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_ast::tokenstream::TokenTree;
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_hir::{intravisit::FnKind, FnDecl, Body, Attribute, AttrArgs, ExprKind, intravisit::{self, Visitor}};
use rustc_span::{Span, def_id::LocalDefId};

dylint_linting::declare_late_lint! {
    pub EXECUTE_WITHOUT_EXPLICIT_PERMISSIONS,
    Warn,
    "description goes here"
}

impl<'tcx> LateLintPass<'tcx> for ExecuteWithoutExplicitPermissions {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fn_kind: FnKind<'tcx>,
        _fn_decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        local_def_id: LocalDefId,
    ) {
        match fn_kind {
            FnKind::ItemFn(ident, ..) => {
                if ident.name.as_str() != "execute" {
                    return;
                }

                let attrs = cx.tcx.get_all_attrs(local_def_id.to_def_id());
                let is_entry_point = is_entry_point(attrs);

                if !is_entry_point {
                    return;
                }

                let has_permissions_match = has_match_on_permissions(cx, body);

                if has_permissions_match {
                    cx.span_lint(EXECUTE_WITHOUT_EXPLICIT_PERMISSIONS, span, |diag| {
                        diag.primary_message("good!");
                    });
                } else {
                    cx.span_lint(EXECUTE_WITHOUT_EXPLICIT_PERMISSIONS, span, |diag| {
                        diag.primary_message("bad!");
                    });
                }
            }
            _ => {}
        }
    }
}

fn is_entry_point<'a, I>(attrs: I) -> bool
where
    I: Iterator<Item = &'a Attribute>,
{
    for attr in attrs {
        // println!("{:#?}", attr);
        if let Attribute::Unparsed(b_attr_item) = attr {
            let attr_item = &b_attr_item;

            let is_cfg_attr = if let Some(ident) = &attr_item.path.segments.get(0) {
                ident.name.as_str() == "<cfg_attr>"
            } else {
                false
            };

            if !is_cfg_attr {
                continue;
            }

            if let AttrArgs::Delimited(delim_args) = &attr_item.args {
                for token_kind in delim_args.tokens.iter() {
                    if let TokenTree::Token(token, _) = token_kind {
                        if let Some((ident, _)) = token.ident() {
                            if ident.name.as_str() == "entry_point" {
                                return true;
                            }
                        }
                    }
                }
            }
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

fn has_match_on_permissions(
    _cx: &LateContext<'_>,
    body: &Body<'_>,
) -> bool {
    // println!("{:#?}", body);

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
