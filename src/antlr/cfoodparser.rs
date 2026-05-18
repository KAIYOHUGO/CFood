// Generated from ./CFood.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]

    #![allow(unused_imports)]
	use crate::ty::*;
	use crate::tlt::*;

use dbt_antlr4::Arena;
use dbt_antlr4::PredictionContextCache;
use dbt_antlr4::parser::{Parser, BaseParser, ParserRecog, ListenerId};
use dbt_antlr4::token::CommonToken;
use dbt_antlr4::token_stream::TokenStream;
use dbt_antlr4::TokenSource;
use dbt_antlr4::parser_atn_simulator::ParserATNSimulator;
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::rule_context::{CustomRuleContext, RuleContext};
use dbt_antlr4::recognizer::{Recognizer,Actions};
use dbt_antlr4::atn_config_set::ATNConfigSet;
use dbt_antlr4::atn_deserializer::ATNDeserializer;
use dbt_antlr4::atn_simulator::BaseATNSimulator;
use dbt_antlr4::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use dbt_antlr4::atn::{ATN, INVALID_ALT};
use dbt_antlr4::error_strategy::{DefaultErrorStrategy, ErrorStrategyDelegate, ErrorStrategy};
use dbt_antlr4::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use dbt_antlr4::tree::*;
use dbt_antlr4::token::{TOKEN_EOF,Token};
use dbt_antlr4::int_stream::EOF;
use dbt_antlr4::vocabulary::{Vocabulary,VocabularyImpl};
use dbt_antlr4::token_factory::TokenFactory;
use super::cfoodlistener::*;
use std::marker::PhantomData;
use std::sync::LazyLock;
use std::rc::Rc;
use std::ops::{DerefMut, Deref};

dbt_antlr4::check_version!("1","3");
pub const CFood_KW_while:i32=1; 
pub const CFood_KW_if:i32=2; 
pub const CFood_KW_else:i32=3; 
pub const CFood_KW_return:i32=4; 
pub const CFood_KW_type:i32=5; 
pub const CFood_KW_let:i32=6; 
pub const CFood_TY_int:i32=7; 
pub const CFood_TY_float:i32=8; 
pub const CFood_TY_void:i32=9; 
pub const CFood_ARROW:i32=10; 
pub const CFood_PAREN_L:i32=11; 
pub const CFood_PAREN_R:i32=12; 
pub const CFood_BRACE_L:i32=13; 
pub const CFood_BRACE_R:i32=14; 
pub const CFood_BRACKET_L:i32=15; 
pub const CFood_BRACKET_R:i32=16; 
pub const CFood_NE:i32=17; 
pub const CFood_EQ:i32=18; 
pub const CFood_LT:i32=19; 
pub const CFood_GT:i32=20; 
pub const CFood_LE:i32=21; 
pub const CFood_GE:i32=22; 
pub const CFood_PLUS:i32=23; 
pub const CFood_SUB:i32=24; 
pub const CFood_MOD:i32=25; 
pub const CFood_MUL:i32=26; 
pub const CFood_DIV:i32=27; 
pub const CFood_ASSIGN:i32=28; 
pub const CFood_COMMA:i32=29; 
pub const CFood_SEMICOLON:i32=30; 
pub const CFood_TYPE:i32=31; 
pub const CFood_IDENT:i32=32; 
pub const CFood_INT:i32=33; 
pub const CFood_FLOAT:i32=34; 
pub const CFood_LINE_COMMENT:i32=35; 
pub const CFood_COMMENT:i32=36; 
pub const CFood_WS:i32=37;
pub const CFood_EOF:i32=EOF;
pub const RULE_file:usize = 0; 
pub const RULE_decls:usize = 1; 
pub const RULE_decl:usize = 2; 
pub const RULE_var_decl:usize = 3; 
pub const RULE_var_decl_ty:usize = 4; 
pub const RULE_var_decl_init:usize = 5; 
pub const RULE_fn_decl:usize = 6; 
pub const RULE_ty_decl:usize = 7; 
pub const RULE_params:usize = 8; 
pub const RULE_param_list:usize = 9; 
pub const RULE_param:usize = 10; 
pub const RULE_number:usize = 11; 
pub const RULE_tys:usize = 12; 
pub const RULE_ty:usize = 13; 
pub const RULE_ty_kind:usize = 14; 
pub const RULE_block:usize = 15; 
pub const RULE_stmts:usize = 16; 
pub const RULE_stmt:usize = 17; 
pub const RULE_may_empty_stmt:usize = 18; 
pub const RULE_branch_stmt:usize = 19; 
pub const RULE_iter_stmt:usize = 20; 
pub const RULE_return_stmt:usize = 21; 
pub const RULE_let_stmt:usize = 22; 
pub const RULE_expr:usize = 23; 
pub const RULE_assign_expr:usize = 24; 
pub const RULE_var:usize = 25; 
pub const RULE_calc_expr:usize = 26; 
pub const RULE_call_preced_expr:usize = 27; 
pub const RULE_add_preced_expr:usize = 28; 
pub const RULE_mul_preced_expr:usize = 29; 
pub const RULE_atom_preced_expr:usize = 30; 
pub const RULE_cmp_preced_op:usize = 31; 
pub const RULE_add_preced_op:usize = 32; 
pub const RULE_mul_preced_op:usize = 33; 
pub const RULE_apply_list:usize = 34; 
pub const RULE_args:usize = 35;
pub const ruleNames: [&'static str; 36] = [
    "file", "decls", "decl", "var_decl", "var_decl_ty", "var_decl_init", 
    "fn_decl", "ty_decl", "params", "param_list", "param", "number", "tys", 
    "ty", "ty_kind", "block", "stmts", "stmt", "may_empty_stmt", "branch_stmt", 
    "iter_stmt", "return_stmt", "let_stmt", "expr", "assign_expr", "var", 
    "calc_expr", "call_preced_expr", "add_preced_expr", "mul_preced_expr", 
    "atom_preced_expr", "cmp_preced_op", "add_preced_op", "mul_preced_op", 
    "apply_list", "args"
];

pub const _LITERAL_NAMES: [Option<&'static str>;31] = [
	None, Some("'while'"), Some("'if'"), Some("'else'"), Some("'return'"), 
	Some("'type'"), Some("'let'"), Some("'int'"), Some("'float'"), Some("'void'"), 
	Some("'->'"), Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), Some("'['"), 
	Some("']'"), Some("'!='"), Some("'=='"), Some("'<'"), Some("'>'"), Some("'<='"), 
	Some("'>='"), Some("'+'"), Some("'-'"), Some("'%'"), Some("'*'"), Some("'/'"), 
	Some("'='"), Some("','"), Some("';'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;38]  = [
	None, Some("KW_while"), Some("KW_if"), Some("KW_else"), Some("KW_return"), 
	Some("KW_type"), Some("KW_let"), Some("TY_int"), Some("TY_float"), Some("TY_void"), 
	Some("ARROW"), Some("PAREN_L"), Some("PAREN_R"), Some("BRACE_L"), Some("BRACE_R"), 
	Some("BRACKET_L"), Some("BRACKET_R"), Some("NE"), Some("EQ"), Some("LT"), 
	Some("GT"), Some("LE"), Some("GE"), Some("PLUS"), Some("SUB"), Some("MOD"), 
	Some("MUL"), Some("DIV"), Some("ASSIGN"), Some("COMMA"), Some("SEMICOLON"), 
	Some("TYPE"), Some("IDENT"), Some("INT"), Some("FLOAT"), Some("LINE_COMMENT"), 
	Some("COMMENT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, CFoodParserExt<'input, 'arena>, CFoodParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				CFoodParserExt {
					_pd: Default::default(),

					    tlt: Default::default(),

				}
			),
            err_handler: unsafe { ErrorStrategyDelegate::new(strategy) },
        }
    }

    pub fn new(arena: &'arena Arena, input: Input) -> Self{
    	Self::with_strategy(arena, input, Box::new(DefaultErrorStrategy::new()))
    }

    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) {
        self.err_handler = unsafe { ErrorStrategyDelegate::new(strategy) };
    }

    /// Adds parse listener for this parser
    /// returns `listener_id` that can be used later to get listener back
    ///
    /// ### Example for listener usage:
    /// todo
    pub fn add_parse_listener<L>(
        &mut self,
        listener: Box<L>,
    ) -> ListenerId<L>
    where
        L: CFoodListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct CFoodTreeWalker;
impl CFoodTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: CFoodListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, CFoodParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <CFoodParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum CFoodParserNodeKind {
    FileContext,
    DeclsContext,
    DeclContext,
    Var_declContext,
    Var_decl_tyContext,
    Var_decl_initContext,
    Fn_declContext,
    Ty_declContext,
    ParamsContext,
    Param_listContext,
    ParamContext,
    NumberContext,
    TysContext,
    TyContext,
    Ty_kindContext,
    BlockContext,
    StmtsContext,
    StmtContext,
    May_empty_stmtContext,
    Branch_stmtContext,
    Iter_stmtContext,
    Return_stmtContext,
    Let_stmtContext,
    ExprContext,
    Assign_exprContext,
    VarContext,
    Calc_exprContext,
    Call_preced_exprContext,
    Add_preced_exprContext,
    Mul_preced_exprContext,
    Atom_preced_exprContext,
    Cmp_preced_opContext,
    Add_preced_opContext,
    Mul_preced_opContext,
    Apply_listContext,
    ArgsContext,
    Terminal,
    Error,
}
pub type CFoodParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, CFoodParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => CFoodParser }
dbt_antlr4::impl_node_kind! { CFoodParserNodeKind {
; FileContext(enter_file, exit_file, ), DeclsContext(enter_decls, exit_decls, ), DeclContext(enter_decl, exit_decl, ), Var_declContext(enter_var_decl, exit_var_decl, ), Var_decl_tyContext(enter_var_decl_ty, exit_var_decl_ty, ), Var_decl_initContext(enter_var_decl_init, exit_var_decl_init, ), Fn_declContext(enter_fn_decl, exit_fn_decl, ), Ty_declContext(enter_ty_decl, exit_ty_decl, ), ParamsContext(enter_params, exit_params, ), Param_listContext(enter_param_list, exit_param_list, ), ParamContext(enter_param, exit_param, ), NumberContext(enter_number, exit_number, ), TysContext(enter_tys, exit_tys, ), TyContext(enter_ty, exit_ty, ), Ty_kindContext(enter_ty_kind, exit_ty_kind, ), BlockContext(enter_block, exit_block, ), StmtsContext(enter_stmts, exit_stmts, ), StmtContext(enter_stmt, exit_stmt, ), May_empty_stmtContext(enter_may_empty_stmt, exit_may_empty_stmt, ), Branch_stmtContext(enter_branch_stmt, exit_branch_stmt, ), Iter_stmtContext(enter_iter_stmt, exit_iter_stmt, ), Return_stmtContext(enter_return_stmt, exit_return_stmt, ), Let_stmtContext(enter_let_stmt, exit_let_stmt, ), ExprContext(enter_expr, exit_expr, ), Assign_exprContext(enter_assign_expr, exit_assign_expr, ), VarContext(enter_var, exit_var, ), Calc_exprContext(enter_calc_expr, exit_calc_expr, ), Call_preced_exprContext(enter_call_preced_expr, exit_call_preced_expr, ), Add_preced_exprContext(enter_add_preced_expr, exit_add_preced_expr, ), Mul_preced_exprContext(enter_mul_preced_expr, exit_mul_preced_expr, ), Atom_preced_exprContext(enter_atom_preced_expr, exit_atom_preced_expr, ), Cmp_preced_opContext(enter_cmp_preced_op, exit_cmp_preced_op, ), Add_preced_opContext(enter_add_preced_op, exit_add_preced_op, ), Mul_preced_opContext(enter_mul_preced_op, exit_mul_preced_op, ), Apply_listContext(enter_apply_list, exit_apply_list, ), ArgsContext(enter_args, exit_args, ), 
    }; listener = dyn CFoodListener<'arena, Tok>,
}

pub struct CFoodParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,

		pub tlt: TLT,

}

impl<'input, 'arena> CFoodParserExt<'input, 'arena> {
}

impl<'input, 'arena, Input, TF> ParserRecog<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for CFoodParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena {
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }        
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for CFoodParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn get_grammar_file_name(&self) -> & str{ "CFood.g4" }
   	fn get_rule_names(&self) -> &[& str] { &ruleNames }
   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- file ----------------
pub type FileContextAll<'input, 'arena, Tok = CommonToken<'input>> = FileContext<'input, 'arena, Tok>;

pub type FileContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, FileContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct FileContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for FileContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::FileContext }
	fn get_rule_index(&self) -> usize { RULE_file }
    fn make_node(
        arena: &'arena Arena,
        ctx: FileContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a FileContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => FileContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut FileContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut FileContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> FileContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, FileContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait FileContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn decls(&self) -> Option<&'arena DeclsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> FileContextAttrs<'input, 'arena, Tok> for FileContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn decls(&self) -> Option<&'arena DeclsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn file(&mut self,) -> Result<&'arena FileContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(FileContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 0, RULE_file)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena FileContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule decls*/
			recog.base.set_state(72);
			recog.decls()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- decls ----------------
pub type DeclsContextAll<'input, 'arena, Tok = CommonToken<'input>> = DeclsContext<'input, 'arena, Tok>;

pub type DeclsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, DeclsContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct DeclsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for DeclsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::DeclsContext }
	fn get_rule_index(&self) -> usize { RULE_decls }
    fn make_node(
        arena: &'arena Arena,
        ctx: DeclsContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a DeclsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => DeclsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut DeclsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut DeclsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DeclsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, DeclsContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait DeclsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn decl(&self) -> Option<&'arena DeclContextAll<'input, 'arena, Tok>>;
    fn decls(&self) -> Option<&'arena DeclsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> DeclsContextAttrs<'input, 'arena, Tok> for DeclsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn decl(&self) -> Option<&'arena DeclContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn decls(&self) -> Option<&'arena DeclsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn decls(&mut self,) -> Result<&'arena DeclsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(DeclsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 2, RULE_decls)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena DeclsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(78);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_KW_type |CFood_TY_int |CFood_TY_float |CFood_TY_void |CFood_TYPE  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule decl*/
			        recog.base.set_state(74);
			        recog.decl()?;
			        /*InvokeRule decls*/
			        recog.base.set_state(75);
			        recog.decls()?;
			        }}
			    CFood_EOF  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- decl ----------------
pub type DeclContextAll<'input, 'arena, Tok = CommonToken<'input>> = DeclContext<'input, 'arena, Tok>;

