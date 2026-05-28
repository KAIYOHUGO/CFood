use crate::cst::tys::*;

pub trait Visitor: Sized {
    type Res: Default;
    type Error;

    fn visit_file(&mut self, n: &File) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_decl(&mut self, n: &Decl) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_decl_var(&mut self, n: &DeclVar) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_kind(&mut self, n: &Kind) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_alias(&mut self, n: &Alias) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_expr(&mut self, n: &Expr) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_expr_binary(&mut self, n: &ExprBinary) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_op(&mut self, n: &Op) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_expr_call(&mut self, n: &ExprCall) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_expr_magic(&mut self, n: &ExprMagic) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_magic(&mut self, n: &Magic) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_expr_assign(&mut self, n: &ExprAssign) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_expr_var(&mut self, n: &ExprVar) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_expr_refer(&mut self, n: &ExprRefer) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_lit(&mut self, n: &ExprLit) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }

    fn visit_decl_alias(&mut self, n: &DeclAlias) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_decl_func(&mut self, n: &DeclFunc) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_param(&mut self, n: &Param) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }

    fn visit_stmt(&mut self, n: &Stmt) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_stmt_branch(&mut self, n: &StmtBranch) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_stmt_iter(&mut self, n: &StmtIter) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_stmt_block(&mut self, n: &StmtBlock) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_stmt_let(&mut self, n: &StmtLet) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
    fn visit_stmt_ret(&mut self, n: &StmtRet) -> Result<Self::Res, Self::Error> {
        n.visit(self)
    }
}

pub trait VisitAble {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor;
}

impl<TNode> VisitAble for Box<TNode>
where
    TNode: VisitAble,
{
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        (**self).visit(ctx)
    }
}

impl<TNode> VisitAble for Vec<TNode>
where
    TNode: VisitAble,
{
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        for n in self {
            n.visit(ctx)?;
        }
        Ok(T::Res::default())
    }
}

impl<TNode> VisitAble for Option<TNode>
where
    TNode: VisitAble,
{
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        if let Some(n) = self {
            n.visit(ctx)?;
        }
        Ok(T::Res::default())
    }
}

// -------------------------
// AST impls (logic here)
// -------------------------
impl VisitAble for File {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        for d in &self.decls {
            ctx.visit_decl(d)?;
        }
        Ok(T::Res::default())
    }
}

impl VisitAble for Decl {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        match self {
            Decl::Var(x) => ctx.visit_decl_var(x)?,
            Decl::Alias(x) => ctx.visit_decl_alias(x)?,
            Decl::Func(x) => ctx.visit_decl_func(x)?,
        };
        Ok(T::Res::default())
    }
}

impl VisitAble for DeclVar {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        ctx.visit_kind(&self.ty)?;
        if let Some(init) = &self.init {
            ctx.visit_expr(init)?;
        }
        Ok(T::Res::default())
    }
}

impl VisitAble for Kind {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        match self {
            Kind::Alias(a) => {
                ctx.visit_alias(a)?;
            }
            Kind::Int(_) | Kind::Float(_) | Kind::Void(_) | Kind::Bool(_) | Kind::ConStr(_) => {}
        };
        Ok(T::Res::default())
    }
}

impl VisitAble for Alias {
    fn visit<T>(&self, _ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        // ignore id / Token<String>
        Ok(T::Res::default())
    }
}

impl VisitAble for Expr {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        match self {
            Expr::Binary(x) => ctx.visit_expr_binary(x)?,
            Expr::Assign(x) => ctx.visit_expr_assign(x)?,
            Expr::Call(x) => ctx.visit_expr_call(x)?,
            Expr::Magic(x) => ctx.visit_expr_magic(x)?,
            Expr::Lit(x) => ctx.visit_lit(x)?,
            Expr::Var(x) => ctx.visit_expr_var(x)?,
            Expr::Refer(x) => ctx.visit_expr_refer(x)?,
        };
        Ok(T::Res::default())
    }
}

