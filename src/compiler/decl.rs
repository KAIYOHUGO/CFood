use crate::{
    compiler::{
        compiler::Compiler,
        tys::{LLVMFunc, LLVMValue},
    },
    cst::tys::{DeclFunc, DeclVar},
};
use anyhow::Result;
use inkwell::types::BasicMetadataTypeEnum;

pub fn compile_decl_func(com: &mut Compiler, n: &DeclFunc) -> Result<()> {
    let id = com.type_store.get_type_id(n.id).unwrap();
    let ty = com.type_store.get(id).as_c_type().unwrap();
    let inputs: Vec<BasicMetadataTypeEnum> = ty
        .inputs
        .iter()
        .map(|x| com.to_llvm_type(x.kind).into())
        .collect();
    let outputs: Vec<_> = ty
        .outputs
        .iter()
        .map(|x| com.to_llvm_type(x.kind))
        .collect();

    let ret_ty = com.llvm.context.struct_type(&outputs, false);
    let ty = ret_ty.fn_type(&inputs, false);
    let func = com.llvm.module.add_function(&n.name.inner, ty, None);

    let entry = com.llvm.context.append_basic_block(func, "entry");
    com.llvm.builder.position_at_end(entry);

    let func = LLVMFunc { id, ty, func };
    com.var_store.new_func(n.id, func.clone());
    com.current_func = Some(func);

    com.compile_stmt_block(&n.block)?;

    com.current_func = None;
    com.llvm.builder.build_return(Some(&ret_ty.get_undef()))?;

    Ok(())
}

pub fn compile_decl_var(com: &mut Compiler, n: &DeclVar) -> Result<()> {
    let id = com.type_store.get_type_id(n.id).unwrap();
    let ty = com.type_store.get(id).as_c_type().unwrap();

    assert!(ty.inputs.is_empty());

    let outputs: Vec<_> = ty
        .outputs
        .iter()
        .map(|x| com.to_llvm_type(x.kind))
        .collect();
    let ty = com.llvm.context.struct_type(&outputs, false);
    let value = if com.current_func.is_some() {
        com.llvm.builder.build_alloca(ty, &n.name.inner)?
    } else {
        com.llvm
            .module
            .add_global(ty, None, &n.name.inner)
            .as_pointer_value()
    };
    if let Some(expr) = &n.init {
        let ret = com.compile_expr(expr)?;
        com.llvm.builder.build_store(value, ret)?;
    }

    let value = LLVMValue { id, ty, value };
    com.var_store.new_value(n.id, value);

    Ok(())
}