pub type DeclContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, DeclContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct DeclContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for DeclContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::DeclContext }
	fn get_rule_index(&self) -> usize { RULE_decl }
    fn make_node(
        arena: &'arena Arena,
        ctx: DeclContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a DeclContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => DeclContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut DeclContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut DeclContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DeclContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, DeclContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait DeclContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn var_decl(&self) -> Option<&'arena Var_declContextAll<'input, 'arena, Tok>>;
    fn ty_decl(&self) -> Option<&'arena Ty_declContextAll<'input, 'arena, Tok>>;
    fn fn_decl(&self) -> Option<&'arena Fn_declContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> DeclContextAttrs<'input, 'arena, Tok> for DeclContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn var_decl(&self) -> Option<&'arena Var_declContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ty_decl(&self) -> Option<&'arena Ty_declContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn fn_decl(&self) -> Option<&'arena Fn_declContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn decl(&mut self,) -> Result<&'arena DeclContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(DeclContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 4, RULE_decl)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena DeclContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(83);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(1,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule var_decl*/
					recog.base.set_state(80);
					recog.var_decl()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule ty_decl*/
					recog.base.set_state(81);
					recog.ty_decl()?;
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					/*InvokeRule fn_decl*/
					recog.base.set_state(82);
					recog.fn_decl()?;
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- var_decl ----------------
pub type Var_declContextAll<'input, 'arena, Tok = CommonToken<'input>> = Var_declContext<'input, 'arena, Tok>;

pub type Var_declContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Var_declContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Var_declContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub var_decl_ty: Option<&'arena Var_decl_tyContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Var_declContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Var_declContext }
	fn get_rule_index(&self) -> usize { RULE_var_decl }
    fn make_node(
        arena: &'arena Arena,
        ctx: Var_declContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Var_declContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Var_declContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Var_declContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Var_declContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Var_declContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Var_declContextExt {
				var_decl_ty: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Var_declContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn var_decl_ty(&self) -> Option<&'arena Var_decl_tyContextAll<'input, 'arena, Tok>>;
    fn var_decl_init(&self) -> Option<&'arena Var_decl_initContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Var_declContextAttrs<'input, 'arena, Tok> for Var_declContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn var_decl_ty(&self) -> Option<&'arena Var_decl_tyContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn var_decl_init(&self) -> Option<&'arena Var_decl_initContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_SEMICOLON)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn var_decl(&mut self,) -> Result<&'arena Var_declContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Var_declContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 6, RULE_var_decl)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Var_declContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule var_decl_ty*/
			recog.base.set_state(85);
			let tmp = recog.var_decl_ty()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_declContext<TF::Tok>>().unwrap().var_decl_ty = Some(tmp); } 

			        let tmp = { *recog.ctx().unwrap().as_rule_context::<Var_declContext<TF::Tok>>().unwrap().var_decl_ty.as_ref().unwrap().get_ty_id()}.to_owned();
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_declContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
			    
			/*InvokeRule var_decl_init*/
			recog.base.set_state(87);
			recog.var_decl_init(*recog.ctx().unwrap().as_rule_context::<Var_declContext<TF::Tok>>().unwrap().get_ty_id())?;
			recog.base.set_state(88);
			recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- var_decl_ty ----------------
pub type Var_decl_tyContextAll<'input, 'arena, Tok = CommonToken<'input>> = Var_decl_tyContext<'input, 'arena, Tok>;

pub type Var_decl_tyContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Var_decl_tyContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Var_decl_tyContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub ty: Option<&'arena TyContextAll<'input, 'arena, Tok>>,
	pub IDENT: Option<&'arena dyn Token >,
	pub ty_kind: Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>,
	pub number: Option<&'arena NumberContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Var_decl_tyContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Var_decl_tyContext }
	fn get_rule_index(&self) -> usize { RULE_var_decl_ty }
    fn make_node(
        arena: &'arena Arena,
        ctx: Var_decl_tyContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Var_decl_tyContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Var_decl_tyContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Var_decl_tyContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Var_decl_tyContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Var_decl_tyContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Var_decl_tyContextExt {
				IDENT: None, 
				ty: None, ty_kind: None, number: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Var_decl_tyContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn ty(&self) -> Option<&'arena TyContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token BRACKET_L
    /// Returns `None` if there is no child corresponding to token BRACKET_L
    fn BRACKET_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn number(&self) -> Option<&'arena NumberContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token BRACKET_R
    /// Returns `None` if there is no child corresponding to token BRACKET_R
    fn BRACKET_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Var_decl_tyContextAttrs<'input, 'arena, Tok> for Var_decl_tyContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn ty(&self) -> Option<&'arena TyContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_IDENT)
    }
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACKET_L
    /// Returns `None` if there is no child corresponding to token BRACKET_L
    fn BRACKET_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_BRACKET_L)
    }
    fn number(&self) -> Option<&'arena NumberContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACKET_R
    /// Returns `None` if there is no child corresponding to token BRACKET_R
    fn BRACKET_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_BRACKET_R)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn var_decl_ty(&mut self,) -> Result<&'arena Var_decl_tyContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Var_decl_tyContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 8, RULE_var_decl_ty)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Var_decl_tyContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(101);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(2,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule ty*/
					recog.base.set_state(90);
					let tmp = recog.ty()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_decl_tyContext<TF::Tok>>().unwrap().ty = Some(tmp); } 
					recog.base.set_state(91);
					let tmp = recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_decl_tyContext<TF::Tok>>().unwrap().IDENT = Some(tmp); } 

					        let name = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Var_decl_tyContext<TF::Tok>>().unwrap().IDENT { it.get_text() } else { "null" } ;
					        let ty_raw = recog.ctx().unwrap().as_rule_context::<Var_decl_tyContext<TF::Tok>>().unwrap().ty.as_ref().unwrap().get_ty_raw();
					        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Var_decl_tyContext<TF::Tok>>().unwrap().IDENT { it.get_line() } else { 0 } ;

					        let tmp = { recog.tlt.new_var_raw(
					            name,
					            ty_raw,
					            line
					        )}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_decl_tyContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(94);
					let tmp = recog.ty_kind()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_decl_tyContext<TF::Tok>>().unwrap().ty_kind = Some(tmp); } 
					recog.base.set_state(95);
					let tmp = recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_decl_tyContext<TF::Tok>>().unwrap().IDENT = Some(tmp); } 
					recog.base.set_state(96);
					recog.base.match_token(CFood_BRACKET_L,&mut recog.err_handler)?;
					/*InvokeRule number*/
					recog.base.set_state(97);
					let tmp = recog.number()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_decl_tyContext<TF::Tok>>().unwrap().number = Some(tmp); } 
					recog.base.set_state(98);
					recog.base.match_token(CFood_BRACKET_R,&mut recog.err_handler)?;

					        let name = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Var_decl_tyContext<TF::Tok>>().unwrap().IDENT { it.get_text() } else { "null" } ;
					        let kind = recog.ctx().unwrap().as_rule_context::<Var_decl_tyContext<TF::Tok>>().unwrap().ty_kind.as_ref().unwrap().get_kind();
					        let num = recog.ctx().unwrap().as_rule_context::<Var_decl_tyContext<TF::Tok>>().unwrap().number.as_ref().unwrap().get_num();
					        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Var_decl_tyContext<TF::Tok>>().unwrap().IDENT { it.get_line() } else { 0 } ;

					        let tmp = { recog.tlt.new_var_arr(
					            name,
					            kind,
					            num,
					            line,
					        )}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_decl_tyContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- var_decl_init ----------------
pub type Var_decl_initContextAll<'input, 'arena, Tok = CommonToken<'input>> = Var_decl_initContext<'input, 'arena, Tok>;

pub type Var_decl_initContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Var_decl_initContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Var_decl_initContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub ASSIGN: Option<&'arena dyn Token >,
	pub expr: Option<&'arena ExprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Var_decl_initContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Var_decl_initContext }
	fn get_rule_index(&self) -> usize { RULE_var_decl_init }
    fn make_node(
        arena: &'arena Arena,
        ctx: Var_decl_initContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Var_decl_initContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Var_decl_initContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Var_decl_initContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Var_decl_initContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Var_decl_initContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32, ty_id: TyId) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

		 _init_ty_id = ty_id;

        BaseParserRuleContext::create(arena, parent, invoking_state, Var_decl_initContextExt {
				ASSIGN: None, 
				expr: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Var_decl_initContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Var_decl_initContextAttrs<'input, 'arena, Tok> for Var_decl_initContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_ASSIGN)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn var_decl_init(&mut self,ty_id: TyId) -> Result<&'arena Var_decl_initContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Var_decl_initContextExt::create(recog.get_arena(), _parentctx, recog.get_state(), ty_id)?, 10, RULE_var_decl_init)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Var_decl_initContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(108);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_ASSIGN  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(103);
			        let tmp = recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_decl_initContext<TF::Tok>>().unwrap().ASSIGN = Some(tmp); } 
			        /*InvokeRule expr*/
			        recog.base.set_state(104);
			        let tmp = recog.expr()?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Var_decl_initContext<TF::Tok>>().unwrap().expr = Some(tmp); } 

			                let bound = *recog.ctx().unwrap().as_rule_context::<Var_decl_initContext<TF::Tok>>().unwrap().get_ty_id();
			                let expr = *recog.ctx().unwrap().as_rule_context::<Var_decl_initContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id();
			                let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Var_decl_initContext<TF::Tok>>().unwrap().ASSIGN { it.get_line() } else { 0 } ;
			                recog.tlt.assert_ty_id(bound, expr, line);
			            
			        }}
			    CFood_SEMICOLON  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- fn_decl ----------------
pub type Fn_declContextAll<'input, 'arena, Tok = CommonToken<'input>> = Fn_declContext<'input, 'arena, Tok>;

pub type Fn_declContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Fn_declContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Fn_declContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub ty: Option<&'arena TyContextAll<'input, 'arena, Tok>>,
	pub IDENT: Option<&'arena dyn Token >,
	pub params: Option<&'arena ParamsContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Fn_declContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Fn_declContext }
	fn get_rule_index(&self) -> usize { RULE_fn_decl }
    fn make_node(
        arena: &'arena Arena,
        ctx: Fn_declContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Fn_declContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Fn_declContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Fn_declContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Fn_declContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Fn_declContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Fn_declContextExt {
				IDENT: None, 
				ty: None, params: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Fn_declContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn ty(&self) -> Option<&'arena TyContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn params(&self) -> Option<&'arena ParamsContextAll<'input, 'arena, Tok>>;
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Fn_declContextAttrs<'input, 'arena, Tok> for Fn_declContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn ty(&self) -> Option<&'arena TyContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_IDENT)
    }
    fn params(&self) -> Option<&'arena ParamsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn fn_decl(&mut self,) -> Result<&'arena Fn_declContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Fn_declContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 12, RULE_fn_decl)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Fn_declContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule ty*/
			recog.base.set_state(110);
			let tmp = recog.ty()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Fn_declContext<TF::Tok>>().unwrap().ty = Some(tmp); } 
			recog.base.set_state(111);
			let tmp = recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Fn_declContext<TF::Tok>>().unwrap().IDENT = Some(tmp); } 

			        recog.tlt.enter_block();
			    
			/*InvokeRule params*/
			recog.base.set_state(113);
			let tmp = recog.params()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Fn_declContext<TF::Tok>>().unwrap().params = Some(tmp); } 

			        let name = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Fn_declContext<TF::Tok>>().unwrap().IDENT { it.get_text() } else { "null" } ;
			        let ty_raw = recog.ctx().unwrap().as_rule_context::<Fn_declContext<TF::Tok>>().unwrap().ty.as_ref().unwrap().get_ty_raw();
			        let ty_ids = recog.ctx().unwrap().as_rule_context::<Fn_declContext<TF::Tok>>().unwrap().params.as_ref().unwrap().get_ty_ids();
			        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Fn_declContext<TF::Tok>>().unwrap().IDENT { it.get_line() } else { 0 } ;
			        let tmp = { recog.tlt.new_fn(name, ty_raw, ty_ids, line)}.to_owned();
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Fn_declContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
			    
			/*InvokeRule block*/
			recog.base.set_state(115);
			recog.block()?;

			        recog.tlt.exit_block();
			    
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ty_decl ----------------
pub type Ty_declContextAll<'input, 'arena, Tok = CommonToken<'input>> = Ty_declContext<'input, 'arena, Tok>;

pub type Ty_declContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Ty_declContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Ty_declContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub TYPE: Option<&'arena dyn Token >,
	pub tys: Option<&'arena TysContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Ty_declContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Ty_declContext }
	fn get_rule_index(&self) -> usize { RULE_ty_decl }
    fn make_node(
        arena: &'arena Arena,
        ctx: Ty_declContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Ty_declContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Ty_declContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Ty_declContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Ty_declContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Ty_declContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Ty_declContextExt {
				TYPE: None, 
				tys: None, 
				ph: PhantomData
			}
		)
	}
}

pub trait Ty_declContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token KW_type
    /// Returns `None` if there is no child corresponding to token KW_type
    fn KW_type(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TYPE
    /// Returns `None` if there is no child corresponding to token TYPE
    fn TYPE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn tys(&self) -> Option<&'arena TysContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Ty_declContextAttrs<'input, 'arena, Tok> for Ty_declContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token KW_type
    /// Returns `None` if there is no child corresponding to token KW_type
    fn KW_type(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_KW_type)
    }
    /// Retrieves first TerminalNode corresponding to token TYPE
    /// Returns `None` if there is no child corresponding to token TYPE
    fn TYPE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TYPE)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_ASSIGN)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_L)
    }
    fn tys(&self) -> Option<&'arena TysContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_R)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_SEMICOLON)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ty_decl(&mut self,) -> Result<&'arena Ty_declContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Ty_declContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 14, RULE_ty_decl)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_declContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(118);
			recog.base.match_token(CFood_KW_type,&mut recog.err_handler)?;
			recog.base.set_state(119);
			let tmp = recog.base.match_token(CFood_TYPE,&mut recog.err_handler)?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Ty_declContext<TF::Tok>>().unwrap().TYPE = Some(tmp); } 
			recog.base.set_state(120);
			recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			recog.base.set_state(121);
			recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
			/*InvokeRule tys*/
			recog.base.set_state(122);
			let tmp = recog.tys()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Ty_declContext<TF::Tok>>().unwrap().tys = Some(tmp); } 
			recog.base.set_state(123);
			recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
			recog.base.set_state(124);
			recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;

			        let name = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Ty_declContext<TF::Tok>>().unwrap().TYPE { it.get_text() } else { "null" } .to_owned();
			        let kinds = recog.ctx().unwrap().as_rule_context::<Ty_declContext<TF::Tok>>().unwrap().tys.as_ref().unwrap().get_kinds();
			        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Ty_declContext<TF::Tok>>().unwrap().TYPE { it.get_line() } else { 0 } ;
			        recog.tlt.new_alias(
			            name,
			            kinds,
			            line,
			        );
			    
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- params ----------------
pub type ParamsContextAll<'input, 'arena, Tok = CommonToken<'input>> = ParamsContext<'input, 'arena, Tok>;

pub type ParamsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ParamsContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ParamsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_ids: Vec<TyId>,
	pub param_list: Option<&'arena Param_listContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ParamsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::ParamsContext }
	fn get_rule_index(&self) -> usize { RULE_params }
    fn make_node(
        arena: &'arena Arena,
        ctx: ParamsContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ParamsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => ParamsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ParamsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut ParamsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ParamsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_ids = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, ParamsContextExt {
				param_list: None, 
				ty_ids: _init_ty_ids, 
				ph: PhantomData
			}
		)
	}
}

