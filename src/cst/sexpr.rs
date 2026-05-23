use std::convert::Infallible;

use crate::cst::{SpanStore, Spanned, Visitor, tys::*};

pub struct CstToSexpr<'a>(&'a SpanStore);

impl<'a> CstToSexpr<'a> {
    pub fn new(span_store: &'a SpanStore) -> Self {
        Self(span_store)
    }
    fn list(tag: &str, items: Vec<String>) -> String {
        if items.is_empty() {
            format!("({})", tag)
        } else {
            format!("({} {})", tag, items.join(" "))
        }
    }

    fn atom_str(&self, s: &Token<String>) -> String {
        format!("{} \"{}\"", self.span(s), s.inner)
    }

    fn op_name(op: &Op) -> &'static str {
        match op {
            Op::Add(_) => "add",
            Op::Sub(_) => "sub",
            Op::Mul(_) => "mul",
            Op::Div(_) => "div",
            Op::OpMod(_) => "mod",
            Op::Ne(_) => "ne",
            Op::Eq(_) => "eq",
            Op::Lt(_) => "lt",
            Op::Gt(_) => "gt",
            Op::Le(_) => "le",
            Op::Ge(_) => "ge",
            Op::PEO(_) => "poe",
        }
    }

    fn span<T: Spanned>(&self, t: &T) -> String {
        match t.span(self.0) {
            Some(span) => format!(
                "(@span {} {} {} {})",
                span.start.line, span.start.col, span.end.line, span.end.col
            ),
            None => "".to_owned(),
        }
    }
}

impl<'a> Visitor for CstToSexpr<'a> {
    type Res = String;
    type Error = Infallible;

