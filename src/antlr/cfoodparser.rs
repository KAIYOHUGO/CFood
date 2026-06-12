// Generated from ./CFood.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]
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
use super::cfoodvisitor::*;

use std::marker::PhantomData;
use std::sync::LazyLock;
use std::rc::Rc;
use std::ops::{DerefMut, Deref};

dbt_antlr4::check_version!("1","3");
pub const CFood_KW_while:i32=1; 
pub const CFood_KW_for:i32=2; 
pub const CFood_KW_if:i32=3; 
pub const CFood_KW_else:i32=4; 
pub const CFood_KW_return:i32=5; 
pub const CFood_KW_type:i32=6; 
pub const CFood_KW_let:i32=7; 
pub const CFood_KW_as:i32=8; 
pub const CFood_LIT_true:i32=9; 
pub const CFood_LIT_false:i32=10; 
pub const CFood_TY_int:i32=11; 
pub const CFood_TY_float:i32=12; 
pub const CFood_TY_str:i32=13; 
pub const CFood_TY_void:i32=14; 
pub const CFood_TY_bool:i32=15; 
pub const CFood_MAGIC_printf:i32=16; 
pub const CFood_MAGIC_scanf:i32=17; 
pub const CFood_MAGIC_new:i32=18; 
pub const CFood_PAREN_L:i32=19; 
pub const CFood_PAREN_R:i32=20; 
pub const CFood_BRACE_L:i32=21; 
pub const CFood_BRACE_R:i32=22; 
pub const CFood_NE:i32=23; 
pub const CFood_EQ:i32=24; 
pub const CFood_LT:i32=25; 
pub const CFood_GT:i32=26; 
pub const CFood_LE:i32=27; 
pub const CFood_GE:i32=28; 
pub const CFood_NOT:i32=29; 
pub const CFood_AND:i32=30; 
pub const CFood_OR:i32=31; 
pub const CFood_PLUS:i32=32; 
pub const CFood_SUB:i32=33; 
pub const CFood_MOD:i32=34; 
pub const CFood_MUL:i32=35; 
pub const CFood_DIV:i32=36; 
pub const CFood_PEO:i32=37; 
pub const CFood_ASSIGN:i32=38; 
pub const CFood_COMMA:i32=39; 
pub const CFood_REFER:i32=40; 
pub const CFood_SEMICOLON:i32=41; 
pub const CFood_TYPE:i32=42; 
pub const CFood_IDENT:i32=43; 
pub const CFood_INT:i32=44; 
pub const CFood_FLOAT:i32=45; 
pub const CFood_CONSTR:i32=46; 
pub const CFood_LINE_COMMENT:i32=47; 
pub const CFood_COMMENT:i32=48; 
pub const CFood_WS:i32=49;
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
pub const RULE_lit:usize = 11; 
pub const RULE_tys:usize = 12; 
pub const RULE_ty_kind:usize = 13; 
pub const RULE_block:usize = 14; 
pub const RULE_stmts:usize = 15; 
pub const RULE_stmt:usize = 16; 
pub const RULE_expr_stmt:usize = 17; 
pub const RULE_branch_stmt:usize = 18; 
pub const RULE_iter_stmt:usize = 19; 
pub const RULE_inline_stmts:usize = 20; 
pub const RULE_inline_stmt:usize = 21; 
pub const RULE_for_stmt:usize = 22; 
pub const RULE_return_stmt:usize = 23; 
pub const RULE_let_stmt:usize = 24; 
pub const RULE_var:usize = 25; 
pub const RULE_refer:usize = 26; 
pub const RULE_expr:usize = 27; 
pub const RULE_expr_assign:usize = 28; 
pub const RULE_expr_logic:usize = 29; 
pub const RULE_expr_cmp:usize = 30; 
pub const RULE_expr_add:usize = 31; 
pub const RULE_expr_mul:usize = 32; 
pub const RULE_expr_cast:usize = 33; 
pub const RULE_expr_unary:usize = 34; 
pub const RULE_expr_magic:usize = 35; 
pub const RULE_expr_call:usize = 36; 
pub const RULE_atom:usize = 37; 
pub const RULE_magic:usize = 38; 
pub const RULE_logic_preced_op:usize = 39; 
pub const RULE_cmp_preced_op:usize = 40; 
pub const RULE_add_preced_op:usize = 41; 
pub const RULE_mul_preced_op:usize = 42; 
pub const RULE_unary_preced_op:usize = 43; 
pub const RULE_apply_list:usize = 44; 
pub const RULE_args:usize = 45;
pub const ruleNames: [&'static str; 46] = [
    "file", "decls", "decl", "var_decl", "var_decl_ty", "var_decl_init", 
    "fn_decl", "ty_decl", "params", "param_list", "param", "lit", "tys", 
    "ty_kind", "block", "stmts", "stmt", "expr_stmt", "branch_stmt", "iter_stmt", 
    "inline_stmts", "inline_stmt", "for_stmt", "return_stmt", "let_stmt", 
    "var", "refer", "expr", "expr_assign", "expr_logic", "expr_cmp", "expr_add", 
    "expr_mul", "expr_cast", "expr_unary", "expr_magic", "expr_call", "atom", 
    "magic", "logic_preced_op", "cmp_preced_op", "add_preced_op", "mul_preced_op", 
    "unary_preced_op", "apply_list", "args"
];

pub const _LITERAL_NAMES: [Option<&'static str>;42] = [
	None, Some("'while'"), Some("'for'"), Some("'if'"), Some("'else'"), Some("'return'"), 
	Some("'type'"), Some("'let'"), Some("'as'"), Some("'true'"), Some("'false'"), 
	Some("'int'"), Some("'float'"), Some("'str'"), Some("'void'"), Some("'bool'"), 
	Some("'printf'"), Some("'scanf'"), Some("'new'"), Some("'('"), Some("')'"), 
	Some("'{'"), Some("'}'"), Some("'!='"), Some("'=='"), Some("'<'"), Some("'>'"), 
	Some("'<='"), Some("'>='"), Some("'!'"), Some("'&&'"), Some("'||'"), Some("'+'"), 
	Some("'-'"), Some("'%'"), Some("'*'"), Some("'/'"), Some("'##'"), Some("'='"), 
	Some("','"), Some("'&'"), Some("';'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;50]  = [
	None, Some("KW_while"), Some("KW_for"), Some("KW_if"), Some("KW_else"), 
	Some("KW_return"), Some("KW_type"), Some("KW_let"), Some("KW_as"), Some("LIT_true"), 
	Some("LIT_false"), Some("TY_int"), Some("TY_float"), Some("TY_str"), Some("TY_void"), 
	Some("TY_bool"), Some("MAGIC_printf"), Some("MAGIC_scanf"), Some("MAGIC_new"), 
	Some("PAREN_L"), Some("PAREN_R"), Some("BRACE_L"), Some("BRACE_R"), Some("NE"), 
	Some("EQ"), Some("LT"), Some("GT"), Some("LE"), Some("GE"), Some("NOT"), 
	Some("AND"), Some("OR"), Some("PLUS"), Some("SUB"), Some("MOD"), Some("MUL"), 
	Some("DIV"), Some("PEO"), Some("ASSIGN"), Some("COMMA"), Some("REFER"), 
	Some("SEMICOLON"), Some("TYPE"), Some("IDENT"), Some("INT"), Some("FLOAT"), 
	Some("CONSTR"), Some("LINE_COMMENT"), Some("COMMENT"), Some("WS")
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
pub trait Visitable<'input: 'arena, 'arena, Tok: Token + 'input> {
    fn accept<V>(&'arena self, visitor: &mut V) -> Result<V::Return, ANTLRError>
    where
        'input: 'arena,
        V: CFoodVisitor<'input, 'arena, Tok> + ?Sized;
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
    LitContext,
    TysContext,
    Ty_kindContext,
    BlockContext,
    StmtsContext,
    StmtContext,
    Expr_stmtContext,
    Branch_stmtContext,
    Iter_stmtContext,
    Inline_stmtsContext,
    Inline_stmtContext,
    For_stmtContext,
    Return_stmtContext,
    Let_stmtContext,
    VarContext,
    ReferContext,
    ExprContext,
    Expr_assignContext,
    Expr_logicContext,
    Expr_cmpContext,
    Expr_addContext,
    Expr_mulContext,
    Expr_castContext,
    Expr_unaryContext,
    Expr_magicContext,
    Expr_callContext,
    AtomContext,
    MagicContext,
    Logic_preced_opContext,
    Cmp_preced_opContext,
    Add_preced_opContext,
    Mul_preced_opContext,
    Unary_preced_opContext,
    Apply_listContext,
    ArgsContext,
    Terminal,
    Error,
}
pub type CFoodParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, CFoodParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => CFoodParser }
dbt_antlr4::impl_node_kind! { CFoodParserNodeKind {
    LitContext(LitContextAll), Ty_kindContext(Ty_kindContextAll), Expr_assignContext(Expr_assignContextAll), Expr_logicContext(Expr_logicContextAll), Expr_cmpContext(Expr_cmpContextAll), Expr_addContext(Expr_addContextAll), Expr_mulContext(Expr_mulContextAll), Expr_castContext(Expr_castContextAll), Expr_unaryContext(Expr_unaryContextAll), Expr_magicContext(Expr_magicContextAll), Expr_callContext(Expr_callContextAll), AtomContext(AtomContextAll), ; FileContext(enter_file, exit_file,  visit_file), DeclsContext(enter_decls, exit_decls,  visit_decls), DeclContext(enter_decl, exit_decl,  visit_decl), Var_declContext(enter_var_decl, exit_var_decl,  visit_var_decl), Var_decl_tyContext(enter_var_decl_ty, exit_var_decl_ty,  visit_var_decl_ty), Var_decl_initContext(enter_var_decl_init, exit_var_decl_init,  visit_var_decl_init), Fn_declContext(enter_fn_decl, exit_fn_decl,  visit_fn_decl), Ty_declContext(enter_ty_decl, exit_ty_decl,  visit_ty_decl), ParamsContext(enter_params, exit_params,  visit_params), Param_listContext(enter_param_list, exit_param_list,  visit_param_list), ParamContext(enter_param, exit_param,  visit_param), TysContext(enter_tys, exit_tys,  visit_tys), BlockContext(enter_block, exit_block,  visit_block), StmtsContext(enter_stmts, exit_stmts,  visit_stmts), StmtContext(enter_stmt, exit_stmt,  visit_stmt), Expr_stmtContext(enter_expr_stmt, exit_expr_stmt,  visit_expr_stmt), Branch_stmtContext(enter_branch_stmt, exit_branch_stmt,  visit_branch_stmt), Iter_stmtContext(enter_iter_stmt, exit_iter_stmt,  visit_iter_stmt), Inline_stmtsContext(enter_inline_stmts, exit_inline_stmts,  visit_inline_stmts), Inline_stmtContext(enter_inline_stmt, exit_inline_stmt,  visit_inline_stmt), For_stmtContext(enter_for_stmt, exit_for_stmt,  visit_for_stmt), Return_stmtContext(enter_return_stmt, exit_return_stmt,  visit_return_stmt), Let_stmtContext(enter_let_stmt, exit_let_stmt,  visit_let_stmt), VarContext(enter_var, exit_var,  visit_var), ReferContext(enter_refer, exit_refer,  visit_refer), ExprContext(enter_expr, exit_expr,  visit_expr), MagicContext(enter_magic, exit_magic,  visit_magic), Logic_preced_opContext(enter_logic_preced_op, exit_logic_preced_op,  visit_logic_preced_op), Cmp_preced_opContext(enter_cmp_preced_op, exit_cmp_preced_op,  visit_cmp_preced_op), Add_preced_opContext(enter_add_preced_op, exit_add_preced_op,  visit_add_preced_op), Mul_preced_opContext(enter_mul_preced_op, exit_mul_preced_op,  visit_mul_preced_op), Unary_preced_opContext(enter_unary_preced_op, exit_unary_preced_op,  visit_unary_preced_op), Apply_listContext(enter_apply_list, exit_apply_list,  visit_apply_list), ArgsContext(enter_args, exit_args,  visit_args), 
    }; listener = dyn CFoodListener<'arena, Tok>, visitor = CFoodVisitor,
}

pub struct CFoodParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
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
dbt_antlr4::impl_visitable! { CFoodVisitor::FileContext(visit_file) }
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
			recog.base.set_state(92);
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
dbt_antlr4::impl_visitable! { CFoodVisitor::DeclsContext(visit_decls) }
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
			recog.base.set_state(98);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_KW_type |CFood_TY_int |CFood_TY_float |CFood_TY_str |CFood_TY_void |
			    CFood_TY_bool |CFood_TYPE  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule decl*/
			        recog.base.set_state(94);
			        recog.decl()?;
			        /*InvokeRule decls*/
			        recog.base.set_state(95);
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
dbt_antlr4::impl_visitable! { CFoodVisitor::DeclContext(visit_decl) }
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
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
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
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_SEMICOLON)
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
			recog.base.set_state(105);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(1,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule var_decl*/
					recog.base.set_state(100);
					recog.var_decl()?;
					recog.base.set_state(101);
					recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule ty_decl*/
					recog.base.set_state(103);
					recog.ty_decl()?;
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					/*InvokeRule fn_decl*/
					recog.base.set_state(104);
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Var_declContext(visit_var_decl) }
#[derive(Debug)]
pub struct Var_declContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Var_declContextExt {
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
    fn var_decl_ty(&self) -> Option<&'arena Var_decl_tyContextAll<'input, 'arena, Tok>>;
    fn var_decl_init(&self) -> Option<&'arena Var_decl_initContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Var_declContextAttrs<'input, 'arena, Tok> for Var_declContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn var_decl_ty(&self) -> Option<&'arena Var_decl_tyContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn var_decl_init(&self) -> Option<&'arena Var_decl_initContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
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
			recog.base.set_state(107);
			recog.var_decl_ty()?;
			/*InvokeRule var_decl_init*/
			recog.base.set_state(108);
			recog.var_decl_init()?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Var_decl_tyContext(visit_var_decl_ty) }
#[derive(Debug)]
pub struct Var_decl_tyContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Var_decl_tyContextExt {
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
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Var_decl_tyContextAttrs<'input, 'arena, Tok> for Var_decl_tyContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_IDENT)
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
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule ty_kind*/
			recog.base.set_state(110);
			recog.ty_kind()?;
			recog.base.set_state(111);
			recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Var_decl_initContext(visit_var_decl_init) }
