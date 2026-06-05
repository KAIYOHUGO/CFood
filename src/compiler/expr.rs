use std::mem;

use crate::{
    checker::PrimKind,
    compiler::Compiler,
    cst::{
        Marked,
        tys::{
            Expr, ExprAssign, ExprBinary, ExprCall, ExprCast, ExprLit, ExprMagic, ExprRefer,
            ExprUnary, ExprVar, Magic, Op, UnaryOp,
        },
    },
};
use anyhow::Result;
use inkwell::{
    FloatPredicate, IntPredicate,
    values::{AnyValue, BasicMetadataValueEnum, BasicValueEnum, StructValue},
};

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
    let mut output = output_ty.get_undef().into();

    for i in 0..output_ty.get_field_types().len() {
        output = com.llvm.builder.build_insert_value(
            output,
            s.stack[i],
            i as u32,
            &format!("field_{i}"),
        )?;
    }

    Ok(output.into_struct_value())
}
type Stack<'ctx> = Vec<BasicValueEnum<'ctx>>;

#[derive(Debug, Clone, Default)]
struct ExprCompiler<'ctx> {
    stack: Stack<'ctx>,
}

impl<'ctx> ExprCompiler<'ctx> {
    fn compile_expr(&mut self, com: &mut Compiler<'_, 'ctx>, n: &Expr) -> Result<()> {
        match n {
            Expr::Binary(expr_binary) => self.compile_binary(com, expr_binary)?,
            Expr::Unary(expr_unary) => self.compile_unary(com, expr_unary)?,
            Expr::Assign(expr_assign) => self.compile_assign(com, expr_assign)?,
            Expr::Call(expr_call) => self.compile_call(com, expr_call)?,
            Expr::Cast(expr_cast) => self.compile_cast(com, expr_cast)?,
            Expr::Magic(expr_magic) => self.compile_magic(com, expr_magic)?,
            Expr::Lit(expr_lit) => self.compile_lit(com, expr_lit)?,
            Expr::Var(expr_var) => self.compile_var(com, expr_var)?,
            Expr::Refer(expr_refer) => self.compile_refer(com, expr_refer)?,
        }
        Ok(())
    }

