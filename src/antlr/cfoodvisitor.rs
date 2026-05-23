#![allow(nonstandard_style)]
#![allow(dead_code)]
// Generated from ./CFood.g4 by ANTLR 4.13.2
use dbt_antlr4::token::{CommonToken, Token};
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::tree::*;
use super::cfoodparser::*;

/// This interface defines a complete generic visitor for a parse tree produced
/// by {@link CFoodParser}.
pub trait CFoodVisitor<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    type Return: Default;

    /// Visit a parse tree produced by {@link CFoodParser#file}.
    /// @param ctx the parse tree
    fn visit_file(&mut self, ctx: &'arena FileContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#decls}.
    /// @param ctx the parse tree
    fn visit_decls(&mut self, ctx: &'arena DeclsContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#decl}.
    /// @param ctx the parse tree
    fn visit_decl(&mut self, ctx: &'arena DeclContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#var_decl}.
    /// @param ctx the parse tree
    fn visit_var_decl(&mut self, ctx: &'arena Var_declContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#var_decl_ty}.
    /// @param ctx the parse tree
    fn visit_var_decl_ty(&mut self, ctx: &'arena Var_decl_tyContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#var_decl_init}.
    /// @param ctx the parse tree
    fn visit_var_decl_init(&mut self, ctx: &'arena Var_decl_initContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#fn_decl}.
    /// @param ctx the parse tree
    fn visit_fn_decl(&mut self, ctx: &'arena Fn_declContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#ty_decl}.
    /// @param ctx the parse tree
    fn visit_ty_decl(&mut self, ctx: &'arena Ty_declContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#params}.
    /// @param ctx the parse tree
    fn visit_params(&mut self, ctx: &'arena ParamsContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#param_list}.
    /// @param ctx the parse tree
    fn visit_param_list(&mut self, ctx: &'arena Param_listContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#param}.
    /// @param ctx the parse tree
    fn visit_param(&mut self, ctx: &'arena ParamContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code lit_int}
    /// labeled alternative in {@link CFoodParser#lit}.
    /// @param ctx the parse tree
    fn visit_lit_int(&mut self, ctx: &'arena Lit_intContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code lit_float}
    /// labeled alternative in {@link CFoodParser#lit}.
    /// @param ctx the parse tree
    fn visit_lit_float(&mut self, ctx: &'arena Lit_floatContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code lit_constr}
    /// labeled alternative in {@link CFoodParser#lit}.
    /// @param ctx the parse tree
    fn visit_lit_constr(&mut self, ctx: &'arena Lit_constrContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#tys}.
    /// @param ctx the parse tree
    fn visit_tys(&mut self, ctx: &'arena TysContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#ty}.
    /// @param ctx the parse tree
    fn visit_ty(&mut self, ctx: &'arena TyContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code ty_kind_ty}
    /// labeled alternative in {@link CFoodParser#ty_kind}.
    /// @param ctx the parse tree
    fn visit_ty_kind_ty(&mut self, ctx: &'arena Ty_kind_tyContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code ty_kind_type}
    /// labeled alternative in {@link CFoodParser#ty_kind}.
    /// @param ctx the parse tree
    fn visit_ty_kind_type(&mut self, ctx: &'arena Ty_kind_typeContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#block}.
    /// @param ctx the parse tree
    fn visit_block(&mut self, ctx: &'arena BlockContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#stmts}.
    /// @param ctx the parse tree
    fn visit_stmts(&mut self, ctx: &'arena StmtsContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#stmt}.
    /// @param ctx the parse tree
    fn visit_stmt(&mut self, ctx: &'arena StmtContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#expr_stmt}.
    /// @param ctx the parse tree
    fn visit_expr_stmt(&mut self, ctx: &'arena Expr_stmtContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#branch_stmt}.
    /// @param ctx the parse tree
    fn visit_branch_stmt(&mut self, ctx: &'arena Branch_stmtContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#iter_stmt}.
    /// @param ctx the parse tree
    fn visit_iter_stmt(&mut self, ctx: &'arena Iter_stmtContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#return_stmt}.
    /// @param ctx the parse tree
    fn visit_return_stmt(&mut self, ctx: &'arena Return_stmtContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#let_stmt}.
    /// @param ctx the parse tree
    fn visit_let_stmt(&mut self, ctx: &'arena Let_stmtContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#expr}.
    /// @param ctx the parse tree
    fn visit_expr(&mut self, ctx: &'arena ExprContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#assign_expr}.
    /// @param ctx the parse tree
    fn visit_assign_expr(&mut self, ctx: &'arena Assign_exprContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#var}.
    /// @param ctx the parse tree
    fn visit_var(&mut self, ctx: &'arena VarContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code calc_expr_use}
    /// labeled alternative in {@link CFoodParser#calc_expr}.
    /// @param ctx the parse tree
    fn visit_calc_expr_use(&mut self, ctx: &'arena Calc_expr_useContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code calc_expr_pass}
    /// labeled alternative in {@link CFoodParser#calc_expr}.
    /// @param ctx the parse tree
    fn visit_calc_expr_pass(&mut self, ctx: &'arena Calc_expr_passContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code call_preced_expr_use}
    /// labeled alternative in {@link CFoodParser#call_preced_expr}.
    /// @param ctx the parse tree
    fn visit_call_preced_expr_use(&mut self, ctx: &'arena Call_preced_expr_useContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code call_preced_expr_pass}
    /// labeled alternative in {@link CFoodParser#call_preced_expr}.
    /// @param ctx the parse tree
    fn visit_call_preced_expr_pass(&mut self, ctx: &'arena Call_preced_expr_passContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code add_preced_expr_use}
    /// labeled alternative in {@link CFoodParser#add_preced_expr}.
    /// @param ctx the parse tree
    fn visit_add_preced_expr_use(&mut self, ctx: &'arena Add_preced_expr_useContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code add_preced_expr_pass}
    /// labeled alternative in {@link CFoodParser#add_preced_expr}.
    /// @param ctx the parse tree
    fn visit_add_preced_expr_pass(&mut self, ctx: &'arena Add_preced_expr_passContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code mul_preced_expr_use}
    /// labeled alternative in {@link CFoodParser#mul_preced_expr}.
    /// @param ctx the parse tree
    fn visit_mul_preced_expr_use(&mut self, ctx: &'arena Mul_preced_expr_useContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code mul_preced_expr_pass}
    /// labeled alternative in {@link CFoodParser#mul_preced_expr}.
    /// @param ctx the parse tree
    fn visit_mul_preced_expr_pass(&mut self, ctx: &'arena Mul_preced_expr_passContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code atom_preced_expr_apply_list}
    /// labeled alternative in {@link CFoodParser#atom_preced_expr}.
    /// @param ctx the parse tree
    fn visit_atom_preced_expr_apply_list(&mut self, ctx: &'arena Atom_preced_expr_apply_listContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code atom_preced_expr_var}
    /// labeled alternative in {@link CFoodParser#atom_preced_expr}.
    /// @param ctx the parse tree
    fn visit_atom_preced_expr_var(&mut self, ctx: &'arena Atom_preced_expr_varContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by the {@code atom_preced_expr_lit}
    /// labeled alternative in {@link CFoodParser#atom_preced_expr}.
    /// @param ctx the parse tree
    fn visit_atom_preced_expr_lit(&mut self, ctx: &'arena Atom_preced_expr_litContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#cmp_preced_op}.
    /// @param ctx the parse tree
    fn visit_cmp_preced_op(&mut self, ctx: &'arena Cmp_preced_opContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#add_preced_op}.
    /// @param ctx the parse tree
    fn visit_add_preced_op(&mut self, ctx: &'arena Add_preced_opContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#mul_preced_op}.
    /// @param ctx the parse tree
    fn visit_mul_preced_op(&mut self, ctx: &'arena Mul_preced_opContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#apply_list}.
    /// @param ctx the parse tree
    fn visit_apply_list(&mut self, ctx: &'arena Apply_listContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }

    /// Visit a parse tree produced by {@link CFoodParser#args}.
    /// @param ctx the parse tree
    fn visit_args(&mut self, ctx: &'arena ArgsContext<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> { self.visit_children(ctx) }


    /// Called on terminal(leaf) node
    fn visit_terminal(
        &mut self,
        _node: &'arena TerminalNode<'input, 'arena, Tok>,
    ) -> Result<Self::Return, ANTLRError> { Ok(Self::Return::default()) }

    /// Called on error node
    fn visit_error_node(
        &mut self,
        _node: &'arena ErrorNode<'input, 'arena, Tok>,
    ) -> Result<Self::Return, ANTLRError> { Ok(Self::Return::default()) }

    fn visit(&mut self, tree: &'arena dyn NodeInner<'input, 'arena, CFoodParserNodeKind, Tok>) -> Result<Self::Return, ANTLRError> {
        self.visit_node(tree.as_node())
    }

    fn visit_node(&mut self, node: &'arena CFoodParserNode<'input, 'arena, Tok>) -> Result<Self::Return, ANTLRError> {
        node.accept(self)
    }

    fn visit_children(
        &mut self,
        node: &'arena dyn NodeInner<'input, 'arena, CFoodParserNodeKind, Tok>,
    ) -> Result<Self::Return, ANTLRError> {
        let mut result = Self::Return::default();
        for child in node.iter_child_nodes() {
            if !self.should_visit_next_child(child, &result) {
                break;
            }

            let child_result = self.visit_node(child)?;
            result = self.aggregate_results(result, child_result)?;
        }
        Ok(result)
    }

    fn aggregate_results(
        &self,
        _aggregate: Self::Return,
        next: Self::Return,
    ) -> Result<Self::Return, ANTLRError> { Ok(next) }

    fn should_visit_next_child(&self, _node: &'arena CFoodParserNode<'input, 'arena, Tok>, _current: &Self::Return) -> bool { true }
}