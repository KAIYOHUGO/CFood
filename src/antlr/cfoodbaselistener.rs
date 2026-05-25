// Generated from ./CFood.g4 by ANTLR 4.13.2

use super::cfoodparser::*;
use dbt_antlr4::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by CFoodParser.

pub trait CFoodBaseListener<'arena>:
    ParseTreeListener<'arena, CFoodParserNodeKind> {

    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_file(&mut self, _ctx: &FileContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_file(&mut self, _ctx: &FileContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_decls(&mut self, _ctx: &DeclsContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_decls(&mut self, _ctx: &DeclsContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_decl(&mut self, _ctx: &DeclContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_decl(&mut self, _ctx: &DeclContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_var_decl(&mut self, _ctx: &Var_declContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_var_decl(&mut self, _ctx: &Var_declContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_var_decl_ty(&mut self, _ctx: &Var_decl_tyContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_var_decl_ty(&mut self, _ctx: &Var_decl_tyContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_var_decl_init(&mut self, _ctx: &Var_decl_initContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_var_decl_init(&mut self, _ctx: &Var_decl_initContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_fn_decl(&mut self, _ctx: &Fn_declContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fn_decl(&mut self, _ctx: &Fn_declContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_ty_decl(&mut self, _ctx: &Ty_declContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ty_decl(&mut self, _ctx: &Ty_declContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_params(&mut self, _ctx: &ParamsContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_params(&mut self, _ctx: &ParamsContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_param_list(&mut self, _ctx: &Param_listContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_param_list(&mut self, _ctx: &Param_listContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_param(&mut self, _ctx: &ParamContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_param(&mut self, _ctx: &ParamContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_lit_int(&mut self, _ctx: &Lit_intContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_lit_int(&mut self, _ctx: &Lit_intContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_lit_float(&mut self, _ctx: &Lit_floatContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_lit_float(&mut self, _ctx: &Lit_floatContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_lit_constr(&mut self, _ctx: &Lit_constrContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_lit_constr(&mut self, _ctx: &Lit_constrContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_tys(&mut self, _ctx: &TysContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tys(&mut self, _ctx: &TysContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_ty(&mut self, _ctx: &TyContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ty(&mut self, _ctx: &TyContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_ty_kind_ty(&mut self, _ctx: &Ty_kind_tyContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ty_kind_ty(&mut self, _ctx: &Ty_kind_tyContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_ty_kind_type(&mut self, _ctx: &Ty_kind_typeContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ty_kind_type(&mut self, _ctx: &Ty_kind_typeContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_block(&mut self, _ctx: &BlockContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_block(&mut self, _ctx: &BlockContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_stmts(&mut self, _ctx: &StmtsContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stmts(&mut self, _ctx: &StmtsContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_stmt(&mut self, _ctx: &StmtContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stmt(&mut self, _ctx: &StmtContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_expr_stmt(&mut self, _ctx: &Expr_stmtContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expr_stmt(&mut self, _ctx: &Expr_stmtContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_branch_stmt(&mut self, _ctx: &Branch_stmtContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_branch_stmt(&mut self, _ctx: &Branch_stmtContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_iter_stmt(&mut self, _ctx: &Iter_stmtContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_iter_stmt(&mut self, _ctx: &Iter_stmtContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_return_stmt(&mut self, _ctx: &Return_stmtContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_return_stmt(&mut self, _ctx: &Return_stmtContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_let_stmt(&mut self, _ctx: &Let_stmtContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_let_stmt(&mut self, _ctx: &Let_stmtContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_expr(&mut self, _ctx: &ExprContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expr(&mut self, _ctx: &ExprContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_assign_expr(&mut self, _ctx: &Assign_exprContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_assign_expr(&mut self, _ctx: &Assign_exprContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_var(&mut self, _ctx: &VarContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_var(&mut self, _ctx: &VarContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_calc_expr_use(&mut self, _ctx: &Calc_expr_useContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_calc_expr_use(&mut self, _ctx: &Calc_expr_useContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_calc_expr_pass(&mut self, _ctx: &Calc_expr_passContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_calc_expr_pass(&mut self, _ctx: &Calc_expr_passContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_call_preced_expr_use(&mut self, _ctx: &Call_preced_expr_useContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_call_preced_expr_use(&mut self, _ctx: &Call_preced_expr_useContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_call_preced_expr_magic(&mut self, _ctx: &Call_preced_expr_magicContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_call_preced_expr_magic(&mut self, _ctx: &Call_preced_expr_magicContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_call_preced_expr_pass(&mut self, _ctx: &Call_preced_expr_passContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_call_preced_expr_pass(&mut self, _ctx: &Call_preced_expr_passContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_magic(&mut self, _ctx: &MagicContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_magic(&mut self, _ctx: &MagicContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_add_preced_expr_use(&mut self, _ctx: &Add_preced_expr_useContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_add_preced_expr_use(&mut self, _ctx: &Add_preced_expr_useContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_add_preced_expr_pass(&mut self, _ctx: &Add_preced_expr_passContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_add_preced_expr_pass(&mut self, _ctx: &Add_preced_expr_passContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_mul_preced_expr_use(&mut self, _ctx: &Mul_preced_expr_useContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mul_preced_expr_use(&mut self, _ctx: &Mul_preced_expr_useContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_mul_preced_expr_pass(&mut self, _ctx: &Mul_preced_expr_passContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mul_preced_expr_pass(&mut self, _ctx: &Mul_preced_expr_passContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_atom_preced_expr_apply_list(&mut self, _ctx: &Atom_preced_expr_apply_listContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom_preced_expr_apply_list(&mut self, _ctx: &Atom_preced_expr_apply_listContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_atom_preced_expr_var(&mut self, _ctx: &Atom_preced_expr_varContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom_preced_expr_var(&mut self, _ctx: &Atom_preced_expr_varContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_atom_preced_expr_lit(&mut self, _ctx: &Atom_preced_expr_litContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom_preced_expr_lit(&mut self, _ctx: &Atom_preced_expr_litContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_cmp_preced_op(&mut self, _ctx: &Cmp_preced_opContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cmp_preced_op(&mut self, _ctx: &Cmp_preced_opContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_add_preced_op(&mut self, _ctx: &Add_preced_opContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_add_preced_op(&mut self, _ctx: &Add_preced_opContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_mul_preced_op(&mut self, _ctx: &Mul_preced_opContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mul_preced_op(&mut self, _ctx: &Mul_preced_opContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_apply_list(&mut self, _ctx: &Apply_listContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_apply_list(&mut self, _ctx: &Apply_listContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_args(&mut self, _ctx: &ArgsContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_args(&mut self, _ctx: &ArgsContext<'input, 'arena>) {}


}