pub trait ParamsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_ids(&self) -> &Vec<TyId>; 
    fn set_ty_ids(&mut self,attr: Vec<TyId>); 
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn param_list(&self) -> Option<&'arena Param_listContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TY_void
    /// Returns `None` if there is no child corresponding to token TY_void
    fn TY_void(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ParamsContextAttrs<'input, 'arena, Tok> for ParamsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_ids(&self) -> &Vec<TyId> { &self.deref().ty_ids }  
    fn set_ty_ids(&mut self,attr: Vec<TyId>) { self.deref_mut().ty_ids = attr; }  
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_L)
    }
    fn param_list(&self) -> Option<&'arena Param_listContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_R)
    }
    /// Retrieves first TerminalNode corresponding to token TY_void
    /// Returns `None` if there is no child corresponding to token TY_void
    fn TY_void(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TY_void)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn params(&mut self,) -> Result<&'arena ParamsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ParamsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 16, RULE_params)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ParamsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(139);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(4,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(127);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule param_list*/
					recog.base.set_state(128);
					let tmp = recog.param_list()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ParamsContext<TF::Tok>>().unwrap().param_list = Some(tmp); } 
					recog.base.set_state(129);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;

					        let tmp = { recog.ctx().unwrap().as_rule_context::<ParamsContext<TF::Tok>>().unwrap().param_list.as_ref().unwrap().get_ty_ids()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ParamsContext<TF::Tok>>().unwrap().set_ty_ids(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(132);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					recog.base.set_state(133);
					recog.base.match_token(CFood_TY_void,&mut recog.err_handler)?;
					recog.base.set_state(134);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					let tmp = { vec![]}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ParamsContext<TF::Tok>>().unwrap().set_ty_ids(tmp); }
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					recog.base.set_state(136);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					recog.base.set_state(137);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					let tmp = { vec![]}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ParamsContext<TF::Tok>>().unwrap().set_ty_ids(tmp); }
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- param_list ----------------
pub type Param_listContextAll<'input, 'arena, Tok = CommonToken<'input>> = Param_listContext<'input, 'arena, Tok>;

pub type Param_listContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Param_listContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Param_listContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_ids: Vec<TyId>,
	pub param: Option<&'arena ParamContextAll<'input, 'arena, Tok>>,
	pub param_list: Option<&'arena Param_listContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Param_listContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Param_listContext }
	fn get_rule_index(&self) -> usize { RULE_param_list }
    fn make_node(
        arena: &'arena Arena,
        ctx: Param_listContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Param_listContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Param_listContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Param_listContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Param_listContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Param_listContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_ids = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Param_listContextExt {
				param: None, param_list: None, 
				ty_ids: _init_ty_ids, 
				ph: PhantomData
			}
		)
	}
}

pub trait Param_listContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_ids(&self) -> &Vec<TyId>; 
    fn set_ty_ids(&mut self,attr: Vec<TyId>); 
    fn param(&self) -> Option<&'arena ParamContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn param_list(&self) -> Option<&'arena Param_listContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Param_listContextAttrs<'input, 'arena, Tok> for Param_listContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_ids(&self) -> &Vec<TyId> { &self.deref().ty_ids }  
    fn set_ty_ids(&mut self,attr: Vec<TyId>) { self.deref_mut().ty_ids = attr; }  
    fn param(&self) -> Option<&'arena ParamContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_COMMA)
    }
    fn param_list(&self) -> Option<&'arena Param_listContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn param_list(&mut self,) -> Result<&'arena Param_listContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Param_listContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 18, RULE_param_list)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Param_listContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(149);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(5,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule param*/
					recog.base.set_state(141);
					let tmp = recog.param()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Param_listContext<TF::Tok>>().unwrap().param = Some(tmp); } 
					recog.base.set_state(142);
					recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					/*InvokeRule param_list*/
					recog.base.set_state(143);
					let tmp = recog.param_list()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Param_listContext<TF::Tok>>().unwrap().param_list = Some(tmp); } 

					        let tmp = { [vec![*recog.ctx().unwrap().as_rule_context::<Param_listContext<TF::Tok>>().unwrap().param.as_ref().unwrap().get_ty_id()], recog.ctx().unwrap().as_rule_context::<Param_listContext<TF::Tok>>().unwrap().param_list.as_ref().unwrap().get_ty_ids().to_owned()].concat()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Param_listContext<TF::Tok>>().unwrap().set_ty_ids(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule param*/
					recog.base.set_state(146);
					let tmp = recog.param()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Param_listContext<TF::Tok>>().unwrap().param = Some(tmp); } 

					        let tmp = { vec![*recog.ctx().unwrap().as_rule_context::<Param_listContext<TF::Tok>>().unwrap().param.as_ref().unwrap().get_ty_id()]}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Param_listContext<TF::Tok>>().unwrap().set_ty_ids(tmp); }
					    
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- param ----------------
pub type ParamContextAll<'input, 'arena, Tok = CommonToken<'input>> = ParamContext<'input, 'arena, Tok>;

pub type ParamContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ParamContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ParamContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub var_decl_ty: Option<&'arena Var_decl_tyContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ParamContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::ParamContext }
	fn get_rule_index(&self) -> usize { RULE_param }
    fn make_node(
        arena: &'arena Arena,
        ctx: ParamContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ParamContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => ParamContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ParamContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut ParamContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ParamContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, ParamContextExt {
				var_decl_ty: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait ParamContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn var_decl_ty(&self) -> Option<&'arena Var_decl_tyContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ParamContextAttrs<'input, 'arena, Tok> for ParamContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn var_decl_ty(&self) -> Option<&'arena Var_decl_tyContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn param(&mut self,) -> Result<&'arena ParamContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ParamContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 20, RULE_param)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ParamContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule var_decl_ty*/
			recog.base.set_state(151);
			let tmp = recog.var_decl_ty()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ParamContext<TF::Tok>>().unwrap().var_decl_ty = Some(tmp); } 
			let tmp = { recog.ctx().unwrap().as_rule_context::<ParamContext<TF::Tok>>().unwrap().var_decl_ty.as_ref().unwrap().get_ty_id()}.to_owned();
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ParamContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- number ----------------
pub type NumberContextAll<'input, 'arena, Tok = CommonToken<'input>> = NumberContext<'input, 'arena, Tok>;

pub type NumberContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NumberContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct NumberContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub num: Number,
	pub INT: Option<&'arena dyn Token >,
	pub FLOAT: Option<&'arena dyn Token >,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for NumberContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::NumberContext }
	fn get_rule_index(&self) -> usize { RULE_number }
    fn make_node(
        arena: &'arena Arena,
        ctx: NumberContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a NumberContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => NumberContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut NumberContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut NumberContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NumberContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_num = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, NumberContextExt {
				INT: None, FLOAT: None, 
				num: _init_num, 
				ph: PhantomData
			}
		)
	}
}

pub trait NumberContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_num(&self) -> &Number; 
    fn set_num(&mut self,attr: Number); 
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token FLOAT
    /// Returns `None` if there is no child corresponding to token FLOAT
    fn FLOAT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> NumberContextAttrs<'input, 'arena, Tok> for NumberContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_num(&self) -> &Number { &self.deref().num }  
    fn set_num(&mut self,attr: Number) { self.deref_mut().num = attr; }  
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_INT)
    }
    /// Retrieves first TerminalNode corresponding to token FLOAT
    /// Returns `None` if there is no child corresponding to token FLOAT
    fn FLOAT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_FLOAT)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn number(&mut self,) -> Result<&'arena NumberContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(NumberContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 22, RULE_number)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena NumberContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(158);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_INT  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(154);
			        let tmp = recog.base.match_token(CFood_INT,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<NumberContext<TF::Tok>>().unwrap().INT = Some(tmp); } 
			        let tmp = { Number::parse_int(if let Some(it) = &recog.ctx().unwrap().as_rule_context::<NumberContext<TF::Tok>>().unwrap().INT { it.get_text() } else { "null" } )}.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<NumberContext<TF::Tok>>().unwrap().set_num(tmp); }
			        }}
			    CFood_FLOAT  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(156);
			        let tmp = recog.base.match_token(CFood_FLOAT,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<NumberContext<TF::Tok>>().unwrap().FLOAT = Some(tmp); } 
			        let tmp = { Number::parse_float(if let Some(it) = &recog.ctx().unwrap().as_rule_context::<NumberContext<TF::Tok>>().unwrap().FLOAT { it.get_text() } else { "null" } )}.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<NumberContext<TF::Tok>>().unwrap().set_num(tmp); }
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- tys ----------------
pub type TysContextAll<'input, 'arena, Tok = CommonToken<'input>> = TysContext<'input, 'arena, Tok>;

pub type TysContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, TysContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct TysContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub kinds: Vec<TyKind>,
	pub ty_kind: Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>,
	pub tys: Option<&'arena TysContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for TysContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::TysContext }
	fn get_rule_index(&self) -> usize { RULE_tys }
    fn make_node(
        arena: &'arena Arena,
        ctx: TysContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a TysContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => TysContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut TysContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut TysContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TysContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_kinds = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, TysContextExt {
				ty_kind: None, tys: None, 
				kinds: _init_kinds, 
				ph: PhantomData
			}
		)
	}
}

pub trait TysContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_kinds(&self) -> &Vec<TyKind>; 
    fn set_kinds(&mut self,attr: Vec<TyKind>); 
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn tys(&self) -> Option<&'arena TysContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TysContextAttrs<'input, 'arena, Tok> for TysContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_kinds(&self) -> &Vec<TyKind> { &self.deref().kinds }  
    fn set_kinds(&mut self,attr: Vec<TyKind>) { self.deref_mut().kinds = attr; }  
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_COMMA)
    }
    fn tys(&self) -> Option<&'arena TysContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn tys(&mut self,) -> Result<&'arena TysContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(TysContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 24, RULE_tys)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena TysContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(168);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(7,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(160);
					let tmp = recog.ty_kind()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TysContext<TF::Tok>>().unwrap().ty_kind = Some(tmp); } 
					recog.base.set_state(161);
					recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					/*InvokeRule tys*/
					recog.base.set_state(162);
					let tmp = recog.tys()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TysContext<TF::Tok>>().unwrap().tys = Some(tmp); } 

					        let tmp = { [vec![recog.ctx().unwrap().as_rule_context::<TysContext<TF::Tok>>().unwrap().ty_kind.as_ref().unwrap().get_kind().clone()], recog.ctx().unwrap().as_rule_context::<TysContext<TF::Tok>>().unwrap().tys.as_ref().unwrap().get_kinds().clone()].concat()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TysContext<TF::Tok>>().unwrap().set_kinds(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(165);
					let tmp = recog.ty_kind()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TysContext<TF::Tok>>().unwrap().ty_kind = Some(tmp); } 

					        let tmp = { vec![recog.ctx().unwrap().as_rule_context::<TysContext<TF::Tok>>().unwrap().ty_kind.as_ref().unwrap().get_kind().clone()]}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TysContext<TF::Tok>>().unwrap().set_kinds(tmp); }
					    
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ty ----------------
pub type TyContextAll<'input, 'arena, Tok = CommonToken<'input>> = TyContext<'input, 'arena, Tok>;

pub type TyContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, TyContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct TyContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_raw: TyRaw,
	pub ty_kind: Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>,
	pub ty: Option<&'arena TyContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for TyContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::TyContext }
	fn get_rule_index(&self) -> usize { RULE_ty }
    fn make_node(
        arena: &'arena Arena,
        ctx: TyContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a TyContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => TyContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut TyContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut TyContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TyContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_raw = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, TyContextExt {
				ty_kind: None, ty: None, 
				ty_raw: _init_ty_raw, 
				ph: PhantomData
			}
		)
	}
}

pub trait TyContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_raw(&self) -> &TyRaw; 
    fn set_ty_raw(&mut self,attr: TyRaw); 
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ARROW
    /// Returns `None` if there is no child corresponding to token ARROW
    fn ARROW(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn ty(&self) -> Option<&'arena TyContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TyContextAttrs<'input, 'arena, Tok> for TyContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_raw(&self) -> &TyRaw { &self.deref().ty_raw }  
    fn set_ty_raw(&mut self,attr: TyRaw) { self.deref_mut().ty_raw = attr; }  
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ARROW
    /// Returns `None` if there is no child corresponding to token ARROW
    fn ARROW(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_ARROW)
    }
    fn ty(&self) -> Option<&'arena TyContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ty(&mut self,) -> Result<&'arena TyContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(TyContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 26, RULE_ty)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena TyContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(178);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(8,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(170);
					let tmp = recog.ty_kind()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TyContext<TF::Tok>>().unwrap().ty_kind = Some(tmp); } 
					let tmp = { TyRaw::Kind(recog.ctx().unwrap().as_rule_context::<TyContext<TF::Tok>>().unwrap().ty_kind.as_ref().unwrap().get_kind().clone())}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TyContext<TF::Tok>>().unwrap().set_ty_raw(tmp); }
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(173);
					let tmp = recog.ty_kind()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TyContext<TF::Tok>>().unwrap().ty_kind = Some(tmp); } 
					recog.base.set_state(174);
					recog.base.match_token(CFood_ARROW,&mut recog.err_handler)?;
					/*InvokeRule ty*/
					recog.base.set_state(175);
					let tmp = recog.ty()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TyContext<TF::Tok>>().unwrap().ty = Some(tmp); } 
					let tmp = { TyRaw::Arrow(
					            recog.ctx().unwrap().as_rule_context::<TyContext<TF::Tok>>().unwrap().ty_kind.as_ref().unwrap().get_kind().clone(),
					            Box::new(recog.ctx().unwrap().as_rule_context::<TyContext<TF::Tok>>().unwrap().ty.as_ref().unwrap().get_ty_raw().clone())
					        )}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<TyContext<TF::Tok>>().unwrap().set_ty_raw(tmp); }
					    
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ty_kind ----------------
pub type Ty_kindContextAll<'input, 'arena, Tok = CommonToken<'input>> = Ty_kindContext<'input, 'arena, Tok>;

pub type Ty_kindContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Ty_kindContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Ty_kindContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub kind: TyKind,
	pub TYPE: Option<&'arena dyn Token >,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Ty_kindContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Ty_kindContext }
	fn get_rule_index(&self) -> usize { RULE_ty_kind }
    fn make_node(
        arena: &'arena Arena,
        ctx: Ty_kindContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Ty_kindContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Ty_kindContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Ty_kindContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Ty_kindContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Ty_kindContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_kind = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Ty_kindContextExt {
				TYPE: None, 
				kind: _init_kind, 
				ph: PhantomData
			}
		)
	}
}

