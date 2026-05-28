use std::convert::Infallible;

use crate::cst::{Marked, SpanStore, Spanned, Visitor, tys::*};

pub struct CstToSexpr<'a>(&'a SpanStore, Vec<&'a dyn ExtraCstInfo>);

pub trait ExtraCstInfo {
    fn get_info(&self, cst_id: usize) -> Option<String>;
}

impl<'a> CstToSexpr<'a> {
    pub fn new(span_store: &'a SpanStore, extra: Vec<&'a dyn ExtraCstInfo>) -> Self {
        Self(span_store, extra)
    }
    fn list(tag: &str, items: Vec<String>) -> String {
        if items.is_empty() {
            format!("({})", tag)
        } else {
            format!("({} {})", tag, items.join(" "))
        }
    }

    fn atom_str(&self, s: &Token<String>) -> String {
        format!("{} {} \"{}\"", self.span(s), self.extra_marked(s), s.inner)
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

    fn extra(&self, cst_id: usize) -> String {
        self.1
            .iter()
            .filter_map(|x| x.get_info(cst_id))
            .fold("".to_owned(), |acc, x| format!("{acc} {x}"))
    }

    fn extra_marked<T: Marked>(&self, t: &T) -> String {
        self.extra(t.mark())
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
            "(file {}{} {})",
            self.span(n),
            self.extra(n.id),
            Self::list("decls", decls)
        ))
    }

    fn visit_decl(&mut self, n: &Decl) -> Result<Self::Res, Self::Error> {
        let inner = match n {
            Decl::Var(x) => self.visit_decl_var(x)?,
            Decl::Alias(x) => self.visit_decl_alias(x)?,
            Decl::Func(x) => self.visit_decl_func(x)?,
        };
        Ok(format!(
            "(decl {}{} {})",
            self.span(n),
            self.extra_marked(n),
            inner
        ))
    }

    fn visit_decl_var(&mut self, n: &DeclVar) -> Result<Self::Res, Self::Error> {
        let ty = self.visit_kind(&n.ty)?;
        let name = self.atom_str(&n.name);
        let init = match &n.init {
            Some(e) => format!(" (init {})", self.visit_expr(e)?),
            None => String::new(),
        };
        Ok(format!(
            "(decl-var {}{} (name {}) (ty {}){})",
            self.span(n),
            self.extra(n.id),
            name,
            ty,
            init
        ))
    }

    fn visit_kind(&mut self, n: &Kind) -> Result<Self::Res, Self::Error> {
        let span = self.span(n);
        let s = match n {
            Kind::Int(id) => format!("(kind {span}{} int)", self.extra(id.0)),
            Kind::Float(id) => format!("(kind {span}{} float)", self.extra(id.0)),
            Kind::Void(id) => format!("(kind {span}{} void)", self.extra(id.0)),
            Kind::Bool(id) => format!("(kind {span}{} bool)", self.extra(id.0)),
            Kind::ConStr(id) => format!("(kind {span}{} str)", self.extra(id.0)),
            Kind::Alias(a) => format!("(kind {span}{} {})", self.extra(a.id), self.visit_alias(a)?),
        };
        Ok(s)
    }

    fn visit_alias(&mut self, n: &Alias) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(alias {}{} {})",
            self.span(n),
            self.extra(n.id),
            self.atom_str(&n.name)
        ))
    }

    fn visit_decl_alias(&mut self, n: &DeclAlias) -> Result<Self::Res, Self::Error> {
        let kinds = n
            .kinds
            .iter()
            .map(|k| self.visit_kind(k))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!(
            "(alias {}{} (name {}) {})",
            self.span(n),
            self.extra(n.id),
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
        let ret = self.visit_kind(&n.ret)?;
        let block = self.visit_stmt_block(&n.block)?;
        Ok(format!(
            "(func {}{} (name {}) {} (ret {}) {})",
            self.span(n),
            self.extra(n.id),
            self.atom_str(&n.name),
            Self::list("params", params),
            ret,
            block
        ))
    }

    fn visit_param(&mut self, n: &Param) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(param {}{} (name {}) (ty {}))",
            self.span(n),
            self.extra(n.id),
            self.atom_str(&n.name),
            self.visit_kind(&n.ty)?
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
        Ok(format!(
            "(stmt {}{} {})",
            self.span(n),
            self.extra_marked(n),
            inner
        ))
    }

    fn visit_stmt_block(&mut self, n: &StmtBlock) -> Result<Self::Res, Self::Error> {
        let stmts = n
            .stmts
            .iter()
            .map(|s| self.visit_stmt(s))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!(
            "(block {}{} {})",
            self.span(n),
            self.extra(n.id),
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
            "(branch {}{} (cond {}){}{})",
            self.span(n),
            self.extra(n.id),
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
        Ok(format!(
            "(iter {}{} (cond {}){})",
            self.span(n),
            self.extra(n.id),
            cond,
            then_s
        ))
    }

    fn visit_stmt_let(&mut self, n: &StmtLet) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(let {}{} (name {}) (init {}))",
            self.span(n),
            self.extra(n.id),
            self.atom_str(&n.name),
            self.visit_expr(&n.init)?
        ))
    }

    fn visit_stmt_ret(&mut self, n: &StmtRet) -> Result<Self::Res, Self::Error> {
        let span = self.span(n);
        let extra = self.extra(n.id);
        match &n.expr {
            Some(e) => Ok(format!("(ret {span}{extra} {})", self.visit_expr(e)?)),
            None => Ok(format!("(ret {span}{extra})")),
        }
    }

    fn visit_expr_binary(&mut self, n: &ExprBinary) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(binary {}{} (op {}) (lhs {}) (rhs {}))",
            self.span(n),
            self.extra(n.id),
            Self::op_name(&n.op),
            self.visit_expr(&n.lhs)?,
            self.visit_expr(&n.rhs)?
        ))
    }

    fn visit_expr_call(&mut self, n: &ExprCall) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(call {}{} (lhs {}) (rhs {}))",
            self.span(n),
            self.extra(n.id),
            self.visit_expr(&n.lhs)?,
            self.visit_expr(&n.rhs)?
        ))
    }

    fn visit_expr_magic(&mut self, n: &ExprMagic) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(expr-magic {}{} (lhs {}) (rhs {}))",
            self.span(n),
            self.extra(n.id),
            self.visit_magic(&n.lhs)?,
            self.visit_expr(&n.rhs)?
        ))
    }

    fn visit_magic(&mut self, n: &Magic) -> Result<Self::Res, Self::Error> {
        Ok(match n {
            Magic::Printf(id) => format!("(magic printf {}{})", self.span(id), self.extra(id.0)),
            Magic::Scanf(id) => format!("(magic scanf {}{})", self.span(id), self.extra(id.0)),
        })
    }

    fn visit_expr_assign(&mut self, n: &ExprAssign) -> Result<Self::Res, Self::Error> {
        Ok(format!(
            "(assign {}{} (var {}) (rhs {}))",
            self.span(n),
            self.extra(n.id),
            self.visit_expr_var(&n.var)?,
            self.visit_expr(&n.rhs)?
        ))
    }

    fn visit_expr_var(&mut self, n: &ExprVar) -> Result<Self::Res, Self::Error> {
        let span = self.span(n);
        let name = self.atom_str(&n.name);
        Ok(format!("(var {span}{} (name {}))", self.extra(n.id), name))
    }

    fn visit_expr_refer(&mut self, n: &ExprRefer) -> Result<Self::Res, Self::Error> {
        let span = self.span(n);
        let name = self.atom_str(&n.name);
        Ok(format!(
            "(refer {span}{} (name {}))",
            self.extra(n.id),
            name
        ))
    }

    fn visit_lit(&mut self, n: &ExprLit) -> Result<Self::Res, Self::Error> {
        let span = self.span(n);
        let s = match n {
            ExprLit::Int(t) => format!("(int {span}{} {})", self.extra(t.id), t.inner),
            ExprLit::Float(t) => format!("(float {span}{} {})", self.extra(t.id), t.inner),
            ExprLit::ConStr(t) => format!("(str {})", self.atom_str(t)),
        };
        Ok(s)
    }

    fn visit_op(&mut self, n: &Op) -> Result<Self::Res, Self::Error> {
        let extra = match n {
            Op::Add(id)
            | Op::Sub(id)
            | Op::Mul(id)
            | Op::Div(id)
            | Op::OpMod(id)
            | Op::PEO(id)
            | Op::Ne(id)
            | Op::Eq(id)
            | Op::Lt(id)
            | Op::Gt(id)
            | Op::Le(id)
            | Op::Ge(id) => self.extra(id.0),
        };
        Ok(format!(
            "(op {}{} {})",
            self.span(n),
            extra,
            Self::op_name(n)
        ))
    }
}
