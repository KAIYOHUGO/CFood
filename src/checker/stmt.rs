use crate::checker::TLT;
use crate::cst::Marked;
use crate::cst::tys::{Stmt, StmtBlock, StmtBranch, StmtIter, StmtRet};
use crate::error::*;

use anyhow::Result;

pub fn check_stmt<'a, 'b: 'a>(tlt: &mut TLT<'a>, n: &'b Stmt) -> Result<()> {
    match n {
        Stmt::DeclVar(decl_var) => tlt.check_decl_var(decl_var)?,
        Stmt::Branch(stmt_branch) => check_stmt_branch(tlt, stmt_branch)?,
        Stmt::Iter(stmt_iter) => check_stmt_iter(tlt, stmt_iter)?,
        Stmt::Block(stmt_block) => check_stmt_block(tlt, stmt_block)?,
        Stmt::AutoLet(stmt_let) => todo!(),
        Stmt::Ret(stmt_ret) => check_stmt_ret(tlt, stmt_ret)?,
        Stmt::Expr(expr) => tlt.check_expr(expr)?,
    }
    Ok(())
}

pub fn check_stmt_ret<'a, 'b: 'a>(tlt: &mut TLT<'a>, n: &'b StmtRet) -> Result<()> {
    let (cst_id, ret) = tlt.block.get(1).unwrap().get("return").unwrap().clone();

    let (expr_id, expr_ty) = if let Some(expr) = &n.expr {
        tlt.check_expr(expr)?;
        let expr_ty = tlt.type_store.get_type_id(expr.mark()).unwrap();
        (expr.mark(), expr_ty)
    } else {
        (n.id, tlt.type_store.void(n.id))
    };

    if !tlt.type_store.is_eq(expr_ty, ret) {
        let expr_str = tlt.type_store.get(expr_ty).to_string();
        let ret_str = tlt.type_store.get(ret).to_string();
        tlt.errors.push(CFoodError {
            message: "Return type mismatch".to_owned(),
            labels: vec![
                CFoodErrorLabel {
                    cst_id,
                    label: Some(format!("return type is defined as `{ret_str}`")),
                },
                CFoodErrorLabel {
                    cst_id: expr_id,
                    label: Some(format!("return value has type `{expr_str}`")),
                },
            ],
            ..Default::default()
        });
    }
    Ok(())
}
pub fn check_stmt_iter<'a, 'b: 'a>(tlt: &mut TLT<'a>, n: &'b StmtIter) -> Result<()> {
    tlt.check_expr(&n.cond)?;
    n.then_branch.as_ref().map(|x| tlt.check_stmt(&*x));

    let cond = tlt
        .type_store
        .get(tlt.type_store.get_type_id(n.cond.mark()).unwrap())
        .clone();
    let is_bool = cond.as_c_type().is_some_and(|x| {
        x.inputs.is_empty() && x.outputs.len() == 1 && x.outputs[0].kind.is_bool()
    });

    if !is_bool {
        tlt.errors.push(CFoodError {
            message: "Iteration condition must be bool".to_owned(),
            help: Some("Use a comparison or a boolean expression in condition.".to_owned()),
            labels: vec![CFoodErrorLabel {
                cst_id: n.cond.mark(),
                label: Some(format!("this condition has type `{cond}`")),
            }],
        });
    }

    Ok(())
}
pub fn check_stmt_branch<'a, 'b: 'a>(tlt: &mut TLT<'a>, n: &'b StmtBranch) -> Result<()> {
    tlt.check_expr(&n.cond)?;
    n.then_branch.as_ref().map(|x| tlt.check_stmt(&*x));
    n.else_branch.as_ref().map(|x| tlt.check_stmt(&*x));

    let cond = tlt
        .type_store
        .get(tlt.type_store.get_type_id(n.cond.mark()).unwrap())
        .clone();
    let is_bool = cond.as_c_type().is_some_and(|x| {
        x.inputs.is_empty() && x.outputs.len() == 1 && x.outputs[0].kind.is_bool()
    });

    if !is_bool {
        tlt.errors.push(CFoodError {
            message: "Branch condition must be bool".to_owned(),
            help: Some("Use a comparison or a boolean expression in condition.".to_owned()),
            labels: vec![CFoodErrorLabel {
                cst_id: n.cond.mark(),
                label: Some(format!("this condition has type `{cond}`")),
            }],
        });
    }

    Ok(())
}
pub fn check_stmt_block<'a, 'b: 'a>(tlt: &mut TLT<'a>, n: &'b StmtBlock) -> Result<()> {
    tlt.block.push(Default::default());

    for stmt in &n.stmts {
        tlt.check_stmt(stmt)?;
    }

    tlt.block.pop();
    Ok(())
}