pub trait Ty_kindContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_kind(&self) -> &TyKind; 
    fn set_kind(&mut self,attr: TyKind); 
    /// Retrieves first TerminalNode corresponding to token TY_int
    /// Returns `None` if there is no child corresponding to token TY_int
    fn TY_int(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TY_float
    /// Returns `None` if there is no child corresponding to token TY_float
    fn TY_float(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TY_void
    /// Returns `None` if there is no child corresponding to token TY_void
    fn TY_void(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TYPE
    /// Returns `None` if there is no child corresponding to token TYPE
    fn TYPE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Ty_kindContextAttrs<'input, 'arena, Tok> for Ty_kindContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_kind(&self) -> &TyKind { &self.deref().kind }  
    fn set_kind(&mut self,attr: TyKind) { self.deref_mut().kind = attr; }  
    /// Retrieves first TerminalNode corresponding to token TY_int
    /// Returns `None` if there is no child corresponding to token TY_int
    fn TY_int(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TY_int)
    }
    /// Retrieves first TerminalNode corresponding to token TY_float
    /// Returns `None` if there is no child corresponding to token TY_float
    fn TY_float(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TY_float)
    }
    /// Retrieves first TerminalNode corresponding to token TY_void
    /// Returns `None` if there is no child corresponding to token TY_void
    fn TY_void(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TY_void)
    }
    /// Retrieves first TerminalNode corresponding to token TYPE
    /// Returns `None` if there is no child corresponding to token TYPE
    fn TYPE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TYPE)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ty_kind(&mut self,) -> Result<&'arena Ty_kindContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Ty_kindContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 28, RULE_ty_kind)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kindContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(188);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_TY_int  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(180);
			        recog.base.match_token(CFood_TY_int,&mut recog.err_handler)?;
			        let tmp = { TyKind::Primitive(Primitive::Int)}.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Ty_kindContext<TF::Tok>>().unwrap().set_kind(tmp); }
			        }}
			    CFood_TY_float  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(182);
			        recog.base.match_token(CFood_TY_float,&mut recog.err_handler)?;
			        let tmp = { TyKind::Primitive(Primitive::Float)}.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Ty_kindContext<TF::Tok>>().unwrap().set_kind(tmp); }
			        }}
			    CFood_TY_void  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        recog.base.set_state(184);
			        recog.base.match_token(CFood_TY_void,&mut recog.err_handler)?;
			        let tmp = { TyKind::Primitive(Primitive::Void)}.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Ty_kindContext<TF::Tok>>().unwrap().set_kind(tmp); }
			        }}
			    CFood_TYPE  => {
			        /*------- Outer Most Alt 4 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
			        {
			        recog.base.set_state(186);
			        let tmp = recog.base.match_token(CFood_TYPE,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Ty_kindContext<TF::Tok>>().unwrap().TYPE = Some(tmp); } 
			        let tmp = { TyKind::Type(if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Ty_kindContext<TF::Tok>>().unwrap().TYPE { it.get_text() } else { "null" } .to_string())}.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Ty_kindContext<TF::Tok>>().unwrap().set_kind(tmp); }
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- block ----------------
pub type BlockContextAll<'input, 'arena, Tok = CommonToken<'input>> = BlockContext<'input, 'arena, Tok>;

pub type BlockContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, BlockContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct BlockContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for BlockContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::BlockContext }
	fn get_rule_index(&self) -> usize { RULE_block }
    fn make_node(
        arena: &'arena Arena,
        ctx: BlockContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a BlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => BlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut BlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut BlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> BlockContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, BlockContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait BlockContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token BRACE_L
    /// Returns `None` if there is no child corresponding to token BRACE_L
    fn BRACE_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn stmts(&self) -> Option<&'arena StmtsContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token BRACE_R
    /// Returns `None` if there is no child corresponding to token BRACE_R
    fn BRACE_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> BlockContextAttrs<'input, 'arena, Tok> for BlockContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token BRACE_L
    /// Returns `None` if there is no child corresponding to token BRACE_L
    fn BRACE_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_BRACE_L)
    }
    fn stmts(&self) -> Option<&'arena StmtsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACE_R
    /// Returns `None` if there is no child corresponding to token BRACE_R
    fn BRACE_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_BRACE_R)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn block(&mut self,) -> Result<&'arena BlockContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(BlockContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 30, RULE_block)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena BlockContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(190);
			recog.base.match_token(CFood_BRACE_L,&mut recog.err_handler)?;

			        recog.tlt.enter_block();
			    
			/*InvokeRule stmts*/
			recog.base.set_state(192);
			recog.stmts()?;

			        recog.tlt.exit_block();
			    
			recog.base.set_state(194);
			recog.base.match_token(CFood_BRACE_R,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- stmts ----------------
pub type StmtsContextAll<'input, 'arena, Tok = CommonToken<'input>> = StmtsContext<'input, 'arena, Tok>;

pub type StmtsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, StmtsContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct StmtsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for StmtsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::StmtsContext }
	fn get_rule_index(&self) -> usize { RULE_stmts }
    fn make_node(
        arena: &'arena Arena,
        ctx: StmtsContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a StmtsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => StmtsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut StmtsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut StmtsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> StmtsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, StmtsContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait StmtsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn stmt(&self) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>>;
    fn stmts(&self) -> Option<&'arena StmtsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StmtsContextAttrs<'input, 'arena, Tok> for StmtsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn stmt(&self) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn stmts(&self) -> Option<&'arena StmtsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn stmts(&mut self,) -> Result<&'arena StmtsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(StmtsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 32, RULE_stmts)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(200);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_KW_while |CFood_KW_if |CFood_KW_return |CFood_KW_let |CFood_TY_int |
			    CFood_TY_float |CFood_TY_void |CFood_PAREN_L |CFood_BRACE_L |CFood_TYPE |
			    CFood_IDENT |CFood_INT |CFood_FLOAT  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule stmt*/
			        recog.base.set_state(196);
			        recog.stmt()?;
			        /*InvokeRule stmts*/
			        recog.base.set_state(197);
			        recog.stmts()?;
			        }}
			    CFood_BRACE_R  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- stmt ----------------
pub type StmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = StmtContext<'input, 'arena, Tok>;

pub type StmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, StmtContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct StmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for StmtContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::StmtContext }
	fn get_rule_index(&self) -> usize { RULE_stmt }
    fn make_node(
        arena: &'arena Arena,
        ctx: StmtContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a StmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => StmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut StmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut StmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> StmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, StmtContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait StmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn var_decl(&self) -> Option<&'arena Var_declContextAll<'input, 'arena, Tok>>;
    fn let_stmt(&self) -> Option<&'arena Let_stmtContextAll<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn branch_stmt(&self) -> Option<&'arena Branch_stmtContextAll<'input, 'arena, Tok>>;
    fn iter_stmt(&self) -> Option<&'arena Iter_stmtContextAll<'input, 'arena, Tok>>;
    fn return_stmt(&self) -> Option<&'arena Return_stmtContextAll<'input, 'arena, Tok>>;
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StmtContextAttrs<'input, 'arena, Tok> for StmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn var_decl(&self) -> Option<&'arena Var_declContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn let_stmt(&self) -> Option<&'arena Let_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_SEMICOLON)
    }
    fn branch_stmt(&self) -> Option<&'arena Branch_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn iter_stmt(&self) -> Option<&'arena Iter_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn return_stmt(&self) -> Option<&'arena Return_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn stmt(&mut self,) -> Result<&'arena StmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(StmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 34, RULE_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(211);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_TY_int |CFood_TY_float |CFood_TY_void |CFood_TYPE  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule var_decl*/
			        recog.base.set_state(202);
			        recog.var_decl()?;
			        }}
			    CFood_KW_let  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        /*InvokeRule let_stmt*/
			        recog.base.set_state(203);
			        recog.let_stmt()?;
			        }}
			    CFood_PAREN_L |CFood_IDENT |CFood_INT |CFood_FLOAT  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        /*InvokeRule expr*/
			        recog.base.set_state(204);
			        recog.expr()?;
			        recog.base.set_state(205);
			        recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
			        }}
			    CFood_KW_if  => {
			        /*------- Outer Most Alt 4 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
			        {
			        /*InvokeRule branch_stmt*/
			        recog.base.set_state(207);
			        recog.branch_stmt()?;
			        }}
			    CFood_KW_while  => {
			        /*------- Outer Most Alt 5 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
			        {
			        /*InvokeRule iter_stmt*/
			        recog.base.set_state(208);
			        recog.iter_stmt()?;
			        }}
			    CFood_KW_return  => {
			        /*------- Outer Most Alt 6 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(6); }
			        {
			        /*InvokeRule return_stmt*/
			        recog.base.set_state(209);
			        recog.return_stmt()?;
			        }}
			    CFood_BRACE_L  => {
			        /*------- Outer Most Alt 7 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(7); }
			        {
			        /*InvokeRule block*/
			        recog.base.set_state(210);
			        recog.block()?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- may_empty_stmt ----------------
pub type May_empty_stmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = May_empty_stmtContext<'input, 'arena, Tok>;

pub type May_empty_stmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, May_empty_stmtContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct May_empty_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for May_empty_stmtContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::May_empty_stmtContext }
	fn get_rule_index(&self) -> usize { RULE_may_empty_stmt }
    fn make_node(
        arena: &'arena Arena,
        ctx: May_empty_stmtContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a May_empty_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => May_empty_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut May_empty_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut May_empty_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> May_empty_stmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, May_empty_stmtContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait May_empty_stmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn stmt(&self) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> May_empty_stmtContextAttrs<'input, 'arena, Tok> for May_empty_stmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn stmt(&self) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_SEMICOLON)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn may_empty_stmt(&mut self,) -> Result<&'arena May_empty_stmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(May_empty_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 36, RULE_may_empty_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena May_empty_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(215);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_KW_while |CFood_KW_if |CFood_KW_return |CFood_KW_let |CFood_TY_int |
			    CFood_TY_float |CFood_TY_void |CFood_PAREN_L |CFood_BRACE_L |CFood_TYPE |
			    CFood_IDENT |CFood_INT |CFood_FLOAT  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule stmt*/
			        recog.base.set_state(213);
			        recog.stmt()?;
			        }}
			    CFood_SEMICOLON  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(214);
			        recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- branch_stmt ----------------
pub type Branch_stmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = Branch_stmtContext<'input, 'arena, Tok>;

pub type Branch_stmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Branch_stmtContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Branch_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub KW_if: Option<&'arena dyn Token >,
	pub expr: Option<&'arena ExprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Branch_stmtContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Branch_stmtContext }
	fn get_rule_index(&self) -> usize { RULE_branch_stmt }
    fn make_node(
        arena: &'arena Arena,
        ctx: Branch_stmtContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Branch_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Branch_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Branch_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Branch_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Branch_stmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Branch_stmtContextExt {
				KW_if: None, 
				expr: None, 
				ph: PhantomData
			}
		)
	}
}

pub trait Branch_stmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token KW_if
    /// Returns `None` if there is no child corresponding to token KW_if
    fn KW_if(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn may_empty_stmt_all(&self) -> Vec<&'arena May_empty_stmtContextAll<'input, 'arena, Tok>>;
    fn may_empty_stmt(&self, i: usize) -> Option<&'arena May_empty_stmtContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token KW_else
    /// Returns `None` if there is no child corresponding to token KW_else
    fn KW_else(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Branch_stmtContextAttrs<'input, 'arena, Tok> for Branch_stmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token KW_if
    /// Returns `None` if there is no child corresponding to token KW_if
    fn KW_if(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_KW_if)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_L)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_R)
    }
    fn may_empty_stmt_all(&self) -> Vec<&'arena May_empty_stmtContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn may_empty_stmt(&self, i: usize) -> Option<&'arena May_empty_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token KW_else
    /// Returns `None` if there is no child corresponding to token KW_else
    fn KW_else(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_KW_else)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn branch_stmt(&mut self,) -> Result<&'arena Branch_stmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Branch_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 38, RULE_branch_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Branch_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(233);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(13,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(217);
					let tmp = recog.base.match_token(CFood_KW_if,&mut recog.err_handler)?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Branch_stmtContext<TF::Tok>>().unwrap().KW_if = Some(tmp); } 
					recog.base.set_state(218);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(219);
					let tmp = recog.expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Branch_stmtContext<TF::Tok>>().unwrap().expr = Some(tmp); } 
					recog.base.set_state(220);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					/*InvokeRule may_empty_stmt*/
					recog.base.set_state(221);
					recog.may_empty_stmt()?;

					        let bound = recog.tlt.new_ty(Ty::bool());
					        let expr = *recog.ctx().unwrap().as_rule_context::<Branch_stmtContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id();
					        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Branch_stmtContext<TF::Tok>>().unwrap().KW_if { it.get_line() } else { 0 } ;
					        recog.tlt.assert_ty_id(bound, expr, line);
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(224);
					let tmp = recog.base.match_token(CFood_KW_if,&mut recog.err_handler)?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Branch_stmtContext<TF::Tok>>().unwrap().KW_if = Some(tmp); } 
					recog.base.set_state(225);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(226);
					let tmp = recog.expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Branch_stmtContext<TF::Tok>>().unwrap().expr = Some(tmp); } 
					recog.base.set_state(227);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					/*InvokeRule may_empty_stmt*/
					recog.base.set_state(228);
					recog.may_empty_stmt()?;
					recog.base.set_state(229);
					recog.base.match_token(CFood_KW_else,&mut recog.err_handler)?;
					/*InvokeRule may_empty_stmt*/
					recog.base.set_state(230);
					recog.may_empty_stmt()?;

					        let bound = recog.tlt.new_ty(Ty::bool());
					        let expr = *recog.ctx().unwrap().as_rule_context::<Branch_stmtContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id();
					        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Branch_stmtContext<TF::Tok>>().unwrap().KW_if { it.get_line() } else { 0 } ;
					        recog.tlt.assert_ty_id(bound, expr, line);
					    
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- iter_stmt ----------------
pub type Iter_stmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = Iter_stmtContext<'input, 'arena, Tok>;

pub type Iter_stmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Iter_stmtContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Iter_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub KW_while: Option<&'arena dyn Token >,
	pub expr: Option<&'arena ExprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Iter_stmtContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Iter_stmtContext }
	fn get_rule_index(&self) -> usize { RULE_iter_stmt }
    fn make_node(
        arena: &'arena Arena,
        ctx: Iter_stmtContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Iter_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Iter_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Iter_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Iter_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Iter_stmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Iter_stmtContextExt {
				KW_while: None, 
				expr: None, 
				ph: PhantomData
			}
		)
	}
}

pub trait Iter_stmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token KW_while
    /// Returns `None` if there is no child corresponding to token KW_while
    fn KW_while(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn may_empty_stmt(&self) -> Option<&'arena May_empty_stmtContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Iter_stmtContextAttrs<'input, 'arena, Tok> for Iter_stmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token KW_while
    /// Returns `None` if there is no child corresponding to token KW_while
    fn KW_while(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_KW_while)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_L)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_R)
    }
    fn may_empty_stmt(&self) -> Option<&'arena May_empty_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn iter_stmt(&mut self,) -> Result<&'arena Iter_stmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Iter_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 40, RULE_iter_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Iter_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(235);
			let tmp = recog.base.match_token(CFood_KW_while,&mut recog.err_handler)?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Iter_stmtContext<TF::Tok>>().unwrap().KW_while = Some(tmp); } 
			recog.base.set_state(236);
			recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
			/*InvokeRule expr*/
			recog.base.set_state(237);
			let tmp = recog.expr()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Iter_stmtContext<TF::Tok>>().unwrap().expr = Some(tmp); } 
			recog.base.set_state(238);
			recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
			/*InvokeRule may_empty_stmt*/
			recog.base.set_state(239);
			recog.may_empty_stmt()?;

			        let bound = recog.tlt.new_ty(Ty::bool());
			        let expr = *recog.ctx().unwrap().as_rule_context::<Iter_stmtContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id();
			        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Iter_stmtContext<TF::Tok>>().unwrap().KW_while { it.get_line() } else { 0 } ;
			        recog.tlt.assert_ty_id(bound, expr, line);
			    
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- return_stmt ----------------
pub type Return_stmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = Return_stmtContext<'input, 'arena, Tok>;

