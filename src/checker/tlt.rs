use anyhow::Result;
use std::collections::BTreeMap;

use crate::{
    checker::tys::*,
    cst::{ExtraCstInfo, Marked, tys::*},
    error::*,
};

#[derive(Clone, Debug)]
pub struct TLT<'a> {
    pub type_store: TypeStore,
    alias: BTreeMap<&'a str, TLTAlias>,

    // cst_id, type_id
    pub(super) block: Vec<BTreeMap<&'a str, (usize, TypeId)>>,
    // cst_id, cst_id
    pub refer_map: BTreeMap<usize, usize>,

    pub errors: Vec<CFoodError>,
}

#[derive(Clone, Debug)]
pub struct TLTAlias {
    prims: Vec<Prim>,
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
        self.hoist(&n.decls);
        for decl in &n.decls {
            match decl {
                Decl::Var(decl_var) => {
                    self.check_decl_var(decl_var)?;
                }
                Decl::Func(decl_func) => {
                    self.check_decl_func(decl_func)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn hoist<'b: 'a>(&mut self, n: &'b Vec<Decl>) {
        // Hoisting
        let max_path = n.len();
        let mut lookup_alias = BTreeMap::new();
        for decl in n {
            match decl {
                Decl::Alias(decl_alias) => {
                    lookup_alias.insert(decl_alias.name.inner.as_str(), decl_alias);
                }
                _ => {}
            }
        }
        fn helper(
            alias: &DeclAlias,
            lookup_alias: &BTreeMap<&str, &DeclAlias>,
            remain: usize,
        ) -> Result<Vec<Prim>, ()> {
            if remain == 0 {
                // TODO
                return Err(());
            }
            let mut outputs = vec![];
            for kind in &alias.kinds {
                match kind {
                    Kind::Int(id) => {
                        outputs.push(Prim {
                            cst_id: id.0,
                            kind: PrimKind::Int,
                        });
                    }
                    Kind::Float(id) => {
                        outputs.push(Prim {
                            cst_id: id.0,
                            kind: PrimKind::Float,
                        });
                    }
                    Kind::Bool(id) => {
                        outputs.push(Prim {
                            cst_id: id.0,
                            kind: PrimKind::Bool,
                        });
                    }
                    Kind::ConStr(id) => {
                        outputs.push(Prim {
                            cst_id: id.0,
                            kind: PrimKind::ConStr,
                        });
                    }
                    Kind::Void(_) => {}
                    Kind::Alias(alias) => {
                        let Some(alias) = lookup_alias.get(alias.name.inner.as_str()) else {
                            return Err(());
                        };
                        let res = helper(*alias, lookup_alias, remain - 1)?;
                        outputs.extend(res);
                    }
                }
            }

            Ok(vec![])
        }

        for decl in n {
            match decl {
                Decl::Alias(decl_alias) => {
                    let res = helper(decl_alias, &lookup_alias, max_path);
                    match res {
                        Ok(prims) => {
                            let alias = TLTAlias { prims };
                            self.alias.insert(&decl_alias.name.inner, alias);
                        }
                        Err(_) => {
                            self.errors.push(CFoodError {
                                message: "Invalid alias declaration".to_owned(),
                                help: Some(
                                    "Ensure all aliases exist and avoid cyclic alias chains."
                                        .to_owned(),
                                ),
                                labels: vec![CFoodErrorLabel {
                                    cst_id: decl_alias.id,
                                    label: Some("cannot resolve alias types here".to_owned()),
                                }],
                            });
                        }
                    }
                }
                _ => {}
            }
        }
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
        let Some(mut outputs) = self.normaliaze_kind(&n.ty) else {
            self.errors.push(CFoodError {
                message: "Cannot resolve declared type".to_owned(),
                help: Some(
                    "Use a built-in type or declare the alias before this variable.".to_owned(),
                ),
                labels: vec![CFoodErrorLabel {
                    cst_id: n.ty.mark(),
                    label: Some("type cannot be resolved here".to_owned()),
                }],
            });
            self.type_store.unknown(n.id);
            return Ok(());
        };
        outputs.reverse();

        let id = self.type_store.c_type(CType {
            cst_id: n.id,
            outputs,
            ..Default::default()
        });

        if let Some(expr) = &n.init {
            self.check_expr(expr)?;
            let expr_id = self.type_store.get_type_id(expr.mark()).unwrap();
            if !self.type_store.is_eq(id, expr_id) {
                let declared = self.type_store.get(id).to_string();
                let actual = self.type_store.get(expr_id).to_string();
                self.errors.push(CFoodError {
                    message: "Type mismatch in variable initialization".to_owned(),
                    help: Some(
                        "Change the initializer or declared type so both types match.".to_owned(),
                    ),
                    labels: vec![
                        CFoodErrorLabel {
                            cst_id: n.ty.mark(),
                            label: Some(format!("declared as `{declared}`")),
                        },
                        CFoodErrorLabel {
                            cst_id: expr.mark(),
                            label: Some(format!("initializer has type `{actual}`")),
                        },
                    ],
                });
            }
        }
        let block = self.block.last_mut().unwrap();
        let prev_declare = block.insert(&n.name.inner, (n.id, id));
        if let Some((cst_id, _)) = prev_declare {
            self.errors.push(CFoodError {
                message: "Redeclaration of variable".to_owned(),
                labels: vec![
                    CFoodErrorLabel {
                        cst_id: cst_id,
                        label: Some(format!("first variable declared here")),
                    },
                    CFoodErrorLabel {
                        cst_id: n.id,
                        label: Some(format!("redeclared here")),
                    },
                ],
                ..Default::default()
            });
        }

        Ok(())
    }

    fn check_decl_func<'b: 'a>(&mut self, n: &'b DeclFunc) -> Result<()> {
        let mut params = vec![];
        for param in &n.params {
            let Some(mut outputs) = self.normaliaze_kind(&param.ty) else {
                self.errors.push(CFoodError {
                    message: "Cannot resolve parameter type".to_owned(),
                    help: Some(
                        "Use a built-in type or declare the alias before this function."
                            .to_owned(),
                    ),
                    labels: vec![CFoodErrorLabel {
                        cst_id: param.ty.mark(),
                        label: Some("parameter type cannot be resolved here".to_owned()),
                    }],
                });
                self.type_store.unknown(param.id);
                return Ok(());
            };
            outputs.reverse();

            let id = self.type_store.c_type(CType {
                cst_id: param.id,
                outputs,
                ..Default::default()
            });
            params.push(id);
        }

        let Some(mut outputs) = self.normaliaze_kind(&n.ret) else {
            self.errors.push(CFoodError {
                message: "Cannot resolve return type".to_owned(),
                help: Some(
                    "Use a built-in type or declare the alias before this function.".to_owned(),
                ),
                labels: vec![CFoodErrorLabel {
                    cst_id: n.ret.mark(),
                    label: Some("return type cannot be resolved here".to_owned()),
                }],
            });
            self.type_store.unknown(n.id);
            return Ok(());
        };
        outputs.reverse();

        let ret = self.type_store.c_type(CType {
            cst_id: n.ret.mark(),
            outputs,
            ..Default::default()
        });

        let mut inputs = params
            .iter()
            .map(|x| self.type_store.get(*x))
            .fold(vec![], |acc, x| {
                [acc, x.clone().expect_c_type().outputs].concat()
            });
        inputs.extend(self.type_store.get(ret).clone().expect_c_type().inputs);
        let outputs = self.type_store.get(ret).clone().expect_c_type().outputs;
        let func = self.type_store.c_type(CType {
            cst_id: n.id,
            inputs,
            outputs,
        });
        self.block
            .first_mut()
            .unwrap()
            .insert(&n.name.inner, (n.id, func));

        self.block.push(Default::default());
        let block = self.block.last_mut().unwrap();

        for (id, param) in params.into_iter().zip(&n.params) {
            block.insert(&param.name.inner, (param.id, id));
        }
        block.insert("return", (n.ret.mark(), ret));
        self.check_stmt_block(&n.block)?;

        self.block.pop();
        Ok(())
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
