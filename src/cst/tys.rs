use from_variant::FromVariant;
use is_macro::Is;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token<T> {
    pub id: usize,
    pub inner: T,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Id(pub usize);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct File {
    pub id: usize,
    pub decls: Vec<Decl>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Is, FromVariant)]
pub enum Decl {
    Var(DeclVar),
    Alias(DeclAlias),
    Func(DeclFunc),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DeclVar {
    pub id: usize,
    pub ty: Ty,
    pub name: Token<String>,
    pub init: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Is, FromVariant)]
pub enum Ty {
    Kind(Kind),
    Arrow(TyArrow),
    Array(TyArray),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TyArrow {
    pub id: usize,
    pub input: Kind,
    pub output: Box<Ty>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TyArray {
    pub id: usize,
    pub size: Token<usize>,
    pub kind: Kind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Is)]
pub enum Kind {
    Int(Id),
    Float(Id),
    Void(Id),
    Bool(Id),
    Alias(Alias),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Alias {
    pub id: usize,
    pub name: Token<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Is, FromVariant)]
pub enum Expr {
    Binary(ExprBinary),
    Assign(ExprAssign),
    Call(ExprCall),
    Lit(Lit),
    Var(ExprVar),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExprBinary {
    pub id: usize,
    pub op: Op,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Is)]
pub enum Op {
    Add(Id),
    Sub(Id),
    Mul(Id),
    Div(Id),
    OpMod(Id),
    PEO(Id),

    // Cmp
    Ne(Id),
    Eq(Id),
    Lt(Id),
    Gt(Id),
    Le(Id),
    Ge(Id),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExprCall {
    pub id: usize,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExprAssign {
    pub id: usize,
    pub var: Box<ExprVar>,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExprVar {
    pub id: usize,
    pub name: Token<String>,
    pub index: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Is, FromVariant)]
pub enum Lit {
    Int(Token<i32>),
    Float(Token<f32>),
    ConStr(Token<String>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DeclAlias {
    pub id: usize,
    pub name: Token<String>,
    pub kinds: Vec<Kind>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DeclFunc {
    pub id: usize,
    pub name: Token<String>,
    pub params: Vec<Param>,
    pub ret: Ty,
    pub block: StmtBlock,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Param {
    pub id: usize,
    pub ty: Ty,
    pub name: Token<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Is, FromVariant)]
pub enum Stmt {
    DeclVar(DeclVar),
    Branch(StmtBranch),
    Iter(StmtIter),
    Block(StmtBlock),
    AutoLet(StmtLet),
    Ret(StmtRet),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StmtBranch {
    pub id: usize,
    pub cond: Expr,
    pub then_branch: Option<Box<Stmt>>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StmtIter {
    pub id: usize,
    pub cond: Expr,
    pub then_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StmtBlock {
    pub id: usize,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StmtLet {
    pub id: usize,
    pub name: Token<String>,
    pub init: Expr,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StmtRet {
    pub id: usize,
    pub expr: Option<Expr>,
}