pub type Return_stmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Return_stmtContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Return_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub KW_return: Option<&'arena dyn Token >,
	pub expr: Option<&'arena ExprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Return_stmtContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Return_stmtContext }
	fn get_rule_index(&self) -> usize { RULE_return_stmt }
    fn make_node(
        arena: &'arena Arena,
        ctx: Return_stmtContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Return_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Return_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Return_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Return_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Return_stmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Return_stmtContextExt {
				KW_return: None, 
				expr: None, 
				ph: PhantomData
			}
		)
	}
}

pub trait Return_stmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token KW_return
    /// Returns `None` if there is no child corresponding to token KW_return
    fn KW_return(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Return_stmtContextAttrs<'input, 'arena, Tok> for Return_stmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token KW_return
    /// Returns `None` if there is no child corresponding to token KW_return
    fn KW_return(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_KW_return)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_SEMICOLON)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn return_stmt(&mut self,) -> Result<&'arena Return_stmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Return_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 42, RULE_return_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Return_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(250);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(14,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(242);
					let tmp = recog.base.match_token(CFood_KW_return,&mut recog.err_handler)?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Return_stmtContext<TF::Tok>>().unwrap().KW_return = Some(tmp); } 
					recog.base.set_state(243);
					recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;

					        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Return_stmtContext<TF::Tok>>().unwrap().KW_return { it.get_line() } else { 0 } ;
					        let bound = recog.tlt.var("return", line);
					        let expr = recog.tlt.new_ty(Ty::void());
					        recog.tlt.assert_ty_id(bound, expr, line);
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(245);
					let tmp = recog.base.match_token(CFood_KW_return,&mut recog.err_handler)?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Return_stmtContext<TF::Tok>>().unwrap().KW_return = Some(tmp); } 
					/*InvokeRule expr*/
					recog.base.set_state(246);
					let tmp = recog.expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Return_stmtContext<TF::Tok>>().unwrap().expr = Some(tmp); } 
					recog.base.set_state(247);
					recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;

					        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Return_stmtContext<TF::Tok>>().unwrap().KW_return { it.get_line() } else { 0 } ;
					        let bound = recog.tlt.var("return", line);
					        let expr = *recog.ctx().unwrap().as_rule_context::<Return_stmtContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id();
					        recog.tlt.assert_ty_id(bound, expr, line);
					    
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- let_stmt ----------------
pub type Let_stmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = Let_stmtContext<'input, 'arena, Tok>;

pub type Let_stmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Let_stmtContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Let_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub KW_let: Option<&'arena dyn Token >,
	pub IDENT: Option<&'arena dyn Token >,
	pub expr: Option<&'arena ExprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Let_stmtContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Let_stmtContext }
	fn get_rule_index(&self) -> usize { RULE_let_stmt }
    fn make_node(
        arena: &'arena Arena,
        ctx: Let_stmtContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Let_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Let_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Let_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Let_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Let_stmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Let_stmtContextExt {
				KW_let: None, IDENT: None, 
				expr: None, 
				ph: PhantomData
			}
		)
	}
}

pub trait Let_stmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token KW_let
    /// Returns `None` if there is no child corresponding to token KW_let
    fn KW_let(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Let_stmtContextAttrs<'input, 'arena, Tok> for Let_stmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token KW_let
    /// Returns `None` if there is no child corresponding to token KW_let
    fn KW_let(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_KW_let)
    }
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_IDENT)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_ASSIGN)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_SEMICOLON)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn let_stmt(&mut self,) -> Result<&'arena Let_stmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Let_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 44, RULE_let_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Let_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(252);
			let tmp = recog.base.match_token(CFood_KW_let,&mut recog.err_handler)?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Let_stmtContext<TF::Tok>>().unwrap().KW_let = Some(tmp); } 
			recog.base.set_state(253);
			let tmp = recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Let_stmtContext<TF::Tok>>().unwrap().IDENT = Some(tmp); } 
			recog.base.set_state(254);
			recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			/*InvokeRule expr*/
			recog.base.set_state(255);
			let tmp = recog.expr()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Let_stmtContext<TF::Tok>>().unwrap().expr = Some(tmp); } 
			recog.base.set_state(256);
			recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;

			        let name = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Let_stmtContext<TF::Tok>>().unwrap().IDENT { it.get_text() } else { "null" } ;
			        let expr = *recog.ctx().unwrap().as_rule_context::<Let_stmtContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id();
			        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Let_stmtContext<TF::Tok>>().unwrap().KW_let { it.get_line() } else { 0 } ;

			        recog.tlt.new_let(
			            name,
			            expr,
			            line
			        );
			        
			    
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input, 'arena, Tok = CommonToken<'input>> = ExprContext<'input, 'arena, Tok>;

pub type ExprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ExprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ExprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub assign_expr: Option<&'arena Assign_exprContextAll<'input, 'arena, Tok>>,
	pub calc_expr: Option<&'arena Calc_exprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ExprContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::ExprContext }
	fn get_rule_index(&self) -> usize { RULE_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: ExprContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ExprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => ExprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ExprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut ExprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ExprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, ExprContextExt {
				assign_expr: None, calc_expr: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait ExprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn assign_expr(&self) -> Option<&'arena Assign_exprContextAll<'input, 'arena, Tok>>;
    fn calc_expr(&self) -> Option<&'arena Calc_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ExprContextAttrs<'input, 'arena, Tok> for ExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn assign_expr(&self) -> Option<&'arena Assign_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn calc_expr(&self) -> Option<&'arena Calc_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr(&mut self,) -> Result<&'arena ExprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ExprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 46, RULE_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ExprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(265);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(15,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule assign_expr*/
					recog.base.set_state(259);
					let tmp = recog.assign_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ExprContext<TF::Tok>>().unwrap().assign_expr = Some(tmp); } 

					        let tmp = { *recog.ctx().unwrap().as_rule_context::<ExprContext<TF::Tok>>().unwrap().assign_expr.as_ref().unwrap().get_ty_id()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ExprContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule calc_expr*/
					recog.base.set_state(262);
					let tmp = recog.calc_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ExprContext<TF::Tok>>().unwrap().calc_expr = Some(tmp); } 

					        let tmp = { *recog.ctx().unwrap().as_rule_context::<ExprContext<TF::Tok>>().unwrap().calc_expr.as_ref().unwrap().get_ty_id()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ExprContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- assign_expr ----------------
pub type Assign_exprContextAll<'input, 'arena, Tok = CommonToken<'input>> = Assign_exprContext<'input, 'arena, Tok>;

pub type Assign_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Assign_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Assign_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub var: Option<&'arena VarContextAll<'input, 'arena, Tok>>,
	pub ASSIGN: Option<&'arena dyn Token >,
	pub expr: Option<&'arena ExprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Assign_exprContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Assign_exprContext }
	fn get_rule_index(&self) -> usize { RULE_assign_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Assign_exprContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Assign_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Assign_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Assign_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Assign_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Assign_exprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Assign_exprContextExt {
				ASSIGN: None, 
				var: None, expr: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Assign_exprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Assign_exprContextAttrs<'input, 'arena, Tok> for Assign_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_ASSIGN)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn assign_expr(&mut self,) -> Result<&'arena Assign_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Assign_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 48, RULE_assign_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Assign_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule var*/
			recog.base.set_state(267);
			let tmp = recog.var()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Assign_exprContext<TF::Tok>>().unwrap().var = Some(tmp); } 
			recog.base.set_state(268);
			let tmp = recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Assign_exprContext<TF::Tok>>().unwrap().ASSIGN = Some(tmp); } 
			/*InvokeRule expr*/
			recog.base.set_state(269);
			let tmp = recog.expr()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Assign_exprContext<TF::Tok>>().unwrap().expr = Some(tmp); } 

			        let tmp = { *recog.ctx().unwrap().as_rule_context::<Assign_exprContext<TF::Tok>>().unwrap().var.as_ref().unwrap().get_ty_id()}.to_owned();
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Assign_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); }

			        let bound = *recog.ctx().unwrap().as_rule_context::<Assign_exprContext<TF::Tok>>().unwrap().var.as_ref().unwrap().get_ty_id();
			        let expr = *recog.ctx().unwrap().as_rule_context::<Assign_exprContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id();
			        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Assign_exprContext<TF::Tok>>().unwrap().ASSIGN { it.get_line() } else { 0 } ;
			        recog.tlt.assert_ty_id(bound, expr, line);
			    
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- var ----------------
pub type VarContextAll<'input, 'arena, Tok = CommonToken<'input>> = VarContext<'input, 'arena, Tok>;

pub type VarContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, VarContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct VarContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub IDENT: Option<&'arena dyn Token >,
	pub expr: Option<&'arena ExprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for VarContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::VarContext }
	fn get_rule_index(&self) -> usize { RULE_var }
    fn make_node(
        arena: &'arena Arena,
        ctx: VarContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a VarContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => VarContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut VarContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut VarContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> VarContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, VarContextExt {
				IDENT: None, 
				expr: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait VarContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token BRACKET_L
    /// Returns `None` if there is no child corresponding to token BRACKET_L
    fn BRACKET_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token BRACKET_R
    /// Returns `None` if there is no child corresponding to token BRACKET_R
    fn BRACKET_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> VarContextAttrs<'input, 'arena, Tok> for VarContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_IDENT)
    }
    /// Retrieves first TerminalNode corresponding to token BRACKET_L
    /// Returns `None` if there is no child corresponding to token BRACKET_L
    fn BRACKET_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_BRACKET_L)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACKET_R
    /// Returns `None` if there is no child corresponding to token BRACKET_R
    fn BRACKET_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_BRACKET_R)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn var(&mut self,) -> Result<&'arena VarContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(VarContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 50, RULE_var)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena VarContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(280);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(16,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(272);
					let tmp = recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<VarContext<TF::Tok>>().unwrap().IDENT = Some(tmp); } 

					        let name = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<VarContext<TF::Tok>>().unwrap().IDENT { it.get_text() } else { "null" } ;
					        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<VarContext<TF::Tok>>().unwrap().IDENT { it.get_line() } else { 0 } ;
					        let tmp = { recog.tlt.var(name, line)}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<VarContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(274);
					let tmp = recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<VarContext<TF::Tok>>().unwrap().IDENT = Some(tmp); } 
					recog.base.set_state(275);
					recog.base.match_token(CFood_BRACKET_L,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(276);
					let tmp = recog.expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<VarContext<TF::Tok>>().unwrap().expr = Some(tmp); } 
					recog.base.set_state(277);
					recog.base.match_token(CFood_BRACKET_R,&mut recog.err_handler)?;

					        let name = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<VarContext<TF::Tok>>().unwrap().IDENT { it.get_text() } else { "null" } ;
					        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<VarContext<TF::Tok>>().unwrap().IDENT { it.get_line() } else { 0 } ;
					        let tmp = { recog.tlt.var_arr(name, line)}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<VarContext<TF::Tok>>().unwrap().set_ty_id(tmp); }

					        let bound = recog.tlt.new_ty(Ty::int());
					        let expr = *recog.ctx().unwrap().as_rule_context::<VarContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id();
					        recog.tlt.assert_ty_id(bound, expr, line);
					    
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- calc_expr ----------------
pub type Calc_exprContextAll<'input, 'arena, Tok = CommonToken<'input>> = Calc_exprContext<'input, 'arena, Tok>;

pub type Calc_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Calc_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Calc_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub lhs: Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>,
	pub cmp_preced_op: Option<&'arena Cmp_preced_opContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>,
	pub call_preced_expr: Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Calc_exprContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Calc_exprContext }
	fn get_rule_index(&self) -> usize { RULE_calc_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Calc_exprContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Calc_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Calc_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Calc_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Calc_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Calc_exprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Calc_exprContextExt {
				lhs: None, cmp_preced_op: None, rhs: None, call_preced_expr: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Calc_exprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn cmp_preced_op(&self) -> Option<&'arena Cmp_preced_opContextAll<'input, 'arena, Tok>>;
    fn call_preced_expr_all(&self) -> Vec<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>;
    fn call_preced_expr(&self, i: usize) -> Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Calc_exprContextAttrs<'input, 'arena, Tok> for Calc_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn cmp_preced_op(&self) -> Option<&'arena Cmp_preced_opContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn call_preced_expr_all(&self) -> Vec<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn call_preced_expr(&self, i: usize) -> Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn calc_expr(&mut self,) -> Result<&'arena Calc_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Calc_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 52, RULE_calc_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Calc_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(290);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(17,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule call_preced_expr*/
					recog.base.set_state(282);
					let tmp = recog.call_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Calc_exprContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					/*InvokeRule cmp_preced_op*/
					recog.base.set_state(283);
					let tmp = recog.cmp_preced_op()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Calc_exprContext<TF::Tok>>().unwrap().cmp_preced_op = Some(tmp); } 
					/*InvokeRule call_preced_expr*/
					recog.base.set_state(284);
					let tmp = recog.call_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Calc_exprContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 

					        let lhs= *recog.ctx().unwrap().as_rule_context::<Calc_exprContext<TF::Tok>>().unwrap().lhs.as_ref().unwrap().get_ty_id();
					        let rhs = *recog.ctx().unwrap().as_rule_context::<Calc_exprContext<TF::Tok>>().unwrap().rhs.as_ref().unwrap().get_ty_id();
					        let line = *recog.ctx().unwrap().as_rule_context::<Calc_exprContext<TF::Tok>>().unwrap().cmp_preced_op.as_ref().unwrap().get_line();
					        let tmp = { recog.tlt.cmp_op(lhs, rhs, line)}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Calc_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule call_preced_expr*/
					recog.base.set_state(287);
					let tmp = recog.call_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Calc_exprContext<TF::Tok>>().unwrap().call_preced_expr = Some(tmp); } 
					 let tmp = { recog.ctx().unwrap().as_rule_context::<Calc_exprContext<TF::Tok>>().unwrap().call_preced_expr.as_ref().unwrap().get_ty_id()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Calc_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); } 
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- call_preced_expr ----------------
pub type Call_preced_exprContextAll<'input, 'arena, Tok = CommonToken<'input>> = Call_preced_exprContext<'input, 'arena, Tok>;

