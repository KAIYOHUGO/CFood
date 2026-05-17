// Generated from ./CFood.g4 by ANTLR 4.13.2



    #![allow(unused_imports)]
	use crate::ty::*;
	use crate::tlt::*;


use super::cfoodparser::*;
use dbt_antlr4::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by CFoodParser.

pub trait CFoodBaseListener<'input>:
    ParseTreeListener<'input, CFoodParserContextNode> {

    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_file(&mut self, _ctx: &FileContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_file(&mut self, _ctx: &FileContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_decls(&mut self, _ctx: &DeclsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_decls(&mut self, _ctx: &DeclsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_decl(&mut self, _ctx: &DeclContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_decl(&mut self, _ctx: &DeclContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_var_decl(&mut self, _ctx: &Var_declContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_var_decl(&mut self, _ctx: &Var_declContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_var_decl_ty(&mut self, _ctx: &Var_decl_tyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_var_decl_ty(&mut self, _ctx: &Var_decl_tyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_var_decl_init(&mut self, _ctx: &Var_decl_initContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_var_decl_init(&mut self, _ctx: &Var_decl_initContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fn_decl(&mut self, _ctx: &Fn_declContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fn_decl(&mut self, _ctx: &Fn_declContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ty_decl(&mut self, _ctx: &Ty_declContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ty_decl(&mut self, _ctx: &Ty_declContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_params(&mut self, _ctx: &ParamsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_params(&mut self, _ctx: &ParamsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_param_list(&mut self, _ctx: &Param_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_param_list(&mut self, _ctx: &Param_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_param(&mut self, _ctx: &ParamContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_param(&mut self, _ctx: &ParamContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_number(&mut self, _ctx: &NumberContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_number(&mut self, _ctx: &NumberContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tys(&mut self, _ctx: &TysContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tys(&mut self, _ctx: &TysContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ty(&mut self, _ctx: &TyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ty(&mut self, _ctx: &TyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ty_kind(&mut self, _ctx: &Ty_kindContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ty_kind(&mut self, _ctx: &Ty_kindContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_block(&mut self, _ctx: &BlockContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_block(&mut self, _ctx: &BlockContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stmts(&mut self, _ctx: &StmtsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stmts(&mut self, _ctx: &StmtsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stmt(&mut self, _ctx: &StmtContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stmt(&mut self, _ctx: &StmtContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_may_empty_stmt(&mut self, _ctx: &May_empty_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_may_empty_stmt(&mut self, _ctx: &May_empty_stmtContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_branch_stmt(&mut self, _ctx: &Branch_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_branch_stmt(&mut self, _ctx: &Branch_stmtContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_iter_stmt(&mut self, _ctx: &Iter_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_iter_stmt(&mut self, _ctx: &Iter_stmtContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_return_stmt(&mut self, _ctx: &Return_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_return_stmt(&mut self, _ctx: &Return_stmtContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_let_stmt(&mut self, _ctx: &Let_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_let_stmt(&mut self, _ctx: &Let_stmtContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expr(&mut self, _ctx: &ExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expr(&mut self, _ctx: &ExprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_assign_expr(&mut self, _ctx: &Assign_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_assign_expr(&mut self, _ctx: &Assign_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_var(&mut self, _ctx: &VarContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_var(&mut self, _ctx: &VarContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_calc_expr(&mut self, _ctx: &Calc_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_calc_expr(&mut self, _ctx: &Calc_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_call_preced_expr(&mut self, _ctx: &Call_preced_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_call_preced_expr(&mut self, _ctx: &Call_preced_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_add_preced_expr(&mut self, _ctx: &Add_preced_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_add_preced_expr(&mut self, _ctx: &Add_preced_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_mul_preced_expr(&mut self, _ctx: &Mul_preced_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mul_preced_expr(&mut self, _ctx: &Mul_preced_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atom_preced_expr(&mut self, _ctx: &Atom_preced_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom_preced_expr(&mut self, _ctx: &Atom_preced_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_cmp_preced_op(&mut self, _ctx: &Cmp_preced_opContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cmp_preced_op(&mut self, _ctx: &Cmp_preced_opContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_add_preced_op(&mut self, _ctx: &Add_preced_opContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_add_preced_op(&mut self, _ctx: &Add_preced_opContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_mul_preced_op(&mut self, _ctx: &Mul_preced_opContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mul_preced_op(&mut self, _ctx: &Mul_preced_opContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_apply_list(&mut self, _ctx: &Apply_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_apply_list(&mut self, _ctx: &Apply_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_args(&mut self, _ctx: &ArgsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CFoodBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_args(&mut self, _ctx: &ArgsContext<'input>) {}


}