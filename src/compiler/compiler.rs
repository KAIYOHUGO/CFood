use std::{collections::BTreeMap, mem, path::Path};

use anyhow::{Context as _, Result};
use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    targets::{Target, TargetMachine, TargetTriple},
    types::BasicTypeEnum,
    values::{FunctionValue, StructValue},
};

use crate::{
    checker::{PrimKind, TypeStore},
    compiler::{
        expr::compile_expr,
        stmt,
        tys::{LLVMFunc, LLVMVarStore},
    },
    cst::tys::*,
};

use super::decl;

pub struct Compiler<'a, 'ctx> {
    pub llvm: LLVMCtx<'a, 'ctx>,
    pub var_store: LLVMVarStore<'ctx>,
    pub type_store: TypeStore,
    pub(super) symbol: Symbol<'ctx>,
    pub(super) current_func: Option<LLVMFunc<'ctx>>,
    pub(super) target_machine: TargetMachine,
}

pub(super) struct Symbol<'ctx> {
    pub printf: FunctionValue<'ctx>,
    pub scanf: FunctionValue<'ctx>,
    pub power_both_side: FunctionValue<'ctx>,
    pub ctors: Vec<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        llvm: LLVMCtx<'a, 'ctx>,
        target_triple: &str,
        refer_map: BTreeMap<usize, usize>,
        type_store: TypeStore,
    ) -> Result<Self> {
        let var_store = LLVMVarStore::new(refer_map);

        let triple = TargetTriple::create(target_triple);
        let target = Target::from_triple(&triple)?;
        let target_machine = target
            .create_target_machine_from_options(&triple, Default::default())
            .context("Cannot create tuple")?;

        let int_ty = llvm
            .context
            .ptr_sized_int_type(&target_machine.get_target_data(), None);
        let magic_type = int_ty.fn_type(&[llvm.context.ptr_type(Default::default()).into()], true);
        let printf = llvm.module.add_function("printf", magic_type, None);
        let scanf = llvm.module.add_function("scanf", magic_type, None);

        let f64_ty = llvm.context.f64_type();
        let power_both_side = llvm.module.add_function(
            "power_both_side",
            f64_ty.fn_type(&[f64_ty.into(), f64_ty.into()], false),
            None,
        );
        let symbol = Symbol {
            printf,
            scanf,
            power_both_side,
            ctors: vec![],
        };
        Ok(Self {
            llvm,
            var_store,
            current_func: None,
            type_store,
            target_machine,
            symbol,
        })
    }

    pub fn compile(&mut self, n: &'a File, path: impl AsRef<Path>) -> Result<()> {
        self.compile_file(n)?;

        // ctors
        let i32_ty = self.llvm.context.i32_type();
        let ptr_ty = self.llvm.context.ptr_type(Default::default());
        let ctor_entry_type = self
            .llvm
            .context
            .struct_type(&[i32_ty.into(), ptr_ty.into(), ptr_ty.into()], false);
        let array_ty = ctor_entry_type.array_type(self.symbol.ctors.len() as u32);

        let ctor_records: Vec<_> = mem::take(&mut self.symbol.ctors)
            .into_iter()
            .map(|func| {
                ctor_entry_type.const_named_struct(&[
                    i32_ty.const_int(65535, false).into(),
                    func.as_global_value().as_pointer_value().into(),
                    ptr_ty.const_null().into(),
                ])
            })
            .collect();
        let array_value = ctor_entry_type.const_array(&ctor_records);

        let global_ctors = self
            .llvm
            .module
            .add_global(array_ty, None, "llvm.global_ctors");
        global_ctors.set_linkage(Linkage::Appending);
        global_ctors.set_initializer(&array_value);

        self.llvm.module.print_to_file(path).unwrap();
        Ok(())
    }

    pub(super) fn to_llvm_type(&self, p: PrimKind) -> BasicTypeEnum<'ctx> {
        match p {
            PrimKind::Int => self
                .llvm
                .context
                .ptr_sized_int_type(&self.target_machine.get_target_data(), None)
                .into(),
            PrimKind::Float => self.llvm.context.f64_type().into(),
            PrimKind::Bool => self.llvm.context.bool_type().into(),
            PrimKind::ConStr => self.llvm.context.ptr_type(Default::default()).into(),
        }
    }
}

pub struct LLVMCtx<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn compile_file(&mut self, n: &File) -> Result<()> {
        for decl in &n.decls {
            self.compile_decl(decl)?;
        }
        Ok(())
    }

    pub(super) fn compile_decl(&mut self, n: &Decl) -> Result<()> {
        match n {
            Decl::Var(decl_var) => decl::compile_decl_var(self, decl_var),
            Decl::Func(decl_func) => decl::compile_decl_func(self, decl_func),
            Decl::Alias(_) => Ok(()),
        }
    }

    pub(super) fn compile_stmt(&mut self, n: &Stmt) -> Result<()> {
        match n {
            Stmt::DeclVar(decl_var) => decl::compile_decl_var(self, decl_var),
            Stmt::Branch(stmt_branch) => stmt::compile_stmt_branch(self, stmt_branch),
            Stmt::Iter(stmt_iter) => stmt::compile_stmt_iter(self, stmt_iter),
            Stmt::Block(stmt_block) => self.compile_stmt_block(stmt_block),
            Stmt::AutoLet(stmt_let) => stmt::compile_stmt_let(self, stmt_let),
            Stmt::Ret(stmt_ret) => stmt::compile_stmt_ret(self, stmt_ret),
            Stmt::Expr(expr) => self.compile_expr(expr).map(|_| ()),
        }
    }
    pub(super) fn compile_stmt_block(&mut self, n: &StmtBlock) -> Result<()> {
        for stmt in &n.stmts {
            self.compile_stmt(stmt)?;
        }
        Ok(())
    }

    pub(super) fn compile_expr(&mut self, n: &Expr) -> Result<StructValue<'ctx>> {
        compile_expr(self, n)
    }
}