#[derive(Debug)]
pub struct Var_decl_initContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Var_decl_initContextExt {
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
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Var_decl_initContextAttrs<'input, 'arena, Tok> for Var_decl_initContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
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
	pub fn var_decl_init(&mut self,) -> Result<&'arena Var_decl_initContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Var_decl_initContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 10, RULE_var_decl_init)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Var_decl_initContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(116);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_ASSIGN  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(113);
			        recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			        /*InvokeRule expr*/
			        recog.base.set_state(114);
			        recog.expr()?;
			        }}
			    CFood_PAREN_R |CFood_COMMA |CFood_SEMICOLON  => {
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Fn_declContext(visit_fn_decl) }
#[derive(Debug)]
pub struct Fn_declContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Fn_declContextExt {
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
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>;
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
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>> {
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
			/*InvokeRule ty_kind*/
			recog.base.set_state(118);
			recog.ty_kind()?;
			recog.base.set_state(119);
			recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
			/*InvokeRule params*/
			recog.base.set_state(120);
			recog.params()?;
			/*InvokeRule block*/
			recog.base.set_state(121);
			recog.block()?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Ty_declContext(visit_ty_decl) }
#[derive(Debug)]
pub struct Ty_declContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
			recog.base.set_state(123);
			recog.base.match_token(CFood_KW_type,&mut recog.err_handler)?;
			recog.base.set_state(124);
			recog.base.match_token(CFood_TYPE,&mut recog.err_handler)?;
			recog.base.set_state(125);
			recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			recog.base.set_state(126);
			recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
			/*InvokeRule tys*/
			recog.base.set_state(127);
			recog.tys()?;
			recog.base.set_state(128);
			recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
			recog.base.set_state(129);
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
//------------------- params ----------------
pub type ParamsContextAll<'input, 'arena, Tok = CommonToken<'input>> = ParamsContext<'input, 'arena, Tok>;

pub type ParamsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ParamsContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::ParamsContext(visit_params) }
#[derive(Debug)]
pub struct ParamsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, ParamsContextExt {
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
			recog.base.set_state(140);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(3,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(131);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule param_list*/
					recog.base.set_state(132);
					recog.param_list()?;
					recog.base.set_state(133);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(135);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					recog.base.set_state(136);
					recog.base.match_token(CFood_TY_void,&mut recog.err_handler)?;
					recog.base.set_state(137);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					recog.base.set_state(138);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					recog.base.set_state(139);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Param_listContext(visit_param_list) }
#[derive(Debug)]
pub struct Param_listContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Param_listContextExt {
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
			recog.base.set_state(147);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(4,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule param*/
					recog.base.set_state(142);
					recog.param()?;
					recog.base.set_state(143);
					recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					/*InvokeRule param_list*/
					recog.base.set_state(144);
					recog.param_list()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule param*/
					recog.base.set_state(146);
					recog.param()?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::ParamContext(visit_param) }
#[derive(Debug)]
pub struct ParamContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, ParamContextExt {
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
    fn var_decl_ty(&self) -> Option<&'arena Var_decl_tyContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ParamContextAttrs<'input, 'arena, Tok> for ParamContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
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
			recog.base.set_state(149);
			recog.var_decl_ty()?;
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
//------------------- lit ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum LitContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Lit_constrContext(Lit_constrContext<'input, 'arena, Tok>),
	Lit_trueContext(Lit_trueContext<'input, 'arena, Tok>),
	Lit_intContext(Lit_intContext<'input, 'arena, Tok>),
	Lit_floatContext(Lit_floatContext<'input, 'arena, Tok>),
	Lit_falseContext(Lit_falseContext<'input, 'arena, Tok>),
    Error(LitContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { LitContextAll { } { Lit_constrContext, Lit_trueContext, Lit_intContext, Lit_floatContext, Lit_falseContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { LitContextAll { } { Lit_constrContext, Lit_trueContext, Lit_intContext, Lit_floatContext, Lit_falseContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::LitContextAll { Lit_constrContext, Lit_trueContext, Lit_intContext, Lit_floatContext, Lit_falseContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::LitContext::LitContextAll { Lit_constrContext, Lit_trueContext, Lit_intContext, Lit_floatContext, Lit_falseContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::LitContextAll { Lit_constrContext(enter_lit_constr, exit_lit_constr), Lit_trueContext(enter_lit_true, exit_lit_true), Lit_intContext(enter_lit_int, exit_lit_int), Lit_floatContext(enter_lit_float, exit_lit_float), Lit_falseContext(enter_lit_false, exit_lit_false), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::LitContextAll { Lit_constrContext(visit_lit_constr), Lit_trueContext(visit_lit_true), Lit_intContext(visit_lit_int), Lit_floatContext(visit_lit_float), Lit_falseContext(visit_lit_false), } }

impl<'input, 'arena, Tok> Deref for LitContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn LitContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use LitContextAll::*;
		match self{
			Lit_constrContext(inner) => inner,
			Lit_trueContext(inner) => inner,
			Lit_intContext(inner) => inner,
			Lit_floatContext(inner) => inner,
			Lit_falseContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type LitContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LitContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LitContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LitContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::LitContext }
	fn get_rule_index(&self) -> usize { RULE_lit }
    fn make_node(
        arena: &'arena Arena,
        ctx: LitContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(LitContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LitContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => LitContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LitContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut LitContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LitContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LitContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LitContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> LitContextAttrs<'input, 'arena, Tok> for LitContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Lit_constrContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Lit_constrContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Lit_constrContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token CONSTR
	/// Returns `None` if there is no child corresponding to token CONSTR
	fn CONSTR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_constrContextAttrs<'input, 'arena, Tok> for Lit_constrContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token CONSTR
    /// Returns `None` if there is no child corresponding to token CONSTR
    fn CONSTR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_CONSTR)
    }
}
#[derive(Debug)]
pub struct Lit_constrContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: LitContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Lit_constrContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::LitContext }
	fn get_rule_index(&self) -> usize { RULE_lit }
    fn make_node(
        arena: &'arena Arena,
        ctx: Lit_constrContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(LitContextAll::Lit_constrContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Lit_constrContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_constrContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Lit_constrContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_constrContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> LitContextAttrs<'input, 'arena, Tok> for Lit_constrContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_constrContextExt<'input, 'arena, Tok> {
	fn new(base: LitContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut LitContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            LitContextAll::Lit_constrContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut LitContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Lit_trueContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Lit_trueContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Lit_trueContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token LIT_true
	/// Returns `None` if there is no child corresponding to token LIT_true
	fn LIT_true(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_trueContextAttrs<'input, 'arena, Tok> for Lit_trueContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token LIT_true
    /// Returns `None` if there is no child corresponding to token LIT_true
    fn LIT_true(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_LIT_true)
    }
}
#[derive(Debug)]
pub struct Lit_trueContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: LitContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Lit_trueContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::LitContext }
	fn get_rule_index(&self) -> usize { RULE_lit }
    fn make_node(
        arena: &'arena Arena,
        ctx: Lit_trueContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(LitContextAll::Lit_trueContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Lit_trueContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_trueContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Lit_trueContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_trueContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> LitContextAttrs<'input, 'arena, Tok> for Lit_trueContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_trueContextExt<'input, 'arena, Tok> {
	fn new(base: LitContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut LitContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            LitContextAll::Lit_trueContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut LitContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Lit_intContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Lit_intContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Lit_intContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token INT
	/// Returns `None` if there is no child corresponding to token INT
	fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_intContextAttrs<'input, 'arena, Tok> for Lit_intContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_INT)
    }
}
#[derive(Debug)]
pub struct Lit_intContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: LitContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Lit_intContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::LitContext }
	fn get_rule_index(&self) -> usize { RULE_lit }
    fn make_node(
        arena: &'arena Arena,
        ctx: Lit_intContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(LitContextAll::Lit_intContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Lit_intContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_intContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Lit_intContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_intContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> LitContextAttrs<'input, 'arena, Tok> for Lit_intContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_intContextExt<'input, 'arena, Tok> {
	fn new(base: LitContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut LitContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            LitContextAll::Lit_intContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut LitContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Lit_floatContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Lit_floatContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Lit_floatContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token FLOAT
	/// Returns `None` if there is no child corresponding to token FLOAT
	fn FLOAT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_floatContextAttrs<'input, 'arena, Tok> for Lit_floatContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token FLOAT
    /// Returns `None` if there is no child corresponding to token FLOAT
    fn FLOAT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_FLOAT)
    }
}
#[derive(Debug)]
pub struct Lit_floatContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: LitContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Lit_floatContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::LitContext }
	fn get_rule_index(&self) -> usize { RULE_lit }
    fn make_node(
        arena: &'arena Arena,
        ctx: Lit_floatContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(LitContextAll::Lit_floatContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Lit_floatContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_floatContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Lit_floatContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_floatContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> LitContextAttrs<'input, 'arena, Tok> for Lit_floatContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_floatContextExt<'input, 'arena, Tok> {
	fn new(base: LitContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut LitContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            LitContextAll::Lit_floatContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut LitContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Lit_falseContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Lit_falseContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Lit_falseContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token LIT_false
	/// Returns `None` if there is no child corresponding to token LIT_false
	fn LIT_false(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_falseContextAttrs<'input, 'arena, Tok> for Lit_falseContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token LIT_false
    /// Returns `None` if there is no child corresponding to token LIT_false
    fn LIT_false(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_LIT_false)
    }
}
#[derive(Debug)]
pub struct Lit_falseContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: LitContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Lit_falseContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::LitContext }
	fn get_rule_index(&self) -> usize { RULE_lit }
    fn make_node(
        arena: &'arena Arena,
        ctx: Lit_falseContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(LitContextAll::Lit_falseContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Lit_falseContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_falseContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Lit_falseContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut LitContextAll<'input, 'arena, Tok>) {
                LitContextAll::Lit_falseContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> LitContextAttrs<'input, 'arena, Tok> for Lit_falseContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Lit_falseContextExt<'input, 'arena, Tok> {
	fn new(base: LitContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut LitContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            LitContextAll::Lit_falseContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut LitContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lit(&mut self,) -> Result<&'arena LitContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LitContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 22, RULE_lit)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LitContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(156);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_INT  => {
			        /*------- Outer Most Alt 1 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Lit_intContextExt::copy_from(ctx);
			            ctx.set_alt_number(1);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Lit_intContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(151);
			        recog.base.match_token(CFood_INT,&mut recog.err_handler)?;
			        }}
			    CFood_FLOAT  => {
			        /*------- Outer Most Alt 2 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Lit_floatContextExt::copy_from(ctx);
			            ctx.set_alt_number(2);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Lit_floatContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(152);
			        recog.base.match_token(CFood_FLOAT,&mut recog.err_handler)?;
			        }}
			    CFood_CONSTR  => {
			        /*------- Outer Most Alt 3 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Lit_constrContextExt::copy_from(ctx);
			            ctx.set_alt_number(3);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Lit_constrContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(153);
			        recog.base.match_token(CFood_CONSTR,&mut recog.err_handler)?;
			        }}
			    CFood_LIT_true  => {
			        /*------- Outer Most Alt 4 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Lit_trueContextExt::copy_from(ctx);
			            ctx.set_alt_number(4);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Lit_trueContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(154);
			        recog.base.match_token(CFood_LIT_true,&mut recog.err_handler)?;
			        }}
			    CFood_LIT_false  => {
			        /*------- Outer Most Alt 5 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Lit_falseContextExt::copy_from(ctx);
			            ctx.set_alt_number(5);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Lit_falseContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(155);
			        recog.base.match_token(CFood_LIT_false,&mut recog.err_handler)?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::TysContext(visit_tys) }
#[derive(Debug)]
pub struct TysContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, TysContextExt {
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
			recog.base.set_state(163);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(6,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(158);
					recog.ty_kind()?;
					recog.base.set_state(159);
					recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					/*InvokeRule tys*/
					recog.base.set_state(160);
					recog.tys()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(162);
					recog.ty_kind()?;
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
#[derive(Debug)]
#[repr(C, u16)]
pub enum Ty_kindContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Ty_kind_tyContext(Ty_kind_tyContext<'input, 'arena, Tok>),
	Ty_kind_typeContext(Ty_kind_typeContext<'input, 'arena, Tok>),
    Error(Ty_kindContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Ty_kindContextAll { } { Ty_kind_tyContext, Ty_kind_typeContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Ty_kindContextAll { } { Ty_kind_tyContext, Ty_kind_typeContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Ty_kindContextAll { Ty_kind_tyContext, Ty_kind_typeContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Ty_kindContext::Ty_kindContextAll { Ty_kind_tyContext, Ty_kind_typeContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Ty_kindContextAll { Ty_kind_tyContext(enter_ty_kind_ty, exit_ty_kind_ty), Ty_kind_typeContext(enter_ty_kind_type, exit_ty_kind_type), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Ty_kindContextAll { Ty_kind_tyContext(visit_ty_kind_ty), Ty_kind_typeContext(visit_ty_kind_type), } }

impl<'input, 'arena, Tok> Deref for Ty_kindContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Ty_kindContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Ty_kindContextAll::*;
		match self{
			Ty_kind_tyContext(inner) => inner,
			Ty_kind_typeContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Ty_kindContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Ty_kindContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Ty_kindContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        arena.alloc_labeled_node(Ty_kindContextAll::Error(ctx))
    }
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Ty_kindContextExt {
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
}

impl<'input, 'arena, Tok: Token + 'input> Ty_kindContextAttrs<'input, 'arena, Tok> for Ty_kindContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Ty_kind_tyContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Ty_kind_tyContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Ty_kind_tyContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token TY_int
	/// Returns `None` if there is no child corresponding to token TY_int
	fn TY_int(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
	/// Retrieves first TerminalNode corresponding to token TY_float
	/// Returns `None` if there is no child corresponding to token TY_float
	fn TY_float(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
	/// Retrieves first TerminalNode corresponding to token TY_str
	/// Returns `None` if there is no child corresponding to token TY_str
	fn TY_str(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
	/// Retrieves first TerminalNode corresponding to token TY_void
	/// Returns `None` if there is no child corresponding to token TY_void
	fn TY_void(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
	/// Retrieves first TerminalNode corresponding to token TY_bool
	/// Returns `None` if there is no child corresponding to token TY_bool
	fn TY_bool(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Ty_kind_tyContextAttrs<'input, 'arena, Tok> for Ty_kind_tyContext<'input, 'arena, Tok>
{
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
    /// Retrieves first TerminalNode corresponding to token TY_str
    /// Returns `None` if there is no child corresponding to token TY_str
    fn TY_str(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TY_str)
    }
    /// Retrieves first TerminalNode corresponding to token TY_void
    /// Returns `None` if there is no child corresponding to token TY_void
    fn TY_void(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TY_void)
    }
    /// Retrieves first TerminalNode corresponding to token TY_bool
    /// Returns `None` if there is no child corresponding to token TY_bool
    fn TY_bool(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TY_bool)
    }
}
#[derive(Debug)]
pub struct Ty_kind_tyContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Ty_kindContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Ty_kind_tyContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Ty_kindContext }
	fn get_rule_index(&self) -> usize { RULE_ty_kind }
    fn make_node(
        arena: &'arena Arena,
        ctx: Ty_kind_tyContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Ty_kindContextAll::Ty_kind_tyContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Ty_kind_tyContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Ty_kindContextAll<'input, 'arena, Tok>) {
                Ty_kindContextAll::Ty_kind_tyContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Ty_kind_tyContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Ty_kindContextAll<'input, 'arena, Tok>) {
                Ty_kindContextAll::Ty_kind_tyContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Ty_kindContextAttrs<'input, 'arena, Tok> for Ty_kind_tyContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Ty_kind_tyContextExt<'input, 'arena, Tok> {
	fn new(base: Ty_kindContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Ty_kindContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Ty_kindContextAll::Ty_kind_tyContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Ty_kindContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Ty_kind_typeContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Ty_kind_typeContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Ty_kind_typeContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token TYPE
	/// Returns `None` if there is no child corresponding to token TYPE
	fn TYPE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Ty_kind_typeContextAttrs<'input, 'arena, Tok> for Ty_kind_typeContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token TYPE
    /// Returns `None` if there is no child corresponding to token TYPE
    fn TYPE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TYPE)
    }
}
#[derive(Debug)]
pub struct Ty_kind_typeContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Ty_kindContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Ty_kind_typeContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Ty_kindContext }
	fn get_rule_index(&self) -> usize { RULE_ty_kind }
    fn make_node(
        arena: &'arena Arena,
        ctx: Ty_kind_typeContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Ty_kindContextAll::Ty_kind_typeContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Ty_kind_typeContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Ty_kindContextAll<'input, 'arena, Tok>) {
                Ty_kindContextAll::Ty_kind_typeContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Ty_kind_typeContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Ty_kindContextAll<'input, 'arena, Tok>) {
                Ty_kindContextAll::Ty_kind_typeContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Ty_kindContextAttrs<'input, 'arena, Tok> for Ty_kind_typeContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Ty_kind_typeContextExt<'input, 'arena, Tok> {
	fn new(base: Ty_kindContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Ty_kindContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Ty_kindContextAll::Ty_kind_typeContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Ty_kindContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
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
        recog.base.enter_rule(Ty_kindContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 26, RULE_ty_kind)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kindContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(171);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_TY_int  => {
			        /*------- Outer Most Alt 1 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Ty_kind_tyContextExt::copy_from(ctx);
			            ctx.set_alt_number(1);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kind_tyContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(165);
			        recog.base.match_token(CFood_TY_int,&mut recog.err_handler)?;
			        }}
			    CFood_TY_float  => {
			        /*------- Outer Most Alt 2 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Ty_kind_tyContextExt::copy_from(ctx);
			            ctx.set_alt_number(2);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kind_tyContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(166);
			        recog.base.match_token(CFood_TY_float,&mut recog.err_handler)?;
			        }}
			    CFood_TY_str  => {
			        /*------- Outer Most Alt 3 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Ty_kind_tyContextExt::copy_from(ctx);
			            ctx.set_alt_number(3);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kind_tyContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(167);
			        recog.base.match_token(CFood_TY_str,&mut recog.err_handler)?;
			        }}
			    CFood_TY_void  => {
			        /*------- Outer Most Alt 4 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Ty_kind_tyContextExt::copy_from(ctx);
			            ctx.set_alt_number(4);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kind_tyContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(168);
			        recog.base.match_token(CFood_TY_void,&mut recog.err_handler)?;
			        }}
			    CFood_TY_bool  => {
			        /*------- Outer Most Alt 5 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Ty_kind_tyContextExt::copy_from(ctx);
			            ctx.set_alt_number(5);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kind_tyContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(169);
			        recog.base.match_token(CFood_TY_bool,&mut recog.err_handler)?;
			        }}
			    CFood_TYPE  => {
			        /*------- Outer Most Alt 6 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Ty_kind_typeContextExt::copy_from(ctx);
			            ctx.set_alt_number(6);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kind_typeContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(170);
			        recog.base.match_token(CFood_TYPE,&mut recog.err_handler)?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::BlockContext(visit_block) }
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
        recog.base.enter_rule(BlockContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 28, RULE_block)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena BlockContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(173);
			recog.base.match_token(CFood_BRACE_L,&mut recog.err_handler)?;
			/*InvokeRule stmts*/
			recog.base.set_state(174);
			recog.stmts()?;
			recog.base.set_state(175);
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
dbt_antlr4::impl_visitable! { CFoodVisitor::StmtsContext(visit_stmts) }
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
        recog.base.enter_rule(StmtsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 30, RULE_stmts)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(181);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_KW_while |CFood_KW_for |CFood_KW_if |CFood_KW_return |CFood_KW_let |
			    CFood_LIT_true |CFood_LIT_false |CFood_TY_int |CFood_TY_float |CFood_TY_str |
			    CFood_TY_void |CFood_TY_bool |CFood_MAGIC_printf |CFood_MAGIC_scanf |
			    CFood_MAGIC_new |CFood_PAREN_L |CFood_BRACE_L |CFood_NOT |CFood_PLUS |
			    CFood_SUB |CFood_REFER |CFood_SEMICOLON |CFood_TYPE |CFood_IDENT |
			    CFood_INT |CFood_FLOAT |CFood_CONSTR  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule stmt*/
			        recog.base.set_state(177);
			        recog.stmt()?;
			        /*InvokeRule stmts*/
			        recog.base.set_state(178);
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
dbt_antlr4::impl_visitable! { CFoodVisitor::StmtContext(visit_stmt) }
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
    fn branch_stmt(&self) -> Option<&'arena Branch_stmtContextAll<'input, 'arena, Tok>>;
    fn iter_stmt(&self) -> Option<&'arena Iter_stmtContextAll<'input, 'arena, Tok>>;
    fn for_stmt(&self) -> Option<&'arena For_stmtContextAll<'input, 'arena, Tok>>;
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>>;
    fn var_decl(&self) -> Option<&'arena Var_declContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr_stmt(&self) -> Option<&'arena Expr_stmtContextAll<'input, 'arena, Tok>>;
    fn let_stmt(&self) -> Option<&'arena Let_stmtContextAll<'input, 'arena, Tok>>;
    fn return_stmt(&self) -> Option<&'arena Return_stmtContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StmtContextAttrs<'input, 'arena, Tok> for StmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn branch_stmt(&self) -> Option<&'arena Branch_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn iter_stmt(&self) -> Option<&'arena Iter_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn for_stmt(&self) -> Option<&'arena For_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn var_decl(&self) -> Option<&'arena Var_declContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_SEMICOLON)
    }
    fn expr_stmt(&self) -> Option<&'arena Expr_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn let_stmt(&self) -> Option<&'arena Let_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn return_stmt(&self) -> Option<&'arena Return_stmtContextAll<'input, 'arena, Tok>> {
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
        recog.base.enter_rule(StmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 32, RULE_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(200);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_KW_if  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule branch_stmt*/
			        recog.base.set_state(183);
			        recog.branch_stmt()?;
			        }}
			    CFood_KW_while  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        /*InvokeRule iter_stmt*/
			        recog.base.set_state(184);
			        recog.iter_stmt()?;
			        }}
			    CFood_KW_for  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        /*InvokeRule for_stmt*/
			        recog.base.set_state(185);
			        recog.for_stmt()?;
			        }}
			    CFood_BRACE_L  => {
			        /*------- Outer Most Alt 4 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
			        {
			        /*InvokeRule block*/
			        recog.base.set_state(186);
			        recog.block()?;
			        }}
			    CFood_TY_int |CFood_TY_float |CFood_TY_str |CFood_TY_void |CFood_TY_bool |
			    CFood_TYPE  => {
			        /*------- Outer Most Alt 5 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
			        {
			        /*InvokeRule var_decl*/
			        recog.base.set_state(187);
			        recog.var_decl()?;
			        recog.base.set_state(188);
			        recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
			        }}
			    CFood_LIT_true |CFood_LIT_false |CFood_MAGIC_printf |CFood_MAGIC_scanf |
			    CFood_MAGIC_new |CFood_PAREN_L |CFood_NOT |CFood_PLUS |CFood_SUB |
			    CFood_REFER |CFood_IDENT |CFood_INT |CFood_FLOAT |CFood_CONSTR  => {
			        /*------- Outer Most Alt 6 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(6); }
			        {
			        /*InvokeRule expr_stmt*/
			        recog.base.set_state(190);
			        recog.expr_stmt()?;
			        recog.base.set_state(191);
			        recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
			        }}
			    CFood_KW_let  => {
			        /*------- Outer Most Alt 7 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(7); }
			        {
			        /*InvokeRule let_stmt*/
			        recog.base.set_state(193);
			        recog.let_stmt()?;
			        recog.base.set_state(194);
			        recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
			        }}
			    CFood_KW_return  => {
			        /*------- Outer Most Alt 8 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(8); }
			        {
			        /*InvokeRule return_stmt*/
			        recog.base.set_state(196);
			        recog.return_stmt()?;
			        recog.base.set_state(197);
			        recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
			        }}
			    CFood_SEMICOLON  => {
			        /*------- Outer Most Alt 9 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(9); }
			        {
			        recog.base.set_state(199);
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
//------------------- expr_stmt ----------------
pub type Expr_stmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = Expr_stmtContext<'input, 'arena, Tok>;

pub type Expr_stmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_stmtContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_stmtContext(visit_expr_stmt) }
#[derive(Debug)]
pub struct Expr_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_stmtContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_stmtContext }
	fn get_rule_index(&self) -> usize { RULE_expr_stmt }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_stmtContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_stmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_stmtContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_stmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Expr_stmtContextAttrs<'input, 'arena, Tok> for Expr_stmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
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
	pub fn expr_stmt(&mut self,) -> Result<&'arena Expr_stmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 34, RULE_expr_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule expr*/
			recog.base.set_state(202);
			recog.expr()?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Branch_stmtContext(visit_branch_stmt) }
#[derive(Debug)]
pub struct Branch_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub then_branch: Option<&'arena StmtContextAll<'input, 'arena, Tok>>,
	pub else_branch: Option<&'arena StmtContextAll<'input, 'arena, Tok>>,
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
				then_branch: None, else_branch: None, 
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
    fn stmt_all(&self) -> Vec<&'arena StmtContextAll<'input, 'arena, Tok>>;
    fn stmt(&self, i: usize) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>>;
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
    fn stmt_all(&self) -> Vec<&'arena StmtContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn stmt(&self, i: usize) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>> {
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
        recog.base.enter_rule(Branch_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 36, RULE_branch_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Branch_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(218);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(10,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(204);
					recog.base.match_token(CFood_KW_if,&mut recog.err_handler)?;
					recog.base.set_state(205);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(206);
					recog.expr()?;
					recog.base.set_state(207);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					/*InvokeRule stmt*/
					recog.base.set_state(208);
					let tmp = recog.stmt()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Branch_stmtContext<TF::Tok>>().unwrap().then_branch = Some(tmp); } 
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(210);
					recog.base.match_token(CFood_KW_if,&mut recog.err_handler)?;
					recog.base.set_state(211);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(212);
					recog.expr()?;
					recog.base.set_state(213);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					/*InvokeRule stmt*/
					recog.base.set_state(214);
					let tmp = recog.stmt()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Branch_stmtContext<TF::Tok>>().unwrap().then_branch = Some(tmp); } 
					recog.base.set_state(215);
					recog.base.match_token(CFood_KW_else,&mut recog.err_handler)?;
					/*InvokeRule stmt*/
					recog.base.set_state(216);
					let tmp = recog.stmt()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Branch_stmtContext<TF::Tok>>().unwrap().else_branch = Some(tmp); } 
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Iter_stmtContext(visit_iter_stmt) }
#[derive(Debug)]
pub struct Iter_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
    fn stmt(&self) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>>;
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
    fn stmt(&self) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>> {
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
        recog.base.enter_rule(Iter_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 38, RULE_iter_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Iter_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(220);
			recog.base.match_token(CFood_KW_while,&mut recog.err_handler)?;
			recog.base.set_state(221);
			recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
			/*InvokeRule expr*/
			recog.base.set_state(222);
			recog.expr()?;
			recog.base.set_state(223);
			recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
			/*InvokeRule stmt*/
			recog.base.set_state(224);
			recog.stmt()?;
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
//------------------- inline_stmts ----------------
pub type Inline_stmtsContextAll<'input, 'arena, Tok = CommonToken<'input>> = Inline_stmtsContext<'input, 'arena, Tok>;

pub type Inline_stmtsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Inline_stmtsContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::Inline_stmtsContext(visit_inline_stmts) }
#[derive(Debug)]
pub struct Inline_stmtsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Inline_stmtsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Inline_stmtsContext }
	fn get_rule_index(&self) -> usize { RULE_inline_stmts }
    fn make_node(
        arena: &'arena Arena,
        ctx: Inline_stmtsContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Inline_stmtsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Inline_stmtsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Inline_stmtsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Inline_stmtsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Inline_stmtsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Inline_stmtsContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Inline_stmtsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn inline_stmt(&self) -> Option<&'arena Inline_stmtContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn inline_stmts(&self) -> Option<&'arena Inline_stmtsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Inline_stmtsContextAttrs<'input, 'arena, Tok> for Inline_stmtsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn inline_stmt(&self) -> Option<&'arena Inline_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_COMMA)
    }
    fn inline_stmts(&self) -> Option<&'arena Inline_stmtsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn inline_stmts(&mut self,) -> Result<&'arena Inline_stmtsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Inline_stmtsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 40, RULE_inline_stmts)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Inline_stmtsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(231);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(11,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule inline_stmt*/
					recog.base.set_state(226);
					recog.inline_stmt()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule inline_stmt*/
					recog.base.set_state(227);
					recog.inline_stmt()?;
					recog.base.set_state(228);
					recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					/*InvokeRule inline_stmts*/
					recog.base.set_state(229);
					recog.inline_stmts()?;
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
//------------------- inline_stmt ----------------
pub type Inline_stmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = Inline_stmtContext<'input, 'arena, Tok>;

pub type Inline_stmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Inline_stmtContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::Inline_stmtContext(visit_inline_stmt) }
#[derive(Debug)]
pub struct Inline_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Inline_stmtContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Inline_stmtContext }
	fn get_rule_index(&self) -> usize { RULE_inline_stmt }
    fn make_node(
        arena: &'arena Arena,
        ctx: Inline_stmtContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Inline_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Inline_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Inline_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Inline_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Inline_stmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Inline_stmtContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Inline_stmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn var_decl(&self) -> Option<&'arena Var_declContextAll<'input, 'arena, Tok>>;
    fn expr_stmt(&self) -> Option<&'arena Expr_stmtContextAll<'input, 'arena, Tok>>;
    fn let_stmt(&self) -> Option<&'arena Let_stmtContextAll<'input, 'arena, Tok>>;
    fn return_stmt(&self) -> Option<&'arena Return_stmtContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Inline_stmtContextAttrs<'input, 'arena, Tok> for Inline_stmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn var_decl(&self) -> Option<&'arena Var_declContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_stmt(&self) -> Option<&'arena Expr_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn let_stmt(&self) -> Option<&'arena Let_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn return_stmt(&self) -> Option<&'arena Return_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn inline_stmt(&mut self,) -> Result<&'arena Inline_stmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Inline_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 42, RULE_inline_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Inline_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(238);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_TY_int |CFood_TY_float |CFood_TY_str |CFood_TY_void |CFood_TY_bool |
			    CFood_TYPE  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule var_decl*/
			        recog.base.set_state(233);
			        recog.var_decl()?;
			        }}
			    CFood_LIT_true |CFood_LIT_false |CFood_MAGIC_printf |CFood_MAGIC_scanf |
			    CFood_MAGIC_new |CFood_PAREN_L |CFood_NOT |CFood_PLUS |CFood_SUB |
			    CFood_REFER |CFood_IDENT |CFood_INT |CFood_FLOAT |CFood_CONSTR  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        /*InvokeRule expr_stmt*/
			        recog.base.set_state(234);
			        recog.expr_stmt()?;
			        }}
			    CFood_KW_let  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        /*InvokeRule let_stmt*/
			        recog.base.set_state(235);
			        recog.let_stmt()?;
			        }}
			    CFood_KW_return  => {
			        /*------- Outer Most Alt 4 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
			        {
			        /*InvokeRule return_stmt*/
			        recog.base.set_state(236);
			        recog.return_stmt()?;
			        }}
			    CFood_PAREN_R |CFood_COMMA |CFood_SEMICOLON  => {
			        /*------- Outer Most Alt 5 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
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
//------------------- for_stmt ----------------
pub type For_stmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = For_stmtContext<'input, 'arena, Tok>;

pub type For_stmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, For_stmtContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::For_stmtContext(visit_for_stmt) }
#[derive(Debug)]
pub struct For_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub init: Option<&'arena Inline_stmtsContextAll<'input, 'arena, Tok>>,
	pub cond: Option<&'arena ExprContextAll<'input, 'arena, Tok>>,
	pub mutate: Option<&'arena Inline_stmtsContextAll<'input, 'arena, Tok>>,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for For_stmtContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::For_stmtContext }
	fn get_rule_index(&self) -> usize { RULE_for_stmt }
    fn make_node(
        arena: &'arena Arena,
        ctx: For_stmtContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a For_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => For_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut For_stmtContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut For_stmtContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> For_stmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, For_stmtContextExt {
				init: None, cond: None, mutate: None, 
				ph: PhantomData
			}
		)
	}
}

pub trait For_stmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token KW_for
    /// Returns `None` if there is no child corresponding to token KW_for
    fn KW_for(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
    fn SEMICOLON_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token SEMICOLON, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMICOLON is less than or equal to `i`.
    fn SEMICOLON(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn stmt(&self) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>>;
    fn inline_stmts_all(&self) -> Vec<&'arena Inline_stmtsContextAll<'input, 'arena, Tok>>;
    fn inline_stmts(&self, i: usize) -> Option<&'arena Inline_stmtsContextAll<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> For_stmtContextAttrs<'input, 'arena, Tok> for For_stmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token KW_for
    /// Returns `None` if there is no child corresponding to token KW_for
    fn KW_for(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_KW_for)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_L)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
    fn SEMICOLON_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == CFood_SEMICOLON).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token SEMICOLON, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMICOLON is less than or equal to `i`.
    fn SEMICOLON(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == CFood_SEMICOLON).nth(i)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_R)
    }
    fn stmt(&self) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn inline_stmts_all(&self) -> Vec<&'arena Inline_stmtsContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn inline_stmts(&self, i: usize) -> Option<&'arena Inline_stmtsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
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
	pub fn for_stmt(&mut self,) -> Result<&'arena For_stmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(For_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 44, RULE_for_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena For_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(240);
			recog.base.match_token(CFood_KW_for,&mut recog.err_handler)?;
			recog.base.set_state(241);
			recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
			/*InvokeRule inline_stmts*/
			recog.base.set_state(242);
			let tmp = recog.inline_stmts()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<For_stmtContext<TF::Tok>>().unwrap().init = Some(tmp); } 
			recog.base.set_state(243);
			recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
			/*InvokeRule expr*/
			recog.base.set_state(244);
			let tmp = recog.expr()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<For_stmtContext<TF::Tok>>().unwrap().cond = Some(tmp); } 
			recog.base.set_state(245);
			recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
			/*InvokeRule inline_stmts*/
			recog.base.set_state(246);
			let tmp = recog.inline_stmts()?;
			unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<For_stmtContext<TF::Tok>>().unwrap().mutate = Some(tmp); } 
			recog.base.set_state(247);
			recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
			/*InvokeRule stmt*/
			recog.base.set_state(248);
			recog.stmt()?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Return_stmtContext(visit_return_stmt) }
#[derive(Debug)]
pub struct Return_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        recog.base.enter_rule(Return_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 46, RULE_return_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Return_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(253);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(13,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(250);
					recog.base.match_token(CFood_KW_return,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(251);
					recog.base.match_token(CFood_KW_return,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(252);
					recog.expr()?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Let_stmtContext(visit_let_stmt) }
#[derive(Debug)]
pub struct Let_stmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        recog.base.enter_rule(Let_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 48, RULE_let_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Let_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(255);
			recog.base.match_token(CFood_KW_let,&mut recog.err_handler)?;
			recog.base.set_state(256);
			recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
			recog.base.set_state(257);
			recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			/*InvokeRule expr*/
			recog.base.set_state(258);
			recog.expr()?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::VarContext(visit_var) }
#[derive(Debug)]
pub struct VarContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, VarContextExt {
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
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> VarContextAttrs<'input, 'arena, Tok> for VarContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_IDENT)
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
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(260);
			recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
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
//------------------- refer ----------------
pub type ReferContextAll<'input, 'arena, Tok = CommonToken<'input>> = ReferContext<'input, 'arena, Tok>;

pub type ReferContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ReferContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::ReferContext(visit_refer) }
#[derive(Debug)]
pub struct ReferContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ReferContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::ReferContext }
	fn get_rule_index(&self) -> usize { RULE_refer }
    fn make_node(
        arena: &'arena Arena,
        ctx: ReferContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ReferContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => ReferContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ReferContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut ReferContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ReferContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ReferContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ReferContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token REFER
    /// Returns `None` if there is no child corresponding to token REFER
    fn REFER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ReferContextAttrs<'input, 'arena, Tok> for ReferContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token REFER
    /// Returns `None` if there is no child corresponding to token REFER
    fn REFER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_REFER)
    }
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_IDENT)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn refer(&mut self,) -> Result<&'arena ReferContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ReferContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 52, RULE_refer)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ReferContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(262);
			recog.base.match_token(CFood_REFER,&mut recog.err_handler)?;
			recog.base.set_state(263);
			recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::ExprContext(visit_expr) }
