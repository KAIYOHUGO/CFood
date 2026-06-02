use super::Compiler;
use crate::cst::tys::StmtBranch;
use anyhow::Result;

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
        com.compile_stmt(&*stmt)?;
    }
    com.llvm.builder.build_unconditional_branch(next_block)?;

    com.llvm.builder.position_at_end(else_block);
    if let Some(stmt) = &n.else_branch {
        com.compile_stmt(&*stmt)?;
    }
    com.llvm.builder.build_unconditional_branch(next_block)?;

    com.llvm.builder.position_at_end(next_block);
    Ok(())
}
