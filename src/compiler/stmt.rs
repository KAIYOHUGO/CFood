use super::Compiler;
use crate::cst::tys::{StmtBranch, StmtIter, StmtRet};
use anyhow::Result;
use inkwell::values::BasicValue;

pub fn compile_stmt_ret<'ctx>(com: &mut Compiler<'_, 'ctx>, n: &StmtRet) -> Result<()> {
    if let Some(expr) = &n.expr {
        let ret = com.compile_expr(expr)?;
        com.llvm
            .builder
            .build_return(Some(&ret.as_basic_value_enum()))?;
    } else {
        let ret_ty = com
            .current_func
            .as_ref()
            .unwrap()
            .ty
            .get_return_type()
            .unwrap();
        com.llvm.builder.build_return(Some(&ret_ty.const_zero()))?;
    }
    Ok(())
}
pub fn compile_stmt_iter<'ctx>(com: &mut Compiler<'_, 'ctx>, n: &StmtIter) -> Result<()> {
    let cond_block = com
        .llvm
        .context
        .append_basic_block(com.current_func.as_ref().unwrap().func, "iter_cond_block");
    let then_block = com
        .llvm
        .context
        .append_basic_block(com.current_func.as_ref().unwrap().func, "iter_block");
    let next_block = com
        .llvm
        .context
        .append_basic_block(com.current_func.as_ref().unwrap().func, "next_block");

    com.llvm.builder.build_unconditional_branch(cond_block)?;

    // condition
    com.llvm.builder.position_at_end(cond_block);
    let cond = com.compile_expr(&n.cond)?;
    let cond = com
        .llvm
        .builder
        .build_extract_value(cond, 0, "cond")?
        .into_int_value();
    com.llvm
        .builder
        .build_conditional_branch(cond, then_block, next_block)?;

    // then block
    com.llvm.builder.position_at_end(then_block);
    if let Some(stmt) = &n.then_branch {
        com.compile_stmt(stmt)?;
    }
    com.llvm.builder.build_unconditional_branch(cond_block)?;

    com.llvm.builder.position_at_end(next_block);
    Ok(())
}

pub fn compile_stmt_branch<'ctx>(com: &mut Compiler<'_, 'ctx>, n: &StmtBranch) -> Result<()> {
    let cond = com.compile_expr(&n.cond)?;
    let cond = com
        .llvm
        .builder
        .build_extract_value(cond, 0, "cond")?
        .into_int_value();

    let then_block = com
        .llvm
        .context
        .append_basic_block(com.current_func.as_ref().unwrap().func, "then_block");
    let else_block = com
        .llvm
        .context
        .append_basic_block(com.current_func.as_ref().unwrap().func, "else_block");
    let next_block = com
        .llvm
        .context
        .append_basic_block(com.current_func.as_ref().unwrap().func, "next_block");

    com.llvm
        .builder
        .build_conditional_branch(cond, then_block, else_block)?;

    com.llvm.builder.position_at_end(then_block);
    if let Some(stmt) = &n.then_branch {
        com.compile_stmt(stmt)?;
    }
    com.llvm.builder.build_unconditional_branch(next_block)?;

    com.llvm.builder.position_at_end(else_block);
    if let Some(stmt) = &n.else_branch {
        com.compile_stmt(stmt)?;
    }
    com.llvm.builder.build_unconditional_branch(next_block)?;

    com.llvm.builder.position_at_end(next_block);
    Ok(())
}
