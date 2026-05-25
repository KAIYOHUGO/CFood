use std::collections::BTreeMap;

use crate::{
    checker::tys::*,
    cst::{ExtraCstInfo, Marked, tys::*},
    error::*,
};

#[derive(Clone, Debug)]
pub struct TLT<'a> {
    pub type_store: TypeStore,
    pub alias: BTreeMap<&'a str, TLTAlias>,
    pub block: Vec<BTreeMap<&'a str, TypeId>>,
    pub errors: Vec<CFoodError>,
}

#[derive(Clone, Debug)]
pub struct TLTAlias {
    cst_id: usize,
    prims: Vec<Prim>,
}

impl<'a> Default for TLT<'a> {
    fn default() -> Self {
        Self {
            alias: Default::default(),
            block: vec![Default::default()],
            type_store: Default::default(),
            errors: Default::default(),
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

type EResult = Result<(), ()>;
impl<'a> TLT<'a> {
    pub fn check_file(&mut self, n: &'a File) -> EResult {
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

    fn hoist(&mut self, n: &'a Vec<Decl>) {
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
                            let alias = TLTAlias {
                                cst_id: decl_alias.id,
                                prims,
                            };
                            self.alias.insert(&decl_alias.name.inner, alias);
                        }
                        // FIXME: add diag
                        Err(_) => {}
                    }
                }
                _ => {}
            }
        }
    }

    // left to right = first to last
    fn normaliaze_kind(&mut self, kind: &Kind) -> Option<Vec<Prim>> {
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

    // left to right = first to last
    fn normalize_arrow(&mut self, arrow: &TyArrow) -> Option<(Vec<Prim>, Vec<Prim>)> {
        let mut inputs = self.normaliaze_kind(&arrow.input)?;
        let outputs;

        let mut ty = &*arrow.output;
        loop {
            match ty {
                Ty::Kind(kind) => {
                    outputs = self.normaliaze_kind(kind)?;
                    break;
                }
                Ty::Arrow(ty_arrow) => {
                    inputs.extend(self.normaliaze_kind(&ty_arrow.input)?);
                    ty = &ty_arrow.output;
                }
            }
        }

        Some((inputs, outputs))
    }

    fn check_decl_var(&mut self, n: &'a DeclVar) -> EResult {
        let id = match &n.ty {
            Ty::Kind(kind) => {
                let mut outputs = self.normaliaze_kind(&kind).unwrap();
                outputs.reverse();

                self.type_store.c_type(CType {
                    cst_id: n.id,
                    outputs,
                    ..Default::default()
                })
            }
            Ty::Arrow(ty_arrow) => {
                let (inputs, mut outputs) = self.normalize_arrow(&ty_arrow).unwrap();
                outputs.reverse();
                self.type_store.c_type(CType {
                    cst_id: n.id,
                    inputs,
                    outputs,
                })
            }
        };
        let block = self.block.last_mut().unwrap();
        block.insert(&n.name.inner, id);
        if let Some(expr) = &n.init {
            self.check_expr(expr)?;
            let expr_id = self.type_store.get_type_id(expr.mark()).unwrap();
            if !self.type_store.is_eq(id, expr_id) {
                panic!()
            }
        }
        Ok(())
    }

    fn check_decl_func(&mut self, n: &'a DeclFunc) -> EResult {
        let mut params = vec![];
        for param in &n.params {
            let id = match &param.ty {
                Ty::Kind(kind) => {
                    let mut outputs = self.normaliaze_kind(&kind).unwrap();
                    outputs.reverse();

                    self.type_store.c_type(CType {
                        cst_id: param.id,
                        outputs,
                        ..Default::default()
                    })
                }
                Ty::Arrow(_ty_arrow) => {
                    // FIXME: add diag
                    // FIXME: remove panic
                    panic!()
                }
            };
            params.push(id);
        }

        let ret = match &n.ret {
            Ty::Kind(kind) => {
                let mut outputs = self.normaliaze_kind(&kind).unwrap();
                outputs.reverse();

                self.type_store.c_type(CType {
                    cst_id: n.ret.mark(),
                    outputs,
                    ..Default::default()
                })
            }
            Ty::Arrow(ty_arrow) => {
                let (inputs, mut outputs) = self.normalize_arrow(&ty_arrow).unwrap();
                outputs.reverse();
                self.type_store.c_type(CType {
                    cst_id: n.ret.mark(),
                    inputs,
                    outputs,
                })
            }
        };

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
        self.block.first_mut().unwrap().insert(&n.name.inner, func);

        self.block.push(Default::default());
        let block = self.block.last_mut().unwrap();

        for (id, param) in params.into_iter().zip(&n.params) {
            block.insert(&param.name.inner, id);
        }
        block.insert("return", ret);
        self.check_stmt_block(&n.block)?;

        self.block.pop();
        Ok(())
    }

    fn check_stmt(&mut self, n: &'a Stmt) -> EResult {
        match n {
            Stmt::DeclVar(decl_var) => self.check_decl_var(decl_var)?,
            Stmt::Branch(stmt_branch) => self.check_stmt_branch(stmt_branch)?,
            Stmt::Iter(stmt_iter) => todo!(),
            Stmt::Block(stmt_block) => self.check_stmt_block(stmt_block)?,
            Stmt::AutoLet(stmt_let) => todo!(),
            Stmt::Ret(stmt_ret) => todo!(),
            Stmt::Expr(expr) => self.check_expr(expr)?,
        }
        Ok(())
    }

    fn check_stmt_branch(&mut self, n: &'a StmtBranch) -> EResult {
        self.check_expr(&n.cond)?;
        n.then_branch.as_ref().map(|x| self.check_stmt(&*x));
        n.else_branch.as_ref().map(|x| self.check_stmt(&*x));

        let cond = self
            .type_store
            .get(self.type_store.get_type_id(n.cond.mark()).unwrap());
        let is_bool = cond.as_c_type().is_some_and(|x| {
            x.inputs.is_empty() && x.outputs.len() == 1 && x.outputs[0].kind.is_bool()
        });

        if !is_bool {
            // FIXME: add diag
        }

        Ok(())
    }
    fn check_stmt_block(&mut self, n: &'a StmtBlock) -> EResult {
        self.block.push(Default::default());

        for stmt in &n.stmts {
            self.check_stmt(stmt)?;
        }

        self.block.pop();
        Ok(())
    }

    fn check_expr(&mut self, n: &'a Expr) -> EResult {
        match n {
            Expr::Binary(expr_binary) => self.check_expr_binary(expr_binary),
            Expr::Assign(expr_assign) => self.check_expr_assign(expr_assign),
            Expr::Call(expr_call) => self.check_expr_call(expr_call),
            Expr::Magic(expr_magic) => self.check_expr_magic(expr_magic),
            Expr::Lit(lit) => self.check_lit(lit),
            Expr::Var(expr_var) => self.check_expr_var(expr_var),
        }
    }

    fn check_expr_binary(&mut self, n: &'a ExprBinary) -> EResult {
        self.check_expr(&n.lhs)?;
        self.check_expr(&n.rhs)?;
        let lhs = self
            .type_store
            .get(self.type_store.get_type_id(n.lhs.mark()).unwrap());
        let rhs = self
            .type_store
            .get(self.type_store.get_type_id(n.rhs.mark()).unwrap());

        match (lhs, rhs) {
            (AType::CType(lhs), AType::CType(rhs)) => {
                if !lhs.inputs.is_empty() || !rhs.inputs.is_empty() {
                    // FIXME: add diag
                    // about higher order...

                    self.type_store.unknown(n.id);
                    return Ok(());
                }
                if lhs.outputs.len() != 1 || rhs.outputs.len() != 1 {
                    // FIXME: add diag
                    // about apply list

                    self.type_store.unknown(n.id);
                    return Ok(());
                }

                if lhs.outputs[0] != rhs.outputs[0] {
                    // FIXME: add diag
                    // about not the same type
                    self.type_store.unknown(n.id);
                    return Ok(());
                }

                let kind = match n.op {
                    Op::Add(_) | Op::Sub(_) | Op::Mul(_) | Op::Div(_) | Op::OpMod(_) => {
                        lhs.outputs[0].kind
                    }

                    Op::Ne(_) | Op::Eq(_) | Op::Lt(_) | Op::Gt(_) | Op::Le(_) | Op::Ge(_) => {
                        PrimKind::Bool
                    }

                    Op::PEO(_) => {
                        if !lhs.outputs[0].kind.is_float() {
                            // FIXME: add diag
                            // about not the same type
                            self.type_store.unknown(n.id);
                            return Ok(());
                        }
                        PrimKind::Float
                    }
                };

                self.type_store.prim(kind, n.id);
            }
            _ => {
                self.type_store.unknown(n.id);
            }
        }

        Ok(())
    }
    fn check_expr_assign(&mut self, n: &'a ExprAssign) -> EResult {
        self.check_expr_var(&n.var)?;
        self.check_expr(&n.rhs)?;
        let var = self.type_store.get_type_id(n.var.mark()).unwrap();
        let rhs = self.type_store.get_type_id(n.rhs.mark()).unwrap();
        let res = self.type_store.is_eq(var, rhs);
        if !res {
            // FIXME: Add diag
        }

        let mut atype = self.type_store.get(var).clone();
        match &mut atype {
            AType::CType(ctype) => ctype.cst_id = n.id,
            AType::Unknown(id) => *id = n.id,
        }
        self.type_store.a_type(atype);

        Ok(())
    }
    fn check_expr_call(&mut self, n: &'a ExprCall) -> EResult {
        self.check_expr(&n.lhs)?;
        self.check_expr(&n.rhs)?;
        let lhs = self.type_store.get_type_id(n.lhs.mark()).unwrap();
        let rhs = self.type_store.get_type_id(n.rhs.mark()).unwrap();
        // FIXME: Add diag
        let _res = self.type_store.apply(lhs, rhs, n.id).unwrap();

        Ok(())
    }
    fn check_expr_magic(&mut self, n: &'a ExprMagic) -> EResult {
        self.check_expr(&n.rhs)?;
        let rhs = self.type_store.get_type_id(n.rhs.mark()).unwrap();
        let rhs = self.type_store.get(rhs);
        let is_vaild = rhs.as_c_type().is_some_and(|x| {
            x.inputs.is_empty()
                && x.outputs
                    .first()
                    .is_some_and(|x| x.kind == PrimKind::ConStr)
        });
        if !is_vaild {
            // FIXME: Add diag
        }

        self.type_store.void(n.id);

        Ok(())
    }
    fn check_lit(&mut self, n: &'a Lit) -> EResult {
        match n {
            Lit::Int(token) => self.type_store.prim(PrimKind::Int, token.id),
            Lit::Float(token) => self.type_store.prim(PrimKind::Float, token.id),
            Lit::ConStr(token) => self.type_store.prim(PrimKind::ConStr, token.id),
        };
        Ok(())
    }
    fn check_expr_var(&mut self, n: &'a ExprVar) -> EResult {
        for scope in self.block.iter().rev() {
            if let Some(id) = scope.get(n.name.inner.as_str()) {
                let mut a_type = self.type_store.get(*id).clone();
                match &mut a_type {
                    AType::CType(ctype) => ctype.cst_id = n.id,
                    AType::Unknown(id) => *id = n.id,
                }
                self.type_store.a_type(a_type);

                return Ok(());
            }
        }
        // FIXME: Add diag
        self.type_store.unknown(n.id);
        Ok(())
    }
}
