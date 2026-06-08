use crate::{
    compiler::{
        compiler::Compiler,
        tys::{LLVMFunc, LLVMValue},
    },
    cst::tys::{DeclFunc, DeclVar},
};
use anyhow::Result;
use inkwell::{module::Linkage, types::BasicMetadataTypeEnum};

pub fn hoist_decl_func(com: &mut Compiler, n: &DeclFunc) -> Result<()> {
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
    let func = LLVMFunc { id, ty, func };
    com.var_store.new_func(n.id, func.clone());
    Ok(())
}

pub fn compile_decl_func(com: &mut Compiler, n: &DeclFunc) -> Result<()> {
    // let id = com.type_store.get_type_id(n.id).unwrap();
    // let ty = com.type_store.get(id).as_c_type().unwrap();
    // let inputs: Vec<BasicMetadataTypeEnum> = ty
    //     .inputs
    //     .iter()
    //     .map(|x| com.to_llvm_type(x.kind).into())
    //     .collect();
    // let outputs: Vec<_> = ty
    //     .outputs
    //     .iter()
    //     .map(|x| com.to_llvm_type(x.kind))
    //     .collect();

    // let ret_ty = com.llvm.context.struct_type(&outputs, false);
    // let ty = ret_ty.fn_type(&inputs, false);
    // let func = com.llvm.module.add_function(&n.name.inner, ty, None);
    let func = com.var_store.get(n.id).clone().expect_func();

    let entry = com.llvm.context.append_basic_block(func.func, "entry");
    com.llvm.builder.position_at_end(entry);
    // declare the params
    let mut param_values = func.func.get_params().into_iter().rev();
    for param in n.params.iter().rev() {
        let id = com.type_store.get_type_id(param.id).unwrap();
        let cty = com.type_store.get(id).as_c_type().unwrap();

        assert!(cty.inputs.is_empty());

        let outputs: Vec<_> = cty
            .outputs
            .iter()
            .map(|x| com.to_llvm_type(x.kind))
            .collect();
        let ty = com.llvm.context.struct_type(&outputs, false);
        let value = com.llvm.builder.build_alloca(ty, &param.name.inner)?;

        let mut init = ty.get_undef().into();
        for i in 0..cty.outputs.len() {
            init = com.llvm.builder.build_insert_value(
                init,
                param_values
                    .next()
                    .expect("Param len is not match the input len"),
                i as u32,
                &format!("field_{i}"),
            )?;
        }
        com.llvm.builder.build_store(value, init)?;
        let value = LLVMValue { id, ty, value };
        com.var_store.new_value(param.id, value);
    }

    let ret = func.ty.get_return_type().unwrap().into_struct_type();
    // let func = LLVMFunc { id, ty, func };
    // com.var_store.new_func(n.id, func.clone());
    com.current_func = Some(func);

    com.compile_stmt_block(&n.block)?;

    com.current_func = None;
    com.llvm.builder.build_return(Some(&ret.const_zero()))?;

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
        let var = com.llvm.module.add_global(ty, None, &n.name.inner);
        var.set_externally_initialized(false);
        var.set_initializer(&ty.const_zero());
        var.set_linkage(Linkage::Common);

        var.as_pointer_value()
    };
    if let Some(expr) = &n.init {
        let have_fn = com.current_func.is_none();
        if have_fn {
            let ctor_ty = com.llvm.context.void_type().fn_type(&[], false);
            let ctor_func = com.llvm.module.add_function(
                &format!("ctor_{}", n.name.inner),
                ctor_ty,
                Some(Linkage::Private),
            );
            let entry = com.llvm.context.append_basic_block(ctor_func, "entry");
            com.llvm.builder.position_at_end(entry);
            com.current_func = Some(LLVMFunc {
                id,
                ty: ctor_ty,
                func: ctor_func,
            });

            com.symbol.ctors.push(ctor_func);
        }

        let ret = com.compile_expr(expr)?;
        com.llvm.builder.build_store(value, ret)?;

        if have_fn {
            com.llvm.builder.build_return(None)?;
            com.current_func = None;
        }
    }

    let value = LLVMValue { id, ty, value };
    com.var_store.new_value(n.id, value);

    Ok(())
}
