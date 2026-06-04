use super::Compiler;
use crate::{
    compiler::tys::LLVMValue,
    cst::tys::{StmtBranch, StmtIter, StmtLet, StmtRet},
};
use anyhow::Result;
use inkwell::values::BasicValue;

pub fn compile_stmt_let<'ctx>(com: &mut Compiler<'_, 'ctx>, n: &StmtLet) -> Result<()> {
    let id = com.type_store.get_type_id(n.id).unwrap();
    let ty = com.type_store.get(id).as_c_type().unwrap();

    assert!(ty.inputs.is_empty());

    let outputs: Vec<_> = ty
        .outputs
        .iter()
        .map(|x| com.to_llvm_type(x.kind))
        .collect();
    let ty = com.llvm.context.struct_type(&outputs, false);
    let value = com.llvm.builder.build_alloca(ty, &n.name.inner)?;
    let ret = com.compile_expr(&n.init)?;
    com.llvm.builder.build_store(value, ret)?;

    let value = LLVMValue { id, ty, value };
    com.var_store.new_value(n.id, value);

    Ok(())
}

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