#[derive(Debug)]
pub struct ExprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, ExprContextExt {
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
    fn expr_assign(&self) -> Option<&'arena Expr_assignContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ExprContextAttrs<'input, 'arena, Tok> for ExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn expr_assign(&self) -> Option<&'arena Expr_assignContextAll<'input, 'arena, Tok>> {
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
        recog.base.enter_rule(ExprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 54, RULE_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ExprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule expr_assign*/
			recog.base.set_state(265);
			recog.expr_assign()?;
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
//------------------- expr_assign ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum Expr_assignContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Expr_assign_useContext(Expr_assign_useContext<'input, 'arena, Tok>),
	Expr_assign_passContext(Expr_assign_passContext<'input, 'arena, Tok>),
    Error(Expr_assignContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Expr_assignContextAll { } { Expr_assign_useContext, Expr_assign_passContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Expr_assignContextAll { } { Expr_assign_useContext, Expr_assign_passContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Expr_assignContextAll { Expr_assign_useContext, Expr_assign_passContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Expr_assignContext::Expr_assignContextAll { Expr_assign_useContext, Expr_assign_passContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Expr_assignContextAll { Expr_assign_useContext(enter_expr_assign_use, exit_expr_assign_use), Expr_assign_passContext(enter_expr_assign_pass, exit_expr_assign_pass), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_assignContextAll { Expr_assign_useContext(visit_expr_assign_use), Expr_assign_passContext(visit_expr_assign_pass), } }

impl<'input, 'arena, Tok> Deref for Expr_assignContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Expr_assignContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Expr_assignContextAll::*;
		match self{
			Expr_assign_useContext(inner) => inner,
			Expr_assign_passContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Expr_assignContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_assignContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Expr_assignContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_assignContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_assignContext }
	fn get_rule_index(&self) -> usize { RULE_expr_assign }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_assignContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_assignContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_assignContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_assignContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_assignContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_assignContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_assignContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_assignContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_assignContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> Expr_assignContextAttrs<'input, 'arena, Tok> for Expr_assignContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Expr_assign_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_assign_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_assign_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ASSIGN
	/// Returns `None` if there is no child corresponding to token ASSIGN
	fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
	fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>>;
	fn expr_assign(&self) -> Option<&'arena Expr_assignContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_assign_useContextAttrs<'input, 'arena, Tok> for Expr_assign_useContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_ASSIGN)
    }
    fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_assign(&self) -> Option<&'arena Expr_assignContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_assign_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_assignContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena VarContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Expr_assignContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_assign_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_assignContext }
	fn get_rule_index(&self) -> usize { RULE_expr_assign }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_assign_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_assignContextAll::Expr_assign_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_assign_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_assignContextAll<'input, 'arena, Tok>) {
                Expr_assignContextAll::Expr_assign_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_assign_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_assignContextAll<'input, 'arena, Tok>) {
                Expr_assignContextAll::Expr_assign_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_assignContextAttrs<'input, 'arena, Tok> for Expr_assign_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_assign_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_assignContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_assignContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_assignContextAll::Expr_assign_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_assignContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_assign_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_assign_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_assign_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_logic(&self) -> Option<&'arena Expr_logicContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_assign_passContextAttrs<'input, 'arena, Tok> for Expr_assign_passContext<'input, 'arena, Tok>
{
    fn expr_logic(&self) -> Option<&'arena Expr_logicContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_assign_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_assignContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_assign_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_assignContext }
	fn get_rule_index(&self) -> usize { RULE_expr_assign }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_assign_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_assignContextAll::Expr_assign_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_assign_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_assignContextAll<'input, 'arena, Tok>) {
                Expr_assignContextAll::Expr_assign_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_assign_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_assignContextAll<'input, 'arena, Tok>) {
                Expr_assignContextAll::Expr_assign_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_assignContextAttrs<'input, 'arena, Tok> for Expr_assign_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_assign_passContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_assignContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_assignContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_assignContextAll::Expr_assign_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_assignContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr_assign(&mut self,) -> Result<&'arena Expr_assignContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_assignContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 56, RULE_expr_assign)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_assignContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(272);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(14,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_assign_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_assign_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_logic*/
					recog.base.set_state(267);
					recog.expr_logic()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_assign_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_assign_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule var*/
					recog.base.set_state(268);
					let tmp = recog.var()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_assign_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					recog.base.set_state(269);
					recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
					/*InvokeRule expr_assign*/
					recog.base.set_state(270);
					let tmp = recog.expr_assign()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_assign_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
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
//------------------- expr_logic ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum Expr_logicContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Expr_logic_useContext(Expr_logic_useContext<'input, 'arena, Tok>),
	Expr_logic_passContext(Expr_logic_passContext<'input, 'arena, Tok>),
    Error(Expr_logicContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Expr_logicContextAll { } { Expr_logic_useContext, Expr_logic_passContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Expr_logicContextAll { } { Expr_logic_useContext, Expr_logic_passContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Expr_logicContextAll { Expr_logic_useContext, Expr_logic_passContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Expr_logicContext::Expr_logicContextAll { Expr_logic_useContext, Expr_logic_passContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Expr_logicContextAll { Expr_logic_useContext(enter_expr_logic_use, exit_expr_logic_use), Expr_logic_passContext(enter_expr_logic_pass, exit_expr_logic_pass), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_logicContextAll { Expr_logic_useContext(visit_expr_logic_use), Expr_logic_passContext(visit_expr_logic_pass), } }

impl<'input, 'arena, Tok> Deref for Expr_logicContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Expr_logicContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Expr_logicContextAll::*;
		match self{
			Expr_logic_useContext(inner) => inner,
			Expr_logic_passContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Expr_logicContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_logicContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Expr_logicContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_logicContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_logicContext }
	fn get_rule_index(&self) -> usize { RULE_expr_logic }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_logicContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_logicContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_logicContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_logicContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_logicContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_logicContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_logicContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_logicContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_logicContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> Expr_logicContextAttrs<'input, 'arena, Tok> for Expr_logicContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Expr_logic_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_logic_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_logic_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn logic_preced_op(&self) -> Option<&'arena Logic_preced_opContextAll<'input, 'arena, Tok>>;
	fn expr_cmp(&self) -> Option<&'arena Expr_cmpContextAll<'input, 'arena, Tok>>;
	fn expr_logic(&self) -> Option<&'arena Expr_logicContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_logic_useContextAttrs<'input, 'arena, Tok> for Expr_logic_useContext<'input, 'arena, Tok>
{
    fn logic_preced_op(&self) -> Option<&'arena Logic_preced_opContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_cmp(&self) -> Option<&'arena Expr_cmpContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_logic(&self) -> Option<&'arena Expr_logicContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_logic_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_logicContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena Expr_cmpContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Expr_logicContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_logic_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_logicContext }
	fn get_rule_index(&self) -> usize { RULE_expr_logic }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_logic_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_logicContextAll::Expr_logic_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_logic_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_logicContextAll<'input, 'arena, Tok>) {
                Expr_logicContextAll::Expr_logic_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_logic_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_logicContextAll<'input, 'arena, Tok>) {
                Expr_logicContextAll::Expr_logic_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_logicContextAttrs<'input, 'arena, Tok> for Expr_logic_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_logic_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_logicContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_logicContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_logicContextAll::Expr_logic_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_logicContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_logic_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_logic_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_logic_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_cmp(&self) -> Option<&'arena Expr_cmpContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_logic_passContextAttrs<'input, 'arena, Tok> for Expr_logic_passContext<'input, 'arena, Tok>
{
    fn expr_cmp(&self) -> Option<&'arena Expr_cmpContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_logic_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_logicContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_logic_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_logicContext }
	fn get_rule_index(&self) -> usize { RULE_expr_logic }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_logic_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_logicContextAll::Expr_logic_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_logic_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_logicContextAll<'input, 'arena, Tok>) {
                Expr_logicContextAll::Expr_logic_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_logic_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_logicContextAll<'input, 'arena, Tok>) {
                Expr_logicContextAll::Expr_logic_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_logicContextAttrs<'input, 'arena, Tok> for Expr_logic_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_logic_passContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_logicContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_logicContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_logicContextAll::Expr_logic_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_logicContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr_logic(&mut self,) -> Result<&'arena Expr_logicContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_logicContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 58, RULE_expr_logic)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_logicContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(279);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(15,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_logic_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_logic_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_cmp*/
					recog.base.set_state(274);
					recog.expr_cmp()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_logic_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_logic_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_cmp*/
					recog.base.set_state(275);
					let tmp = recog.expr_cmp()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_logic_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					/*InvokeRule logic_preced_op*/
					recog.base.set_state(276);
					recog.logic_preced_op()?;
					/*InvokeRule expr_logic*/
					recog.base.set_state(277);
					let tmp = recog.expr_logic()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_logic_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
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
//------------------- expr_cmp ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum Expr_cmpContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Expr_cmp_useContext(Expr_cmp_useContext<'input, 'arena, Tok>),
	Expr_cmp_passContext(Expr_cmp_passContext<'input, 'arena, Tok>),
    Error(Expr_cmpContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Expr_cmpContextAll { } { Expr_cmp_useContext, Expr_cmp_passContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Expr_cmpContextAll { } { Expr_cmp_useContext, Expr_cmp_passContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Expr_cmpContextAll { Expr_cmp_useContext, Expr_cmp_passContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Expr_cmpContext::Expr_cmpContextAll { Expr_cmp_useContext, Expr_cmp_passContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Expr_cmpContextAll { Expr_cmp_useContext(enter_expr_cmp_use, exit_expr_cmp_use), Expr_cmp_passContext(enter_expr_cmp_pass, exit_expr_cmp_pass), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_cmpContextAll { Expr_cmp_useContext(visit_expr_cmp_use), Expr_cmp_passContext(visit_expr_cmp_pass), } }

impl<'input, 'arena, Tok> Deref for Expr_cmpContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Expr_cmpContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Expr_cmpContextAll::*;
		match self{
			Expr_cmp_useContext(inner) => inner,
			Expr_cmp_passContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Expr_cmpContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_cmpContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Expr_cmpContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_cmpContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_cmpContext }
	fn get_rule_index(&self) -> usize { RULE_expr_cmp }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_cmpContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_cmpContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_cmpContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_cmpContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_cmpContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_cmpContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cmpContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_cmpContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_cmpContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> Expr_cmpContextAttrs<'input, 'arena, Tok> for Expr_cmpContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Expr_cmp_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_cmp_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_cmp_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn cmp_preced_op(&self) -> Option<&'arena Cmp_preced_opContextAll<'input, 'arena, Tok>>;
	fn expr_add(&self) -> Option<&'arena Expr_addContextAll<'input, 'arena, Tok>>;
	fn expr_cmp(&self) -> Option<&'arena Expr_cmpContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cmp_useContextAttrs<'input, 'arena, Tok> for Expr_cmp_useContext<'input, 'arena, Tok>
{
    fn cmp_preced_op(&self) -> Option<&'arena Cmp_preced_opContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_add(&self) -> Option<&'arena Expr_addContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_cmp(&self) -> Option<&'arena Expr_cmpContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_cmp_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_cmpContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena Expr_addContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Expr_cmpContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_cmp_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_cmpContext }
	fn get_rule_index(&self) -> usize { RULE_expr_cmp }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_cmp_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_cmpContextAll::Expr_cmp_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_cmp_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_cmpContextAll<'input, 'arena, Tok>) {
                Expr_cmpContextAll::Expr_cmp_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_cmp_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_cmpContextAll<'input, 'arena, Tok>) {
                Expr_cmpContextAll::Expr_cmp_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_cmpContextAttrs<'input, 'arena, Tok> for Expr_cmp_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cmp_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_cmpContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_cmpContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_cmpContextAll::Expr_cmp_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_cmpContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_cmp_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_cmp_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_cmp_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_add(&self) -> Option<&'arena Expr_addContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cmp_passContextAttrs<'input, 'arena, Tok> for Expr_cmp_passContext<'input, 'arena, Tok>
{
    fn expr_add(&self) -> Option<&'arena Expr_addContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_cmp_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_cmpContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_cmp_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_cmpContext }
	fn get_rule_index(&self) -> usize { RULE_expr_cmp }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_cmp_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_cmpContextAll::Expr_cmp_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_cmp_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_cmpContextAll<'input, 'arena, Tok>) {
                Expr_cmpContextAll::Expr_cmp_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_cmp_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_cmpContextAll<'input, 'arena, Tok>) {
                Expr_cmpContextAll::Expr_cmp_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_cmpContextAttrs<'input, 'arena, Tok> for Expr_cmp_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cmp_passContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_cmpContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_cmpContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_cmpContextAll::Expr_cmp_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_cmpContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr_cmp(&mut self,) -> Result<&'arena Expr_cmpContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_cmpContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 60, RULE_expr_cmp)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_cmpContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(286);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(16,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_cmp_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_cmp_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_add*/
					recog.base.set_state(281);
					recog.expr_add()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_cmp_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_cmp_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_add*/
					recog.base.set_state(282);
					let tmp = recog.expr_add()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_cmp_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					/*InvokeRule cmp_preced_op*/
					recog.base.set_state(283);
					recog.cmp_preced_op()?;
					/*InvokeRule expr_cmp*/
					recog.base.set_state(284);
					let tmp = recog.expr_cmp()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_cmp_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
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
//------------------- expr_add ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum Expr_addContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Expr_add_passContext(Expr_add_passContext<'input, 'arena, Tok>),
	Expr_add_useContext(Expr_add_useContext<'input, 'arena, Tok>),
    Error(Expr_addContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Expr_addContextAll { } { Expr_add_passContext, Expr_add_useContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Expr_addContextAll { } { Expr_add_passContext, Expr_add_useContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Expr_addContextAll { Expr_add_passContext, Expr_add_useContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Expr_addContext::Expr_addContextAll { Expr_add_passContext, Expr_add_useContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Expr_addContextAll { Expr_add_passContext(enter_expr_add_pass, exit_expr_add_pass), Expr_add_useContext(enter_expr_add_use, exit_expr_add_use), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_addContextAll { Expr_add_passContext(visit_expr_add_pass), Expr_add_useContext(visit_expr_add_use), } }

impl<'input, 'arena, Tok> Deref for Expr_addContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Expr_addContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Expr_addContextAll::*;
		match self{
			Expr_add_passContext(inner) => inner,
			Expr_add_useContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Expr_addContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_addContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Expr_addContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_addContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_addContext }
	fn get_rule_index(&self) -> usize { RULE_expr_add }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_addContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_addContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_addContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_addContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_addContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_addContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_addContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_addContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_addContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> Expr_addContextAttrs<'input, 'arena, Tok> for Expr_addContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Expr_add_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_add_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_add_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_mul(&self) -> Option<&'arena Expr_mulContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_add_passContextAttrs<'input, 'arena, Tok> for Expr_add_passContext<'input, 'arena, Tok>
{
    fn expr_mul(&self) -> Option<&'arena Expr_mulContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_add_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_addContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_add_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_addContext }
	fn get_rule_index(&self) -> usize { RULE_expr_add }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_add_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_addContextAll::Expr_add_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_add_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_addContextAll<'input, 'arena, Tok>) {
                Expr_addContextAll::Expr_add_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_add_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_addContextAll<'input, 'arena, Tok>) {
                Expr_addContextAll::Expr_add_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_addContextAttrs<'input, 'arena, Tok> for Expr_add_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_add_passContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_addContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_addContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_addContextAll::Expr_add_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_addContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_add_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_add_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_add_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn add_preced_op(&self) -> Option<&'arena Add_preced_opContextAll<'input, 'arena, Tok>>;
	fn expr_mul(&self) -> Option<&'arena Expr_mulContextAll<'input, 'arena, Tok>>;
	fn expr_add(&self) -> Option<&'arena Expr_addContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_add_useContextAttrs<'input, 'arena, Tok> for Expr_add_useContext<'input, 'arena, Tok>
{
    fn add_preced_op(&self) -> Option<&'arena Add_preced_opContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_mul(&self) -> Option<&'arena Expr_mulContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_add(&self) -> Option<&'arena Expr_addContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_add_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_addContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena Expr_mulContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Expr_addContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_add_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_addContext }
	fn get_rule_index(&self) -> usize { RULE_expr_add }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_add_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_addContextAll::Expr_add_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_add_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_addContextAll<'input, 'arena, Tok>) {
                Expr_addContextAll::Expr_add_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_add_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_addContextAll<'input, 'arena, Tok>) {
                Expr_addContextAll::Expr_add_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_addContextAttrs<'input, 'arena, Tok> for Expr_add_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_add_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_addContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_addContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_addContextAll::Expr_add_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_addContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr_add(&mut self,) -> Result<&'arena Expr_addContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_addContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 62, RULE_expr_add)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_addContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(293);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(17,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_add_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_add_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_mul*/
					recog.base.set_state(288);
					recog.expr_mul()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_add_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_add_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_mul*/
					recog.base.set_state(289);
					let tmp = recog.expr_mul()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_add_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					/*InvokeRule add_preced_op*/
					recog.base.set_state(290);
					recog.add_preced_op()?;
					/*InvokeRule expr_add*/
					recog.base.set_state(291);
					let tmp = recog.expr_add()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_add_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
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
//------------------- expr_mul ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum Expr_mulContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Expr_mul_useContext(Expr_mul_useContext<'input, 'arena, Tok>),
	Expr_mul_passContext(Expr_mul_passContext<'input, 'arena, Tok>),
    Error(Expr_mulContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Expr_mulContextAll { } { Expr_mul_useContext, Expr_mul_passContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Expr_mulContextAll { } { Expr_mul_useContext, Expr_mul_passContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Expr_mulContextAll { Expr_mul_useContext, Expr_mul_passContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Expr_mulContext::Expr_mulContextAll { Expr_mul_useContext, Expr_mul_passContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Expr_mulContextAll { Expr_mul_useContext(enter_expr_mul_use, exit_expr_mul_use), Expr_mul_passContext(enter_expr_mul_pass, exit_expr_mul_pass), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_mulContextAll { Expr_mul_useContext(visit_expr_mul_use), Expr_mul_passContext(visit_expr_mul_pass), } }

impl<'input, 'arena, Tok> Deref for Expr_mulContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Expr_mulContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Expr_mulContextAll::*;
		match self{
			Expr_mul_useContext(inner) => inner,
			Expr_mul_passContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Expr_mulContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_mulContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Expr_mulContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_mulContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_mulContext }
	fn get_rule_index(&self) -> usize { RULE_expr_mul }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_mulContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_mulContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_mulContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_mulContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_mulContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_mulContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_mulContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_mulContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_mulContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> Expr_mulContextAttrs<'input, 'arena, Tok> for Expr_mulContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Expr_mul_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_mul_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_mul_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn mul_preced_op(&self) -> Option<&'arena Mul_preced_opContextAll<'input, 'arena, Tok>>;
	fn expr_cast(&self) -> Option<&'arena Expr_castContextAll<'input, 'arena, Tok>>;
	fn expr_mul(&self) -> Option<&'arena Expr_mulContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_mul_useContextAttrs<'input, 'arena, Tok> for Expr_mul_useContext<'input, 'arena, Tok>
{
    fn mul_preced_op(&self) -> Option<&'arena Mul_preced_opContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_cast(&self) -> Option<&'arena Expr_castContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_mul(&self) -> Option<&'arena Expr_mulContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_mul_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_mulContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena Expr_castContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Expr_mulContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_mul_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_mulContext }
	fn get_rule_index(&self) -> usize { RULE_expr_mul }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_mul_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_mulContextAll::Expr_mul_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_mul_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_mulContextAll<'input, 'arena, Tok>) {
                Expr_mulContextAll::Expr_mul_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_mul_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_mulContextAll<'input, 'arena, Tok>) {
                Expr_mulContextAll::Expr_mul_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_mulContextAttrs<'input, 'arena, Tok> for Expr_mul_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_mul_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_mulContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_mulContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_mulContextAll::Expr_mul_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_mulContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_mul_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_mul_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_mul_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_cast(&self) -> Option<&'arena Expr_castContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_mul_passContextAttrs<'input, 'arena, Tok> for Expr_mul_passContext<'input, 'arena, Tok>
{
    fn expr_cast(&self) -> Option<&'arena Expr_castContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_mul_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_mulContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_mul_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_mulContext }
	fn get_rule_index(&self) -> usize { RULE_expr_mul }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_mul_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_mulContextAll::Expr_mul_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_mul_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_mulContextAll<'input, 'arena, Tok>) {
                Expr_mulContextAll::Expr_mul_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_mul_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_mulContextAll<'input, 'arena, Tok>) {
                Expr_mulContextAll::Expr_mul_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_mulContextAttrs<'input, 'arena, Tok> for Expr_mul_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_mul_passContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_mulContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_mulContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_mulContextAll::Expr_mul_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_mulContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr_mul(&mut self,) -> Result<&'arena Expr_mulContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_mulContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 64, RULE_expr_mul)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_mulContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(300);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(18,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_mul_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_mul_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_cast*/
					recog.base.set_state(295);
					recog.expr_cast()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_mul_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_mul_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_cast*/
					recog.base.set_state(296);
					let tmp = recog.expr_cast()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_mul_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					/*InvokeRule mul_preced_op*/
					recog.base.set_state(297);
					recog.mul_preced_op()?;
					/*InvokeRule expr_mul*/
					recog.base.set_state(298);
					let tmp = recog.expr_mul()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_mul_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
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
//------------------- expr_cast ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum Expr_castContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Expr_cast_passContext(Expr_cast_passContext<'input, 'arena, Tok>),
	Expr_cast_useContext(Expr_cast_useContext<'input, 'arena, Tok>),
	Expr_cast_refer_useContext(Expr_cast_refer_useContext<'input, 'arena, Tok>),
    Error(Expr_castContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Expr_castContextAll { } { Expr_cast_passContext, Expr_cast_useContext, Expr_cast_refer_useContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Expr_castContextAll { } { Expr_cast_passContext, Expr_cast_useContext, Expr_cast_refer_useContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Expr_castContextAll { Expr_cast_passContext, Expr_cast_useContext, Expr_cast_refer_useContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Expr_castContext::Expr_castContextAll { Expr_cast_passContext, Expr_cast_useContext, Expr_cast_refer_useContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Expr_castContextAll { Expr_cast_passContext(enter_expr_cast_pass, exit_expr_cast_pass), Expr_cast_useContext(enter_expr_cast_use, exit_expr_cast_use), Expr_cast_refer_useContext(enter_expr_cast_refer_use, exit_expr_cast_refer_use), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_castContextAll { Expr_cast_passContext(visit_expr_cast_pass), Expr_cast_useContext(visit_expr_cast_use), Expr_cast_refer_useContext(visit_expr_cast_refer_use), } }

impl<'input, 'arena, Tok> Deref for Expr_castContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Expr_castContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Expr_castContextAll::*;
		match self{
			Expr_cast_passContext(inner) => inner,
			Expr_cast_useContext(inner) => inner,
			Expr_cast_refer_useContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Expr_castContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_castContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Expr_castContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_castContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_castContext }
	fn get_rule_index(&self) -> usize { RULE_expr_cast }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_castContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_castContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_castContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_castContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_castContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_castContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_castContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_castContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_castContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> Expr_castContextAttrs<'input, 'arena, Tok> for Expr_castContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Expr_cast_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_cast_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_cast_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_unary(&self) -> Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cast_passContextAttrs<'input, 'arena, Tok> for Expr_cast_passContext<'input, 'arena, Tok>
{
    fn expr_unary(&self) -> Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_cast_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_castContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_cast_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_castContext }
	fn get_rule_index(&self) -> usize { RULE_expr_cast }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_cast_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_castContextAll::Expr_cast_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_cast_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_castContextAll<'input, 'arena, Tok>) {
                Expr_castContextAll::Expr_cast_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_cast_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_castContextAll<'input, 'arena, Tok>) {
                Expr_castContextAll::Expr_cast_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_castContextAttrs<'input, 'arena, Tok> for Expr_cast_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cast_passContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_castContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_castContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_castContextAll::Expr_cast_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_castContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_cast_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_cast_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_cast_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token KW_as
	/// Returns `None` if there is no child corresponding to token KW_as
	fn KW_as(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
	fn expr_unary(&self) -> Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>>;
	fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cast_useContextAttrs<'input, 'arena, Tok> for Expr_cast_useContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token KW_as
    /// Returns `None` if there is no child corresponding to token KW_as
    fn KW_as(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_KW_as)
    }
    fn expr_unary(&self) -> Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_cast_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_castContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_cast_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_castContext }
	fn get_rule_index(&self) -> usize { RULE_expr_cast }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_cast_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_castContextAll::Expr_cast_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_cast_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_castContextAll<'input, 'arena, Tok>) {
                Expr_castContextAll::Expr_cast_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_cast_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_castContextAll<'input, 'arena, Tok>) {
                Expr_castContextAll::Expr_cast_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_castContextAttrs<'input, 'arena, Tok> for Expr_cast_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cast_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_castContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_castContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_castContextAll::Expr_cast_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_castContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_cast_refer_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_cast_refer_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_cast_refer_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token KW_as
	/// Returns `None` if there is no child corresponding to token KW_as
	fn KW_as(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
	/// Retrieves first TerminalNode corresponding to token REFER
	/// Returns `None` if there is no child corresponding to token REFER
	fn REFER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
	fn expr_unary(&self) -> Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>>;
	fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cast_refer_useContextAttrs<'input, 'arena, Tok> for Expr_cast_refer_useContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token KW_as
    /// Returns `None` if there is no child corresponding to token KW_as
    fn KW_as(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_KW_as)
    }
    /// Retrieves first TerminalNode corresponding to token REFER
    /// Returns `None` if there is no child corresponding to token REFER
    fn REFER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_REFER)
    }
    fn expr_unary(&self) -> Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ty_kind(&self) -> Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_cast_refer_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_castContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Ty_kindContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_cast_refer_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_castContext }
	fn get_rule_index(&self) -> usize { RULE_expr_cast }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_cast_refer_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_castContextAll::Expr_cast_refer_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_cast_refer_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_castContextAll<'input, 'arena, Tok>) {
                Expr_castContextAll::Expr_cast_refer_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_cast_refer_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_castContextAll<'input, 'arena, Tok>) {
                Expr_castContextAll::Expr_cast_refer_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_castContextAttrs<'input, 'arena, Tok> for Expr_cast_refer_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_cast_refer_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_castContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_castContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_castContextAll::Expr_cast_refer_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_castContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr_cast(&mut self,) -> Result<&'arena Expr_castContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_castContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 66, RULE_expr_cast)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_castContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(312);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(19,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_cast_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_cast_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_unary*/
					recog.base.set_state(302);
					recog.expr_unary()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_cast_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_cast_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_unary*/
					recog.base.set_state(303);
					let tmp = recog.expr_unary()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_cast_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					recog.base.set_state(304);
					recog.base.match_token(CFood_KW_as,&mut recog.err_handler)?;
					/*InvokeRule ty_kind*/
					recog.base.set_state(305);
					let tmp = recog.ty_kind()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_cast_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_cast_refer_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(3);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_cast_refer_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule expr_unary*/
					recog.base.set_state(307);
					let tmp = recog.expr_unary()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_cast_refer_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					recog.base.set_state(308);
					recog.base.match_token(CFood_KW_as,&mut recog.err_handler)?;
					recog.base.set_state(309);
					recog.base.match_token(CFood_REFER,&mut recog.err_handler)?;
					/*InvokeRule ty_kind*/
					recog.base.set_state(310);
					let tmp = recog.ty_kind()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_cast_refer_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
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
//------------------- expr_unary ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum Expr_unaryContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Expr_unary_useContext(Expr_unary_useContext<'input, 'arena, Tok>),
	Expr_unary_passContext(Expr_unary_passContext<'input, 'arena, Tok>),
    Error(Expr_unaryContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Expr_unaryContextAll { } { Expr_unary_useContext, Expr_unary_passContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Expr_unaryContextAll { } { Expr_unary_useContext, Expr_unary_passContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Expr_unaryContextAll { Expr_unary_useContext, Expr_unary_passContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Expr_unaryContext::Expr_unaryContextAll { Expr_unary_useContext, Expr_unary_passContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Expr_unaryContextAll { Expr_unary_useContext(enter_expr_unary_use, exit_expr_unary_use), Expr_unary_passContext(enter_expr_unary_pass, exit_expr_unary_pass), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_unaryContextAll { Expr_unary_useContext(visit_expr_unary_use), Expr_unary_passContext(visit_expr_unary_pass), } }

impl<'input, 'arena, Tok> Deref for Expr_unaryContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Expr_unaryContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Expr_unaryContextAll::*;
		match self{
			Expr_unary_useContext(inner) => inner,
			Expr_unary_passContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Expr_unaryContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_unaryContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Expr_unaryContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_unaryContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_unaryContext }
	fn get_rule_index(&self) -> usize { RULE_expr_unary }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_unaryContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_unaryContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_unaryContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_unaryContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_unaryContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_unaryContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_unaryContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_unaryContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_unaryContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> Expr_unaryContextAttrs<'input, 'arena, Tok> for Expr_unaryContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Expr_unary_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_unary_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_unary_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn unary_preced_op(&self) -> Option<&'arena Unary_preced_opContextAll<'input, 'arena, Tok>>;
	fn expr_unary(&self) -> Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_unary_useContextAttrs<'input, 'arena, Tok> for Expr_unary_useContext<'input, 'arena, Tok>
{
    fn unary_preced_op(&self) -> Option<&'arena Unary_preced_opContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_unary(&self) -> Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_unary_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_unaryContextExt<'input, 'arena, Tok>,
	pub rhs: Option<&'arena Expr_unaryContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_unary_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_unaryContext }
	fn get_rule_index(&self) -> usize { RULE_expr_unary }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_unary_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_unaryContextAll::Expr_unary_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_unary_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_unaryContextAll<'input, 'arena, Tok>) {
                Expr_unaryContextAll::Expr_unary_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_unary_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_unaryContextAll<'input, 'arena, Tok>) {
                Expr_unaryContextAll::Expr_unary_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_unaryContextAttrs<'input, 'arena, Tok> for Expr_unary_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_unary_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_unaryContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_unaryContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_unaryContextAll::Expr_unary_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_unaryContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_unary_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_unary_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_unary_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_magic(&self) -> Option<&'arena Expr_magicContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_unary_passContextAttrs<'input, 'arena, Tok> for Expr_unary_passContext<'input, 'arena, Tok>
{
    fn expr_magic(&self) -> Option<&'arena Expr_magicContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_unary_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_unaryContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_unary_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_unaryContext }
	fn get_rule_index(&self) -> usize { RULE_expr_unary }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_unary_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_unaryContextAll::Expr_unary_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_unary_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_unaryContextAll<'input, 'arena, Tok>) {
                Expr_unaryContextAll::Expr_unary_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_unary_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_unaryContextAll<'input, 'arena, Tok>) {
                Expr_unaryContextAll::Expr_unary_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_unaryContextAttrs<'input, 'arena, Tok> for Expr_unary_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_unary_passContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_unaryContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_unaryContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_unaryContextAll::Expr_unary_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_unaryContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr_unary(&mut self,) -> Result<&'arena Expr_unaryContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_unaryContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 68, RULE_expr_unary)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_unaryContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(318);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_LIT_true |CFood_LIT_false |CFood_MAGIC_printf |CFood_MAGIC_scanf |
			    CFood_MAGIC_new |CFood_PAREN_L |CFood_REFER |CFood_IDENT |CFood_INT |
			    CFood_FLOAT |CFood_CONSTR  => {
			        /*------- Outer Most Alt 1 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Expr_unary_passContextExt::copy_from(ctx);
			            ctx.set_alt_number(1);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_unary_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule expr_magic*/
			        recog.base.set_state(314);
			        recog.expr_magic()?;
			        }}
			    CFood_NOT |CFood_PLUS |CFood_SUB  => {
			        /*------- Outer Most Alt 2 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Expr_unary_useContextExt::copy_from(ctx);
			            ctx.set_alt_number(2);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_unary_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule unary_preced_op*/
			        recog.base.set_state(315);
			        recog.unary_preced_op()?;
			        /*InvokeRule expr_unary*/
			        recog.base.set_state(316);
			        let tmp = recog.expr_unary()?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_unary_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
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
//------------------- expr_magic ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum Expr_magicContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Expr_magic_passContext(Expr_magic_passContext<'input, 'arena, Tok>),
	Expr_magic_useContext(Expr_magic_useContext<'input, 'arena, Tok>),
    Error(Expr_magicContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Expr_magicContextAll { } { Expr_magic_passContext, Expr_magic_useContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Expr_magicContextAll { } { Expr_magic_passContext, Expr_magic_useContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Expr_magicContextAll { Expr_magic_passContext, Expr_magic_useContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Expr_magicContext::Expr_magicContextAll { Expr_magic_passContext, Expr_magic_useContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Expr_magicContextAll { Expr_magic_passContext(enter_expr_magic_pass, exit_expr_magic_pass), Expr_magic_useContext(enter_expr_magic_use, exit_expr_magic_use), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_magicContextAll { Expr_magic_passContext(visit_expr_magic_pass), Expr_magic_useContext(visit_expr_magic_use), } }

impl<'input, 'arena, Tok> Deref for Expr_magicContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Expr_magicContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Expr_magicContextAll::*;
		match self{
			Expr_magic_passContext(inner) => inner,
			Expr_magic_useContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Expr_magicContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_magicContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Expr_magicContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_magicContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_magicContext }
	fn get_rule_index(&self) -> usize { RULE_expr_magic }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_magicContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_magicContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_magicContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_magicContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_magicContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_magicContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_magicContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_magicContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_magicContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> Expr_magicContextAttrs<'input, 'arena, Tok> for Expr_magicContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Expr_magic_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_magic_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_magic_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_call(&self) -> Option<&'arena Expr_callContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_magic_passContextAttrs<'input, 'arena, Tok> for Expr_magic_passContext<'input, 'arena, Tok>
{
    fn expr_call(&self) -> Option<&'arena Expr_callContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_magic_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_magicContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_magic_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_magicContext }
	fn get_rule_index(&self) -> usize { RULE_expr_magic }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_magic_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_magicContextAll::Expr_magic_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_magic_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_magicContextAll<'input, 'arena, Tok>) {
                Expr_magicContextAll::Expr_magic_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_magic_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_magicContextAll<'input, 'arena, Tok>) {
                Expr_magicContextAll::Expr_magic_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_magicContextAttrs<'input, 'arena, Tok> for Expr_magic_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_magic_passContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_magicContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_magicContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_magicContextAll::Expr_magic_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_magicContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_magic_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_magic_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_magic_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn magic(&self) -> Option<&'arena MagicContextAll<'input, 'arena, Tok>>;
	fn expr_magic(&self) -> Option<&'arena Expr_magicContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_magic_useContextAttrs<'input, 'arena, Tok> for Expr_magic_useContext<'input, 'arena, Tok>
{
    fn magic(&self) -> Option<&'arena MagicContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_magic(&self) -> Option<&'arena Expr_magicContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_magic_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_magicContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena MagicContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Expr_magicContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_magic_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_magicContext }
	fn get_rule_index(&self) -> usize { RULE_expr_magic }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_magic_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_magicContextAll::Expr_magic_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_magic_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_magicContextAll<'input, 'arena, Tok>) {
                Expr_magicContextAll::Expr_magic_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_magic_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_magicContextAll<'input, 'arena, Tok>) {
                Expr_magicContextAll::Expr_magic_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_magicContextAttrs<'input, 'arena, Tok> for Expr_magic_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_magic_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_magicContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_magicContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_magicContextAll::Expr_magic_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_magicContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr_magic(&mut self,) -> Result<&'arena Expr_magicContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_magicContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 70, RULE_expr_magic)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_magicContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(324);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_LIT_true |CFood_LIT_false |CFood_PAREN_L |CFood_REFER |CFood_IDENT |
			    CFood_INT |CFood_FLOAT |CFood_CONSTR  => {
			        /*------- Outer Most Alt 1 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Expr_magic_passContextExt::copy_from(ctx);
			            ctx.set_alt_number(1);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_magic_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule expr_call*/
			        recog.base.set_state(320);
			        recog.expr_call()?;
			        }}
			    CFood_MAGIC_printf |CFood_MAGIC_scanf |CFood_MAGIC_new  => {
			        /*------- Outer Most Alt 2 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Expr_magic_useContextExt::copy_from(ctx);
			            ctx.set_alt_number(2);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_magic_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule magic*/
			        recog.base.set_state(321);
			        let tmp = recog.magic()?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_magic_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
			        /*InvokeRule expr_magic*/
			        recog.base.set_state(322);
			        let tmp = recog.expr_magic()?;
			        unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_magic_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
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
//------------------- expr_call ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum Expr_callContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Expr_call_passContext(Expr_call_passContext<'input, 'arena, Tok>),
	Expr_call_useContext(Expr_call_useContext<'input, 'arena, Tok>),
    Error(Expr_callContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Expr_callContextAll { } { Expr_call_passContext, Expr_call_useContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Expr_callContextAll { } { Expr_call_passContext, Expr_call_useContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Expr_callContextAll { Expr_call_passContext, Expr_call_useContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Expr_callContext::Expr_callContextAll { Expr_call_passContext, Expr_call_useContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Expr_callContextAll { Expr_call_passContext(enter_expr_call_pass, exit_expr_call_pass), Expr_call_useContext(enter_expr_call_use, exit_expr_call_use), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Expr_callContextAll { Expr_call_passContext(visit_expr_call_pass), Expr_call_useContext(visit_expr_call_use), } }

impl<'input, 'arena, Tok> Deref for Expr_callContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Expr_callContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Expr_callContextAll::*;
		match self{
			Expr_call_passContext(inner) => inner,
			Expr_call_useContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Expr_callContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_callContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Expr_callContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Expr_callContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_callContext }
	fn get_rule_index(&self) -> usize { RULE_expr_call }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_callContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_callContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_callContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Expr_callContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_callContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Expr_callContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_callContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Expr_callContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Expr_callContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> Expr_callContextAttrs<'input, 'arena, Tok> for Expr_callContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Expr_call_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_call_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_call_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_call_passContextAttrs<'input, 'arena, Tok> for Expr_call_passContext<'input, 'arena, Tok>
{
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_call_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_callContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_call_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_callContext }
	fn get_rule_index(&self) -> usize { RULE_expr_call }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_call_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_callContextAll::Expr_call_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_call_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_callContextAll<'input, 'arena, Tok>) {
                Expr_callContextAll::Expr_call_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_call_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_callContextAll<'input, 'arena, Tok>) {
                Expr_callContextAll::Expr_call_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_callContextAttrs<'input, 'arena, Tok> for Expr_call_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_call_passContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_callContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_callContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_callContextAll::Expr_call_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_callContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Expr_call_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Expr_call_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Expr_call_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>>;
	fn expr_call(&self) -> Option<&'arena Expr_callContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_call_useContextAttrs<'input, 'arena, Tok> for Expr_call_useContext<'input, 'arena, Tok>
{
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_call(&self) -> Option<&'arena Expr_callContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Expr_call_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Expr_callContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena AtomContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Expr_callContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Expr_call_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Expr_callContext }
	fn get_rule_index(&self) -> usize { RULE_expr_call }
    fn make_node(
        arena: &'arena Arena,
        ctx: Expr_call_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Expr_callContextAll::Expr_call_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Expr_call_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Expr_callContextAll<'input, 'arena, Tok>) {
                Expr_callContextAll::Expr_call_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Expr_call_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Expr_callContextAll<'input, 'arena, Tok>) {
                Expr_callContextAll::Expr_call_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Expr_callContextAttrs<'input, 'arena, Tok> for Expr_call_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Expr_call_useContextExt<'input, 'arena, Tok> {
	fn new(base: Expr_callContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Expr_callContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Expr_callContextAll::Expr_call_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Expr_callContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr_call(&mut self,) -> Result<&'arena Expr_callContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_callContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 72, RULE_expr_call)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_callContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(330);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(22,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_call_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_call_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule atom*/
					recog.base.set_state(326);
					recog.atom()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Expr_call_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Expr_call_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule atom*/
					recog.base.set_state(327);
					let tmp = recog.atom()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_call_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					/*InvokeRule expr_call*/
					recog.base.set_state(328);
					let tmp = recog.expr_call()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Expr_call_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
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
//------------------- atom ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum AtomContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Atom_referContext(Atom_referContext<'input, 'arena, Tok>),
	Atom_varContext(Atom_varContext<'input, 'arena, Tok>),
	Atom_litContext(Atom_litContext<'input, 'arena, Tok>),
	Atom_apply_listContext(Atom_apply_listContext<'input, 'arena, Tok>),
    Error(AtomContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { AtomContextAll { } { Atom_referContext, Atom_varContext, Atom_litContext, Atom_apply_listContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { AtomContextAll { } { Atom_referContext, Atom_varContext, Atom_litContext, Atom_apply_listContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::AtomContextAll { Atom_referContext, Atom_varContext, Atom_litContext, Atom_apply_listContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::AtomContext::AtomContextAll { Atom_referContext, Atom_varContext, Atom_litContext, Atom_apply_listContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::AtomContextAll { Atom_referContext(enter_atom_refer, exit_atom_refer), Atom_varContext(enter_atom_var, exit_atom_var), Atom_litContext(enter_atom_lit, exit_atom_lit), Atom_apply_listContext(enter_atom_apply_list, exit_atom_apply_list), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::AtomContextAll { Atom_referContext(visit_atom_refer), Atom_varContext(visit_atom_var), Atom_litContext(visit_atom_lit), Atom_apply_listContext(visit_atom_apply_list), } }

impl<'input, 'arena, Tok> Deref for AtomContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn AtomContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use AtomContextAll::*;
		match self{
			Atom_referContext(inner) => inner,
			Atom_varContext(inner) => inner,
			Atom_litContext(inner) => inner,
			Atom_apply_listContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type AtomContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AtomContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct AtomContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for AtomContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::AtomContext }
	fn get_rule_index(&self) -> usize { RULE_atom }
    fn make_node(
        arena: &'arena Arena,
        ctx: AtomContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(AtomContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a AtomContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => AtomContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut AtomContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut AtomContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AtomContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, AtomContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait AtomContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> AtomContextAttrs<'input, 'arena, Tok> for AtomContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Atom_referContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Atom_referContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Atom_referContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn refer(&self) -> Option<&'arena ReferContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_referContextAttrs<'input, 'arena, Tok> for Atom_referContext<'input, 'arena, Tok>
{
    fn refer(&self) -> Option<&'arena ReferContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Atom_referContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: AtomContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Atom_referContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::AtomContext }
	fn get_rule_index(&self) -> usize { RULE_atom }
    fn make_node(
        arena: &'arena Arena,
        ctx: Atom_referContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(AtomContextAll::Atom_referContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Atom_referContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => AtomContextAll<'input, 'arena, Tok>) {
                AtomContextAll::Atom_referContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Atom_referContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut AtomContextAll<'input, 'arena, Tok>) {
                AtomContextAll::Atom_referContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> AtomContextAttrs<'input, 'arena, Tok> for Atom_referContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_referContextExt<'input, 'arena, Tok> {
	fn new(base: AtomContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut AtomContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            AtomContextAll::Atom_referContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut AtomContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Atom_varContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Atom_varContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Atom_varContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_varContextAttrs<'input, 'arena, Tok> for Atom_varContext<'input, 'arena, Tok>
{
    fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Atom_varContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: AtomContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Atom_varContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::AtomContext }
	fn get_rule_index(&self) -> usize { RULE_atom }
    fn make_node(
        arena: &'arena Arena,
        ctx: Atom_varContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(AtomContextAll::Atom_varContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Atom_varContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => AtomContextAll<'input, 'arena, Tok>) {
                AtomContextAll::Atom_varContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Atom_varContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut AtomContextAll<'input, 'arena, Tok>) {
                AtomContextAll::Atom_varContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> AtomContextAttrs<'input, 'arena, Tok> for Atom_varContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_varContextExt<'input, 'arena, Tok> {
	fn new(base: AtomContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut AtomContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            AtomContextAll::Atom_varContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut AtomContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Atom_litContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Atom_litContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Atom_litContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn lit(&self) -> Option<&'arena LitContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_litContextAttrs<'input, 'arena, Tok> for Atom_litContext<'input, 'arena, Tok>
{
    fn lit(&self) -> Option<&'arena LitContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Atom_litContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: AtomContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Atom_litContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::AtomContext }
	fn get_rule_index(&self) -> usize { RULE_atom }
    fn make_node(
        arena: &'arena Arena,
        ctx: Atom_litContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(AtomContextAll::Atom_litContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Atom_litContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => AtomContextAll<'input, 'arena, Tok>) {
                AtomContextAll::Atom_litContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Atom_litContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut AtomContextAll<'input, 'arena, Tok>) {
                AtomContextAll::Atom_litContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> AtomContextAttrs<'input, 'arena, Tok> for Atom_litContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_litContextExt<'input, 'arena, Tok> {
	fn new(base: AtomContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut AtomContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            AtomContextAll::Atom_litContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut AtomContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Atom_apply_listContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Atom_apply_listContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Atom_apply_listContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn apply_list(&self) -> Option<&'arena Apply_listContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_apply_listContextAttrs<'input, 'arena, Tok> for Atom_apply_listContext<'input, 'arena, Tok>
{
    fn apply_list(&self) -> Option<&'arena Apply_listContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Atom_apply_listContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: AtomContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Atom_apply_listContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::AtomContext }
	fn get_rule_index(&self) -> usize { RULE_atom }
    fn make_node(
        arena: &'arena Arena,
        ctx: Atom_apply_listContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(AtomContextAll::Atom_apply_listContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Atom_apply_listContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => AtomContextAll<'input, 'arena, Tok>) {
                AtomContextAll::Atom_apply_listContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Atom_apply_listContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut AtomContextAll<'input, 'arena, Tok>) {
                AtomContextAll::Atom_apply_listContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> AtomContextAttrs<'input, 'arena, Tok> for Atom_apply_listContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_apply_listContextExt<'input, 'arena, Tok> {
	fn new(base: AtomContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut AtomContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            AtomContextAll::Atom_apply_listContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut AtomContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn atom(&mut self,) -> Result<&'arena AtomContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(AtomContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 74, RULE_atom)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena AtomContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(336);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_PAREN_L  => {
			        /*------- Outer Most Alt 1 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Atom_apply_listContextExt::copy_from(ctx);
			            ctx.set_alt_number(1);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Atom_apply_listContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule apply_list*/
			        recog.base.set_state(332);
			        recog.apply_list()?;
			        }}
			    CFood_IDENT  => {
			        /*------- Outer Most Alt 2 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Atom_varContextExt::copy_from(ctx);
			            ctx.set_alt_number(2);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Atom_varContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule var*/
			        recog.base.set_state(333);
			        recog.var()?;
			        }}
			    CFood_REFER  => {
			        /*------- Outer Most Alt 3 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Atom_referContextExt::copy_from(ctx);
			            ctx.set_alt_number(3);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Atom_referContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule refer*/
			        recog.base.set_state(334);
			        recog.refer()?;
			        }}
			    CFood_LIT_true |CFood_LIT_false |CFood_INT |CFood_FLOAT |CFood_CONSTR  => {
			        /*------- Outer Most Alt 4 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Atom_litContextExt::copy_from(ctx);
			            ctx.set_alt_number(4);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Atom_litContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule lit*/
			        recog.base.set_state(335);
			        recog.lit()?;
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
//------------------- magic ----------------
pub type MagicContextAll<'input, 'arena, Tok = CommonToken<'input>> = MagicContext<'input, 'arena, Tok>;

pub type MagicContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, MagicContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::MagicContext(visit_magic) }
#[derive(Debug)]
pub struct MagicContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for MagicContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::MagicContext }
	fn get_rule_index(&self) -> usize { RULE_magic }
    fn make_node(
        arena: &'arena Arena,
        ctx: MagicContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a MagicContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => MagicContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut MagicContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut MagicContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> MagicContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, MagicContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait MagicContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token MAGIC_printf
    /// Returns `None` if there is no child corresponding to token MAGIC_printf
    fn MAGIC_printf(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token MAGIC_scanf
    /// Returns `None` if there is no child corresponding to token MAGIC_scanf
    fn MAGIC_scanf(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token MAGIC_new
    /// Returns `None` if there is no child corresponding to token MAGIC_new
    fn MAGIC_new(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> MagicContextAttrs<'input, 'arena, Tok> for MagicContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token MAGIC_printf
    /// Returns `None` if there is no child corresponding to token MAGIC_printf
    fn MAGIC_printf(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_MAGIC_printf)
    }
    /// Retrieves first TerminalNode corresponding to token MAGIC_scanf
    /// Returns `None` if there is no child corresponding to token MAGIC_scanf
    fn MAGIC_scanf(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_MAGIC_scanf)
    }
    /// Retrieves first TerminalNode corresponding to token MAGIC_new
    /// Returns `None` if there is no child corresponding to token MAGIC_new
    fn MAGIC_new(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_MAGIC_new)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn magic(&mut self,) -> Result<&'arena MagicContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(MagicContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 76, RULE_magic)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena MagicContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(338);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 458752) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;
			}
			else {
				if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler)?;
			}
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
//------------------- logic_preced_op ----------------
pub type Logic_preced_opContextAll<'input, 'arena, Tok = CommonToken<'input>> = Logic_preced_opContext<'input, 'arena, Tok>;

pub type Logic_preced_opContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Logic_preced_opContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::Logic_preced_opContext(visit_logic_preced_op) }
#[derive(Debug)]
pub struct Logic_preced_opContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Logic_preced_opContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Logic_preced_opContext }
	fn get_rule_index(&self) -> usize { RULE_logic_preced_op }
    fn make_node(
        arena: &'arena Arena,
        ctx: Logic_preced_opContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Logic_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Logic_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Logic_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Logic_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Logic_preced_opContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Logic_preced_opContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Logic_preced_opContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token AND
    /// Returns `None` if there is no child corresponding to token AND
    fn AND(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token OR
    /// Returns `None` if there is no child corresponding to token OR
    fn OR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Logic_preced_opContextAttrs<'input, 'arena, Tok> for Logic_preced_opContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token AND
    /// Returns `None` if there is no child corresponding to token AND
    fn AND(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_AND)
    }
    /// Retrieves first TerminalNode corresponding to token OR
    /// Returns `None` if there is no child corresponding to token OR
    fn OR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_OR)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn logic_preced_op(&mut self,) -> Result<&'arena Logic_preced_opContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Logic_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 78, RULE_logic_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Logic_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(340);
			_la = recog.base.input.la(1);
			if { !(_la==CFood_AND || _la==CFood_OR) } {
				recog.err_handler.recover_inline(&mut recog.base)?;
			}
			else {
				if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler)?;
			}
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Cmp_preced_opContext(visit_cmp_preced_op) }
#[derive(Debug)]
pub struct Cmp_preced_opContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Cmp_preced_opContextExt {
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
        recog.base.enter_rule(Cmp_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 80, RULE_cmp_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Cmp_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(342);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 528482304) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;
			}
			else {
				if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler)?;
			}
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Add_preced_opContext(visit_add_preced_op) }
#[derive(Debug)]
pub struct Add_preced_opContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Add_preced_opContextExt {
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
        recog.base.enter_rule(Add_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 82, RULE_add_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Add_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(344);
			_la = recog.base.input.la(1);
			if { !(_la==CFood_PLUS || _la==CFood_SUB) } {
				recog.err_handler.recover_inline(&mut recog.base)?;
			}
			else {
				if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler)?;
			}
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Mul_preced_opContext(visit_mul_preced_op) }
#[derive(Debug)]
pub struct Mul_preced_opContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Mul_preced_opContextExt {
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
    /// Retrieves first TerminalNode corresponding to token MUL
    /// Returns `None` if there is no child corresponding to token MUL
    fn MUL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token DIV
    /// Returns `None` if there is no child corresponding to token DIV
    fn DIV(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token MOD
    /// Returns `None` if there is no child corresponding to token MOD
    fn MOD(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PEO
    /// Returns `None` if there is no child corresponding to token PEO
    fn PEO(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Mul_preced_opContextAttrs<'input, 'arena, Tok> for Mul_preced_opContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
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
    /// Retrieves first TerminalNode corresponding to token PEO
    /// Returns `None` if there is no child corresponding to token PEO
    fn PEO(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PEO)
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
        recog.base.enter_rule(Mul_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 84, RULE_mul_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Mul_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(346);
			_la = recog.base.input.la(1);
			if { !(((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & 15) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;
			}
			else {
				if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler)?;
			}
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
//------------------- unary_preced_op ----------------
pub type Unary_preced_opContextAll<'input, 'arena, Tok = CommonToken<'input>> = Unary_preced_opContext<'input, 'arena, Tok>;

pub type Unary_preced_opContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Unary_preced_opContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::Unary_preced_opContext(visit_unary_preced_op) }
#[derive(Debug)]
pub struct Unary_preced_opContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Unary_preced_opContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Unary_preced_opContext }
	fn get_rule_index(&self) -> usize { RULE_unary_preced_op }
    fn make_node(
        arena: &'arena Arena,
        ctx: Unary_preced_opContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Unary_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => Unary_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Unary_preced_opContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut Unary_preced_opContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Unary_preced_opContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena CFoodParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut CFoodParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Unary_preced_opContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Unary_preced_opContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SUB
    /// Returns `None` if there is no child corresponding to token SUB
    fn SUB(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token NOT
    /// Returns `None` if there is no child corresponding to token NOT
    fn NOT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Unary_preced_opContextAttrs<'input, 'arena, Tok> for Unary_preced_opContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
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
    /// Retrieves first TerminalNode corresponding to token NOT
    /// Returns `None` if there is no child corresponding to token NOT
    fn NOT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_NOT)
    }
}

impl<'input, 'arena, Input, TF> CFoodParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn unary_preced_op(&mut self,) -> Result<&'arena Unary_preced_opContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Unary_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 86, RULE_unary_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Unary_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(348);
			_la = recog.base.input.la(1);
			if { !(((((_la - 29)) & !0x3f) == 0 && ((1usize << (_la - 29)) & 25) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;
			}
			else {
				if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler)?;
			}
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Apply_listContext(visit_apply_list) }
#[derive(Debug)]
pub struct Apply_listContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Apply_listContextExt {
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
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn args(&self) -> Option<&'arena ArgsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Apply_listContextAttrs<'input, 'arena, Tok> for Apply_listContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_L)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_PAREN_R)
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
	pub fn apply_list(&mut self,) -> Result<&'arena Apply_listContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Apply_listContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 88, RULE_apply_list)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Apply_listContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(356);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(24,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(350);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					recog.base.set_state(351);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(352);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule args*/
					recog.base.set_state(353);
					recog.args()?;
					recog.base.set_state(354);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
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
//------------------- args ----------------
pub type ArgsContextAll<'input, 'arena, Tok = CommonToken<'input>> = ArgsContext<'input, 'arena, Tok>;

pub type ArgsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ArgsContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::ArgsContext(visit_args) }
#[derive(Debug)]
pub struct ArgsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, ArgsContextExt {
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
        recog.base.enter_rule(ArgsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 90, RULE_args)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ArgsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(364);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(25,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule expr*/
					recog.base.set_state(358);
					recog.expr()?;
					recog.base.set_state(359);
					recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					/*InvokeRule args*/
					recog.base.set_state(360);
					recog.args()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule expr*/
					recog.base.set_state(362);
					recog.expr()?;
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
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
    4, 1, 49, 367, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 
    7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 
    10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 
    15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 
    20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 
    25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 
    30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 
    35, 7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 39, 2, 
    40, 7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 44, 2, 
    45, 7, 45, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 99, 8, 1, 1, 2, 
    1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 106, 8, 2, 1, 3, 1, 3, 1, 3, 1, 4, 1, 
    4, 1, 4, 1, 5, 1, 5, 1, 5, 3, 5, 117, 8, 5, 1, 6, 1, 6, 1, 6, 1, 6, 
    1, 6, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 8, 1, 8, 1, 
    8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 3, 8, 141, 8, 8, 1, 9, 1, 9, 
    1, 9, 1, 9, 1, 9, 3, 9, 148, 8, 9, 1, 10, 1, 10, 1, 11, 1, 11, 1, 11, 
    1, 11, 1, 11, 3, 11, 157, 8, 11, 1, 12, 1, 12, 1, 12, 1, 12, 1, 12, 
    3, 12, 164, 8, 12, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 3, 13, 
    172, 8, 13, 1, 14, 1, 14, 1, 14, 1, 14, 1, 15, 1, 15, 1, 15, 1, 15, 
    3, 15, 182, 8, 15, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 
    1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 
    3, 16, 201, 8, 16, 1, 17, 1, 17, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 
    1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 3, 18, 
    219, 8, 18, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 20, 1, 20, 
    1, 20, 1, 20, 1, 20, 3, 20, 232, 8, 20, 1, 21, 1, 21, 1, 21, 1, 21, 
    1, 21, 3, 21, 239, 8, 21, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 
    1, 22, 1, 22, 1, 22, 1, 22, 1, 23, 1, 23, 1, 23, 3, 23, 254, 8, 23, 
    1, 24, 1, 24, 1, 24, 1, 24, 1, 24, 1, 25, 1, 25, 1, 26, 1, 26, 1, 26, 
    1, 27, 1, 27, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 3, 28, 273, 8, 28, 
    1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 3, 29, 280, 8, 29, 1, 30, 1, 30, 
    1, 30, 1, 30, 1, 30, 3, 30, 287, 8, 30, 1, 31, 1, 31, 1, 31, 1, 31, 
    1, 31, 3, 31, 294, 8, 31, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 3, 32, 
    301, 8, 32, 1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 1, 33, 
    1, 33, 1, 33, 3, 33, 313, 8, 33, 1, 34, 1, 34, 1, 34, 1, 34, 3, 34, 
    319, 8, 34, 1, 35, 1, 35, 1, 35, 1, 35, 3, 35, 325, 8, 35, 1, 36, 1, 
    36, 1, 36, 1, 36, 3, 36, 331, 8, 36, 1, 37, 1, 37, 1, 37, 1, 37, 3, 
    37, 337, 8, 37, 1, 38, 1, 38, 1, 39, 1, 39, 1, 40, 1, 40, 1, 41, 1, 
    41, 1, 42, 1, 42, 1, 43, 1, 43, 1, 44, 1, 44, 1, 44, 1, 44, 1, 44, 1, 
    44, 3, 44, 357, 8, 44, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 3, 
    45, 365, 8, 45, 1, 45, 0, 0, 46, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 
    20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 
    54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 
    88, 90, 0, 6, 1, 0, 16, 18, 1, 0, 30, 31, 1, 0, 23, 28, 1, 0, 32, 33, 
    1, 0, 34, 37, 2, 0, 29, 29, 32, 33, 369, 0, 92, 1, 0, 0, 0, 2, 98, 1, 
    0, 0, 0, 4, 105, 1, 0, 0, 0, 6, 107, 1, 0, 0, 0, 8, 110, 1, 0, 0, 0, 
    10, 116, 1, 0, 0, 0, 12, 118, 1, 0, 0, 0, 14, 123, 1, 0, 0, 0, 16, 140, 
    1, 0, 0, 0, 18, 147, 1, 0, 0, 0, 20, 149, 1, 0, 0, 0, 22, 156, 1, 0, 
    0, 0, 24, 163, 1, 0, 0, 0, 26, 171, 1, 0, 0, 0, 28, 173, 1, 0, 0, 0, 
    30, 181, 1, 0, 0, 0, 32, 200, 1, 0, 0, 0, 34, 202, 1, 0, 0, 0, 36, 218, 
    1, 0, 0, 0, 38, 220, 1, 0, 0, 0, 40, 231, 1, 0, 0, 0, 42, 238, 1, 0, 
    0, 0, 44, 240, 1, 0, 0, 0, 46, 253, 1, 0, 0, 0, 48, 255, 1, 0, 0, 0, 
    50, 260, 1, 0, 0, 0, 52, 262, 1, 0, 0, 0, 54, 265, 1, 0, 0, 0, 56, 272, 
    1, 0, 0, 0, 58, 279, 1, 0, 0, 0, 60, 286, 1, 0, 0, 0, 62, 293, 1, 0, 
    0, 0, 64, 300, 1, 0, 0, 0, 66, 312, 1, 0, 0, 0, 68, 318, 1, 0, 0, 0, 
    70, 324, 1, 0, 0, 0, 72, 330, 1, 0, 0, 0, 74, 336, 1, 0, 0, 0, 76, 338, 
    1, 0, 0, 0, 78, 340, 1, 0, 0, 0, 80, 342, 1, 0, 0, 0, 82, 344, 1, 0, 
    0, 0, 84, 346, 1, 0, 0, 0, 86, 348, 1, 0, 0, 0, 88, 356, 1, 0, 0, 0, 
    90, 364, 1, 0, 0, 0, 92, 93, 3, 2, 1, 0, 93, 1, 1, 0, 0, 0, 94, 95, 
    3, 4, 2, 0, 95, 96, 3, 2, 1, 0, 96, 99, 1, 0, 0, 0, 97, 99, 1, 0, 0, 
    0, 98, 94, 1, 0, 0, 0, 98, 97, 1, 0, 0, 0, 99, 3, 1, 0, 0, 0, 100, 101, 
    3, 6, 3, 0, 101, 102, 5, 41, 0, 0, 102, 106, 1, 0, 0, 0, 103, 106, 3, 
    14, 7, 0, 104, 106, 3, 12, 6, 0, 105, 100, 1, 0, 0, 0, 105, 103, 1, 
    0, 0, 0, 105, 104, 1, 0, 0, 0, 106, 5, 1, 0, 0, 0, 107, 108, 3, 8, 4, 
    0, 108, 109, 3, 10, 5, 0, 109, 7, 1, 0, 0, 0, 110, 111, 3, 26, 13, 0, 
    111, 112, 5, 43, 0, 0, 112, 9, 1, 0, 0, 0, 113, 114, 5, 38, 0, 0, 114, 
    117, 3, 54, 27, 0, 115, 117, 1, 0, 0, 0, 116, 113, 1, 0, 0, 0, 116, 
    115, 1, 0, 0, 0, 117, 11, 1, 0, 0, 0, 118, 119, 3, 26, 13, 0, 119, 120, 
    5, 43, 0, 0, 120, 121, 3, 16, 8, 0, 121, 122, 3, 28, 14, 0, 122, 13, 
    1, 0, 0, 0, 123, 124, 5, 6, 0, 0, 124, 125, 5, 42, 0, 0, 125, 126, 5, 
    38, 0, 0, 126, 127, 5, 19, 0, 0, 127, 128, 3, 24, 12, 0, 128, 129, 5, 
    20, 0, 0, 129, 130, 5, 41, 0, 0, 130, 15, 1, 0, 0, 0, 131, 132, 5, 19, 
    0, 0, 132, 133, 3, 18, 9, 0, 133, 134, 5, 20, 0, 0, 134, 141, 1, 0, 
    0, 0, 135, 136, 5, 19, 0, 0, 136, 137, 5, 14, 0, 0, 137, 141, 5, 20, 
    0, 0, 138, 139, 5, 19, 0, 0, 139, 141, 5, 20, 0, 0, 140, 131, 1, 0, 
    0, 0, 140, 135, 1, 0, 0, 0, 140, 138, 1, 0, 0, 0, 141, 17, 1, 0, 0, 
    0, 142, 143, 3, 20, 10, 0, 143, 144, 5, 39, 0, 0, 144, 145, 3, 18, 9, 
    0, 145, 148, 1, 0, 0, 0, 146, 148, 3, 20, 10, 0, 147, 142, 1, 0, 0, 
    0, 147, 146, 1, 0, 0, 0, 148, 19, 1, 0, 0, 0, 149, 150, 3, 8, 4, 0, 
    150, 21, 1, 0, 0, 0, 151, 157, 5, 44, 0, 0, 152, 157, 5, 45, 0, 0, 153, 
    157, 5, 46, 0, 0, 154, 157, 5, 9, 0, 0, 155, 157, 5, 10, 0, 0, 156, 
    151, 1, 0, 0, 0, 156, 152, 1, 0, 0, 0, 156, 153, 1, 0, 0, 0, 156, 154, 
    1, 0, 0, 0, 156, 155, 1, 0, 0, 0, 157, 23, 1, 0, 0, 0, 158, 159, 3, 
    26, 13, 0, 159, 160, 5, 39, 0, 0, 160, 161, 3, 24, 12, 0, 161, 164, 
    1, 0, 0, 0, 162, 164, 3, 26, 13, 0, 163, 158, 1, 0, 0, 0, 163, 162, 
    1, 0, 0, 0, 164, 25, 1, 0, 0, 0, 165, 172, 5, 11, 0, 0, 166, 172, 5, 
    12, 0, 0, 167, 172, 5, 13, 0, 0, 168, 172, 5, 14, 0, 0, 169, 172, 5, 
    15, 0, 0, 170, 172, 5, 42, 0, 0, 171, 165, 1, 0, 0, 0, 171, 166, 1, 
    0, 0, 0, 171, 167, 1, 0, 0, 0, 171, 168, 1, 0, 0, 0, 171, 169, 1, 0, 
    0, 0, 171, 170, 1, 0, 0, 0, 172, 27, 1, 0, 0, 0, 173, 174, 5, 21, 0, 
    0, 174, 175, 3, 30, 15, 0, 175, 176, 5, 22, 0, 0, 176, 29, 1, 0, 0, 
    0, 177, 178, 3, 32, 16, 0, 178, 179, 3, 30, 15, 0, 179, 182, 1, 0, 0, 
    0, 180, 182, 1, 0, 0, 0, 181, 177, 1, 0, 0, 0, 181, 180, 1, 0, 0, 0, 
    182, 31, 1, 0, 0, 0, 183, 201, 3, 36, 18, 0, 184, 201, 3, 38, 19, 0, 
    185, 201, 3, 44, 22, 0, 186, 201, 3, 28, 14, 0, 187, 188, 3, 6, 3, 0, 
    188, 189, 5, 41, 0, 0, 189, 201, 1, 0, 0, 0, 190, 191, 3, 34, 17, 0, 
    191, 192, 5, 41, 0, 0, 192, 201, 1, 0, 0, 0, 193, 194, 3, 48, 24, 0, 
    194, 195, 5, 41, 0, 0, 195, 201, 1, 0, 0, 0, 196, 197, 3, 46, 23, 0, 
    197, 198, 5, 41, 0, 0, 198, 201, 1, 0, 0, 0, 199, 201, 5, 41, 0, 0, 
    200, 183, 1, 0, 0, 0, 200, 184, 1, 0, 0, 0, 200, 185, 1, 0, 0, 0, 200, 
    186, 1, 0, 0, 0, 200, 187, 1, 0, 0, 0, 200, 190, 1, 0, 0, 0, 200, 193, 
    1, 0, 0, 0, 200, 196, 1, 0, 0, 0, 200, 199, 1, 0, 0, 0, 201, 33, 1, 
    0, 0, 0, 202, 203, 3, 54, 27, 0, 203, 35, 1, 0, 0, 0, 204, 205, 5, 3, 
    0, 0, 205, 206, 5, 19, 0, 0, 206, 207, 3, 54, 27, 0, 207, 208, 5, 20, 
    0, 0, 208, 209, 3, 32, 16, 0, 209, 219, 1, 0, 0, 0, 210, 211, 5, 3, 
    0, 0, 211, 212, 5, 19, 0, 0, 212, 213, 3, 54, 27, 0, 213, 214, 5, 20, 
    0, 0, 214, 215, 3, 32, 16, 0, 215, 216, 5, 4, 0, 0, 216, 217, 3, 32, 
    16, 0, 217, 219, 1, 0, 0, 0, 218, 204, 1, 0, 0, 0, 218, 210, 1, 0, 0, 
    0, 219, 37, 1, 0, 0, 0, 220, 221, 5, 1, 0, 0, 221, 222, 5, 19, 0, 0, 
    222, 223, 3, 54, 27, 0, 223, 224, 5, 20, 0, 0, 224, 225, 3, 32, 16, 
    0, 225, 39, 1, 0, 0, 0, 226, 232, 3, 42, 21, 0, 227, 228, 3, 42, 21, 
    0, 228, 229, 5, 39, 0, 0, 229, 230, 3, 40, 20, 0, 230, 232, 1, 0, 0, 
    0, 231, 226, 1, 0, 0, 0, 231, 227, 1, 0, 0, 0, 232, 41, 1, 0, 0, 0, 
    233, 239, 3, 6, 3, 0, 234, 239, 3, 34, 17, 0, 235, 239, 3, 48, 24, 0, 
    236, 239, 3, 46, 23, 0, 237, 239, 1, 0, 0, 0, 238, 233, 1, 0, 0, 0, 
    238, 234, 1, 0, 0, 0, 238, 235, 1, 0, 0, 0, 238, 236, 1, 0, 0, 0, 238, 
    237, 1, 0, 0, 0, 239, 43, 1, 0, 0, 0, 240, 241, 5, 2, 0, 0, 241, 242, 
    5, 19, 0, 0, 242, 243, 3, 40, 20, 0, 243, 244, 5, 41, 0, 0, 244, 245, 
    3, 54, 27, 0, 245, 246, 5, 41, 0, 0, 246, 247, 3, 40, 20, 0, 247, 248, 
    5, 20, 0, 0, 248, 249, 3, 32, 16, 0, 249, 45, 1, 0, 0, 0, 250, 254, 
    5, 5, 0, 0, 251, 252, 5, 5, 0, 0, 252, 254, 3, 54, 27, 0, 253, 250, 
    1, 0, 0, 0, 253, 251, 1, 0, 0, 0, 254, 47, 1, 0, 0, 0, 255, 256, 5, 
    7, 0, 0, 256, 257, 5, 43, 0, 0, 257, 258, 5, 38, 0, 0, 258, 259, 3, 
    54, 27, 0, 259, 49, 1, 0, 0, 0, 260, 261, 5, 43, 0, 0, 261, 51, 1, 0, 
    0, 0, 262, 263, 5, 40, 0, 0, 263, 264, 5, 43, 0, 0, 264, 53, 1, 0, 0, 
    0, 265, 266, 3, 56, 28, 0, 266, 55, 1, 0, 0, 0, 267, 273, 3, 58, 29, 
    0, 268, 269, 3, 50, 25, 0, 269, 270, 5, 38, 0, 0, 270, 271, 3, 56, 28, 
    0, 271, 273, 1, 0, 0, 0, 272, 267, 1, 0, 0, 0, 272, 268, 1, 0, 0, 0, 
    273, 57, 1, 0, 0, 0, 274, 280, 3, 60, 30, 0, 275, 276, 3, 60, 30, 0, 
    276, 277, 3, 78, 39, 0, 277, 278, 3, 58, 29, 0, 278, 280, 1, 0, 0, 0, 
    279, 274, 1, 0, 0, 0, 279, 275, 1, 0, 0, 0, 280, 59, 1, 0, 0, 0, 281, 
    287, 3, 62, 31, 0, 282, 283, 3, 62, 31, 0, 283, 284, 3, 80, 40, 0, 284, 
    285, 3, 60, 30, 0, 285, 287, 1, 0, 0, 0, 286, 281, 1, 0, 0, 0, 286, 
    282, 1, 0, 0, 0, 287, 61, 1, 0, 0, 0, 288, 294, 3, 64, 32, 0, 289, 290, 
    3, 64, 32, 0, 290, 291, 3, 82, 41, 0, 291, 292, 3, 62, 31, 0, 292, 294, 
    1, 0, 0, 0, 293, 288, 1, 0, 0, 0, 293, 289, 1, 0, 0, 0, 294, 63, 1, 
    0, 0, 0, 295, 301, 3, 66, 33, 0, 296, 297, 3, 66, 33, 0, 297, 298, 3, 
    84, 42, 0, 298, 299, 3, 64, 32, 0, 299, 301, 1, 0, 0, 0, 300, 295, 1, 
    0, 0, 0, 300, 296, 1, 0, 0, 0, 301, 65, 1, 0, 0, 0, 302, 313, 3, 68, 
    34, 0, 303, 304, 3, 68, 34, 0, 304, 305, 5, 8, 0, 0, 305, 306, 3, 26, 
    13, 0, 306, 313, 1, 0, 0, 0, 307, 308, 3, 68, 34, 0, 308, 309, 5, 8, 
    0, 0, 309, 310, 5, 40, 0, 0, 310, 311, 3, 26, 13, 0, 311, 313, 1, 0, 
    0, 0, 312, 302, 1, 0, 0, 0, 312, 303, 1, 0, 0, 0, 312, 307, 1, 0, 0, 
    0, 313, 67, 1, 0, 0, 0, 314, 319, 3, 70, 35, 0, 315, 316, 3, 86, 43, 
    0, 316, 317, 3, 68, 34, 0, 317, 319, 1, 0, 0, 0, 318, 314, 1, 0, 0, 
    0, 318, 315, 1, 0, 0, 0, 319, 69, 1, 0, 0, 0, 320, 325, 3, 72, 36, 0, 
    321, 322, 3, 76, 38, 0, 322, 323, 3, 70, 35, 0, 323, 325, 1, 0, 0, 0, 
    324, 320, 1, 0, 0, 0, 324, 321, 1, 0, 0, 0, 325, 71, 1, 0, 0, 0, 326, 
    331, 3, 74, 37, 0, 327, 328, 3, 74, 37, 0, 328, 329, 3, 72, 36, 0, 329, 
    331, 1, 0, 0, 0, 330, 326, 1, 0, 0, 0, 330, 327, 1, 0, 0, 0, 331, 73, 
    1, 0, 0, 0, 332, 337, 3, 88, 44, 0, 333, 337, 3, 50, 25, 0, 334, 337, 
    3, 52, 26, 0, 335, 337, 3, 22, 11, 0, 336, 332, 1, 0, 0, 0, 336, 333, 
    1, 0, 0, 0, 336, 334, 1, 0, 0, 0, 336, 335, 1, 0, 0, 0, 337, 75, 1, 
    0, 0, 0, 338, 339, 7, 0, 0, 0, 339, 77, 1, 0, 0, 0, 340, 341, 7, 1, 
    0, 0, 341, 79, 1, 0, 0, 0, 342, 343, 7, 2, 0, 0, 343, 81, 1, 0, 0, 0, 
    344, 345, 7, 3, 0, 0, 345, 83, 1, 0, 0, 0, 346, 347, 7, 4, 0, 0, 347, 
    85, 1, 0, 0, 0, 348, 349, 7, 5, 0, 0, 349, 87, 1, 0, 0, 0, 350, 351, 
    5, 19, 0, 0, 351, 357, 5, 20, 0, 0, 352, 353, 5, 19, 0, 0, 353, 354, 
    3, 90, 45, 0, 354, 355, 5, 20, 0, 0, 355, 357, 1, 0, 0, 0, 356, 350, 
    1, 0, 0, 0, 356, 352, 1, 0, 0, 0, 357, 89, 1, 0, 0, 0, 358, 359, 3, 
    54, 27, 0, 359, 360, 5, 39, 0, 0, 360, 361, 3, 90, 45, 0, 361, 365, 
    1, 0, 0, 0, 362, 365, 3, 54, 27, 0, 363, 365, 1, 0, 0, 0, 364, 358, 
    1, 0, 0, 0, 364, 362, 1, 0, 0, 0, 364, 363, 1, 0, 0, 0, 365, 91, 1, 
    0, 0, 0, 26, 98, 105, 116, 140, 147, 156, 163, 171, 181, 200, 218, 231, 
    238, 253, 272, 279, 286, 293, 300, 312, 318, 324, 330, 336, 356, 364
]);