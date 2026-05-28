use std::{collections::BTreeMap, path::Path};

use anyhow::{Context as _, Result};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    targets::{Target, TargetMachine, TargetTriple},
    types::BasicTypeEnum,
    values::{FunctionValue, StructValue},
};

use crate::{
    checker::{PrimKind, TypeStore},
    compiler::{
        expr::compile_expr,
        tys::{LLVMFunc, LLVMVarStore},
    },
    cst::{VisitAble, Visitor, tys::*},
};

use super::decl;

pub struct Compiler<'a, 'ctx> {
    pub llvm: LLVMCtx<'a, 'ctx>,
    pub var_store: LLVMVarStore<'ctx>,
    pub type_store: TypeStore,
    pub(super) current_func: Option<LLVMFunc<'ctx>>,
    pub(super) printf: FunctionValue<'ctx>,
    pub(super) scanf: FunctionValue<'ctx>,
    pub(super) target_machine: TargetMachine,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        llvm: LLVMCtx<'a, 'ctx>,
        target_triple: &str,
        refer_map: BTreeMap<usize, usize>,
        type_store: TypeStore,
    ) -> Result<Self> {
        let var_store = LLVMVarStore::new(refer_map);

        let magic_type = llvm
            .context
            .i32_type()
            .fn_type(&[llvm.context.ptr_type(Default::default()).into()], true);
        let printf = llvm.module.add_function("printf", magic_type, None);
        let scanf = llvm.module.add_function("scanf", magic_type, None);

        let triple = TargetTriple::create(target_triple);
        let target = Target::from_triple(&triple)?;
        let target_machine = target
            .create_target_machine_from_options(&triple, Default::default())
            .context("Cannot create tuple")?;
        Ok(Self {
            llvm,
            var_store,
            current_func: None,
            type_store,
            printf,
            scanf,
            target_machine,
        })
    }

    pub fn compile(&mut self, n: &'a File, path: impl AsRef<Path>) -> Result<()> {
        self.compile_file(n)?;
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
            PrimKind::Float => self.llvm.context.f32_type().into(),
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
            Decl::Alias(decl_alias) => Ok(()),
        }
    }

    pub(super) fn compile_stmt(&mut self, n: &Stmt) -> Result<()> {
        match n {
            Stmt::DeclVar(decl_var) => decl::compile_decl_var(self, decl_var),
            Stmt::Branch(stmt_branch) => todo!(),
            Stmt::Iter(stmt_iter) => todo!(),
            Stmt::Block(stmt_block) => self.compile_stmt_block(stmt_block),
            Stmt::AutoLet(stmt_let) => todo!(),
            Stmt::Ret(stmt_ret) => todo!(),
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