impl VisitAble for ExprBinary {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        ctx.visit_op(&self.op)?;
        ctx.visit_expr(&self.lhs)?;
        ctx.visit_expr(&self.rhs)?;
        Ok(T::Res::default())
    }
}

impl VisitAble for Op {
    fn visit<T>(&self, _ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        // ignore Id
        Ok(T::Res::default())
    }
}

impl VisitAble for ExprCall {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        ctx.visit_expr(&self.lhs)?;
        ctx.visit_expr(&self.rhs)?;
        Ok(T::Res::default())
    }
}

impl VisitAble for ExprMagic {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        ctx.visit_magic(&self.lhs)?;
        ctx.visit_expr(&self.rhs)?;
        Ok(T::Res::default())
    }
}

impl VisitAble for Magic {
    fn visit<T>(&self, _ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        // ignore Id
        Ok(T::Res::default())
    }
}

impl VisitAble for ExprAssign {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        ctx.visit_expr_var(&self.var)?;
        ctx.visit_expr(&self.rhs)?;
        Ok(T::Res::default())
    }
}

impl VisitAble for ExprVar {
    fn visit<T>(&self, _ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        Ok(T::Res::default())
    }
}

impl VisitAble for ExprRefer {
    fn visit<T>(&self, _ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        Ok(T::Res::default())
    }
}

impl VisitAble for ExprLit {
    fn visit<T>(&self, _ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        // ignore Token<T>
        Ok(T::Res::default())
    }
}

impl VisitAble for DeclAlias {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        for k in &self.kinds {
            ctx.visit_kind(k)?;
        }
        Ok(T::Res::default())
    }
}

impl VisitAble for DeclFunc {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        for p in &self.params {
            ctx.visit_param(p)?;
        }
        ctx.visit_kind(&self.ret)?;
        ctx.visit_stmt_block(&self.block)?;
        Ok(T::Res::default())
    }
}

impl VisitAble for Param {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        ctx.visit_kind(&self.ty)?;
        Ok(T::Res::default())
    }
}

impl VisitAble for Stmt {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        match self {
            Stmt::DeclVar(x) => ctx.visit_decl_var(x)?,
            Stmt::Branch(x) => ctx.visit_stmt_branch(x)?,
            Stmt::Iter(x) => ctx.visit_stmt_iter(x)?,
            Stmt::Block(x) => ctx.visit_stmt_block(x)?,
            Stmt::AutoLet(x) => ctx.visit_stmt_let(x)?,
            Stmt::Ret(x) => ctx.visit_stmt_ret(x)?,
            Stmt::Expr(x) => ctx.visit_expr(x)?,
        };
        Ok(T::Res::default())
    }
}

impl VisitAble for StmtBranch {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        ctx.visit_expr(&self.cond)?;
        if let Some(t) = &self.then_branch {
            ctx.visit_stmt(t)?;
        }
        if let Some(e) = &self.else_branch {
            ctx.visit_stmt(e)?;
        }
        Ok(T::Res::default())
    }
}

impl VisitAble for StmtIter {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        ctx.visit_expr(&self.cond)?;
        if let Some(t) = &self.then_branch {
            ctx.visit_stmt(t)?;
        }
        Ok(T::Res::default())
    }
}

impl VisitAble for StmtBlock {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        for s in &self.stmts {
            ctx.visit_stmt(s)?;
        }
        Ok(T::Res::default())
    }
}

impl VisitAble for StmtLet {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        ctx.visit_expr(&self.init)?;
        Ok(T::Res::default())
    }
}

impl VisitAble for StmtRet {
    fn visit<T>(&self, ctx: &mut T) -> Result<T::Res, T::Error>
    where
        T: Visitor,
    {
        if let Some(e) = &self.expr {
            ctx.visit_expr(e)?;
        }
        Ok(T::Res::default())
    }
}