pub type Call_preced_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Call_preced_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Call_preced_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub add_preced_expr: Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>>,
	pub call_preced_expr: Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Call_preced_exprContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Call_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_call_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Call_preced_exprContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Call_preced_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Call_preced_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Call_preced_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Call_preced_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Call_preced_exprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Call_preced_exprContextExt {
				add_preced_expr: None, call_preced_expr: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Call_preced_exprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn add_preced_expr(&self) -> Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>>;
    fn call_preced_expr(&self) -> Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Call_preced_exprContextAttrs<'input, 'arena, Tok> for Call_preced_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn add_preced_expr(&self) -> Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn call_preced_expr(&self) -> Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn call_preced_expr(&mut self,) -> Result<&'arena Call_preced_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Call_preced_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 54, RULE_call_preced_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Call_preced_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(299);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(18,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule add_preced_expr*/
					recog.base.set_state(292);
					let tmp = recog.add_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Call_preced_exprContext<TF::Tok>>().unwrap().add_preced_expr = Some(tmp); } 
					/*InvokeRule call_preced_expr*/
					recog.base.set_state(293);
					let tmp = recog.call_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Call_preced_exprContext<TF::Tok>>().unwrap().call_preced_expr = Some(tmp); } 

					        let expr = *recog.ctx().unwrap().as_rule_context::<Call_preced_exprContext<TF::Tok>>().unwrap().add_preced_expr.as_ref().unwrap().get_ty_id();
					        let call = *recog.ctx().unwrap().as_rule_context::<Call_preced_exprContext<TF::Tok>>().unwrap().call_preced_expr.as_ref().unwrap().get_ty_id();
					        let line = recog.get_current_context().start().get_line();
					        let tmp = { recog.tlt.apply(expr, call, line)}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Call_preced_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule add_preced_expr*/
					recog.base.set_state(296);
					let tmp = recog.add_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Call_preced_exprContext<TF::Tok>>().unwrap().add_preced_expr = Some(tmp); } 
					 let tmp = { recog.ctx().unwrap().as_rule_context::<Call_preced_exprContext<TF::Tok>>().unwrap().add_preced_expr.as_ref().unwrap().get_ty_id()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Call_preced_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); } 
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- add_preced_expr ----------------
pub type Add_preced_exprContextAll<'input, 'arena, Tok = CommonToken<'input>> = Add_preced_exprContext<'input, 'arena, Tok>;

pub type Add_preced_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Add_preced_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Add_preced_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub mul_preced_expr: Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>>,
	pub add_preced_op: Option<&'arena Add_preced_opContextAll<'input, 'arena, Tok>>,
	pub add_preced_expr: Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Add_preced_exprContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Add_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_add_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Add_preced_exprContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Add_preced_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Add_preced_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Add_preced_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Add_preced_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Add_preced_exprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Add_preced_exprContextExt {
				mul_preced_expr: None, add_preced_op: None, add_preced_expr: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Add_preced_exprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn mul_preced_expr(&self) -> Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>>;
    fn add_preced_op(&self) -> Option<&'arena Add_preced_opContextAll<'input, 'arena, Tok>>;
    fn add_preced_expr(&self) -> Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Add_preced_exprContextAttrs<'input, 'arena, Tok> for Add_preced_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn mul_preced_expr(&self) -> Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn add_preced_op(&self) -> Option<&'arena Add_preced_opContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn add_preced_expr(&self) -> Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn add_preced_expr(&mut self,) -> Result<&'arena Add_preced_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Add_preced_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 56, RULE_add_preced_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Add_preced_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(309);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(19,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule mul_preced_expr*/
					recog.base.set_state(301);
					let tmp = recog.mul_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_exprContext<TF::Tok>>().unwrap().mul_preced_expr = Some(tmp); } 
					/*InvokeRule add_preced_op*/
					recog.base.set_state(302);
					let tmp = recog.add_preced_op()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_exprContext<TF::Tok>>().unwrap().add_preced_op = Some(tmp); } 
					/*InvokeRule add_preced_expr*/
					recog.base.set_state(303);
					let tmp = recog.add_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_exprContext<TF::Tok>>().unwrap().add_preced_expr = Some(tmp); } 

					        let lhs = *recog.ctx().unwrap().as_rule_context::<Add_preced_exprContext<TF::Tok>>().unwrap().mul_preced_expr.as_ref().unwrap().get_ty_id();
					        let rhs = *recog.ctx().unwrap().as_rule_context::<Add_preced_exprContext<TF::Tok>>().unwrap().add_preced_expr.as_ref().unwrap().get_ty_id();
					        let line = *recog.ctx().unwrap().as_rule_context::<Add_preced_exprContext<TF::Tok>>().unwrap().add_preced_op.as_ref().unwrap().get_line();
					        let tmp = { recog.tlt.binary_op(lhs, rhs, line)}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule mul_preced_expr*/
					recog.base.set_state(306);
					let tmp = recog.mul_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_exprContext<TF::Tok>>().unwrap().mul_preced_expr = Some(tmp); } 
					 let tmp = { *recog.ctx().unwrap().as_rule_context::<Add_preced_exprContext<TF::Tok>>().unwrap().mul_preced_expr.as_ref().unwrap().get_ty_id()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); } 
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- mul_preced_expr ----------------
pub type Mul_preced_exprContextAll<'input, 'arena, Tok = CommonToken<'input>> = Mul_preced_exprContext<'input, 'arena, Tok>;

pub type Mul_preced_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Mul_preced_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Mul_preced_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub atom_preced_expr: Option<&'arena Atom_preced_exprContextAll<'input, 'arena, Tok>>,
	pub mul_preced_op: Option<&'arena Mul_preced_opContextAll<'input, 'arena, Tok>>,
	pub mul_preced_expr: Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Mul_preced_exprContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Mul_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_mul_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Mul_preced_exprContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Mul_preced_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Mul_preced_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Mul_preced_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Mul_preced_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Mul_preced_exprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Mul_preced_exprContextExt {
				atom_preced_expr: None, mul_preced_op: None, mul_preced_expr: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Mul_preced_exprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn atom_preced_expr(&self) -> Option<&'arena Atom_preced_exprContextAll<'input, 'arena, Tok>>;
    fn mul_preced_op(&self) -> Option<&'arena Mul_preced_opContextAll<'input, 'arena, Tok>>;
    fn mul_preced_expr(&self) -> Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Mul_preced_exprContextAttrs<'input, 'arena, Tok> for Mul_preced_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn atom_preced_expr(&self) -> Option<&'arena Atom_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn mul_preced_op(&self) -> Option<&'arena Mul_preced_opContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn mul_preced_expr(&self) -> Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn mul_preced_expr(&mut self,) -> Result<&'arena Mul_preced_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Mul_preced_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 58, RULE_mul_preced_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Mul_preced_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(319);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(20,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule atom_preced_expr*/
					recog.base.set_state(311);
					let tmp = recog.atom_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_exprContext<TF::Tok>>().unwrap().atom_preced_expr = Some(tmp); } 
					/*InvokeRule mul_preced_op*/
					recog.base.set_state(312);
					let tmp = recog.mul_preced_op()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_exprContext<TF::Tok>>().unwrap().mul_preced_op = Some(tmp); } 
					/*InvokeRule mul_preced_expr*/
					recog.base.set_state(313);
					let tmp = recog.mul_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_exprContext<TF::Tok>>().unwrap().mul_preced_expr = Some(tmp); } 

					        let lhs = *recog.ctx().unwrap().as_rule_context::<Mul_preced_exprContext<TF::Tok>>().unwrap().atom_preced_expr.as_ref().unwrap().get_ty_id();
					        let rhs = *recog.ctx().unwrap().as_rule_context::<Mul_preced_exprContext<TF::Tok>>().unwrap().mul_preced_expr.as_ref().unwrap().get_ty_id();
					        let line = *recog.ctx().unwrap().as_rule_context::<Mul_preced_exprContext<TF::Tok>>().unwrap().mul_preced_op.as_ref().unwrap().get_line();
					        let tmp = { recog.tlt.binary_op(lhs, rhs, line)}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule atom_preced_expr*/
					recog.base.set_state(316);
					let tmp = recog.atom_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_exprContext<TF::Tok>>().unwrap().atom_preced_expr = Some(tmp); } 
					 let tmp = { *recog.ctx().unwrap().as_rule_context::<Mul_preced_exprContext<TF::Tok>>().unwrap().atom_preced_expr.as_ref().unwrap().get_ty_id()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); } 
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- atom_preced_expr ----------------
pub type Atom_preced_exprContextAll<'input, 'arena, Tok = CommonToken<'input>> = Atom_preced_exprContext<'input, 'arena, Tok>;

pub type Atom_preced_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Atom_preced_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Atom_preced_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub apply_list: Option<&'arena Apply_listContextAll<'input, 'arena, Tok>>,
	pub var: Option<&'arena VarContextAll<'input, 'arena, Tok>>,
	pub number: Option<&'arena NumberContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Atom_preced_exprContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Atom_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_atom_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Atom_preced_exprContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Atom_preced_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Atom_preced_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Atom_preced_exprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Atom_preced_exprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_preced_exprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Atom_preced_exprContextExt {
				apply_list: None, var: None, number: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Atom_preced_exprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn apply_list(&self) -> Option<&'arena Apply_listContextAll<'input, 'arena, Tok>>;
    fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>>;
    fn number(&self) -> Option<&'arena NumberContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Atom_preced_exprContextAttrs<'input, 'arena, Tok> for Atom_preced_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn apply_list(&self) -> Option<&'arena Apply_listContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn number(&self) -> Option<&'arena NumberContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn atom_preced_expr(&mut self,) -> Result<&'arena Atom_preced_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Atom_preced_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 60, RULE_atom_preced_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Atom_preced_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(330);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_PAREN_L  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule apply_list*/
			        recog.base.set_state(321);
			        let tmp = recog.apply_list()?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Atom_preced_exprContext<TF::Tok>>().unwrap().apply_list = Some(tmp); } 
			        let tmp = { *recog.ctx().unwrap().as_rule_context::<Atom_preced_exprContext<TF::Tok>>().unwrap().apply_list.as_ref().unwrap().get_ty_id()}.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Atom_preced_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); } 
			        }}
			    CFood_IDENT  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        /*InvokeRule var*/
			        recog.base.set_state(324);
			        let tmp = recog.var()?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Atom_preced_exprContext<TF::Tok>>().unwrap().var = Some(tmp); } 
			         let tmp = { *recog.ctx().unwrap().as_rule_context::<Atom_preced_exprContext<TF::Tok>>().unwrap().var.as_ref().unwrap().get_ty_id()}.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Atom_preced_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); } 
			        }}
			    CFood_INT |CFood_FLOAT  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        /*InvokeRule number*/
			        recog.base.set_state(327);
			        let tmp = recog.number()?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Atom_preced_exprContext<TF::Tok>>().unwrap().number = Some(tmp); } 

			                let num = *recog.ctx().unwrap().as_rule_context::<Atom_preced_exprContext<TF::Tok>>().unwrap().number.as_ref().unwrap().get_num();
			                let tmp = { recog.tlt.new_ty_from_number(num)}.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Atom_preced_exprContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
			            
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- cmp_preced_op ----------------
pub type Cmp_preced_opContextAll<'input, 'arena, Tok = CommonToken<'input>> = Cmp_preced_opContext<'input, 'arena, Tok>;

pub type Cmp_preced_opContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Cmp_preced_opContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Cmp_preced_opContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub line: u32,
	pub NE: Option<&'arena dyn Token >,
	pub EQ: Option<&'arena dyn Token >,
	pub LT: Option<&'arena dyn Token >,
	pub GT: Option<&'arena dyn Token >,
	pub LE: Option<&'arena dyn Token >,
	pub GE: Option<&'arena dyn Token >,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Cmp_preced_opContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Cmp_preced_opContext }
	fn get_rule_index(&self) -> usize { RULE_cmp_preced_op }
    fn make_node(
        arena: &'arena Arena,
        ctx: Cmp_preced_opContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Cmp_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Cmp_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Cmp_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Cmp_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Cmp_preced_opContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_line = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Cmp_preced_opContextExt {
				NE: None, EQ: None, LT: None, GT: None, LE: None, GE: None, 
				line: _init_line, 
				ph: PhantomData
			}
		)
	}
}

