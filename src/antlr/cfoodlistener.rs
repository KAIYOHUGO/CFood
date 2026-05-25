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

    /// Enter a parse tree produced by {@link CFoodParser#ty}.
    /// @param ctx the parse tree
    fn enter_ty<'input: 'arena>(&mut self, _ctx: &TyContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#ty}.
    /// @param ctx the parse tree
    fn exit_ty<'input: 'arena>(&mut self, _ctx: &TyContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

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

    /// Enter a parse tree produced by {@link CFoodParser#expr}.
    /// @param ctx the parse tree
    fn enter_expr<'input: 'arena>(&mut self, _ctx: &ExprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#expr}.
    /// @param ctx the parse tree
    fn exit_expr<'input: 'arena>(&mut self, _ctx: &ExprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#assign_expr}.
    /// @param ctx the parse tree
    fn enter_assign_expr<'input: 'arena>(&mut self, _ctx: &Assign_exprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#assign_expr}.
    /// @param ctx the parse tree
    fn exit_assign_expr<'input: 'arena>(&mut self, _ctx: &Assign_exprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#var}.
    /// @param ctx the parse tree
    fn enter_var<'input: 'arena>(&mut self, _ctx: &VarContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#var}.
    /// @param ctx the parse tree
    fn exit_var<'input: 'arena>(&mut self, _ctx: &VarContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code calc_expr_use}
    /// labeled alternative in {@link CFoodParser#calc_expr}.
    /// @param ctx the parse tree
    fn enter_calc_expr_use<'input: 'arena>(&mut self, _ctx: &Calc_expr_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code calc_expr_use}
    /// labeled alternative in {@link CFoodParser#calc_expr}.
    /// @param ctx the parse tree
    fn exit_calc_expr_use<'input: 'arena>(&mut self, _ctx: &Calc_expr_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code calc_expr_pass}
    /// labeled alternative in {@link CFoodParser#calc_expr}.
    /// @param ctx the parse tree
    fn enter_calc_expr_pass<'input: 'arena>(&mut self, _ctx: &Calc_expr_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code calc_expr_pass}
    /// labeled alternative in {@link CFoodParser#calc_expr}.
    /// @param ctx the parse tree
    fn exit_calc_expr_pass<'input: 'arena>(&mut self, _ctx: &Calc_expr_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code call_preced_expr_use}
    /// labeled alternative in {@link CFoodParser#call_preced_expr}.
    /// @param ctx the parse tree
    fn enter_call_preced_expr_use<'input: 'arena>(&mut self, _ctx: &Call_preced_expr_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code call_preced_expr_use}
    /// labeled alternative in {@link CFoodParser#call_preced_expr}.
    /// @param ctx the parse tree
    fn exit_call_preced_expr_use<'input: 'arena>(&mut self, _ctx: &Call_preced_expr_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code call_preced_expr_magic}
    /// labeled alternative in {@link CFoodParser#call_preced_expr}.
    /// @param ctx the parse tree
    fn enter_call_preced_expr_magic<'input: 'arena>(&mut self, _ctx: &Call_preced_expr_magicContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code call_preced_expr_magic}
    /// labeled alternative in {@link CFoodParser#call_preced_expr}.
    /// @param ctx the parse tree
    fn exit_call_preced_expr_magic<'input: 'arena>(&mut self, _ctx: &Call_preced_expr_magicContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code call_preced_expr_pass}
    /// labeled alternative in {@link CFoodParser#call_preced_expr}.
    /// @param ctx the parse tree
    fn enter_call_preced_expr_pass<'input: 'arena>(&mut self, _ctx: &Call_preced_expr_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code call_preced_expr_pass}
    /// labeled alternative in {@link CFoodParser#call_preced_expr}.
    /// @param ctx the parse tree
    fn exit_call_preced_expr_pass<'input: 'arena>(&mut self, _ctx: &Call_preced_expr_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link CFoodParser#magic}.
    /// @param ctx the parse tree
    fn enter_magic<'input: 'arena>(&mut self, _ctx: &MagicContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link CFoodParser#magic}.
    /// @param ctx the parse tree
    fn exit_magic<'input: 'arena>(&mut self, _ctx: &MagicContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code add_preced_expr_use}
    /// labeled alternative in {@link CFoodParser#add_preced_expr}.
    /// @param ctx the parse tree
    fn enter_add_preced_expr_use<'input: 'arena>(&mut self, _ctx: &Add_preced_expr_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code add_preced_expr_use}
    /// labeled alternative in {@link CFoodParser#add_preced_expr}.
    /// @param ctx the parse tree
    fn exit_add_preced_expr_use<'input: 'arena>(&mut self, _ctx: &Add_preced_expr_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code add_preced_expr_pass}
    /// labeled alternative in {@link CFoodParser#add_preced_expr}.
    /// @param ctx the parse tree
    fn enter_add_preced_expr_pass<'input: 'arena>(&mut self, _ctx: &Add_preced_expr_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code add_preced_expr_pass}
    /// labeled alternative in {@link CFoodParser#add_preced_expr}.
    /// @param ctx the parse tree
    fn exit_add_preced_expr_pass<'input: 'arena>(&mut self, _ctx: &Add_preced_expr_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code mul_preced_expr_use}
    /// labeled alternative in {@link CFoodParser#mul_preced_expr}.
    /// @param ctx the parse tree
    fn enter_mul_preced_expr_use<'input: 'arena>(&mut self, _ctx: &Mul_preced_expr_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code mul_preced_expr_use}
    /// labeled alternative in {@link CFoodParser#mul_preced_expr}.
    /// @param ctx the parse tree
    fn exit_mul_preced_expr_use<'input: 'arena>(&mut self, _ctx: &Mul_preced_expr_useContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code mul_preced_expr_pass}
    /// labeled alternative in {@link CFoodParser#mul_preced_expr}.
    /// @param ctx the parse tree
    fn enter_mul_preced_expr_pass<'input: 'arena>(&mut self, _ctx: &Mul_preced_expr_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code mul_preced_expr_pass}
    /// labeled alternative in {@link CFoodParser#mul_preced_expr}.
    /// @param ctx the parse tree
    fn exit_mul_preced_expr_pass<'input: 'arena>(&mut self, _ctx: &Mul_preced_expr_passContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code atom_preced_expr_apply_list}
    /// labeled alternative in {@link CFoodParser#atom_preced_expr}.
    /// @param ctx the parse tree
    fn enter_atom_preced_expr_apply_list<'input: 'arena>(&mut self, _ctx: &Atom_preced_expr_apply_listContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code atom_preced_expr_apply_list}
    /// labeled alternative in {@link CFoodParser#atom_preced_expr}.
    /// @param ctx the parse tree
    fn exit_atom_preced_expr_apply_list<'input: 'arena>(&mut self, _ctx: &Atom_preced_expr_apply_listContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code atom_preced_expr_var}
    /// labeled alternative in {@link CFoodParser#atom_preced_expr}.
    /// @param ctx the parse tree
    fn enter_atom_preced_expr_var<'input: 'arena>(&mut self, _ctx: &Atom_preced_expr_varContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code atom_preced_expr_var}
    /// labeled alternative in {@link CFoodParser#atom_preced_expr}.
    /// @param ctx the parse tree
    fn exit_atom_preced_expr_var<'input: 'arena>(&mut self, _ctx: &Atom_preced_expr_varContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code atom_preced_expr_lit}
    /// labeled alternative in {@link CFoodParser#atom_preced_expr}.
    /// @param ctx the parse tree
    fn enter_atom_preced_expr_lit<'input: 'arena>(&mut self, _ctx: &Atom_preced_expr_litContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code atom_preced_expr_lit}
    /// labeled alternative in {@link CFoodParser#atom_preced_expr}.
    /// @param ctx the parse tree
    fn exit_atom_preced_expr_lit<'input: 'arena>(&mut self, _ctx: &Atom_preced_expr_litContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

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
