#![allow(nonstandard_style)]
// Generated from ./CFood.g4 by ANTLR 4.13.2
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::token::{CommonToken, Token};
use dbt_antlr4::tree::ParseTreeListener;
use super::cfoodparser::*;

pub trait CFoodListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, CFoodParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link CFoodParser#file}.
    /// @param ctx the parse tree
    fn enter_file<'input: 'arena>(&mut self, _ctx: &FileContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#file}.
    /// @param ctx the parse tree
    fn exit_file<'input: 'arena>(&mut self, _ctx: &FileContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#decls}.
    /// @param ctx the parse tree
    fn enter_decls<'input: 'arena>(&mut self, _ctx: &DeclsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#decls}.
    /// @param ctx the parse tree
    fn exit_decls<'input: 'arena>(&mut self, _ctx: &DeclsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#decl}.
    /// @param ctx the parse tree
    fn enter_decl<'input: 'arena>(&mut self, _ctx: &DeclContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#decl}.
    /// @param ctx the parse tree
    fn exit_decl<'input: 'arena>(&mut self, _ctx: &DeclContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#var_decl}.
    /// @param ctx the parse tree
    fn enter_var_decl<'input: 'arena>(&mut self, _ctx: &Var_declContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#var_decl}.
    /// @param ctx the parse tree
    fn exit_var_decl<'input: 'arena>(&mut self, _ctx: &Var_declContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#var_decl_ty}.
    /// @param ctx the parse tree
    fn enter_var_decl_ty<'input: 'arena>(&mut self, _ctx: &Var_decl_tyContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#var_decl_ty}.
    /// @param ctx the parse tree
    fn exit_var_decl_ty<'input: 'arena>(&mut self, _ctx: &Var_decl_tyContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#var_decl_init}.
    /// @param ctx the parse tree
    fn enter_var_decl_init<'input: 'arena>(&mut self, _ctx: &Var_decl_initContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#var_decl_init}.
    /// @param ctx the parse tree
    fn exit_var_decl_init<'input: 'arena>(&mut self, _ctx: &Var_decl_initContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#fn_decl}.
    /// @param ctx the parse tree
    fn enter_fn_decl<'input: 'arena>(&mut self, _ctx: &Fn_declContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#fn_decl}.
    /// @param ctx the parse tree
    fn exit_fn_decl<'input: 'arena>(&mut self, _ctx: &Fn_declContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#ty_decl}.
    /// @param ctx the parse tree
    fn enter_ty_decl<'input: 'arena>(&mut self, _ctx: &Ty_declContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#ty_decl}.
    /// @param ctx the parse tree
    fn exit_ty_decl<'input: 'arena>(&mut self, _ctx: &Ty_declContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#params}.
    /// @param ctx the parse tree
    fn enter_params<'input: 'arena>(&mut self, _ctx: &ParamsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#params}.
    /// @param ctx the parse tree
    fn exit_params<'input: 'arena>(&mut self, _ctx: &ParamsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#param_list}.
    /// @param ctx the parse tree
    fn enter_param_list<'input: 'arena>(&mut self, _ctx: &Param_listContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#param_list}.
    /// @param ctx the parse tree
    fn exit_param_list<'input: 'arena>(&mut self, _ctx: &Param_listContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#param}.
    /// @param ctx the parse tree
    fn enter_param<'input: 'arena>(&mut self, _ctx: &ParamContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#param}.
    /// @param ctx the parse tree
    fn exit_param<'input: 'arena>(&mut self, _ctx: &ParamContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code lit_int}
    /// labeled alternative in {@link CFoodParser#lit}.
    /// @param ctx the parse tree
    fn enter_lit_int<'input: 'arena>(&mut self, _ctx: &Lit_intContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code lit_int}
    /// labeled alternative in {@link CFoodParser#lit}.
    /// @param ctx the parse tree
    fn exit_lit_int<'input: 'arena>(&mut self, _ctx: &Lit_intContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code lit_float}
    /// labeled alternative in {@link CFoodParser#lit}.
    /// @param ctx the parse tree
    fn enter_lit_float<'input: 'arena>(&mut self, _ctx: &Lit_floatContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code lit_float}
    /// labeled alternative in {@link CFoodParser#lit}.
    /// @param ctx the parse tree
    fn exit_lit_float<'input: 'arena>(&mut self, _ctx: &Lit_floatContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code lit_constr}
    /// labeled alternative in {@link CFoodParser#lit}.
    /// @param ctx the parse tree
    fn enter_lit_constr<'input: 'arena>(&mut self, _ctx: &Lit_constrContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code lit_constr}
    /// labeled alternative in {@link CFoodParser#lit}.
    /// @param ctx the parse tree
    fn exit_lit_constr<'input: 'arena>(&mut self, _ctx: &Lit_constrContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#tys}.
    /// @param ctx the parse tree
    fn enter_tys<'input: 'arena>(&mut self, _ctx: &TysContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#tys}.
    /// @param ctx the parse tree
    fn exit_tys<'input: 'arena>(&mut self, _ctx: &TysContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code ty_kind_ty}
    /// labeled alternative in {@link CFoodParser#ty_kind}.
    /// @param ctx the parse tree
    fn enter_ty_kind_ty<'input: 'arena>(&mut self, _ctx: &Ty_kind_tyContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code ty_kind_ty}
    /// labeled alternative in {@link CFoodParser#ty_kind}.
    /// @param ctx the parse tree
    fn exit_ty_kind_ty<'input: 'arena>(&mut self, _ctx: &Ty_kind_tyContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code ty_kind_type}
    /// labeled alternative in {@link CFoodParser#ty_kind}.
    /// @param ctx the parse tree
    fn enter_ty_kind_type<'input: 'arena>(&mut self, _ctx: &Ty_kind_typeContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code ty_kind_type}
    /// labeled alternative in {@link CFoodParser#ty_kind}.
    /// @param ctx the parse tree
    fn exit_ty_kind_type<'input: 'arena>(&mut self, _ctx: &Ty_kind_typeContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#block}.
    /// @param ctx the parse tree
    fn enter_block<'input: 'arena>(&mut self, _ctx: &BlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#block}.
    /// @param ctx the parse tree
    fn exit_block<'input: 'arena>(&mut self, _ctx: &BlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#stmts}.
    /// @param ctx the parse tree
    fn enter_stmts<'input: 'arena>(&mut self, _ctx: &StmtsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#stmts}.
    /// @param ctx the parse tree
    fn exit_stmts<'input: 'arena>(&mut self, _ctx: &StmtsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#stmt}.
    /// @param ctx the parse tree
    fn enter_stmt<'input: 'arena>(&mut self, _ctx: &StmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#stmt}.
    /// @param ctx the parse tree
    fn exit_stmt<'input: 'arena>(&mut self, _ctx: &StmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#expr_stmt}.
    /// @param ctx the parse tree
    fn enter_expr_stmt<'input: 'arena>(&mut self, _ctx: &Expr_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#expr_stmt}.
    /// @param ctx the parse tree
    fn exit_expr_stmt<'input: 'arena>(&mut self, _ctx: &Expr_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#branch_stmt}.
    /// @param ctx the parse tree
    fn enter_branch_stmt<'input: 'arena>(&mut self, _ctx: &Branch_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#branch_stmt}.
    /// @param ctx the parse tree
    fn exit_branch_stmt<'input: 'arena>(&mut self, _ctx: &Branch_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#iter_stmt}.
    /// @param ctx the parse tree
    fn enter_iter_stmt<'input: 'arena>(&mut self, _ctx: &Iter_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#iter_stmt}.
    /// @param ctx the parse tree
    fn exit_iter_stmt<'input: 'arena>(&mut self, _ctx: &Iter_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#inline_stmts}.
    /// @param ctx the parse tree
    fn enter_inline_stmts<'input: 'arena>(&mut self, _ctx: &Inline_stmtsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#inline_stmts}.
    /// @param ctx the parse tree
    fn exit_inline_stmts<'input: 'arena>(&mut self, _ctx: &Inline_stmtsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#inline_stmt}.
    /// @param ctx the parse tree
    fn enter_inline_stmt<'input: 'arena>(&mut self, _ctx: &Inline_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#inline_stmt}.
    /// @param ctx the parse tree
    fn exit_inline_stmt<'input: 'arena>(&mut self, _ctx: &Inline_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#for_stmt}.
    /// @param ctx the parse tree
    fn enter_for_stmt<'input: 'arena>(&mut self, _ctx: &For_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#for_stmt}.
    /// @param ctx the parse tree
    fn exit_for_stmt<'input: 'arena>(&mut self, _ctx: &For_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#return_stmt}.
    /// @param ctx the parse tree
    fn enter_return_stmt<'input: 'arena>(&mut self, _ctx: &Return_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#return_stmt}.
    /// @param ctx the parse tree
    fn exit_return_stmt<'input: 'arena>(&mut self, _ctx: &Return_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#let_stmt}.
    /// @param ctx the parse tree
    fn enter_let_stmt<'input: 'arena>(&mut self, _ctx: &Let_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#let_stmt}.
    /// @param ctx the parse tree
    fn exit_let_stmt<'input: 'arena>(&mut self, _ctx: &Let_stmtContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#var}.
    /// @param ctx the parse tree
    fn enter_var<'input: 'arena>(&mut self, _ctx: &VarContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#var}.
    /// @param ctx the parse tree
    fn exit_var<'input: 'arena>(&mut self, _ctx: &VarContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#refer}.
    /// @param ctx the parse tree
    fn enter_refer<'input: 'arena>(&mut self, _ctx: &ReferContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#refer}.
    /// @param ctx the parse tree
    fn exit_refer<'input: 'arena>(&mut self, _ctx: &ReferContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#expr}.
    /// @param ctx the parse tree
    fn enter_expr<'input: 'arena>(&mut self, _ctx: &ExprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#expr}.
    /// @param ctx the parse tree
    fn exit_expr<'input: 'arena>(&mut self, _ctx: &ExprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_assign_pass}
    /// labeled alternative in {@link CFoodParser#expr_assign}.
    /// @param ctx the parse tree
    fn enter_expr_assign_pass<'input: 'arena>(&mut self, _ctx: &Expr_assign_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_assign_pass}
    /// labeled alternative in {@link CFoodParser#expr_assign}.
    /// @param ctx the parse tree
    fn exit_expr_assign_pass<'input: 'arena>(&mut self, _ctx: &Expr_assign_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_assign_use}
    /// labeled alternative in {@link CFoodParser#expr_assign}.
    /// @param ctx the parse tree
    fn enter_expr_assign_use<'input: 'arena>(&mut self, _ctx: &Expr_assign_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_assign_use}
    /// labeled alternative in {@link CFoodParser#expr_assign}.
    /// @param ctx the parse tree
    fn exit_expr_assign_use<'input: 'arena>(&mut self, _ctx: &Expr_assign_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_logic_pass}
    /// labeled alternative in {@link CFoodParser#expr_logic}.
    /// @param ctx the parse tree
    fn enter_expr_logic_pass<'input: 'arena>(&mut self, _ctx: &Expr_logic_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_logic_pass}
    /// labeled alternative in {@link CFoodParser#expr_logic}.
    /// @param ctx the parse tree
    fn exit_expr_logic_pass<'input: 'arena>(&mut self, _ctx: &Expr_logic_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_logic_use}
    /// labeled alternative in {@link CFoodParser#expr_logic}.
    /// @param ctx the parse tree
    fn enter_expr_logic_use<'input: 'arena>(&mut self, _ctx: &Expr_logic_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_logic_use}
    /// labeled alternative in {@link CFoodParser#expr_logic}.
    /// @param ctx the parse tree
    fn exit_expr_logic_use<'input: 'arena>(&mut self, _ctx: &Expr_logic_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_cmp_pass}
    /// labeled alternative in {@link CFoodParser#expr_cmp}.
    /// @param ctx the parse tree
    fn enter_expr_cmp_pass<'input: 'arena>(&mut self, _ctx: &Expr_cmp_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_cmp_pass}
    /// labeled alternative in {@link CFoodParser#expr_cmp}.
    /// @param ctx the parse tree
    fn exit_expr_cmp_pass<'input: 'arena>(&mut self, _ctx: &Expr_cmp_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_cmp_use}
    /// labeled alternative in {@link CFoodParser#expr_cmp}.
    /// @param ctx the parse tree
    fn enter_expr_cmp_use<'input: 'arena>(&mut self, _ctx: &Expr_cmp_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_cmp_use}
    /// labeled alternative in {@link CFoodParser#expr_cmp}.
    /// @param ctx the parse tree
    fn exit_expr_cmp_use<'input: 'arena>(&mut self, _ctx: &Expr_cmp_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_add_pass}
    /// labeled alternative in {@link CFoodParser#expr_add}.
    /// @param ctx the parse tree
    fn enter_expr_add_pass<'input: 'arena>(&mut self, _ctx: &Expr_add_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_add_pass}
    /// labeled alternative in {@link CFoodParser#expr_add}.
    /// @param ctx the parse tree
    fn exit_expr_add_pass<'input: 'arena>(&mut self, _ctx: &Expr_add_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_add_use}
    /// labeled alternative in {@link CFoodParser#expr_add}.
    /// @param ctx the parse tree
    fn enter_expr_add_use<'input: 'arena>(&mut self, _ctx: &Expr_add_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_add_use}
    /// labeled alternative in {@link CFoodParser#expr_add}.
    /// @param ctx the parse tree
    fn exit_expr_add_use<'input: 'arena>(&mut self, _ctx: &Expr_add_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_mul_pass}
    /// labeled alternative in {@link CFoodParser#expr_mul}.
    /// @param ctx the parse tree
    fn enter_expr_mul_pass<'input: 'arena>(&mut self, _ctx: &Expr_mul_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_mul_pass}
    /// labeled alternative in {@link CFoodParser#expr_mul}.
    /// @param ctx the parse tree
    fn exit_expr_mul_pass<'input: 'arena>(&mut self, _ctx: &Expr_mul_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_mul_use}
    /// labeled alternative in {@link CFoodParser#expr_mul}.
    /// @param ctx the parse tree
    fn enter_expr_mul_use<'input: 'arena>(&mut self, _ctx: &Expr_mul_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_mul_use}
    /// labeled alternative in {@link CFoodParser#expr_mul}.
    /// @param ctx the parse tree
    fn exit_expr_mul_use<'input: 'arena>(&mut self, _ctx: &Expr_mul_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_cast_pass}
    /// labeled alternative in {@link CFoodParser#expr_cast}.
    /// @param ctx the parse tree
    fn enter_expr_cast_pass<'input: 'arena>(&mut self, _ctx: &Expr_cast_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_cast_pass}
    /// labeled alternative in {@link CFoodParser#expr_cast}.
    /// @param ctx the parse tree
    fn exit_expr_cast_pass<'input: 'arena>(&mut self, _ctx: &Expr_cast_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_cast_use}
    /// labeled alternative in {@link CFoodParser#expr_cast}.
    /// @param ctx the parse tree
    fn enter_expr_cast_use<'input: 'arena>(&mut self, _ctx: &Expr_cast_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_cast_use}
    /// labeled alternative in {@link CFoodParser#expr_cast}.
    /// @param ctx the parse tree
    fn exit_expr_cast_use<'input: 'arena>(&mut self, _ctx: &Expr_cast_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_cast_refer_use}
    /// labeled alternative in {@link CFoodParser#expr_cast}.
    /// @param ctx the parse tree
    fn enter_expr_cast_refer_use<'input: 'arena>(&mut self, _ctx: &Expr_cast_refer_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_cast_refer_use}
    /// labeled alternative in {@link CFoodParser#expr_cast}.
    /// @param ctx the parse tree
    fn exit_expr_cast_refer_use<'input: 'arena>(&mut self, _ctx: &Expr_cast_refer_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_unary_pass}
    /// labeled alternative in {@link CFoodParser#expr_unary}.
    /// @param ctx the parse tree
    fn enter_expr_unary_pass<'input: 'arena>(&mut self, _ctx: &Expr_unary_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_unary_pass}
    /// labeled alternative in {@link CFoodParser#expr_unary}.
    /// @param ctx the parse tree
    fn exit_expr_unary_pass<'input: 'arena>(&mut self, _ctx: &Expr_unary_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_unary_use}
    /// labeled alternative in {@link CFoodParser#expr_unary}.
    /// @param ctx the parse tree
    fn enter_expr_unary_use<'input: 'arena>(&mut self, _ctx: &Expr_unary_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_unary_use}
    /// labeled alternative in {@link CFoodParser#expr_unary}.
    /// @param ctx the parse tree
    fn exit_expr_unary_use<'input: 'arena>(&mut self, _ctx: &Expr_unary_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_magic_pass}
    /// labeled alternative in {@link CFoodParser#expr_magic}.
    /// @param ctx the parse tree
    fn enter_expr_magic_pass<'input: 'arena>(&mut self, _ctx: &Expr_magic_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_magic_pass}
    /// labeled alternative in {@link CFoodParser#expr_magic}.
    /// @param ctx the parse tree
    fn exit_expr_magic_pass<'input: 'arena>(&mut self, _ctx: &Expr_magic_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_magic_use}
    /// labeled alternative in {@link CFoodParser#expr_magic}.
    /// @param ctx the parse tree
    fn enter_expr_magic_use<'input: 'arena>(&mut self, _ctx: &Expr_magic_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_magic_use}
    /// labeled alternative in {@link CFoodParser#expr_magic}.
    /// @param ctx the parse tree
    fn exit_expr_magic_use<'input: 'arena>(&mut self, _ctx: &Expr_magic_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_call_pass}
    /// labeled alternative in {@link CFoodParser#expr_call}.
    /// @param ctx the parse tree
    fn enter_expr_call_pass<'input: 'arena>(&mut self, _ctx: &Expr_call_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_call_pass}
    /// labeled alternative in {@link CFoodParser#expr_call}.
    /// @param ctx the parse tree
    fn exit_expr_call_pass<'input: 'arena>(&mut self, _ctx: &Expr_call_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code expr_call_use}
    /// labeled alternative in {@link CFoodParser#expr_call}.
    /// @param ctx the parse tree
    fn enter_expr_call_use<'input: 'arena>(&mut self, _ctx: &Expr_call_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code expr_call_use}
    /// labeled alternative in {@link CFoodParser#expr_call}.
    /// @param ctx the parse tree
    fn exit_expr_call_use<'input: 'arena>(&mut self, _ctx: &Expr_call_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code atom_apply_list}
    /// labeled alternative in {@link CFoodParser#atom}.
    /// @param ctx the parse tree
    fn enter_atom_apply_list<'input: 'arena>(&mut self, _ctx: &Atom_apply_listContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code atom_apply_list}
    /// labeled alternative in {@link CFoodParser#atom}.
    /// @param ctx the parse tree
    fn exit_atom_apply_list<'input: 'arena>(&mut self, _ctx: &Atom_apply_listContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code atom_var}
    /// labeled alternative in {@link CFoodParser#atom}.
    /// @param ctx the parse tree
    fn enter_atom_var<'input: 'arena>(&mut self, _ctx: &Atom_varContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code atom_var}
    /// labeled alternative in {@link CFoodParser#atom}.
    /// @param ctx the parse tree
    fn exit_atom_var<'input: 'arena>(&mut self, _ctx: &Atom_varContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code atom_refer}
    /// labeled alternative in {@link CFoodParser#atom}.
    /// @param ctx the parse tree
    fn enter_atom_refer<'input: 'arena>(&mut self, _ctx: &Atom_referContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code atom_refer}
    /// labeled alternative in {@link CFoodParser#atom}.
    /// @param ctx the parse tree
    fn exit_atom_refer<'input: 'arena>(&mut self, _ctx: &Atom_referContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code atom_lit}
    /// labeled alternative in {@link CFoodParser#atom}.
    /// @param ctx the parse tree
    fn enter_atom_lit<'input: 'arena>(&mut self, _ctx: &Atom_litContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code atom_lit}
    /// labeled alternative in {@link CFoodParser#atom}.
    /// @param ctx the parse tree
    fn exit_atom_lit<'input: 'arena>(&mut self, _ctx: &Atom_litContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#magic}.
    /// @param ctx the parse tree
    fn enter_magic<'input: 'arena>(&mut self, _ctx: &MagicContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#magic}.
    /// @param ctx the parse tree
    fn exit_magic<'input: 'arena>(&mut self, _ctx: &MagicContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#logic_preced_op}.
    /// @param ctx the parse tree
    fn enter_logic_preced_op<'input: 'arena>(&mut self, _ctx: &Logic_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#logic_preced_op}.
    /// @param ctx the parse tree
    fn exit_logic_preced_op<'input: 'arena>(&mut self, _ctx: &Logic_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#cmp_preced_op}.
    /// @param ctx the parse tree
    fn enter_cmp_preced_op<'input: 'arena>(&mut self, _ctx: &Cmp_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#cmp_preced_op}.
    /// @param ctx the parse tree
    fn exit_cmp_preced_op<'input: 'arena>(&mut self, _ctx: &Cmp_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#add_preced_op}.
    /// @param ctx the parse tree
    fn enter_add_preced_op<'input: 'arena>(&mut self, _ctx: &Add_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#add_preced_op}.
    /// @param ctx the parse tree
    fn exit_add_preced_op<'input: 'arena>(&mut self, _ctx: &Add_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#mul_preced_op}.
    /// @param ctx the parse tree
    fn enter_mul_preced_op<'input: 'arena>(&mut self, _ctx: &Mul_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#mul_preced_op}.
    /// @param ctx the parse tree
    fn exit_mul_preced_op<'input: 'arena>(&mut self, _ctx: &Mul_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#unary_preced_op}.
    /// @param ctx the parse tree
    fn enter_unary_preced_op<'input: 'arena>(&mut self, _ctx: &Unary_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#unary_preced_op}.
    /// @param ctx the parse tree
    fn exit_unary_preced_op<'input: 'arena>(&mut self, _ctx: &Unary_preced_opContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#apply_list}.
    /// @param ctx the parse tree
    fn enter_apply_list<'input: 'arena>(&mut self, _ctx: &Apply_listContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#apply_list}.
    /// @param ctx the parse tree
    fn exit_apply_list<'input: 'arena>(&mut self, _ctx: &Apply_listContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#args}.
    /// @param ctx the parse tree
    fn enter_args<'input: 'arena>(&mut self, _ctx: &ArgsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#args}.
    /// @param ctx the parse tree
    fn exit_args<'input: 'arena>(&mut self, _ctx: &ArgsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
