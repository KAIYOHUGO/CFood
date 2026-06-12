use inkwell::types::BasicMetadataTypeEnum;

use crate::{
    checker::PrimKind,
    compiler::{Compiler, tys::LLVMFunc},
};

pub fn buildin_type() -> Vec<(&'static str, Vec<PrimKind>, Vec<PrimKind>)> {
    vec![
        ("enable_raw_mode", vec![], vec![PrimKind::Int]),
        ("disable_raw_mode", vec![], vec![PrimKind::Int]),
        ("delete", vec![PrimKind::Int], vec![]),
    ]
}

pub fn build_buildin_type(com: &mut Compiler, funcs: Vec<(&str, usize)>) {
    for (name, cst_id) in funcs {
        let name = format!("swl_{name}");
        let id = com.type_store.get_type_id(cst_id).unwrap();
        let ty = com.type_store.get(id).as_c_type().unwrap();
        let inputs: Vec<BasicMetadataTypeEnum> = ty
            .inputs
            .iter()
            .rev()
            .map(|x| com.to_llvm_type(x.kind).into())
            .collect();
        let outputs: Vec<_> = ty
            .outputs
            .iter()
            .map(|x| com.to_llvm_type(x.kind))
            .collect();

        let ret_ty = com.llvm.context.struct_type(&outputs, false);
        let ty = ret_ty.fn_type(&inputs, false);

        let func = com.llvm.module.add_function(&name, ty, None);
        let func = LLVMFunc { id, ty, func };
        com.var_store.new_func(cst_id, func);
    }
}
