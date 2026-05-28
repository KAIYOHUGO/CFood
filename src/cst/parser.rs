use crate::{
    antlr::{cfoodlexer, cfoodparser::*, cfoodvisitor::CFoodVisitor},
    cst::{span::*, tys::*},
};
use dbt_antlr4::{
    errors::ANTLRError, parser_rule_context::ParserRuleContext, token::CommonToken, tree::NodeInner,
};
use from_variant::FromVariant;
use is_macro::Is;
use miette::MietteDiagnostic;
use unescaper::unescape;

pub fn parse_to_cst<'input: 'arena, 'arena>(
    node: &'arena CFoodParserNode<'input, 'arena>,
) -> Result<(File, SpanStore), ANTLRError> {
    let mut parser = Parser::default();
    let file = parser
        .visit_node(node)?
        .pop()
        .must_some()?
        .file()
        .must_some()?;
    Ok((file, parser.spanned))
}

#[derive(Debug, Default)]
struct Parser {
    count: usize,
    spanned: SpanStore,

    errors: Vec<MietteDiagnostic>,
}

impl Parser {
    fn get_id(&mut self) -> usize {
        let ret = self.count;
        self.count += 1;
        ret
    }

    fn get_id_with_ctx<'input: 'arena, 'arena, T: ParserRuleContext<'input, 'arena>>(
        &mut self,
        ctx: &T,
    ) -> usize {
        let start = CodeIndex {
            line: ctx.start().get_line() - 1,
            col: ctx.start().get_char_position_in_line() as u32,
            offset: ctx.start().get_start_index() as usize,
        };
        let end = CodeIndex {
            line: ctx.stop().get_line() - 1,
            col: ctx.stop().get_char_position_in_line() as u32 + ctx.stop().get_text().len() as u32,
            offset: ctx.stop().get_start_index() as usize + ctx.stop().get_text().len(),
        };
        let id = self.get_id();

        self.spanned.insert(id, start..end);
        id
    }

    fn get_id_with_symbol<'a>(&mut self, token: &dyn dbt_antlr4::token::Token) -> usize {
        let line = token.get_line() - 1;
        let start = CodeIndex {
            line,
            col: token.get_char_position_in_line() as u32,
            offset: token.get_start_index() as usize,
        };
        let end = CodeIndex {
            line,
            col: token.get_text().len() as u32 + start.col,
            offset: token.get_stop_index() as usize,
        };

        let id = self.get_id();

        self.spanned.insert(id, start..end);
        id
    }
}

#[derive(Debug, Is, FromVariant)]
enum AllType {
    File(File),
    Expr(Expr),
    Var(ExprVar),
    Decl(Decl),
    DeclVar(DeclVar),
    // DeclAlias(DeclAlias),
    // DeclFunc(DeclFunc),
    // TyArrow(TyArrow),
    Kind(Kind),
    // Alias(Alias),
    Param(Param),
    // ExprBinary(ExprBinary),
    // ExprCall(ExprCall),
    // ExprAssign(ExprAssign),
    Stmt(Stmt),
    // StmtBranch(StmtBranch),
    StmtBlock(StmtBlock),
    Op(Op),
    Lit(ExprLit),
}

trait MustSomeNode {
    type Res;
    fn must_some(self) -> Result<Self::Res, ANTLRError>;
}
impl<T> MustSomeNode for Option<T> {
    type Res = T;

    fn must_some(self) -> Result<Self::Res, ANTLRError> {
        self.ok_or_else(|| {
            #[cfg(debug_assertions)]
            panic!();

            ANTLRError::custom_error("This must be some, may cause by a malformed ast".to_owned())
        })
    }
}

macro_rules! bail_cst {
    () => {{
        #[cfg(debug_assertions)]
        panic!();

        return Err(ANTLRError::custom_error("Malformed ast".to_owned()));
    }};
}