pub trait Cmp_preced_opContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_line(&self) -> &u32; 
    fn set_line(&mut self,attr: u32); 
    /// Retrieves first TerminalNode corresponding to token NE
    /// Returns `None` if there is no child corresponding to token NE
    fn NE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EQ
    /// Returns `None` if there is no child corresponding to token EQ
    fn EQ(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token LE
    /// Returns `None` if there is no child corresponding to token LE
    fn LE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token GE
    /// Returns `None` if there is no child corresponding to token GE
    fn GE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Cmp_preced_opContextAttrs<'input, 'arena, Tok> for Cmp_preced_opContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_line(&self) -> &u32 { &self.deref().line }  
    fn set_line(&mut self,attr: u32) { self.deref_mut().line = attr; }  
    /// Retrieves first TerminalNode corresponding to token NE
    /// Returns `None` if there is no child corresponding to token NE
    fn NE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_NE)
    }
    /// Retrieves first TerminalNode corresponding to token EQ
    /// Returns `None` if there is no child corresponding to token EQ
    fn EQ(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_EQ)
    }
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_LT)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_GT)
    }
    /// Retrieves first TerminalNode corresponding to token LE
    /// Returns `None` if there is no child corresponding to token LE
    fn LE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_LE)
    }
    /// Retrieves first TerminalNode corresponding to token GE
    /// Returns `None` if there is no child corresponding to token GE
    fn GE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_GE)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn cmp_preced_op(&mut self,) -> Result<&'arena Cmp_preced_opContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Cmp_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 62, RULE_cmp_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Cmp_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(344);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_NE  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(332);
			        let tmp = recog.base.match_token(CFood_NE,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().NE = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Cmp_preced_opContext<TF::Tok>>().unwrap().NE { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
			    CFood_EQ  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(334);
			        let tmp = recog.base.match_token(CFood_EQ,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().EQ = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Cmp_preced_opContext<TF::Tok>>().unwrap().EQ { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
			    CFood_LT  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        recog.base.set_state(336);
			        let tmp = recog.base.match_token(CFood_LT,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().LT = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Cmp_preced_opContext<TF::Tok>>().unwrap().LT { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
			    CFood_GT  => {
			        /*------- Outer Most Alt 4 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
			        {
			        recog.base.set_state(338);
			        let tmp = recog.base.match_token(CFood_GT,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().GT = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Cmp_preced_opContext<TF::Tok>>().unwrap().GT { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
			    CFood_LE  => {
			        /*------- Outer Most Alt 5 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
			        {
			        recog.base.set_state(340);
			        let tmp = recog.base.match_token(CFood_LE,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().LE = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Cmp_preced_opContext<TF::Tok>>().unwrap().LE { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
			    CFood_GE  => {
			        /*------- Outer Most Alt 6 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(6); }
			        {
			        recog.base.set_state(342);
			        let tmp = recog.base.match_token(CFood_GE,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().GE = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Cmp_preced_opContext<TF::Tok>>().unwrap().GE { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Cmp_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- add_preced_op ----------------
pub type Add_preced_opContextAll<'input, 'arena, Tok = CommonToken<'input>> = Add_preced_opContext<'input, 'arena, Tok>;

pub type Add_preced_opContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Add_preced_opContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Add_preced_opContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub line: u32,
	pub PLUS: Option<&'arena dyn Token >,
	pub SUB: Option<&'arena dyn Token >,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Add_preced_opContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Add_preced_opContext }
	fn get_rule_index(&self) -> usize { RULE_add_preced_op }
    fn make_node(
        arena: &'arena Arena,
        ctx: Add_preced_opContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Add_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Add_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Add_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Add_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Add_preced_opContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_line = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Add_preced_opContextExt {
				PLUS: None, SUB: None, 
				line: _init_line, 
				ph: PhantomData
			}
		)
	}
}

pub trait Add_preced_opContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_line(&self) -> &u32; 
    fn set_line(&mut self,attr: u32); 
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SUB
    /// Returns `None` if there is no child corresponding to token SUB
    fn SUB(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Add_preced_opContextAttrs<'input, 'arena, Tok> for Add_preced_opContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_line(&self) -> &u32 { &self.deref().line }  
    fn set_line(&mut self,attr: u32) { self.deref_mut().line = attr; }  
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PLUS)
    }
    /// Retrieves first TerminalNode corresponding to token SUB
    /// Returns `None` if there is no child corresponding to token SUB
    fn SUB(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_SUB)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn add_preced_op(&mut self,) -> Result<&'arena Add_preced_opContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Add_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 64, RULE_add_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Add_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(350);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_PLUS  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(346);
			        let tmp = recog.base.match_token(CFood_PLUS,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_opContext<TF::Tok>>().unwrap().PLUS = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Add_preced_opContext<TF::Tok>>().unwrap().PLUS { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
			    CFood_SUB  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(348);
			        let tmp = recog.base.match_token(CFood_SUB,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_opContext<TF::Tok>>().unwrap().SUB = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Add_preced_opContext<TF::Tok>>().unwrap().SUB { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Add_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- mul_preced_op ----------------
pub type Mul_preced_opContextAll<'input, 'arena, Tok = CommonToken<'input>> = Mul_preced_opContext<'input, 'arena, Tok>;

pub type Mul_preced_opContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Mul_preced_opContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Mul_preced_opContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub line: u32,
	pub MUL: Option<&'arena dyn Token >,
	pub DIV: Option<&'arena dyn Token >,
	pub MOD: Option<&'arena dyn Token >,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Mul_preced_opContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Mul_preced_opContext }
	fn get_rule_index(&self) -> usize { RULE_mul_preced_op }
    fn make_node(
        arena: &'arena Arena,
        ctx: Mul_preced_opContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Mul_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Mul_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Mul_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Mul_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Mul_preced_opContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_line = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Mul_preced_opContextExt {
				MUL: None, DIV: None, MOD: None, 
				line: _init_line, 
				ph: PhantomData
			}
		)
	}
}

pub trait Mul_preced_opContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_line(&self) -> &u32; 
    fn set_line(&mut self,attr: u32); 
    /// Retrieves first TerminalNode corresponding to token MUL
    /// Returns `None` if there is no child corresponding to token MUL
    fn MUL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token DIV
    /// Returns `None` if there is no child corresponding to token DIV
    fn DIV(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token MOD
    /// Returns `None` if there is no child corresponding to token MOD
    fn MOD(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Mul_preced_opContextAttrs<'input, 'arena, Tok> for Mul_preced_opContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_line(&self) -> &u32 { &self.deref().line }  
    fn set_line(&mut self,attr: u32) { self.deref_mut().line = attr; }  
    /// Retrieves first TerminalNode corresponding to token MUL
    /// Returns `None` if there is no child corresponding to token MUL
    fn MUL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_MUL)
    }
    /// Retrieves first TerminalNode corresponding to token DIV
    /// Returns `None` if there is no child corresponding to token DIV
    fn DIV(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_DIV)
    }
    /// Retrieves first TerminalNode corresponding to token MOD
    /// Returns `None` if there is no child corresponding to token MOD
    fn MOD(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_MOD)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn mul_preced_op(&mut self,) -> Result<&'arena Mul_preced_opContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Mul_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 66, RULE_mul_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Mul_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(358);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_MUL  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(352);
			        let tmp = recog.base.match_token(CFood_MUL,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_opContext<TF::Tok>>().unwrap().MUL = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Mul_preced_opContext<TF::Tok>>().unwrap().MUL { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
			    CFood_DIV  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(354);
			        let tmp = recog.base.match_token(CFood_DIV,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_opContext<TF::Tok>>().unwrap().DIV = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Mul_preced_opContext<TF::Tok>>().unwrap().DIV { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
			    CFood_MOD  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        recog.base.set_state(356);
			        let tmp = recog.base.match_token(CFood_MOD,&mut recog.err_handler)?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_opContext<TF::Tok>>().unwrap().MOD = Some(tmp); } 
			         let tmp = { if let Some(it) = &recog.ctx().unwrap().as_rule_context::<Mul_preced_opContext<TF::Tok>>().unwrap().MOD { it.get_line() } else { 0 } }.to_owned();
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Mul_preced_opContext<TF::Tok>>().unwrap().set_line(tmp); } 
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- apply_list ----------------
pub type Apply_listContextAll<'input, 'arena, Tok = CommonToken<'input>> = Apply_listContext<'input, 'arena, Tok>;

pub type Apply_listContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Apply_listContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Apply_listContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub args: Option<&'arena ArgsContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Apply_listContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Apply_listContext }
	fn get_rule_index(&self) -> usize { RULE_apply_list }
    fn make_node(
        arena: &'arena Arena,
        ctx: Apply_listContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Apply_listContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Apply_listContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Apply_listContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Apply_listContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Apply_listContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, Apply_listContextExt {
				args: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait Apply_listContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn args(&self) -> Option<&'arena ArgsContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Apply_listContextAttrs<'input, 'arena, Tok> for Apply_listContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_L)
    }
    fn args(&self) -> Option<&'arena ArgsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_R)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn apply_list(&mut self,) -> Result<&'arena Apply_listContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Apply_listContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 68, RULE_apply_list)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Apply_listContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(360);
			recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
			/*InvokeRule args*/
			recog.base.set_state(361);
			let tmp = recog.args()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Apply_listContext<TF::Tok>>().unwrap().args = Some(tmp); } 
			recog.base.set_state(362);
			recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;

			        let tmp = { *recog.ctx().unwrap().as_rule_context::<Apply_listContext<TF::Tok>>().unwrap().args.as_ref().unwrap().get_ty_id()}.to_owned();
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Apply_listContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
			    
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- args ----------------
pub type ArgsContextAll<'input, 'arena, Tok = CommonToken<'input>> = ArgsContext<'input, 'arena, Tok>;

pub type ArgsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ArgsContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ArgsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub ty_id: TyId,
	pub expr: Option<&'arena ExprContextAll<'input, 'arena, Tok>>,
	pub COMMA: Option<&'arena dyn Token >,
	pub args: Option<&'arena ArgsContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ArgsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::ArgsContext }
	fn get_rule_index(&self) -> usize { RULE_args }
    fn make_node(
        arena: &'arena Arena,
        ctx: ArgsContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ArgsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => ArgsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ArgsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut ArgsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ArgsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_ty_id = Default::default();

        BaseParserRuleContext::create(arena, parent, invoking_state, ArgsContextExt {
				COMMA: None, 
				expr: None, args: None, 
				ty_id: _init_ty_id, 
				ph: PhantomData
			}
		)
	}
}

pub trait ArgsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_ty_id(&self) -> &TyId; 
    fn set_ty_id(&mut self,attr: TyId); 
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn args(&self) -> Option<&'arena ArgsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ArgsContextAttrs<'input, 'arena, Tok> for ArgsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_ty_id(&self) -> &TyId { &self.deref().ty_id }  
    fn set_ty_id(&mut self,attr: TyId) { self.deref_mut().ty_id = attr; }  
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_COMMA)
    }
    fn args(&self) -> Option<&'arena ArgsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn args(&mut self,) -> Result<&'arena ArgsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ArgsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 70, RULE_args)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ArgsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(374);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(25,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule expr*/
					recog.base.set_state(365);
					let tmp = recog.expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ArgsContext<TF::Tok>>().unwrap().expr = Some(tmp); } 
					recog.base.set_state(366);
					let tmp = recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ArgsContext<TF::Tok>>().unwrap().COMMA = Some(tmp); } 
					/*InvokeRule args*/
					recog.base.set_state(367);
					let tmp = recog.args()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ArgsContext<TF::Tok>>().unwrap().args = Some(tmp); } 

					        let expr = *recog.ctx().unwrap().as_rule_context::<ArgsContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id();
					        let args = *recog.ctx().unwrap().as_rule_context::<ArgsContext<TF::Tok>>().unwrap().args.as_ref().unwrap().get_ty_id();
					        let line = if let Some(it) = &recog.ctx().unwrap().as_rule_context::<ArgsContext<TF::Tok>>().unwrap().COMMA { it.get_line() } else { 0 } ;
					        let tmp = { recog.tlt.apply(expr, args, line)}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ArgsContext<TF::Tok>>().unwrap().set_ty_id(tmp); }
					    
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule expr*/
					recog.base.set_state(370);
					let tmp = recog.expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ArgsContext<TF::Tok>>().unwrap().expr = Some(tmp); } 

					        let tmp = { *recog.ctx().unwrap().as_rule_context::<ArgsContext<TF::Tok>>().unwrap().expr.as_ref().unwrap().get_ty_id()}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ArgsContext<TF::Tok>>().unwrap().set_ty_id(tmp); };
					    
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					 let tmp = { recog.tlt.new_ty(Ty::void())}.to_owned();
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<ArgsContext<TF::Tok>>().unwrap().set_ty_id(tmp); } 
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: LazyLock<Vec<i32>> = LazyLock::new(|| vec![
    4, 1, 37, 377, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 
    7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 
    10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 
    15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 
    20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 
    25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 
    30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 
    35, 7, 35, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 79, 8, 1, 1, 2, 
    1, 2, 1, 2, 3, 2, 84, 8, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 
    1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 3, 4, 102, 8, 
    4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 3, 5, 109, 8, 5, 1, 6, 1, 6, 1, 6, 
    1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 
    7, 1, 7, 1, 7, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 
    1, 8, 1, 8, 1, 8, 3, 8, 140, 8, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 
    9, 1, 9, 1, 9, 3, 9, 150, 8, 9, 1, 10, 1, 10, 1, 10, 1, 11, 1, 11, 1, 
    11, 1, 11, 3, 11, 159, 8, 11, 1, 12, 1, 12, 1, 12, 1, 12, 1, 12, 1, 
    12, 1, 12, 1, 12, 3, 12, 169, 8, 12, 1, 13, 1, 13, 1, 13, 1, 13, 1, 
    13, 1, 13, 1, 13, 1, 13, 3, 13, 179, 8, 13, 1, 14, 1, 14, 1, 14, 1, 
    14, 1, 14, 1, 14, 1, 14, 1, 14, 3, 14, 189, 8, 14, 1, 15, 1, 15, 1, 
    15, 1, 15, 1, 15, 1, 15, 1, 16, 1, 16, 1, 16, 1, 16, 3, 16, 201, 8, 
    16, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 3, 
    17, 212, 8, 17, 1, 18, 1, 18, 3, 18, 216, 8, 18, 1, 19, 1, 19, 1, 19, 
    1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 
    1, 19, 1, 19, 1, 19, 3, 19, 234, 8, 19, 1, 20, 1, 20, 1, 20, 1, 20, 
    1, 20, 1, 20, 1, 20, 1, 21, 1, 21, 1, 21, 1, 21, 1, 21, 1, 21, 1, 21, 
    1, 21, 3, 21, 251, 8, 21, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 
    1, 22, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 3, 23, 266, 8, 23, 
    1, 24, 1, 24, 1, 24, 1, 24, 1, 24, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 
    1, 25, 1, 25, 1, 25, 3, 25, 281, 8, 25, 1, 26, 1, 26, 1, 26, 1, 26, 
    1, 26, 1, 26, 1, 26, 1, 26, 3, 26, 291, 8, 26, 1, 27, 1, 27, 1, 27, 
    1, 27, 1, 27, 1, 27, 1, 27, 3, 27, 300, 8, 27, 1, 28, 1, 28, 1, 28, 
    1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 3, 28, 310, 8, 28, 1, 29, 1, 29, 
    1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 3, 29, 320, 8, 29, 1, 30, 
    1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 3, 30, 331, 
    8, 30, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 
    1, 31, 1, 31, 1, 31, 3, 31, 345, 8, 31, 1, 32, 1, 32, 1, 32, 1, 32, 
    3, 32, 351, 8, 32, 1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 3, 33, 
    359, 8, 33, 1, 34, 1, 34, 1, 34, 1, 34, 1, 34, 1, 35, 1, 35, 1, 35, 
    1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 3, 35, 375, 8, 35, 1, 35, 
    0, 0, 36, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 
    32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 
    66, 68, 70, 0, 0, 382, 0, 72, 1, 0, 0, 0, 2, 78, 1, 0, 0, 0, 4, 83, 
    1, 0, 0, 0, 6, 85, 1, 0, 0, 0, 8, 101, 1, 0, 0, 0, 10, 108, 1, 0, 0, 
    0, 12, 110, 1, 0, 0, 0, 14, 118, 1, 0, 0, 0, 16, 139, 1, 0, 0, 0, 18, 
    149, 1, 0, 0, 0, 20, 151, 1, 0, 0, 0, 22, 158, 1, 0, 0, 0, 24, 168, 
    1, 0, 0, 0, 26, 178, 1, 0, 0, 0, 28, 188, 1, 0, 0, 0, 30, 190, 1, 0, 
    0, 0, 32, 200, 1, 0, 0, 0, 34, 211, 1, 0, 0, 0, 36, 215, 1, 0, 0, 0, 
    38, 233, 1, 0, 0, 0, 40, 235, 1, 0, 0, 0, 42, 250, 1, 0, 0, 0, 44, 252, 
    1, 0, 0, 0, 46, 265, 1, 0, 0, 0, 48, 267, 1, 0, 0, 0, 50, 280, 1, 0, 
    0, 0, 52, 290, 1, 0, 0, 0, 54, 299, 1, 0, 0, 0, 56, 309, 1, 0, 0, 0, 
    58, 319, 1, 0, 0, 0, 60, 330, 1, 0, 0, 0, 62, 344, 1, 0, 0, 0, 64, 350, 
    1, 0, 0, 0, 66, 358, 1, 0, 0, 0, 68, 360, 1, 0, 0, 0, 70, 374, 1, 0, 
    0, 0, 72, 73, 3, 2, 1, 0, 73, 1, 1, 0, 0, 0, 74, 75, 3, 4, 2, 0, 75, 
    76, 3, 2, 1, 0, 76, 79, 1, 0, 0, 0, 77, 79, 1, 0, 0, 0, 78, 74, 1, 0, 
    0, 0, 78, 77, 1, 0, 0, 0, 79, 3, 1, 0, 0, 0, 80, 84, 3, 6, 3, 0, 81, 
    84, 3, 14, 7, 0, 82, 84, 3, 12, 6, 0, 83, 80, 1, 0, 0, 0, 83, 81, 1, 
    0, 0, 0, 83, 82, 1, 0, 0, 0, 84, 5, 1, 0, 0, 0, 85, 86, 3, 8, 4, 0, 
    86, 87, 6, 3, -1, 0, 87, 88, 3, 10, 5, 0, 88, 89, 5, 30, 0, 0, 89, 7, 
    1, 0, 0, 0, 90, 91, 3, 26, 13, 0, 91, 92, 5, 32, 0, 0, 92, 93, 6, 4, 
    -1, 0, 93, 102, 1, 0, 0, 0, 94, 95, 3, 28, 14, 0, 95, 96, 5, 32, 0, 
    0, 96, 97, 5, 15, 0, 0, 97, 98, 3, 22, 11, 0, 98, 99, 5, 16, 0, 0, 99, 
    100, 6, 4, -1, 0, 100, 102, 1, 0, 0, 0, 101, 90, 1, 0, 0, 0, 101, 94, 
    1, 0, 0, 0, 102, 9, 1, 0, 0, 0, 103, 104, 5, 28, 0, 0, 104, 105, 3, 
    46, 23, 0, 105, 106, 6, 5, -1, 0, 106, 109, 1, 0, 0, 0, 107, 109, 1, 
    0, 0, 0, 108, 103, 1, 0, 0, 0, 108, 107, 1, 0, 0, 0, 109, 11, 1, 0, 
    0, 0, 110, 111, 3, 26, 13, 0, 111, 112, 5, 32, 0, 0, 112, 113, 6, 6, 
    -1, 0, 113, 114, 3, 16, 8, 0, 114, 115, 6, 6, -1, 0, 115, 116, 3, 30, 
    15, 0, 116, 117, 6, 6, -1, 0, 117, 13, 1, 0, 0, 0, 118, 119, 5, 5, 0, 
    0, 119, 120, 5, 31, 0, 0, 120, 121, 5, 28, 0, 0, 121, 122, 5, 11, 0, 
    0, 122, 123, 3, 24, 12, 0, 123, 124, 5, 12, 0, 0, 124, 125, 5, 30, 0, 
    0, 125, 126, 6, 7, -1, 0, 126, 15, 1, 0, 0, 0, 127, 128, 5, 11, 0, 0, 
    128, 129, 3, 18, 9, 0, 129, 130, 5, 12, 0, 0, 130, 131, 6, 8, -1, 0, 
    131, 140, 1, 0, 0, 0, 132, 133, 5, 11, 0, 0, 133, 134, 5, 9, 0, 0, 134, 
    135, 5, 12, 0, 0, 135, 140, 6, 8, -1, 0, 136, 137, 5, 11, 0, 0, 137, 
    138, 5, 12, 0, 0, 138, 140, 6, 8, -1, 0, 139, 127, 1, 0, 0, 0, 139, 
    132, 1, 0, 0, 0, 139, 136, 1, 0, 0, 0, 140, 17, 1, 0, 0, 0, 141, 142, 
    3, 20, 10, 0, 142, 143, 5, 29, 0, 0, 143, 144, 3, 18, 9, 0, 144, 145, 
    6, 9, -1, 0, 145, 150, 1, 0, 0, 0, 146, 147, 3, 20, 10, 0, 147, 148, 
    6, 9, -1, 0, 148, 150, 1, 0, 0, 0, 149, 141, 1, 0, 0, 0, 149, 146, 1, 
    0, 0, 0, 150, 19, 1, 0, 0, 0, 151, 152, 3, 8, 4, 0, 152, 153, 6, 10, 
    -1, 0, 153, 21, 1, 0, 0, 0, 154, 155, 5, 33, 0, 0, 155, 159, 6, 11, 
    -1, 0, 156, 157, 5, 34, 0, 0, 157, 159, 6, 11, -1, 0, 158, 154, 1, 0, 
    0, 0, 158, 156, 1, 0, 0, 0, 159, 23, 1, 0, 0, 0, 160, 161, 3, 28, 14, 
    0, 161, 162, 5, 29, 0, 0, 162, 163, 3, 24, 12, 0, 163, 164, 6, 12, -1, 
    0, 164, 169, 1, 0, 0, 0, 165, 166, 3, 28, 14, 0, 166, 167, 6, 12, -1, 
    0, 167, 169, 1, 0, 0, 0, 168, 160, 1, 0, 0, 0, 168, 165, 1, 0, 0, 0, 
    169, 25, 1, 0, 0, 0, 170, 171, 3, 28, 14, 0, 171, 172, 6, 13, -1, 0, 
    172, 179, 1, 0, 0, 0, 173, 174, 3, 28, 14, 0, 174, 175, 5, 10, 0, 0, 
    175, 176, 3, 26, 13, 0, 176, 177, 6, 13, -1, 0, 177, 179, 1, 0, 0, 0, 
    178, 170, 1, 0, 0, 0, 178, 173, 1, 0, 0, 0, 179, 27, 1, 0, 0, 0, 180, 
    181, 5, 7, 0, 0, 181, 189, 6, 14, -1, 0, 182, 183, 5, 8, 0, 0, 183, 
    189, 6, 14, -1, 0, 184, 185, 5, 9, 0, 0, 185, 189, 6, 14, -1, 0, 186, 
    187, 5, 31, 0, 0, 187, 189, 6, 14, -1, 0, 188, 180, 1, 0, 0, 0, 188, 
    182, 1, 0, 0, 0, 188, 184, 1, 0, 0, 0, 188, 186, 1, 0, 0, 0, 189, 29, 
    1, 0, 0, 0, 190, 191, 5, 13, 0, 0, 191, 192, 6, 15, -1, 0, 192, 193, 
    3, 32, 16, 0, 193, 194, 6, 15, -1, 0, 194, 195, 5, 14, 0, 0, 195, 31, 
    1, 0, 0, 0, 196, 197, 3, 34, 17, 0, 197, 198, 3, 32, 16, 0, 198, 201, 
    1, 0, 0, 0, 199, 201, 1, 0, 0, 0, 200, 196, 1, 0, 0, 0, 200, 199, 1, 
    0, 0, 0, 201, 33, 1, 0, 0, 0, 202, 212, 3, 6, 3, 0, 203, 212, 3, 44, 
    22, 0, 204, 205, 3, 46, 23, 0, 205, 206, 5, 30, 0, 0, 206, 212, 1, 0, 
    0, 0, 207, 212, 3, 38, 19, 0, 208, 212, 3, 40, 20, 0, 209, 212, 3, 42, 
    21, 0, 210, 212, 3, 30, 15, 0, 211, 202, 1, 0, 0, 0, 211, 203, 1, 0, 
    0, 0, 211, 204, 1, 0, 0, 0, 211, 207, 1, 0, 0, 0, 211, 208, 1, 0, 0, 
    0, 211, 209, 1, 0, 0, 0, 211, 210, 1, 0, 0, 0, 212, 35, 1, 0, 0, 0, 
    213, 216, 3, 34, 17, 0, 214, 216, 5, 30, 0, 0, 215, 213, 1, 0, 0, 0, 
    215, 214, 1, 0, 0, 0, 216, 37, 1, 0, 0, 0, 217, 218, 5, 2, 0, 0, 218, 
    219, 5, 11, 0, 0, 219, 220, 3, 46, 23, 0, 220, 221, 5, 12, 0, 0, 221, 
    222, 3, 36, 18, 0, 222, 223, 6, 19, -1, 0, 223, 234, 1, 0, 0, 0, 224, 
    225, 5, 2, 0, 0, 225, 226, 5, 11, 0, 0, 226, 227, 3, 46, 23, 0, 227, 
    228, 5, 12, 0, 0, 228, 229, 3, 36, 18, 0, 229, 230, 5, 3, 0, 0, 230, 
    231, 3, 36, 18, 0, 231, 232, 6, 19, -1, 0, 232, 234, 1, 0, 0, 0, 233, 
    217, 1, 0, 0, 0, 233, 224, 1, 0, 0, 0, 234, 39, 1, 0, 0, 0, 235, 236, 
    5, 1, 0, 0, 236, 237, 5, 11, 0, 0, 237, 238, 3, 46, 23, 0, 238, 239, 
    5, 12, 0, 0, 239, 240, 3, 36, 18, 0, 240, 241, 6, 20, -1, 0, 241, 41, 
    1, 0, 0, 0, 242, 243, 5, 4, 0, 0, 243, 244, 5, 30, 0, 0, 244, 251, 6, 
    21, -1, 0, 245, 246, 5, 4, 0, 0, 246, 247, 3, 46, 23, 0, 247, 248, 5, 
    30, 0, 0, 248, 249, 6, 21, -1, 0, 249, 251, 1, 0, 0, 0, 250, 242, 1, 
    0, 0, 0, 250, 245, 1, 0, 0, 0, 251, 43, 1, 0, 0, 0, 252, 253, 5, 6, 
    0, 0, 253, 254, 5, 32, 0, 0, 254, 255, 5, 28, 0, 0, 255, 256, 3, 46, 
    23, 0, 256, 257, 5, 30, 0, 0, 257, 258, 6, 22, -1, 0, 258, 45, 1, 0, 
    0, 0, 259, 260, 3, 48, 24, 0, 260, 261, 6, 23, -1, 0, 261, 266, 1, 0, 
    0, 0, 262, 263, 3, 52, 26, 0, 263, 264, 6, 23, -1, 0, 264, 266, 1, 0, 
    0, 0, 265, 259, 1, 0, 0, 0, 265, 262, 1, 0, 0, 0, 266, 47, 1, 0, 0, 
    0, 267, 268, 3, 50, 25, 0, 268, 269, 5, 28, 0, 0, 269, 270, 3, 46, 23, 
    0, 270, 271, 6, 24, -1, 0, 271, 49, 1, 0, 0, 0, 272, 273, 5, 32, 0, 
    0, 273, 281, 6, 25, -1, 0, 274, 275, 5, 32, 0, 0, 275, 276, 5, 15, 0, 
    0, 276, 277, 3, 46, 23, 0, 277, 278, 5, 16, 0, 0, 278, 279, 6, 25, -1, 
    0, 279, 281, 1, 0, 0, 0, 280, 272, 1, 0, 0, 0, 280, 274, 1, 0, 0, 0, 
    281, 51, 1, 0, 0, 0, 282, 283, 3, 54, 27, 0, 283, 284, 3, 62, 31, 0, 
    284, 285, 3, 54, 27, 0, 285, 286, 6, 26, -1, 0, 286, 291, 1, 0, 0, 0, 
    287, 288, 3, 54, 27, 0, 288, 289, 6, 26, -1, 0, 289, 291, 1, 0, 0, 0, 
    290, 282, 1, 0, 0, 0, 290, 287, 1, 0, 0, 0, 291, 53, 1, 0, 0, 0, 292, 
    293, 3, 56, 28, 0, 293, 294, 3, 54, 27, 0, 294, 295, 6, 27, -1, 0, 295, 
    300, 1, 0, 0, 0, 296, 297, 3, 56, 28, 0, 297, 298, 6, 27, -1, 0, 298, 
    300, 1, 0, 0, 0, 299, 292, 1, 0, 0, 0, 299, 296, 1, 0, 0, 0, 300, 55, 
    1, 0, 0, 0, 301, 302, 3, 58, 29, 0, 302, 303, 3, 64, 32, 0, 303, 304, 
    3, 56, 28, 0, 304, 305, 6, 28, -1, 0, 305, 310, 1, 0, 0, 0, 306, 307, 
    3, 58, 29, 0, 307, 308, 6, 28, -1, 0, 308, 310, 1, 0, 0, 0, 309, 301, 
    1, 0, 0, 0, 309, 306, 1, 0, 0, 0, 310, 57, 1, 0, 0, 0, 311, 312, 3, 
    60, 30, 0, 312, 313, 3, 66, 33, 0, 313, 314, 3, 58, 29, 0, 314, 315, 
    6, 29, -1, 0, 315, 320, 1, 0, 0, 0, 316, 317, 3, 60, 30, 0, 317, 318, 
    6, 29, -1, 0, 318, 320, 1, 0, 0, 0, 319, 311, 1, 0, 0, 0, 319, 316, 
    1, 0, 0, 0, 320, 59, 1, 0, 0, 0, 321, 322, 3, 68, 34, 0, 322, 323, 6, 
    30, -1, 0, 323, 331, 1, 0, 0, 0, 324, 325, 3, 50, 25, 0, 325, 326, 6, 
    30, -1, 0, 326, 331, 1, 0, 0, 0, 327, 328, 3, 22, 11, 0, 328, 329, 6, 
    30, -1, 0, 329, 331, 1, 0, 0, 0, 330, 321, 1, 0, 0, 0, 330, 324, 1, 
    0, 0, 0, 330, 327, 1, 0, 0, 0, 331, 61, 1, 0, 0, 0, 332, 333, 5, 17, 
    0, 0, 333, 345, 6, 31, -1, 0, 334, 335, 5, 18, 0, 0, 335, 345, 6, 31, 
    -1, 0, 336, 337, 5, 19, 0, 0, 337, 345, 6, 31, -1, 0, 338, 339, 5, 20, 
    0, 0, 339, 345, 6, 31, -1, 0, 340, 341, 5, 21, 0, 0, 341, 345, 6, 31, 
    -1, 0, 342, 343, 5, 22, 0, 0, 343, 345, 6, 31, -1, 0, 344, 332, 1, 0, 
    0, 0, 344, 334, 1, 0, 0, 0, 344, 336, 1, 0, 0, 0, 344, 338, 1, 0, 0, 
    0, 344, 340, 1, 0, 0, 0, 344, 342, 1, 0, 0, 0, 345, 63, 1, 0, 0, 0, 
    346, 347, 5, 23, 0, 0, 347, 351, 6, 32, -1, 0, 348, 349, 5, 24, 0, 0, 
    349, 351, 6, 32, -1, 0, 350, 346, 1, 0, 0, 0, 350, 348, 1, 0, 0, 0, 
    351, 65, 1, 0, 0, 0, 352, 353, 5, 26, 0, 0, 353, 359, 6, 33, -1, 0, 
    354, 355, 5, 27, 0, 0, 355, 359, 6, 33, -1, 0, 356, 357, 5, 25, 0, 0, 
    357, 359, 6, 33, -1, 0, 358, 352, 1, 0, 0, 0, 358, 354, 1, 0, 0, 0, 
    358, 356, 1, 0, 0, 0, 359, 67, 1, 0, 0, 0, 360, 361, 5, 11, 0, 0, 361, 
    362, 3, 70, 35, 0, 362, 363, 5, 12, 0, 0, 363, 364, 6, 34, -1, 0, 364, 
    69, 1, 0, 0, 0, 365, 366, 3, 46, 23, 0, 366, 367, 5, 29, 0, 0, 367, 
    368, 3, 70, 35, 0, 368, 369, 6, 35, -1, 0, 369, 375, 1, 0, 0, 0, 370, 
    371, 3, 46, 23, 0, 371, 372, 6, 35, -1, 0, 372, 375, 1, 0, 0, 0, 373, 
    375, 6, 35, -1, 0, 374, 365, 1, 0, 0, 0, 374, 370, 1, 0, 0, 0, 374, 
    373, 1, 0, 0, 0, 375, 71, 1, 0, 0, 0, 26, 78, 83, 101, 108, 139, 149, 
    158, 168, 178, 188, 200, 211, 215, 233, 250, 265, 280, 290, 299, 309, 
    319, 330, 344, 350, 358, 374
]);