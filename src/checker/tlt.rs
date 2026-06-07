use anyhow::Result;
use std::collections::BTreeMap;

use crate::{
    checker::tys::*,
    cst::{ExtraCstInfo, tys::*},
    error::*,
};

#[derive(Clone, Debug)]
pub struct TLT<'a> {
    pub type_store: TypeStore,
    pub(super) alias: BTreeMap<&'a str, TLTAlias>,

    // cst_id, type_id
    pub(super) block: Vec<BTreeMap<&'a str, (usize, TypeId)>>,
    // cst_id, cst_id
    pub refer_map: BTreeMap<usize, usize>,

    pub errors: Vec<CFoodError>,
}

#[derive(Clone, Debug)]
pub(super) struct TLTAlias {
    pub prims: Vec<Prim>,
}

impl<'a> Default for TLT<'a> {
    fn default() -> Self {
        Self {
            alias: Default::default(),
            block: vec![Default::default()],
            type_store: Default::default(),
            errors: Default::default(),
            refer_map: Default::default(),
        }
    }
}

impl<'a> ExtraCstInfo for TLT<'a> {
    fn get_info(&self, cst_id: usize) -> Option<String> {
        self.type_store.get_type_id(cst_id).map(|x| {
            let ty = self.type_store.get(x);
            format!("(@type \"{ty}\")")
        })
    }
}

impl<'a> TLT<'a> {
    pub fn check_file<'b: 'a>(&mut self, n: &'b File) -> Result<()> {
        super::decl::hoist(self, &n.decls);
        for decl in &n.decls {
            match decl {
                Decl::Func(decl_func) => {
                    super::decl::hoist_decl_func(self, decl_func)?;
                }
                _ => {}
            }
        }

        for decl in &n.decls {
            match decl {
                Decl::Var(decl_var) => {
                    self.check_decl_var(decl_var)?;
                }
                Decl::Func(decl_func) => {
                    super::decl::check_decl_func(self, decl_func)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    // left to right = first to last
    pub(crate) fn normaliaze_kind(&self, kind: &Kind) -> Option<Vec<Prim>> {
        match kind {
            Kind::Int(id) => Some(vec![Prim {
                cst_id: id.0,
                kind: PrimKind::Int,
            }]),
            Kind::Float(id) => Some(vec![Prim {
                cst_id: id.0,
                kind: PrimKind::Float,
            }]),
            Kind::Void(_) => Some(vec![]),
            Kind::Bool(id) => Some(vec![Prim {
                cst_id: id.0,
                kind: PrimKind::Bool,
            }]),
            Kind::ConStr(id) => Some(vec![Prim {
                cst_id: id.0,
                kind: PrimKind::ConStr,
            }]),
            Kind::Alias(alias) => self
                .alias
                .get(alias.name.inner.as_str())
                .map(|x| x.prims.clone()),
        }
    }

    pub(super) fn check_decl_var<'b: 'a>(&mut self, n: &'b DeclVar) -> Result<()> {
        super::decl::check_decl_var(self, n)
    }
    pub(super) fn check_stmt<'b: 'a>(&mut self, n: &'b Stmt) -> Result<()> {
        super::stmt::check_stmt(self, n)
    }

    pub(super) fn check_stmt_block<'b: 'a>(&mut self, n: &'b StmtBlock) -> Result<()> {
        super::stmt::check_stmt_block(self, n)
    }

    pub(super) fn check_expr(&mut self, n: &Expr) -> Result<()> {
        super::expr::check_expr(self, n)
    }
}