impl<'input: 'arena, 'arena> CFoodVisitor<'input, 'arena> for Parser {
    type Return = Vec<AllType>;

    fn visit_file(
        &mut self,
        ctx: &'arena FileContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let decls = self
            .visit_children(ctx)?
            .into_iter()
            .map(|x| x.expect_decl())
            .collect();

        Ok(vec![
            File {
                id: self.get_id_with_ctx(ctx),
                decls,
            }
            .into(),
        ])
    }

    fn visit_decl(
        &mut self,
        ctx: &'arena DeclContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let all = self.visit_children(ctx)?.pop().must_some()?;
        let decl = match all {
            AllType::DeclVar(var) => var.into(),
            AllType::Decl(decl) => decl,
            _ => bail_cst!(),
        };
        Ok(vec![decl.into()])
    }
    fn visit_var_decl(
        &mut self,
        ctx: &'arena Var_declContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let ty = self
            .visit_var_decl_ty(ctx.var_decl_ty().must_some()?)?
            .pop()
            .must_some()?
            .expect_param();
        let init = self
            .visit_var_decl_init(ctx.var_decl_init().must_some()?)?
            .pop()
            .map(|x| x.expect_expr());

        Ok(vec![
            DeclVar {
                id: self.get_id_with_ctx(ctx),
                ty: ty.ty,
                name: ty.name,
                init,
            }
            .into(),
        ])
    }

    fn visit_var_decl_ty(
        &mut self,
        ctx: &'arena Var_decl_tyContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let ty = self
            .visit_node(ctx.ty_kind().must_some()?.as_node())?
            .pop()
            .must_some()?
            .expect_kind();

        let ident = ctx.IDENT().must_some()?;
        let name = Token {
            id: self.get_id_with_symbol(ident.symbol),
            inner: ident.symbol.text.to_owned(),
        };

        Ok(vec![
            Param {
                id: self.get_id_with_ctx(ctx),
                ty,
                name,
            }
            .into(),
        ])
    }

    fn visit_fn_decl(
        &mut self,
        ctx: &'arena Fn_declContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let ret = self
            .visit_node(ctx.ty_kind().must_some()?.as_node())?
            .pop()
            .must_some()?
            .expect_kind();
        let symbol = ctx.IDENT().must_some()?.symbol;
        let name = Token {
            id: self.get_id_with_symbol(symbol),
            inner: symbol.text.to_owned(),
        };

        let params = self
            .visit_params(ctx.params().must_some()?)?
            .into_iter()
            .map(|x| x.expect_param())
            .collect();
        let block = self
            .visit_block(ctx.block().must_some()?)?
            .pop()
            .must_some()?
            .expect_stmt_block();

        let decl: Decl = DeclFunc {
            id: self.get_id_with_ctx(ctx),
            name,
            params,
            ret,
            block,
        }
        .into();

        Ok(vec![decl.into()])
    }

    fn visit_ty_decl(
        &mut self,
        ctx: &'arena Ty_declContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let name = ctx.TYPE().must_some()?.symbol;
        let name = Token {
            id: self.get_id_with_symbol(name),
            inner: name.text.to_owned(),
        };
        let kinds = self
            .visit_children(ctx)?
            .into_iter()
            .map(|x| x.expect_kind())
            .collect();

        let decl: Decl = DeclAlias {
            id: self.get_id_with_ctx(ctx),
            name,
            kinds,
        }
        .into();

        Ok(vec![decl.into()])
    }

    fn visit_ty_kind_ty(
        &mut self,
        ctx: &'arena Ty_kind_tyContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let id = Id(self.get_id_with_symbol(ctx.start()));
        let kind = match ctx.start().get_token_type() {
            cfoodlexer::TY_int => Kind::Int(id),
            cfoodlexer::TY_float => Kind::Float(id),
            cfoodlexer::TY_str => Kind::ConStr(id),
            cfoodlexer::TY_void => Kind::Void(id),

            _ => bail_cst!(),
        };

        Ok(vec![kind.into()])
    }

    fn visit_ty_kind_type(
        &mut self,
        ctx: &'arena Ty_kind_typeContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let name = ctx.TYPE().must_some()?.symbol;
        let name = Token {
            id: self.get_id_with_symbol(name),
            inner: name.text.to_owned(),
        };

        let ty = Kind::Alias(Alias {
            id: self.get_id_with_ctx(ctx),
            name,
        });

        Ok(vec![ty.into()])
    }

    fn visit_stmt(
        &mut self,
        ctx: &'arena StmtContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let all = self
            .visit_children(ctx)?
            .pop()
            .map(|all| {
                let stmt = match all {
                    AllType::StmtBlock(block) => block.into(),
                    AllType::DeclVar(var) => var.into(),
                    AllType::Stmt(stmt) => stmt,
                    _ => bail_cst!(),
                };

                Ok(vec![stmt.into()])
            })
            .transpose()?;

        Ok(all.unwrap_or_default())
    }

    fn visit_block(
        &mut self,
        ctx: &'arena BlockContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let stmts = self
            .visit_children(ctx)?
            .into_iter()
            .map(|x| x.expect_stmt())
            .collect();

        let block = StmtBlock {
            id: self.get_id_with_ctx(ctx),
            stmts,
        };

        Ok(vec![block.into()])
    }

    fn visit_branch_stmt(
        &mut self,
        ctx: &'arena Branch_stmtContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let then_branch = self
            .visit_stmt(ctx.then_branch.must_some()?)?
            .pop()
            .map(|x| x.expect_stmt())
            .map(Box::new);
        let else_branch = ctx
            .else_branch
            .map(|x| self.visit_stmt(x))
            .transpose()?
            .map(|mut x| x.pop().map(|x| x.expect_stmt()))
            .flatten()
            .map(Box::new);
        let cond = self
            .visit_expr(ctx.expr().must_some()?)?
            .pop()
            .must_some()?
            .expect_expr();

        let stmt: Stmt = StmtBranch {
            id: self.get_id_with_ctx(ctx),
            cond,
            then_branch,
            else_branch,
        }
        .into();

        Ok(vec![stmt.into()])
    }

    fn visit_iter_stmt(
        &mut self,
        ctx: &'arena Iter_stmtContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let then_branch = self
            .visit_stmt(ctx.stmt().must_some()?)?
            .pop()
            .map(|x| x.expect_stmt())
            .map(Box::new);
        let cond = self
            .visit_expr(ctx.expr().must_some()?)?
            .pop()
            .must_some()?
            .expect_expr();

        let stmt: Stmt = StmtIter {
            id: self.get_id_with_ctx(ctx),
            cond,
            then_branch,
        }
        .into();

        Ok(vec![stmt.into()])
    }

    fn visit_return_stmt(
        &mut self,
        ctx: &'arena Return_stmtContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let expr = ctx
            .expr()
            .map(|x| self.visit_expr(x))
            .transpose()?
            .map(|mut x| x.pop().must_some())
            .transpose()?
            .map(|x| x.expect_expr());
        let stmt: Stmt = StmtRet {
            id: self.get_id_with_ctx(ctx),
            expr,
        }
        .into();

        Ok(vec![stmt.into()])
    }

    fn visit_let_stmt(
        &mut self,
        ctx: &'arena Let_stmtContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let init = self.visit_children(ctx)?.pop().must_some()?.expect_expr();
        let name = ctx.IDENT().must_some()?.symbol;
        let name = Token {
            id: self.get_id_with_symbol(name),
            inner: name.text.to_owned(),
        };
        let stmt: Stmt = StmtLet {
            id: self.get_id_with_ctx(ctx),
            name,
            init,
        }
        .into();

        Ok(vec![stmt.into()])
    }

    fn visit_expr_stmt(
        &mut self,
        ctx: &'arena Expr_stmtContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let expr = self.visit_children(ctx)?.pop().must_some()?.expect_expr();

        let stmt: Stmt = expr.into();

        Ok(vec![stmt.into()])
    }

    fn visit_assign_expr(
        &mut self,
        ctx: &'arena Assign_exprContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let var = Box::new(
            self.visit_var(ctx.var().unwrap())?
                .pop()
                .must_some()?
                .expect_var(),
        );

        let rhs = Box::new(
            self.visit_expr(ctx.expr().must_some()?)?
                .pop()
                .must_some()?
                .expect_expr(),
        );

        let expr: Expr = ExprAssign {
            id: self.get_id_with_ctx(ctx),
            var,
            rhs,
        }
        .into();

        Ok(vec![expr.into()])
    }

    fn visit_var(
        &mut self,
        ctx: &'arena VarContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let name = ctx.IDENT().must_some()?;
        let name = Token {
            id: self.get_id_with_symbol(name.symbol),
            inner: name.get_text(),
        };
        Ok(vec![
            ExprVar {
                id: self.get_id_with_ctx(ctx),
                name,
            }
            .into(),
        ])
    }

    fn visit_refer(
        &mut self,
        ctx: &'arena ReferContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let name = ctx.IDENT().must_some()?;
        let name = Token {
            id: self.get_id_with_symbol(name.symbol),
            inner: name.get_text(),
        };

        let expr: Expr = ExprRefer {
            id: self.get_id_with_ctx(ctx),
            name,
        }
        .into();
        Ok(vec![expr.into()])
    }

    fn visit_calc_expr_use(
        &mut self,
        ctx: &'arena Calc_expr_useContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let lhs = Box::new(
            self.visit_node(ctx.lhs.must_some()?.as_node())?
                .pop()
                .must_some()?
                .expect_expr(),
        );
        let rhs = Box::new(
            self.visit_node(ctx.rhs.must_some()?.as_node())?
                .pop()
                .must_some()?
                .expect_expr(),
        );
        let op = self
            .visit_cmp_preced_op(ctx.cmp_preced_op().must_some()?)?
            .pop()
            .must_some()?
            .expect_op();

        let expr: Expr = ExprBinary {
            id: self.get_id_with_ctx(ctx),
            op: op,
            lhs,
            rhs,
        }
        .into();
        Ok(vec![expr.into()])
    }

    fn visit_calc_expr_pass(
        &mut self,
        ctx: &'arena Calc_expr_passContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        self.visit_node(ctx.call_preced_expr().must_some()?.as_node())
    }

    fn visit_call_preced_expr_use(
        &mut self,
        ctx: &'arena Call_preced_expr_useContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let lhs = Box::new(
            self.visit_node(ctx.add_preced_expr().must_some()?.as_node())?
                .pop()
                .must_some()?
                .expect_expr(),
        );
        let rhs = Box::new(
            self.visit_node(ctx.call_preced_expr().must_some()?.as_node())?
                .pop()
                .must_some()?
                .expect_expr(),
        );

        let expr: Expr = ExprCall {
            id: self.get_id_with_ctx(ctx),
            lhs,
            rhs,
        }
        .into();
        Ok(vec![expr.into()])
    }

    fn visit_call_preced_expr_magic(
        &mut self,
        ctx: &'arena Call_preced_expr_magicContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let magic = ctx.magic().must_some()?.start();
        let lhs_id = Id(self.get_id_with_symbol(magic));
        let lhs = match magic.get_token_type() {
            cfoodlexer::MAGIC_printf => Magic::Printf(lhs_id),
            cfoodlexer::MAGIC_scanf => Magic::Scanf(lhs_id),
            _ => bail_cst!(),
        };
        let rhs = Box::new(
            self.visit_node(ctx.call_preced_expr().must_some()?.as_node())?
                .pop()
                .must_some()?
                .expect_expr(),
        );

        let expr: Expr = ExprMagic {
            id: self.get_id_with_ctx(ctx),
            lhs,
            rhs,
        }
        .into();
        Ok(vec![expr.into()])
    }

    fn visit_call_preced_expr_pass(
        &mut self,
        ctx: &'arena Call_preced_expr_passContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        self.visit_node(ctx.add_preced_expr().must_some()?.as_node())
    }

    fn visit_add_preced_expr_use(
        &mut self,
        ctx: &'arena Add_preced_expr_useContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let lhs = Box::new(
            self.visit_node(ctx.mul_preced_expr().must_some()?.as_node())?
                .pop()
                .must_some()?
                .expect_expr(),
        );
        let rhs = Box::new(
            self.visit_node(ctx.add_preced_expr().must_some()?.as_node())?
                .pop()
                .must_some()?
                .expect_expr(),
        );
        let op = self
            .visit_add_preced_op(ctx.add_preced_op().must_some()?)?
            .pop()
            .must_some()?
            .expect_op();

        let expr: Expr = ExprBinary {
            id: self.get_id_with_ctx(ctx),
            op: op,
            lhs,
            rhs,
        }
        .into();
        Ok(vec![expr.into()])
    }

    fn visit_add_preced_expr_pass(
        &mut self,
        ctx: &'arena Add_preced_expr_passContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        self.visit_node(ctx.mul_preced_expr().must_some()?.as_node())
    }

    fn visit_mul_preced_expr_use(
        &mut self,
        ctx: &'arena Mul_preced_expr_useContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let lhs = Box::new(
            self.visit_node(ctx.atom_preced_expr().must_some()?.as_node())?
                .pop()
                .must_some()?
                .expect_expr(),
        );
        let rhs = Box::new(
            self.visit_node(ctx.mul_preced_expr().must_some()?.as_node())?
                .pop()
                .must_some()?
                .expect_expr(),
        );
        let op = self
            .visit_mul_preced_op(ctx.mul_preced_op().must_some()?)?
            .pop()
            .must_some()?
            .expect_op();

        let expr: Expr = ExprBinary {
            id: self.get_id_with_ctx(ctx),
            op: op,
            lhs,
            rhs,
        }
        .into();
        Ok(vec![expr.into()])
    }

    fn visit_mul_preced_expr_pass(
        &mut self,
        ctx: &'arena Mul_preced_expr_passContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        self.visit_node(ctx.atom_preced_expr().must_some()?.as_node())
    }

    fn visit_atom_preced_expr_var(
        &mut self,
        ctx: &'arena Atom_preced_expr_varContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let var = self
            .visit_var(ctx.var().must_some()?)?
            .pop()
            .must_some()?
            .expect_var();
        let expr: Expr = var.into();

        Ok(vec![expr.into()])
    }

    fn visit_atom_preced_expr_lit(
        &mut self,
        ctx: &'arena Atom_preced_expr_litContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let lit = self
            .visit_node(ctx.lit().must_some()?.as_node())?
            .pop()
            .must_some()?
            .expect_lit();

        let expr: Expr = lit.into();
        Ok(vec![expr.into()])
    }

    fn visit_cmp_preced_op(
        &mut self,
        ctx: &'arena Cmp_preced_opContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let id = Id(self.get_id_with_ctx(ctx));
        let op = match ctx.start().get_token_type() {
            cfoodlexer::NE => Op::Ne(id),
            cfoodlexer::EQ => Op::Eq(id),
            cfoodlexer::LT => Op::Lt(id),
            cfoodlexer::GT => Op::Gt(id),
            cfoodlexer::LE => Op::Le(id),
            cfoodlexer::GE => Op::Ge(id),
            _ => bail_cst!(),
        };
        Ok(vec![op.into()])
    }

    fn visit_add_preced_op(
        &mut self,
        ctx: &'arena Add_preced_opContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let id = Id(self.get_id_with_ctx(ctx));
        let op = match ctx.start().get_token_type() {
            cfoodlexer::PLUS => Op::Add(id),
            cfoodlexer::SUB => Op::Sub(id),
            _ => bail_cst!(),
        };
        Ok(vec![op.into()])
    }

    fn visit_mul_preced_op(
        &mut self,
        ctx: &'arena Mul_preced_opContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let id = Id(self.get_id_with_symbol(ctx.start()));
        let op = match ctx.start().get_token_type() {
            cfoodlexer::MUL => Op::Mul(id),
            cfoodlexer::DIV => Op::Div(id),
            cfoodlexer::MOD => Op::OpMod(id),
            cfoodlexer::PEO => Op::PEO(id),
            _ => bail_cst!(),
        };
        Ok(vec![op.into()])
    }

    fn visit_args(
        &mut self,
        ctx: &'arena ArgsContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let Some(expr) = ctx.expr() else {
            return Ok(Default::default());
        };
        let mut lhs = self.visit_expr(expr)?;

        match ctx.args() {
            Some(args) => {
                let lhs = Box::new(lhs.pop().must_some()?.expect_expr());
                let rhs = Box::new(self.visit_args(args)?.pop().must_some()?.expect_expr());

                let expr: Expr = ExprCall {
                    id: self.get_id_with_ctx(ctx),
                    lhs,
                    rhs,
                }
                .into();

                Ok(vec![expr.into()])
            }
            None => Ok(lhs),
        }
    }

    fn visit_lit_int(
        &mut self,
        ctx: &'arena Lit_intContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let int = ctx
            .start()
            .get_text()
            .parse::<i64>()
            .map_err(|e| ANTLRError::custom_error(e.to_string()))?;
        let lit: ExprLit = Token {
            id: self.get_id_with_ctx(ctx),
            inner: int,
        }
        .into();

        Ok(vec![lit.into()])
    }

    fn visit_lit_float(
        &mut self,
        ctx: &'arena Lit_floatContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let float = ctx
            .start()
            .get_text()
            .parse::<f64>()
            .map_err(|e| ANTLRError::custom_error(e.to_string()))?;
        let lit: ExprLit = Token {
            id: self.get_id_with_ctx(ctx),
            inner: float,
        }
        .into();

        Ok(vec![lit.into()])
    }

    fn visit_lit_constr(
        &mut self,
        ctx: &'arena Lit_constrContext<'input, 'arena, CommonToken<'input>>,
    ) -> Result<Self::Return, ANTLRError> {
        let symbol = ctx.CONSTR().must_some()?.symbol;
        let con_str = symbol.text[1..symbol.text.len() - 1].to_owned();
        let lit: ExprLit = Token {
            id: self.get_id_with_symbol(symbol),
            // FIXME: add diag
            inner: unescape(&con_str).map_err(|e| ANTLRError::custom_error(e.to_string()))?,
        }
        .into();

        Ok(vec![lit.into()])
    }

    fn aggregate_results(
        &self,
        mut aggregate: Self::Return,
        next: Self::Return,
    ) -> Result<Self::Return, ANTLRError> {
        aggregate.extend(next);
        Ok(aggregate)
    }
}
