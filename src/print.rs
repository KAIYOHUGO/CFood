use dbt_antlr4::{
    errors::ANTLRError,
    parser_rule_context::ParserRuleContext,
    rule_context::RuleContext,
    token::{CommonToken, Token},
    tree::ParseTreeListener,
};

use crate::antlr::cfoodparser::CFoodParserNodeKind;

#[derive(Debug)]
pub struct SexprAst {
    res: String,
    rules: &'static [&'static str],
    lexer_rules: &'static [&'static str],
}

impl SexprAst {
    pub fn new(rules: &'static [&'static str], lexer_rules: &'static [&'static str]) -> Self {
        Self {
            res: String::new(),
            rules,
            lexer_rules,
        }
    }

    pub fn to_sexpr(self) -> String {
        self.res
    }
}

impl<'input: 'arena, 'arena> ParseTreeListener<'arena, CFoodParserNodeKind, CommonToken<'arena>>
    for SexprAst
{
    fn visit_terminal(
        &mut self,
        node: &dbt_antlr4::tree::TerminalNode<'_, 'arena, CommonToken<'arena>>,
    ) -> Result<(), ANTLRError> {
        self.res.push('(');
        self.res
            .push_str(self.lexer_rules[node.symbol.get_token_type() as usize - 1]);
        self.res.push(' ');

        let line = node.symbol.get_line() - 1;
        let sc = node.symbol.get_char_position_in_line();
        let ec = node.symbol.get_text().len() as i32 + sc;
        self.res
            .push_str(&format!("(@span {line} {sc} {line} {ec})"));

        self.res.push(')');
        Ok(())
    }

    fn visit_error_node(
        &mut self,
        node: &dbt_antlr4::tree::ErrorNode<'_, 'arena, CommonToken<'arena>>,
    ) -> Result<(), ANTLRError> {
        self.res.push_str("(@err ");
        self.res
            .push_str(self.lexer_rules[node.symbol.get_token_type() as usize - 1]);
        self.res.push(')');

        Ok(())
    }

    fn enter_every_rule(
        &mut self,
        ctx: &dbt_antlr4::tree::TreeNode<'_, 'arena, CFoodParserNodeKind, CommonToken<'arena>>,
    ) -> Result<(), ANTLRError> {
        self.res.push('(');
        self.res.push_str(self.rules[ctx.get_rule_index()]);
        self.res.push(' ');

        let sl = ctx.start().get_line() - 1;
        let sc = ctx.start().get_char_position_in_line();
        let el = ctx.stop().get_line() - 1;
        let ec = ctx.stop().get_char_position_in_line() + ctx.stop().get_text().len() as i32;
        self.res.push_str(&format!("(@span {sl} {sc} {el} {ec})"));

        Ok(())
    }

    fn exit_every_rule(
        &mut self,
        _ctx: &dbt_antlr4::tree::TreeNode<'_, 'arena, CFoodParserNodeKind, CommonToken<'arena>>,
    ) -> Result<(), ANTLRError> {
        self.res.push(')');
        Ok(())
    }
}