    fn compile_binary(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprBinary) -> Result<()> {
        self.compile_expr(com, &n.rhs)?;
        let rhs = self.stack.pop().unwrap();
        assert!(self.stack.is_empty());

        let mut lhs = Self::default();
        lhs.compile_expr(com, &n.lhs)?;
        let lhs = lhs.stack.pop().unwrap();

        let ty = com
            .type_store
            .get(com.type_store.get_type_id(n.lhs.mark()).unwrap())
            .as_c_type()
            .unwrap()
            .outputs[0]
            .kind;

        let value: BasicValueEnum = match ty {
            PrimKind::Int => match n.op {
                Op::Add(_) => com
                    .llvm
                    .builder
                    .build_int_add(lhs.into_int_value(), rhs.into_int_value(), "int_add")?
                    .into(),
                Op::Sub(_) => com
                    .llvm
                    .builder
                    .build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "int_sub")?
                    .into(),
                Op::Mul(_) => com
                    .llvm
                    .builder
                    .build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "int_mul")?
                    .into(),
                Op::Div(_) => com
                    .llvm
                    .builder
                    .build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "int_div")?
                    .into(),
                Op::OpMod(_) => com
                    .llvm
                    .builder
                    .build_int_signed_rem(lhs.into_int_value(), rhs.into_int_value(), "int_mod")?
                    .into(),
                Op::And(_) => com
                    .llvm
                    .builder
                    .build_and(lhs.into_int_value(), rhs.into_int_value(), "int_and")?
                    .into(),
                Op::Or(_) => com
                    .llvm
                    .builder
                    .build_or(lhs.into_int_value(), rhs.into_int_value(), "int_or")?
                    .into(),
                op => {
                    let op = match op {
                        Op::Ne(_) => IntPredicate::NE,
                        Op::Eq(_) => IntPredicate::EQ,
                        Op::Lt(_) => IntPredicate::SLT,
                        Op::Gt(_) => IntPredicate::SGT,
                        Op::Le(_) => IntPredicate::SLE,
                        Op::Ge(_) => IntPredicate::SGE,
                        _ => unreachable!(),
                    };
                    com.llvm
                        .builder
                        .build_int_compare(op, lhs.into_int_value(), rhs.into_int_value(), "cmp")?
                        .into()
                }
            },
            PrimKind::Float => match n.op {
                Op::Add(_) => com
                    .llvm
                    .builder
                    .build_float_add(lhs.into_float_value(), rhs.into_float_value(), "float_add")?
                    .into(),
                Op::Sub(_) => com
                    .llvm
                    .builder
                    .build_float_sub(lhs.into_float_value(), rhs.into_float_value(), "float_sub")?
                    .into(),
                Op::Mul(_) => com
                    .llvm
                    .builder
                    .build_float_mul(lhs.into_float_value(), rhs.into_float_value(), "float_mul")?
                    .into(),
                Op::Div(_) => com
                    .llvm
                    .builder
                    .build_float_div(lhs.into_float_value(), rhs.into_float_value(), "float_div")?
                    .into(),
                Op::OpMod(_) => com
                    .llvm
                    .builder
                    .build_float_rem(lhs.into_float_value(), rhs.into_float_value(), "float_mod")?
                    .into(),
                Op::PEO(_) => com
                    .llvm
                    .builder
                    .build_direct_call(
                        com.symbol.power_both_side,
                        &[lhs.into(), rhs.into()],
                        "swl_poe",
                    )?
                    .try_as_basic_value()
                    .unwrap_basic(),
                op => {
                    let op = match op {
                        Op::Ne(_) => FloatPredicate::ONE,
                        Op::Eq(_) => FloatPredicate::OEQ,
                        Op::Lt(_) => FloatPredicate::OLT,
                        Op::Gt(_) => FloatPredicate::OGT,
                        Op::Le(_) => FloatPredicate::OLE,
                        Op::Ge(_) => FloatPredicate::OGE,
                        _ => unreachable!(),
                    };
                    com.llvm
                        .builder
                        .build_float_compare(
                            op,
                            lhs.into_float_value(),
                            rhs.into_float_value(),
                            "float_cmp",
                        )?
                        .into()
                }
            },
            PrimKind::Bool => unreachable!(),
            PrimKind::ConStr => unreachable!(),
        };
        self.stack.push(value);
        Ok(())
    }

    fn compile_unary(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprUnary) -> Result<()> {
        self.compile_expr(com, &n.rhs)?;
        let rhs = self.stack.pop().unwrap();

        let ty = com
            .type_store
            .get(com.type_store.get_type_id(n.rhs.mark()).unwrap())
            .as_c_type()
            .unwrap()
            .outputs[0]
            .kind;

        let value = match (ty, n.op) {
            (PrimKind::Int, UnaryOp::Add(_)) => rhs,
            (PrimKind::Int, UnaryOp::Sub(_)) => com
                .llvm
                .builder
                .build_int_neg(rhs.into_int_value(), "int_neg")?
                .into(),
            (PrimKind::Float, UnaryOp::Add(_)) => rhs,
            (PrimKind::Float, UnaryOp::Sub(_)) => com
                .llvm
                .builder
                .build_float_neg(rhs.into_float_value(), "float_neg")?
                .into(),
            (PrimKind::Bool, UnaryOp::Not(_)) => com
                .llvm
                .builder
                .build_not(rhs.into_int_value(), "bool_not")?
                .into(),
            _ => unreachable!(),
        };

        self.stack.push(value);
        Ok(())
    }

    fn compile_cast(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprCast) -> Result<()> {
        self.compile_expr(com, &n.lhs)?;
        let value = self.stack.pop().unwrap();

        let src = com
            .type_store
            .get(com.type_store.get_type_id(n.lhs.mark()).unwrap())
            .as_c_type()
            .unwrap()
            .outputs[0]
            .kind;
        let dst = com
            .type_store
            .get(com.type_store.get_type_id(n.id).unwrap())
            .as_c_type()
            .unwrap()
            .outputs[0]
            .kind;

        let value = match (src, dst) {
            (PrimKind::Int, PrimKind::Int)
            | (PrimKind::Float, PrimKind::Float)
            | (PrimKind::Bool, PrimKind::Bool)
            | (PrimKind::ConStr, PrimKind::ConStr) => value,
            (PrimKind::Int, PrimKind::Float) => com
                .llvm
                .builder
                .build_signed_int_to_float(
                    value.into_int_value(),
                    com.to_llvm_type(PrimKind::Float).into_float_type(),
                    "int_to_float",
                )?
                .into(),
            (PrimKind::Float, PrimKind::Int) => com
                .llvm
                .builder
                .build_float_to_signed_int(
                    value.into_float_value(),
                    com.to_llvm_type(PrimKind::Int).into_int_type(),
                    "float_to_int",
                )?
                .into(),
            (PrimKind::Bool, PrimKind::Int) => com
                .llvm
                .builder
                .build_int_z_extend(
                    value.into_int_value(),
                    com.to_llvm_type(PrimKind::Int).into_int_type(),
                    "bool_to_int",
                )?
                .into(),
            _ => unreachable!(),
        };

        self.stack.push(value);
        Ok(())
    }

    fn compile_assign(&mut self, com: &mut Compiler<'_, 'ctx>, n: &ExprAssign) -> Result<()> {
        self.compile_expr(com, &n.rhs)?;
        let llvm_value = com.var_store.get(n.var.id).as_value().unwrap();

        let output_ty = llvm_value.ty;
        let mut output = output_ty.get_undef().into();

        for i in 0..output_ty.get_field_types().len() {
            output = com.llvm.builder.build_insert_value(
                output,
                self.stack[i],
                i as u32,
                &format!("field_{i}"),
            )?;
        }

        com.llvm
            .builder
            .build_store(llvm_value.value, output.into_struct_value())?;
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

        let value = match n.lhs {
            Magic::Printf(_) => {
                com.llvm
                    .builder
                    .build_direct_call(com.symbol.printf, &args, "printf")?
            }
            Magic::Scanf(_) => {
                com.llvm
                    .builder
                    .build_direct_call(com.symbol.scanf, &args, "scanf")?
            }
        }
        .try_as_basic_value()
        .unwrap_basic();

        self.stack.push(value);

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