    fn visit_file(&mut self, n: &File) -> Result<Self::Res, Self::Error> {
        let decls = n
            .decls
            .iter()
            .map(|d| self.visit_decl(d))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!(
            "(file {} {})",
            self.span(n),
            Self::list("decls", decls)
        ))
    }

    fn visit_decl(&mut self, n: &Decl) -> Result<Self::Res, Self::Error> {
        let inner = match n {
            Decl::Var(x) => self.visit_decl_var(x)?,
            Decl::Alias(x) => self.visit_decl_alias(x)?,
            Decl::Func(x) => self.visit_decl_func(x)?,
        };
        Ok(format!("(decl {} {})", self.span(n), inner))
    }

    fn visit_decl_var(&mut self, n: &DeclVar) -> Result<Self::Res, Self::Error> {
        let ty = self.visit_ty(&n.ty)?;
        let name = self.atom_str(&n.name);
        let init = match &n.init {
            Some(e) => format!(" (init {})", self.visit_expr(e)?),
            None => String::new(),
        };
        Ok(format!(
            "(var {} (name {}) (ty {}){})",
            self.span(n),
            name,
            ty,
            init
        ))
    }

    fn visit_ty(&mut self, n: &Ty) -> Result<Self::Res, Self::Error> {
        match n {
            Ty::Kind(k) => Ok(format!("(ty {})", self.visit_kind(k)?)),
            Ty::Arrow(a) => Ok(format!("(ty {})", self.visit_ty_arrow(a)?)),
            Ty::Array(a) => Ok(format!("(ty {})", self.visit_ty_array(a)?)),
        }
    }

    fn visit_ty_arrow(&mut self, n: &TyArrow) -> Result<Self::Res, Self::Error> {
        let input = self.visit_kind(&n.input)?;
        let output = self.visit_ty(&n.output)?;
        Ok(format!(
            "(arrow {} (input {}) (output {}))",
            self.span(n),
            input,
            output
        ))
    }

    fn visit_ty_array(&mut self, n: &TyArray) -> Result<Self::Res, Self::Error> {
        let k = self.visit_kind(&n.kind)?;
        Ok(format!(
            "(array {} (size {}) (kind {}))",
            self.span(n),
            n.size.inner,
            k
        ))
    }

    fn visit_kind(&mut self, n: &Kind) -> Result<Self::Res, Self::Error> {
        let span = self.span(n);
        let s = match n {
            Kind::Int(_) => format!("(kind {span} int)"),
            Kind::Float(_) => format!("(kind {span} float)"),
            Kind::Void(_) => format!("(kind {span} void)"),
            Kind::Bool(_) => format!("(kind {span} bool)"),
            Kind::Alias(a) => format!("(kind {span} {})", self.visit_alias(a)?),
        };
        Ok(s)
    }

    fn visit_alias(&mut self, n: &Alias) -> Result<Self::Res, Self::Error> {
        Ok(format!("(alias {})", self.atom_str(&n.name)))
    }

    fn visit_decl_alias(&mut self, n: &DeclAlias) -> Result<Self::Res, Self::Error> {
        let kinds = n
            .kinds
            .iter()
            .map(|k| self.visit_kind(k))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!(
            "(alias (name {}) {})",
            self.atom_str(&n.name),
            Self::list("kinds", kinds)
        ))
    }

    fn visit_decl_func(&mut self, n: &DeclFunc) -> Result<Self::Res, Self::Error> {
        let params = n
            .params
            .iter()
            .map(|p| self.visit_param(p))
            .collect::<Result<Vec<_>, _>>()?;
        let ret = self.visit_ty(&n.ret)?;
        let block = self.visit_stmt_block(&n.block)?;
        Ok(format!(
            "(func {} (name {}) {} (ret {}) {})",
            self.span(n),
            self.atom_str(&n.name),
            Self::list("params", params),
            ret,
            block
        ))
    }

    fn visit_param(&mut self, n: &Param) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(param {} (name {}) (ty {}))",
            self.span(n),
            self.atom_str(&n.name),
            self.visit_ty(&n.ty)?
        ))
    }

    fn visit_stmt(&mut self, n: &Stmt) -> Result<Self::Res, Self::Error> {
        let inner = match n {
            Stmt::DeclVar(x) => self.visit_decl_var(x)?,
            Stmt::Branch(x) => self.visit_stmt_branch(x)?,
            Stmt::Iter(x) => self.visit_stmt_iter(x)?,
            Stmt::Block(x) => self.visit_stmt_block(x)?,
            Stmt::AutoLet(x) => self.visit_stmt_let(x)?,
            Stmt::Ret(x) => self.visit_stmt_ret(x)?,
            Stmt::Expr(x) => self.visit_expr(x)?,
        };
        Ok(format!("(stmt {} {})", self.span(n), inner))
    }

    fn visit_stmt_block(&mut self, n: &StmtBlock) -> Result<Self::Res, Self::Error> {
        let stmts = n
            .stmts
            .iter()
            .map(|s| self.visit_stmt(s))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!(
            "block {} {}",
            self.span(n),
            Self::list("stmts", stmts)
        ))
    }

    fn visit_stmt_branch(&mut self, n: &StmtBranch) -> Result<Self::Res, Self::Error> {
        let cond = self.visit_expr(&n.cond)?;
        let then_s = match &n.then_branch {
            Some(s) => format!(" (then {})", self.visit_stmt(s)?),
            None => String::new(),
        };
        let else_s = match &n.else_branch {
            Some(s) => format!(" (else {})", self.visit_stmt(s)?),
            None => String::new(),
        };
        Ok(format!(
            "(branch {} (cond {}){}{})",
            self.span(n),
            cond,
            then_s,
            else_s
        ))
    }

    fn visit_stmt_iter(&mut self, n: &StmtIter) -> Result<Self::Res, Self::Error> {
        let cond = self.visit_expr(&n.cond)?;
        let then_s = match &n.then_branch {
            Some(s) => format!(" (then {})", self.visit_stmt(s)?),
            None => String::new(),
        };
        Ok(format!("(iter {} (cond {}){})", self.span(n), cond, then_s))
    }

    fn visit_stmt_let(&mut self, n: &StmtLet) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(let {} (name {}) (init {}))",
            self.span(n),
            self.atom_str(&n.name),
            self.visit_expr(&n.init)?
        ))
    }

    fn visit_stmt_ret(&mut self, n: &StmtRet) -> Result<Self::Res, Self::Error> {
        let span = self.span(n);
        match &n.expr {
            Some(e) => Ok(format!("(ret {span} {})", self.visit_expr(e)?)),
            None => Ok("(ret {span})".to_string()),
        }
    }

    fn visit_expr(&mut self, n: &Expr) -> Result<Self::Res, Self::Error> {
        match n {
            Expr::Binary(x) => self.visit_expr_binary(x),
            Expr::Assign(x) => self.visit_expr_assign(x),
            Expr::Call(x) => self.visit_expr_call(x),
            Expr::Lit(x) => self.visit_lit(x),
            Expr::Var(x) => self.visit_expr_var(x),
        }
    }

    fn visit_expr_binary(&mut self, n: &ExprBinary) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(binary {} (op {}) (lhs {}) (rhs {}))",
            self.span(n),
            Self::op_name(&n.op),
            self.visit_expr(&n.lhs)?,
            self.visit_expr(&n.rhs)?
        ))
    }

    fn visit_expr_call(&mut self, n: &ExprCall) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(call {} (lhs {}) (rhs {}))",
            self.span(n),
            self.visit_expr(&n.lhs)?,
            self.visit_expr(&n.rhs)?
        ))
    }

    fn visit_expr_assign(&mut self, n: &ExprAssign) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(assign {} (var {}) (rhs {}))",
            self.span(n),
            self.visit_expr_var(&n.var)?,
            self.visit_expr(&n.rhs)?
        ))
    }

    fn visit_expr_var(&mut self, n: &ExprVar) -> Result<Self::Res, Self::Error> {
        let span = self.span(n);
        let name = self.atom_str(&n.name);
        match &n.index {
            Some(i) => Ok(format!(
                "(var {span} (name {}) (index {}))",
                name,
                self.visit_expr(i)?
            )),
            None => Ok(format!("(var {span} (name {}))", name)),
        }
    }

    fn visit_lit(&mut self, n: &Lit) -> Result<Self::Res, Self::Error> {
        let span = self.span(n);
        let s = match n {
            Lit::Int(t) => format!("(int {span} {})", t.inner),
            Lit::Float(t) => format!("(float {span} {})", t.inner),
            Lit::ConStr(t) => format!("(str {span} {})", self.atom_str(t)),
        };
        Ok(s)
    }

    fn visit_op(&mut self, n: &Op) -> Result<Self::Res, Self::Error> {
        Ok(format!("(op {} {})", self.span(n), Self::op_name(n)))
    }
}
