use std::mem;

use crate::{
    checker::PrimKind,
    compiler::Compiler,
    cst::{
        Marked,
        tys::{Expr, ExprAssign, ExprCall, ExprLit, ExprMagic, ExprRefer, ExprVar, Magic},
    },
};
use anyhow::Result;
use inkwell::values::{AnyValue, BasicMetadataValueEnum, BasicValueEnum, StructValue};

use super::tys::LLVMVar;

pub fn compile_expr<'ctx>(com: &mut Compiler<'_, 'ctx>, n: &Expr) -> Result<StructValue<'ctx>> {
    let ty = com
        .type_store
        .get(com.type_store.get_type_id(n.mark()).unwrap())
        .as_c_type()
        .unwrap();
    assert!(ty.inputs.is_empty());
    let output_ty: Vec<_> = ty
        .outputs
        .iter()
        .map(|x| com.to_llvm_type(x.kind))
        .collect();

    let mut s = ExprCompiler::default();
    s.compile_expr(com, n)?;

    assert_eq!(s.stack.len(), output_ty.len());

    let output_ty = com.llvm.context.struct_type(&output_ty, false);
    let value = output_ty.const_named_struct(&s.stack);

    Ok(value)
}
type Stack<'ctx> = Vec<BasicValueEnum<'ctx>>;

#[derive(Debug, Clone, Default)]
struct ExprCompiler<'ctx> {
    stack: Stack<'ctx>,
}

impl<'ctx> ExprCompiler<'ctx> {
    fn compile_expr(&mut self, com: &mut Compiler<'_, 'ctx>, n: &Expr) -> Result<()> {
        match n {
            Expr::Binary(expr_binary) => todo!(),
            Expr::Assign(expr_assign) => self.compile_assign(com, expr_assign)?,
            Expr::Call(expr_call) => self.compile_call(com, expr_call)?,
            Expr::Magic(expr_magic) => self.compile_magic(com, expr_magic)?,
            Expr::Lit(expr_lit) => self.compile_lit(com, expr_lit)?,
            Expr::Var(expr_var) => self.compile_var(com, expr_var)?,
            Expr::Refer(expr_refer) => self.compile_refer(com, expr_refer)?,
        }
        Ok(())
    }

    fn compile_assign(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprAssign) -> Result<()> {
        self.compile_expr(com, &n.rhs)?;
        let llvm_value = com.var_store.get(n.var.id).as_value().unwrap();
        let value = llvm_value.ty.const_named_struct(&self.stack);
        com.llvm.builder.build_store(llvm_value.value, value)?;
        Ok(())
    }

    fn compile_call(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprCall) -> Result<()> {
        self.compile_expr(com, &n.rhs)?;
        self.compile_expr(com, &n.lhs)?;
        Ok(())
    }

    fn compile_magic(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprMagic) -> Result<()> {
        self.compile_expr(com, &n.rhs)?;
        self.stack.reverse();
        let args: Vec<BasicMetadataValueEnum> = mem::take(&mut self.stack)
            .into_iter()
            .map(|x| x.into())
            .collect();
        match n.lhs {
            Magic::Printf(_) => {
                com.llvm
                    .builder
                    .build_direct_call(com.printf, &args, "printf")?;
            }
            Magic::Scanf(_) => {
                com.llvm
                    .builder
                    .build_direct_call(com.scanf, &args, "scanf")?;
            }
        }

        Ok(())
    }

    fn compile_var<'a>(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprVar) -> Result<()> {
        match com.var_store.get(n.id) {
            LLVMVar::Value(llvmvalue) => {
                llvmvalue.ty.get_field_types();
                let load = com
                    .llvm
                    .builder
                    .build_load(
                        llvmvalue.ty,
                        llvmvalue.value,
                        &format!("load_{}", n.name.inner),
                    )?
                    .into_struct_value();
                for i in 0..load.count_fields() {
                    let field = com.llvm.builder.build_extract_value(
                        load,
                        i,
                        &format!("{}_{}", n.name.inner, i),
                    )?;
                    self.stack.push(field);
                }
            }
            LLVMVar::Func(llvmfunc) => {
                let mut args = vec![];
                let param_count = llvmfunc.ty.count_param_types();
                for _ in 0..param_count {
                    let param: BasicMetadataValueEnum = self.stack.pop().unwrap().into();
                    args.push(param);
                }
                let outputs = com
                    .llvm
                    .builder
                    .build_call(llvmfunc.func, &args, &format!("call_{}", n.name.inner))?
                    .as_any_value_enum()
                    .into_struct_value();
                for i in 0..outputs.count_fields() {
                    let field = com.llvm.builder.build_extract_value(
                        outputs,
                        i,
                        &format!("{}_{}", n.name.inner, i),
                    )?;
                    self.stack.push(field);
                }
            }
        }

        Ok(())
    }

    fn compile_refer(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprRefer) -> Result<()> {
        let int_ty = com.to_llvm_type(PrimKind::Int).into_int_type();
        match com.var_store.get(n.id) {
            LLVMVar::Value(llvmvalue) => {
                let refer = com.llvm.builder.build_ptr_to_int(
                    llvmvalue.value,
                    int_ty,
                    &format!("refer_{}", n.name.inner),
                )?;

                self.stack.push(refer.into());
            }
            LLVMVar::Func(llvmfunc) => {
                let refer = com.llvm.builder.build_ptr_to_int(
                    llvmfunc.func.as_global_value().as_pointer_value(),
                    int_ty,
                    &format!("refer_{}", n.name.inner),
                )?;

                self.stack.push(refer.into());
            }
        }
        Ok(())
    }

    fn compile_lit(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprLit) -> Result<()> {
        match n {
            ExprLit::Int(token) => {
                let value = com
                    .to_llvm_type(PrimKind::Int)
                    .into_int_type()
                    .const_int(u64::from_ne_bytes(token.inner.to_ne_bytes()), true);
                self.stack.push(value.into());
            }
            ExprLit::Float(token) => {
                let value = com
                    .to_llvm_type(PrimKind::Float)
                    .into_float_type()
                    .const_float(token.inner);
                self.stack.push(value.into());
            }
            ExprLit::ConStr(token) => {
                let value = com
                    .llvm
                    .builder
                    .build_global_string_ptr(&token.inner, "inline_str")?;
                self.stack.push(value.as_pointer_value().into());
            }
        }
        Ok(())
    }
}
