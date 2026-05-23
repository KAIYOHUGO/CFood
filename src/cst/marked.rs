use crate::cst::tys::*;

pub trait Marked {
    fn mark(&self) -> usize;
}

impl<T: Marked> Marked for Box<T> {
    fn mark(&self) -> usize {
        self.as_ref().mark()
    }
}

impl<T> Marked for Token<T> {
    fn mark(&self) -> usize {
        self.id
    }
}

impl Marked for Id {
    fn mark(&self) -> usize {
        self.0
    }
}

macro_rules! impl_marked_for_id {
    ($t:ty, $($r:ty),+) => {
        impl_marked_for_id!($t);
        impl_marked_for_id!($($r),+);
    };

    ($t:ty) => {
        impl Marked for $t {
            fn mark(&self) -> usize {
                self.id
            }
        }
    };
}

impl_marked_for_id![
    File, DeclVar, DeclAlias, DeclFunc, TyArrow, TyArray, Alias, ExprBinary, ExprCall, ExprAssign,
    ExprVar, Param, StmtBranch, StmtBlock, StmtIter, StmtLet, StmtRet
];

impl Marked for Ty {
    fn mark(&self) -> usize {
        match self {
            Ty::Kind(s) => s.mark(),
            Ty::Arrow(s) => s.mark(),
            Ty::Array(s) => s.mark(),
        }
    }
}

impl Marked for Kind {
    fn mark(&self) -> usize {
        match self {
            Kind::Int(s) => s.mark(),
            Kind::Float(s) => s.mark(),
            Kind::Void(s) => s.mark(),
            Kind::Bool(s) => s.mark(),
            Kind::Alias(s) => s.mark(),
        }
    }
}

impl Marked for Op {
    fn mark(&self) -> usize {
        match self {
            Op::Add(s) => s.mark(),
            Op::Sub(s) => s.mark(),
            Op::Mul(s) => s.mark(),
            Op::Div(s) => s.mark(),
            Op::OpMod(s) => s.mark(),
            Op::PEO(s) => s.mark(),
            Op::Ne(s) => s.mark(),
            Op::Eq(s) => s.mark(),
            Op::Lt(s) => s.mark(),
            Op::Gt(s) => s.mark(),
            Op::Le(s) => s.mark(),
            Op::Ge(s) => s.mark(),
        }
    }
}

impl Marked for Lit {
    fn mark(&self) -> usize {
        match self {
            Lit::Int(s) => s.mark(),
            Lit::Float(s) => s.mark(),
            Lit::ConStr(s) => s.mark(),
        }
    }
}

impl Marked for Decl {
    fn mark(&self) -> usize {
        match self {
            Decl::Var(s) => s.mark(),
            Decl::Alias(s) => s.mark(),
            Decl::Func(s) => s.mark(),
        }
    }
}

impl Marked for Expr {
    fn mark(&self) -> usize {
        match self {
            Expr::Binary(s) => s.mark(),
            Expr::Assign(s) => s.mark(),
            Expr::Call(s) => s.mark(),
            Expr::Lit(s) => s.mark(),
            Expr::Var(s) => s.mark(),
        }
    }
}

impl Marked for Stmt {
    fn mark(&self) -> usize {
        match self {
            Stmt::DeclVar(s) => s.mark(),
            Stmt::Branch(s) => s.mark(),
            Stmt::Iter(s) => s.mark(),
            Stmt::Block(s) => s.mark(),
            Stmt::AutoLet(s) => s.mark(),
            Stmt::Ret(s) => s.mark(),
            Stmt::Expr(s) => s.mark(),
        }
    }
}
