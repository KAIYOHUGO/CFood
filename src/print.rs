use dbt_antlr4::{
    errors::ANTLRError, parser_rule_context::ParserRuleContext, rule_context::RuleContext,
    tree::ParseTreeListener,
};

use crate::{
    antlr::{
        cfoodlistener::CFoodListener,
        cfoodparser::{self, CFoodParserContextNode},
    },
    tlt::TLT,
};

#[derive(Debug)]
pub struct SexprAst {
    res: String,
    rules: &'static [&'static str],
    tlt: TLT,
}

impl SexprAst {
    pub fn new(rules: &'static [&'static str], tlt: TLT) -> Self {
        Self {
            res: String::new(),
            rules,
            tlt,
        }
    }

    pub fn to_sexpr(self) -> String {
        self.res
    }
}

impl<'input: 'arena, 'arena>
    ParseTreeListener<'input, 'arena, CFoodParserContextNode<'input, 'arena>> for SexprAst
{
    fn visit_terminal(
        &mut self,
        node: &dbt_antlr4::tree::TerminalNode<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str(&format!("(@text '{}')", node.get_text()));

        Ok(())
    }

    fn visit_error_node(
        &mut self,
        node: &dbt_antlr4::tree::ErrorNode<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str(&format!("(@err '{}')", node.get_text()));

        Ok(())
    }

    fn enter_every_rule(
        &mut self,
        ctx: &CFoodParserContextNode<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push('(');
        self.res.push_str(self.rules[ctx.get_rule_index()]);
        self.res.push(' ');
        Ok(())
    }

    fn exit_every_rule(
        &mut self,
        _ctx: &CFoodParserContextNode<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push(')');
        Ok(())
    }
}

impl<'input: 'arena, 'arena> CFoodListener<'input, 'arena> for SexprAst {
    fn exit_var_decl_ty(
        &mut self,
        ctx: &cfoodparser::Var_decl_tyContext<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str("(@ty ");
        self.res.push_str(&format!("{}", self.tlt.ty(ctx.ty_id)));
        self.res.push(')');

        Ok(())
    }

    fn exit_fn_decl(
        &mut self,
        ctx: &cfoodparser::Fn_declContext<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str("(@ty ");
        self.res.push_str(&format!("{}", self.tlt.ty(ctx.ty_id)));
        self.res.push(')');

        Ok(())
    }

    fn exit_atom_preced_expr(
        &mut self,
        ctx: &cfoodparser::Atom_preced_exprContext<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str("(@ty ");
        self.res.push_str(&format!("{}", self.tlt.ty(ctx.ty_id)));
        self.res.push(')');
        Ok(())
    }

    fn exit_add_preced_expr(
        &mut self,
        ctx: &cfoodparser::Add_preced_exprContext<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str("(@ty ");
        self.res.push_str(&format!("{}", self.tlt.ty(ctx.ty_id)));
        self.res.push(')');
        Ok(())
    }

    fn exit_mul_preced_expr(
        &mut self,
        ctx: &cfoodparser::Mul_preced_exprContext<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str("(@ty ");
        self.res.push_str(&format!("{}", self.tlt.ty(ctx.ty_id)));
        self.res.push(')');
        Ok(())
    }

    fn exit_call_preced_expr(
        &mut self,
        ctx: &cfoodparser::Call_preced_exprContext<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str("(@ty ");
        self.res.push_str(&format!("{}", self.tlt.ty(ctx.ty_id)));
        self.res.push(')');
        Ok(())
    }

    fn exit_calc_expr(
        &mut self,
        ctx: &cfoodparser::Calc_exprContext<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str("(@ty ");
        self.res.push_str(&format!("{}", self.tlt.ty(ctx.ty_id)));
        self.res.push(')');
        Ok(())
    }

    fn exit_apply_list(
        &mut self,
        ctx: &cfoodparser::Apply_listContext<'input, 'arena>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str("(@ty ");
        self.res.push_str(&format!("{}", self.tlt.ty(ctx.ty_id)));
        self.res.push(')');
        Ok(())
    }
}
