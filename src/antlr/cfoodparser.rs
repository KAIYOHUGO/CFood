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
pub const CFood_PEO:i32=28; 
pub const CFood_ASSIGN:i32=29; 
pub const CFood_COMMA:i32=30; 
pub const CFood_SEMICOLON:i32=31; 
pub const CFood_TYPE:i32=32; 
pub const CFood_IDENT:i32=33; 
pub const CFood_INT:i32=34; 
pub const CFood_FLOAT:i32=35; 
pub const CFood_CONSTR:i32=36; 
pub const CFood_LINE_COMMENT:i32=37; 
pub const CFood_COMMENT:i32=38; 
pub const CFood_WS:i32=39;
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
pub const RULE_ty:usize = 13; 
pub const RULE_ty_kind:usize = 14; 
pub const RULE_block:usize = 15; 
pub const RULE_stmts:usize = 16; 
pub const RULE_stmt:usize = 17; 
pub const RULE_expr_stmt:usize = 18; 
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
    "fn_decl", "ty_decl", "params", "param_list", "param", "lit", "tys", 
    "ty", "ty_kind", "block", "stmts", "stmt", "expr_stmt", "branch_stmt", 
    "iter_stmt", "return_stmt", "let_stmt", "expr", "assign_expr", "var", 
    "calc_expr", "call_preced_expr", "add_preced_expr", "mul_preced_expr", 
    "atom_preced_expr", "cmp_preced_op", "add_preced_op", "mul_preced_op", 
    "apply_list", "args"
];

pub const _LITERAL_NAMES: [Option<&'static str>;32] = [
	None, Some("'while'"), Some("'if'"), Some("'else'"), Some("'return'"), 
	Some("'type'"), Some("'let'"), Some("'int'"), Some("'float'"), Some("'void'"), 
	Some("'->'"), Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), Some("'['"), 
	Some("']'"), Some("'!='"), Some("'=='"), Some("'<'"), Some("'>'"), Some("'<='"), 
	Some("'>='"), Some("'+'"), Some("'-'"), Some("'%'"), Some("'*'"), Some("'/'"), 
	Some("'##'"), Some("'='"), Some("','"), Some("';'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;40]  = [
	None, Some("KW_while"), Some("KW_if"), Some("KW_else"), Some("KW_return"), 
	Some("KW_type"), Some("KW_let"), Some("TY_int"), Some("TY_float"), Some("TY_void"), 
	Some("ARROW"), Some("PAREN_L"), Some("PAREN_R"), Some("BRACE_L"), Some("BRACE_R"), 
	Some("BRACKET_L"), Some("BRACKET_R"), Some("NE"), Some("EQ"), Some("LT"), 
	Some("GT"), Some("LE"), Some("GE"), Some("PLUS"), Some("SUB"), Some("MOD"), 
	Some("MUL"), Some("DIV"), Some("PEO"), Some("ASSIGN"), Some("COMMA"), Some("SEMICOLON"), 
	Some("TYPE"), Some("IDENT"), Some("INT"), Some("FLOAT"), Some("CONSTR"), 
	Some("LINE_COMMENT"), Some("COMMENT"), Some("WS")
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
    TyContext,
    Ty_kindContext,
    BlockContext,
    StmtsContext,
    StmtContext,
    Expr_stmtContext,
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
    LitContext(LitContextAll), Ty_kindContext(Ty_kindContextAll), Calc_exprContext(Calc_exprContextAll), Call_preced_exprContext(Call_preced_exprContextAll), Add_preced_exprContext(Add_preced_exprContextAll), Mul_preced_exprContext(Mul_preced_exprContextAll), Atom_preced_exprContext(Atom_preced_exprContextAll), ; FileContext(enter_file, exit_file,  visit_file), DeclsContext(enter_decls, exit_decls,  visit_decls), DeclContext(enter_decl, exit_decl,  visit_decl), Var_declContext(enter_var_decl, exit_var_decl,  visit_var_decl), Var_decl_tyContext(enter_var_decl_ty, exit_var_decl_ty,  visit_var_decl_ty), Var_decl_initContext(enter_var_decl_init, exit_var_decl_init,  visit_var_decl_init), Fn_declContext(enter_fn_decl, exit_fn_decl,  visit_fn_decl), Ty_declContext(enter_ty_decl, exit_ty_decl,  visit_ty_decl), ParamsContext(enter_params, exit_params,  visit_params), Param_listContext(enter_param_list, exit_param_list,  visit_param_list), ParamContext(enter_param, exit_param,  visit_param), TysContext(enter_tys, exit_tys,  visit_tys), TyContext(enter_ty, exit_ty,  visit_ty), BlockContext(enter_block, exit_block,  visit_block), StmtsContext(enter_stmts, exit_stmts,  visit_stmts), StmtContext(enter_stmt, exit_stmt,  visit_stmt), Expr_stmtContext(enter_expr_stmt, exit_expr_stmt,  visit_expr_stmt), Branch_stmtContext(enter_branch_stmt, exit_branch_stmt,  visit_branch_stmt), Iter_stmtContext(enter_iter_stmt, exit_iter_stmt,  visit_iter_stmt), Return_stmtContext(enter_return_stmt, exit_return_stmt,  visit_return_stmt), Let_stmtContext(enter_let_stmt, exit_let_stmt,  visit_let_stmt), ExprContext(enter_expr, exit_expr,  visit_expr), Assign_exprContext(enter_assign_expr, exit_assign_expr,  visit_assign_expr), VarContext(enter_var, exit_var,  visit_var), Cmp_preced_opContext(enter_cmp_preced_op, exit_cmp_preced_op,  visit_cmp_preced_op), Add_preced_opContext(enter_add_preced_op, exit_add_preced_op,  visit_add_preced_op), Mul_preced_opContext(enter_mul_preced_op, exit_mul_preced_op,  visit_mul_preced_op), Apply_listContext(enter_apply_list, exit_apply_list,  visit_apply_list), ArgsContext(enter_args, exit_args,  visit_args), 
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
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
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
			recog.var_decl_ty()?;
			/*InvokeRule var_decl_init*/
			recog.base.set_state(86);
			recog.var_decl_init()?;
			recog.base.set_state(87);
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
    fn ty(&self) -> Option<&'arena TyContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token BRACKET_L
    /// Returns `None` if there is no child corresponding to token BRACKET_L
    fn BRACKET_L(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn lit(&self) -> Option<&'arena LitContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token BRACKET_R
    /// Returns `None` if there is no child corresponding to token BRACKET_R
    fn BRACKET_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Var_decl_tyContextAttrs<'input, 'arena, Tok> for Var_decl_tyContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn ty(&self) -> Option<&'arena TyContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
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
    fn lit(&self) -> Option<&'arena LitContextAll<'input, 'arena, Tok>> {
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
			recog.base.set_state(98);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(2,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule ty*/
					recog.base.set_state(89);
					recog.ty()?;
					recog.base.set_state(90);
					recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule ty*/
					recog.base.set_state(92);
					recog.ty()?;
					recog.base.set_state(93);
					recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
					recog.base.set_state(94);
					recog.base.match_token(CFood_BRACKET_L,&mut recog.err_handler)?;
					/*InvokeRule lit*/
					recog.base.set_state(95);
					recog.lit()?;
					recog.base.set_state(96);
					recog.base.match_token(CFood_BRACKET_R,&mut recog.err_handler)?;
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
			recog.base.set_state(103);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_ASSIGN  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(100);
			        recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			        /*InvokeRule expr*/
			        recog.base.set_state(101);
			        recog.expr()?;
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
			recog.base.set_state(105);
			recog.ty()?;
			recog.base.set_state(106);
			recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
			/*InvokeRule params*/
			recog.base.set_state(107);
			recog.params()?;
			/*InvokeRule block*/
			recog.base.set_state(108);
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
			recog.base.set_state(110);
			recog.base.match_token(CFood_KW_type,&mut recog.err_handler)?;
			recog.base.set_state(111);
			recog.base.match_token(CFood_TYPE,&mut recog.err_handler)?;
			recog.base.set_state(112);
			recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			recog.base.set_state(113);
			recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
			/*InvokeRule tys*/
			recog.base.set_state(114);
			recog.tys()?;
			recog.base.set_state(115);
			recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
			recog.base.set_state(116);
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
			recog.base.set_state(127);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(4,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(118);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule param_list*/
					recog.base.set_state(119);
					recog.param_list()?;
					recog.base.set_state(120);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(122);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					recog.base.set_state(123);
					recog.base.match_token(CFood_TY_void,&mut recog.err_handler)?;
					recog.base.set_state(124);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					recog.base.set_state(125);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					recog.base.set_state(126);
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
			recog.base.set_state(134);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(5,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule param*/
					recog.base.set_state(129);
					recog.param()?;
					recog.base.set_state(130);
					recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					/*InvokeRule param_list*/
					recog.base.set_state(131);
					recog.param_list()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule param*/
					recog.base.set_state(133);
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
			recog.base.set_state(136);
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
	Lit_intContext(Lit_intContext<'input, 'arena, Tok>),
	Lit_floatContext(Lit_floatContext<'input, 'arena, Tok>),
    Error(LitContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { LitContextAll { } { Lit_constrContext, Lit_intContext, Lit_floatContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { LitContextAll { } { Lit_constrContext, Lit_intContext, Lit_floatContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::LitContextAll { Lit_constrContext, Lit_intContext, Lit_floatContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::LitContext::LitContextAll { Lit_constrContext, Lit_intContext, Lit_floatContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::LitContextAll { Lit_constrContext(enter_lit_constr, exit_lit_constr), Lit_intContext(enter_lit_int, exit_lit_int), Lit_floatContext(enter_lit_float, exit_lit_float), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::LitContextAll { Lit_constrContext(visit_lit_constr), Lit_intContext(visit_lit_int), Lit_floatContext(visit_lit_float), } }

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
			Lit_intContext(inner) => inner,
			Lit_floatContext(inner) => inner,
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
			recog.base.set_state(141);
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
			        recog.base.set_state(138);
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
			        recog.base.set_state(139);
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
			        recog.base.set_state(140);
			        recog.base.match_token(CFood_CONSTR,&mut recog.err_handler)?;
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
			recog.base.set_state(148);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(7,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(143);
					recog.ty_kind()?;
					recog.base.set_state(144);
					recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					/*InvokeRule tys*/
					recog.base.set_state(145);
					recog.tys()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(147);
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
//------------------- ty ----------------
pub type TyContextAll<'input, 'arena, Tok = CommonToken<'input>> = TyContext<'input, 'arena, Tok>;

pub type TyContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, TyContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
dbt_antlr4::impl_visitable! { CFoodVisitor::TyContext(visit_ty) }
#[derive(Debug)]
pub struct TyContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, TyContextExt {
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
			recog.base.set_state(155);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(8,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(150);
					recog.ty_kind()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule ty_kind*/
					recog.base.set_state(151);
					recog.ty_kind()?;
					recog.base.set_state(152);
					recog.base.match_token(CFood_ARROW,&mut recog.err_handler)?;
					/*InvokeRule ty*/
					recog.base.set_state(153);
					recog.ty()?;
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
	/// Retrieves first TerminalNode corresponding to token TY_void
	/// Returns `None` if there is no child corresponding to token TY_void
	fn TY_void(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
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
    /// Retrieves first TerminalNode corresponding to token TY_void
    /// Returns `None` if there is no child corresponding to token TY_void
    fn TY_void(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CFood_TY_void)
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
        recog.base.enter_rule(Ty_kindContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 28, RULE_ty_kind)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kindContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(161);
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
			        recog.base.set_state(157);
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
			        recog.base.set_state(158);
			        recog.base.match_token(CFood_TY_float,&mut recog.err_handler)?;
			        }}
			    CFood_TY_void  => {
			        /*------- Outer Most Alt 3 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Ty_kind_tyContextExt::copy_from(ctx);
			            ctx.set_alt_number(3);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kind_tyContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(159);
			        recog.base.match_token(CFood_TY_void,&mut recog.err_handler)?;
			        }}
			    CFood_TYPE  => {
			        /*------- Outer Most Alt 4 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Ty_kind_typeContextExt::copy_from(ctx);
			            ctx.set_alt_number(4);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Ty_kind_typeContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(160);
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
        recog.base.enter_rule(BlockContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 30, RULE_block)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena BlockContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(163);
			recog.base.match_token(CFood_BRACE_L,&mut recog.err_handler)?;
			/*InvokeRule stmts*/
			recog.base.set_state(164);
			recog.stmts()?;
			recog.base.set_state(165);
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
        recog.base.enter_rule(StmtsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 32, RULE_stmts)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(171);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_KW_while |CFood_KW_if |CFood_KW_return |CFood_KW_let |CFood_TY_int |
			    CFood_TY_float |CFood_TY_void |CFood_PAREN_L |CFood_BRACE_L |CFood_SEMICOLON |
			    CFood_TYPE |CFood_IDENT |CFood_INT |CFood_FLOAT |CFood_CONSTR  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule stmt*/
			        recog.base.set_state(167);
			        recog.stmt()?;
			        /*InvokeRule stmts*/
			        recog.base.set_state(168);
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
    fn var_decl(&self) -> Option<&'arena Var_declContextAll<'input, 'arena, Tok>>;
    fn let_stmt(&self) -> Option<&'arena Let_stmtContextAll<'input, 'arena, Tok>>;
    fn expr_stmt(&self) -> Option<&'arena Expr_stmtContextAll<'input, 'arena, Tok>>;
    fn branch_stmt(&self) -> Option<&'arena Branch_stmtContextAll<'input, 'arena, Tok>>;
    fn iter_stmt(&self) -> Option<&'arena Iter_stmtContextAll<'input, 'arena, Tok>>;
    fn return_stmt(&self) -> Option<&'arena Return_stmtContextAll<'input, 'arena, Tok>>;
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
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
    fn expr_stmt(&self) -> Option<&'arena Expr_stmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
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
	pub fn stmt(&mut self,) -> Result<&'arena StmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(StmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 34, RULE_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(181);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_TY_int |CFood_TY_float |CFood_TY_void |CFood_TYPE  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule var_decl*/
			        recog.base.set_state(173);
			        recog.var_decl()?;
			        }}
			    CFood_KW_let  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        /*InvokeRule let_stmt*/
			        recog.base.set_state(174);
			        recog.let_stmt()?;
			        }}
			    CFood_PAREN_L |CFood_IDENT |CFood_INT |CFood_FLOAT |CFood_CONSTR  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        /*InvokeRule expr_stmt*/
			        recog.base.set_state(175);
			        recog.expr_stmt()?;
			        }}
			    CFood_KW_if  => {
			        /*------- Outer Most Alt 4 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
			        {
			        /*InvokeRule branch_stmt*/
			        recog.base.set_state(176);
			        recog.branch_stmt()?;
			        }}
			    CFood_KW_while  => {
			        /*------- Outer Most Alt 5 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
			        {
			        /*InvokeRule iter_stmt*/
			        recog.base.set_state(177);
			        recog.iter_stmt()?;
			        }}
			    CFood_KW_return  => {
			        /*------- Outer Most Alt 6 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(6); }
			        {
			        /*InvokeRule return_stmt*/
			        recog.base.set_state(178);
			        recog.return_stmt()?;
			        }}
			    CFood_BRACE_L  => {
			        /*------- Outer Most Alt 7 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(7); }
			        {
			        /*InvokeRule block*/
			        recog.base.set_state(179);
			        recog.block()?;
			        }}
			    CFood_SEMICOLON  => {
			        /*------- Outer Most Alt 8 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(8); }
			        {
			        recog.base.set_state(180);
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
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Expr_stmtContextAttrs<'input, 'arena, Tok> for Expr_stmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
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
	pub fn expr_stmt(&mut self,) -> Result<&'arena Expr_stmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Expr_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 36, RULE_expr_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Expr_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule expr*/
			recog.base.set_state(183);
			recog.expr()?;
			recog.base.set_state(184);
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
        recog.base.enter_rule(Branch_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 38, RULE_branch_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Branch_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(200);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(12,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(186);
					recog.base.match_token(CFood_KW_if,&mut recog.err_handler)?;
					recog.base.set_state(187);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(188);
					recog.expr()?;
					recog.base.set_state(189);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					/*InvokeRule stmt*/
					recog.base.set_state(190);
					let tmp = recog.stmt()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Branch_stmtContext<TF::Tok>>().unwrap().then_branch = Some(tmp); } 
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(192);
					recog.base.match_token(CFood_KW_if,&mut recog.err_handler)?;
					recog.base.set_state(193);
					recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(194);
					recog.expr()?;
					recog.base.set_state(195);
					recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
					/*InvokeRule stmt*/
					recog.base.set_state(196);
					let tmp = recog.stmt()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Branch_stmtContext<TF::Tok>>().unwrap().then_branch = Some(tmp); } 
					recog.base.set_state(197);
					recog.base.match_token(CFood_KW_else,&mut recog.err_handler)?;
					/*InvokeRule stmt*/
					recog.base.set_state(198);
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
        recog.base.enter_rule(Iter_stmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 40, RULE_iter_stmt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Iter_stmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(202);
			recog.base.match_token(CFood_KW_while,&mut recog.err_handler)?;
			recog.base.set_state(203);
			recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
			/*InvokeRule expr*/
			recog.base.set_state(204);
			recog.expr()?;
			recog.base.set_state(205);
			recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
			/*InvokeRule stmt*/
			recog.base.set_state(206);
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
			recog.base.set_state(214);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(13,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(208);
					recog.base.match_token(CFood_KW_return,&mut recog.err_handler)?;
					recog.base.set_state(209);
					recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(210);
					recog.base.match_token(CFood_KW_return,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(211);
					recog.expr()?;
					recog.base.set_state(212);
					recog.base.match_token(CFood_SEMICOLON,&mut recog.err_handler)?;
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
			recog.base.set_state(216);
			recog.base.match_token(CFood_KW_let,&mut recog.err_handler)?;
			recog.base.set_state(217);
			recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
			recog.base.set_state(218);
			recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			/*InvokeRule expr*/
			recog.base.set_state(219);
			recog.expr()?;
			recog.base.set_state(220);
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
    fn assign_expr(&self) -> Option<&'arena Assign_exprContextAll<'input, 'arena, Tok>>;
    fn calc_expr(&self) -> Option<&'arena Calc_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ExprContextAttrs<'input, 'arena, Tok> for ExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
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
			recog.base.set_state(224);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(14,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule assign_expr*/
					recog.base.set_state(222);
					recog.assign_expr()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule calc_expr*/
					recog.base.set_state(223);
					recog.calc_expr()?;
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
dbt_antlr4::impl_visitable! { CFoodVisitor::Assign_exprContext(visit_assign_expr) }
#[derive(Debug)]
pub struct Assign_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Assign_exprContextExt {
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
			recog.base.set_state(226);
			recog.var()?;
			recog.base.set_state(227);
			recog.base.match_token(CFood_ASSIGN,&mut recog.err_handler)?;
			/*InvokeRule expr*/
			recog.base.set_state(228);
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
			recog.base.set_state(236);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(15,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(230);
					recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(231);
					recog.base.match_token(CFood_IDENT,&mut recog.err_handler)?;
					recog.base.set_state(232);
					recog.base.match_token(CFood_BRACKET_L,&mut recog.err_handler)?;
					/*InvokeRule expr*/
					recog.base.set_state(233);
					recog.expr()?;
					recog.base.set_state(234);
					recog.base.match_token(CFood_BRACKET_R,&mut recog.err_handler)?;
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
#[derive(Debug)]
#[repr(C, u16)]
pub enum Calc_exprContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Calc_expr_useContext(Calc_expr_useContext<'input, 'arena, Tok>),
	Calc_expr_passContext(Calc_expr_passContext<'input, 'arena, Tok>),
    Error(Calc_exprContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Calc_exprContextAll { } { Calc_expr_useContext, Calc_expr_passContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Calc_exprContextAll { } { Calc_expr_useContext, Calc_expr_passContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Calc_exprContextAll { Calc_expr_useContext, Calc_expr_passContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Calc_exprContext::Calc_exprContextAll { Calc_expr_useContext, Calc_expr_passContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Calc_exprContextAll { Calc_expr_useContext(enter_calc_expr_use, exit_calc_expr_use), Calc_expr_passContext(enter_calc_expr_pass, exit_calc_expr_pass), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Calc_exprContextAll { Calc_expr_useContext(visit_calc_expr_use), Calc_expr_passContext(visit_calc_expr_pass), } }

impl<'input, 'arena, Tok> Deref for Calc_exprContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Calc_exprContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Calc_exprContextAll::*;
		match self{
			Calc_expr_useContext(inner) => inner,
			Calc_expr_passContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Calc_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Calc_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Calc_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        arena.alloc_labeled_node(Calc_exprContextAll::Error(ctx))
    }
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Calc_exprContextExt {
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
}

impl<'input, 'arena, Tok: Token + 'input> Calc_exprContextAttrs<'input, 'arena, Tok> for Calc_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Calc_expr_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Calc_expr_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Calc_expr_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn cmp_preced_op(&self) -> Option<&'arena Cmp_preced_opContextAll<'input, 'arena, Tok>>;
	fn call_preced_expr_all(&self) -> Vec<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>;
	fn call_preced_expr(&self, i: usize) -> Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Calc_expr_useContextAttrs<'input, 'arena, Tok> for Calc_expr_useContext<'input, 'arena, Tok>
{
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
#[derive(Debug)]
pub struct Calc_expr_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Calc_exprContextExt<'input, 'arena, Tok>,
	pub lhs: Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>,
	pub rhs: Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Calc_expr_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Calc_exprContext }
	fn get_rule_index(&self) -> usize { RULE_calc_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Calc_expr_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Calc_exprContextAll::Calc_expr_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Calc_expr_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Calc_exprContextAll<'input, 'arena, Tok>) {
                Calc_exprContextAll::Calc_expr_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Calc_expr_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Calc_exprContextAll<'input, 'arena, Tok>) {
                Calc_exprContextAll::Calc_expr_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Calc_exprContextAttrs<'input, 'arena, Tok> for Calc_expr_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Calc_expr_useContextExt<'input, 'arena, Tok> {
	fn new(base: Calc_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            lhs:None, rhs:None, 
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Calc_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Calc_exprContextAll::Calc_expr_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Calc_exprContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Calc_expr_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Calc_expr_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Calc_expr_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn call_preced_expr(&self) -> Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Calc_expr_passContextAttrs<'input, 'arena, Tok> for Calc_expr_passContext<'input, 'arena, Tok>
{
    fn call_preced_expr(&self) -> Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Calc_expr_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Calc_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Calc_expr_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Calc_exprContext }
	fn get_rule_index(&self) -> usize { RULE_calc_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Calc_expr_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Calc_exprContextAll::Calc_expr_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Calc_expr_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Calc_exprContextAll<'input, 'arena, Tok>) {
                Calc_exprContextAll::Calc_expr_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Calc_expr_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Calc_exprContextAll<'input, 'arena, Tok>) {
                Calc_exprContextAll::Calc_expr_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Calc_exprContextAttrs<'input, 'arena, Tok> for Calc_expr_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Calc_expr_passContextExt<'input, 'arena, Tok> {
	fn new(base: Calc_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Calc_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Calc_exprContextAll::Calc_expr_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Calc_exprContextAll<'input, 'arena, Tok>) = ctx;
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
	pub fn calc_expr(&mut self,) -> Result<&'arena Calc_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Calc_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 52, RULE_calc_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Calc_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(243);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(16,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Calc_expr_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Calc_expr_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule call_preced_expr*/
					recog.base.set_state(238);
					let tmp = recog.call_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Calc_expr_useContext<TF::Tok>>().unwrap().lhs = Some(tmp); } 
					/*InvokeRule cmp_preced_op*/
					recog.base.set_state(239);
					recog.cmp_preced_op()?;
					/*InvokeRule call_preced_expr*/
					recog.base.set_state(240);
					let tmp = recog.call_preced_expr()?;
					unsafe { recog.ctx_mut().unwrap().as_rule_context_mut::<Calc_expr_useContext<TF::Tok>>().unwrap().rhs = Some(tmp); } 
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Calc_expr_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Calc_expr_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule call_preced_expr*/
					recog.base.set_state(242);
					recog.call_preced_expr()?;
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
#[derive(Debug)]
#[repr(C, u16)]
pub enum Call_preced_exprContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Call_preced_expr_passContext(Call_preced_expr_passContext<'input, 'arena, Tok>),
	Call_preced_expr_useContext(Call_preced_expr_useContext<'input, 'arena, Tok>),
    Error(Call_preced_exprContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Call_preced_exprContextAll { } { Call_preced_expr_passContext, Call_preced_expr_useContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Call_preced_exprContextAll { } { Call_preced_expr_passContext, Call_preced_expr_useContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Call_preced_exprContextAll { Call_preced_expr_passContext, Call_preced_expr_useContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Call_preced_exprContext::Call_preced_exprContextAll { Call_preced_expr_passContext, Call_preced_expr_useContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Call_preced_exprContextAll { Call_preced_expr_passContext(enter_call_preced_expr_pass, exit_call_preced_expr_pass), Call_preced_expr_useContext(enter_call_preced_expr_use, exit_call_preced_expr_use), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Call_preced_exprContextAll { Call_preced_expr_passContext(visit_call_preced_expr_pass), Call_preced_expr_useContext(visit_call_preced_expr_use), } }

impl<'input, 'arena, Tok> Deref for Call_preced_exprContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Call_preced_exprContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Call_preced_exprContextAll::*;
		match self{
			Call_preced_expr_passContext(inner) => inner,
			Call_preced_expr_useContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Call_preced_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Call_preced_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Call_preced_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        arena.alloc_labeled_node(Call_preced_exprContextAll::Error(ctx))
    }
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Call_preced_exprContextExt {
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
}

impl<'input, 'arena, Tok: Token + 'input> Call_preced_exprContextAttrs<'input, 'arena, Tok> for Call_preced_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Call_preced_expr_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Call_preced_expr_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Call_preced_expr_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn add_preced_expr(&self) -> Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Call_preced_expr_passContextAttrs<'input, 'arena, Tok> for Call_preced_expr_passContext<'input, 'arena, Tok>
{
    fn add_preced_expr(&self) -> Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Call_preced_expr_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Call_preced_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Call_preced_expr_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Call_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_call_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Call_preced_expr_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Call_preced_exprContextAll::Call_preced_expr_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Call_preced_expr_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Call_preced_exprContextAll<'input, 'arena, Tok>) {
                Call_preced_exprContextAll::Call_preced_expr_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Call_preced_expr_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Call_preced_exprContextAll<'input, 'arena, Tok>) {
                Call_preced_exprContextAll::Call_preced_expr_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Call_preced_exprContextAttrs<'input, 'arena, Tok> for Call_preced_expr_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Call_preced_expr_passContextExt<'input, 'arena, Tok> {
	fn new(base: Call_preced_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Call_preced_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Call_preced_exprContextAll::Call_preced_expr_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Call_preced_exprContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Call_preced_expr_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Call_preced_expr_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Call_preced_expr_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn add_preced_expr(&self) -> Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>>;
	fn call_preced_expr(&self) -> Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Call_preced_expr_useContextAttrs<'input, 'arena, Tok> for Call_preced_expr_useContext<'input, 'arena, Tok>
{
    fn add_preced_expr(&self) -> Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn call_preced_expr(&self) -> Option<&'arena Call_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Call_preced_expr_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Call_preced_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Call_preced_expr_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Call_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_call_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Call_preced_expr_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Call_preced_exprContextAll::Call_preced_expr_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Call_preced_expr_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Call_preced_exprContextAll<'input, 'arena, Tok>) {
                Call_preced_exprContextAll::Call_preced_expr_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Call_preced_expr_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Call_preced_exprContextAll<'input, 'arena, Tok>) {
                Call_preced_exprContextAll::Call_preced_expr_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Call_preced_exprContextAttrs<'input, 'arena, Tok> for Call_preced_expr_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Call_preced_expr_useContextExt<'input, 'arena, Tok> {
	fn new(base: Call_preced_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Call_preced_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Call_preced_exprContextAll::Call_preced_expr_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Call_preced_exprContextAll<'input, 'arena, Tok>) = ctx;
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
	pub fn call_preced_expr(&mut self,) -> Result<&'arena Call_preced_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Call_preced_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 54, RULE_call_preced_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Call_preced_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(249);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(17,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Call_preced_expr_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Call_preced_expr_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule add_preced_expr*/
					recog.base.set_state(245);
					recog.add_preced_expr()?;
					/*InvokeRule call_preced_expr*/
					recog.base.set_state(246);
					recog.call_preced_expr()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Call_preced_expr_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Call_preced_expr_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule add_preced_expr*/
					recog.base.set_state(248);
					recog.add_preced_expr()?;
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
#[derive(Debug)]
#[repr(C, u16)]
pub enum Add_preced_exprContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Add_preced_expr_passContext(Add_preced_expr_passContext<'input, 'arena, Tok>),
	Add_preced_expr_useContext(Add_preced_expr_useContext<'input, 'arena, Tok>),
    Error(Add_preced_exprContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Add_preced_exprContextAll { } { Add_preced_expr_passContext, Add_preced_expr_useContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Add_preced_exprContextAll { } { Add_preced_expr_passContext, Add_preced_expr_useContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Add_preced_exprContextAll { Add_preced_expr_passContext, Add_preced_expr_useContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Add_preced_exprContext::Add_preced_exprContextAll { Add_preced_expr_passContext, Add_preced_expr_useContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Add_preced_exprContextAll { Add_preced_expr_passContext(enter_add_preced_expr_pass, exit_add_preced_expr_pass), Add_preced_expr_useContext(enter_add_preced_expr_use, exit_add_preced_expr_use), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Add_preced_exprContextAll { Add_preced_expr_passContext(visit_add_preced_expr_pass), Add_preced_expr_useContext(visit_add_preced_expr_use), } }

impl<'input, 'arena, Tok> Deref for Add_preced_exprContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Add_preced_exprContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Add_preced_exprContextAll::*;
		match self{
			Add_preced_expr_passContext(inner) => inner,
			Add_preced_expr_useContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Add_preced_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Add_preced_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Add_preced_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        arena.alloc_labeled_node(Add_preced_exprContextAll::Error(ctx))
    }
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Add_preced_exprContextExt {
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
}

impl<'input, 'arena, Tok: Token + 'input> Add_preced_exprContextAttrs<'input, 'arena, Tok> for Add_preced_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Add_preced_expr_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Add_preced_expr_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Add_preced_expr_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn mul_preced_expr(&self) -> Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Add_preced_expr_passContextAttrs<'input, 'arena, Tok> for Add_preced_expr_passContext<'input, 'arena, Tok>
{
    fn mul_preced_expr(&self) -> Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Add_preced_expr_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Add_preced_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Add_preced_expr_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Add_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_add_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Add_preced_expr_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Add_preced_exprContextAll::Add_preced_expr_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Add_preced_expr_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Add_preced_exprContextAll<'input, 'arena, Tok>) {
                Add_preced_exprContextAll::Add_preced_expr_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Add_preced_expr_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Add_preced_exprContextAll<'input, 'arena, Tok>) {
                Add_preced_exprContextAll::Add_preced_expr_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Add_preced_exprContextAttrs<'input, 'arena, Tok> for Add_preced_expr_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Add_preced_expr_passContextExt<'input, 'arena, Tok> {
	fn new(base: Add_preced_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Add_preced_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Add_preced_exprContextAll::Add_preced_expr_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Add_preced_exprContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Add_preced_expr_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Add_preced_expr_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Add_preced_expr_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn mul_preced_expr(&self) -> Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>>;
	fn add_preced_op(&self) -> Option<&'arena Add_preced_opContextAll<'input, 'arena, Tok>>;
	fn add_preced_expr(&self) -> Option<&'arena Add_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Add_preced_expr_useContextAttrs<'input, 'arena, Tok> for Add_preced_expr_useContext<'input, 'arena, Tok>
{
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
#[derive(Debug)]
pub struct Add_preced_expr_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Add_preced_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Add_preced_expr_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Add_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_add_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Add_preced_expr_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Add_preced_exprContextAll::Add_preced_expr_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Add_preced_expr_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Add_preced_exprContextAll<'input, 'arena, Tok>) {
                Add_preced_exprContextAll::Add_preced_expr_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Add_preced_expr_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Add_preced_exprContextAll<'input, 'arena, Tok>) {
                Add_preced_exprContextAll::Add_preced_expr_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Add_preced_exprContextAttrs<'input, 'arena, Tok> for Add_preced_expr_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Add_preced_expr_useContextExt<'input, 'arena, Tok> {
	fn new(base: Add_preced_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Add_preced_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Add_preced_exprContextAll::Add_preced_expr_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Add_preced_exprContextAll<'input, 'arena, Tok>) = ctx;
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
	pub fn add_preced_expr(&mut self,) -> Result<&'arena Add_preced_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Add_preced_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 56, RULE_add_preced_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Add_preced_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(256);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(18,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Add_preced_expr_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Add_preced_expr_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule mul_preced_expr*/
					recog.base.set_state(251);
					recog.mul_preced_expr()?;
					/*InvokeRule add_preced_op*/
					recog.base.set_state(252);
					recog.add_preced_op()?;
					/*InvokeRule add_preced_expr*/
					recog.base.set_state(253);
					recog.add_preced_expr()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Add_preced_expr_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Add_preced_expr_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule mul_preced_expr*/
					recog.base.set_state(255);
					recog.mul_preced_expr()?;
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
#[derive(Debug)]
#[repr(C, u16)]
pub enum Mul_preced_exprContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Mul_preced_expr_useContext(Mul_preced_expr_useContext<'input, 'arena, Tok>),
	Mul_preced_expr_passContext(Mul_preced_expr_passContext<'input, 'arena, Tok>),
    Error(Mul_preced_exprContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Mul_preced_exprContextAll { } { Mul_preced_expr_useContext, Mul_preced_expr_passContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Mul_preced_exprContextAll { } { Mul_preced_expr_useContext, Mul_preced_expr_passContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Mul_preced_exprContextAll { Mul_preced_expr_useContext, Mul_preced_expr_passContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Mul_preced_exprContext::Mul_preced_exprContextAll { Mul_preced_expr_useContext, Mul_preced_expr_passContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Mul_preced_exprContextAll { Mul_preced_expr_useContext(enter_mul_preced_expr_use, exit_mul_preced_expr_use), Mul_preced_expr_passContext(enter_mul_preced_expr_pass, exit_mul_preced_expr_pass), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Mul_preced_exprContextAll { Mul_preced_expr_useContext(visit_mul_preced_expr_use), Mul_preced_expr_passContext(visit_mul_preced_expr_pass), } }

impl<'input, 'arena, Tok> Deref for Mul_preced_exprContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Mul_preced_exprContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Mul_preced_exprContextAll::*;
		match self{
			Mul_preced_expr_useContext(inner) => inner,
			Mul_preced_expr_passContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Mul_preced_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Mul_preced_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Mul_preced_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        arena.alloc_labeled_node(Mul_preced_exprContextAll::Error(ctx))
    }
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Mul_preced_exprContextExt {
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
}

impl<'input, 'arena, Tok: Token + 'input> Mul_preced_exprContextAttrs<'input, 'arena, Tok> for Mul_preced_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Mul_preced_expr_useContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Mul_preced_expr_useContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Mul_preced_expr_useContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn atom_preced_expr(&self) -> Option<&'arena Atom_preced_exprContextAll<'input, 'arena, Tok>>;
	fn mul_preced_op(&self) -> Option<&'arena Mul_preced_opContextAll<'input, 'arena, Tok>>;
	fn mul_preced_expr(&self) -> Option<&'arena Mul_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Mul_preced_expr_useContextAttrs<'input, 'arena, Tok> for Mul_preced_expr_useContext<'input, 'arena, Tok>
{
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
#[derive(Debug)]
pub struct Mul_preced_expr_useContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Mul_preced_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Mul_preced_expr_useContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Mul_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_mul_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Mul_preced_expr_useContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Mul_preced_exprContextAll::Mul_preced_expr_useContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Mul_preced_expr_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Mul_preced_exprContextAll<'input, 'arena, Tok>) {
                Mul_preced_exprContextAll::Mul_preced_expr_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Mul_preced_expr_useContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Mul_preced_exprContextAll<'input, 'arena, Tok>) {
                Mul_preced_exprContextAll::Mul_preced_expr_useContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Mul_preced_exprContextAttrs<'input, 'arena, Tok> for Mul_preced_expr_useContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Mul_preced_expr_useContextExt<'input, 'arena, Tok> {
	fn new(base: Mul_preced_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Mul_preced_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Mul_preced_exprContextAll::Mul_preced_expr_useContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Mul_preced_exprContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Mul_preced_expr_passContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Mul_preced_expr_passContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Mul_preced_expr_passContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn atom_preced_expr(&self) -> Option<&'arena Atom_preced_exprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Mul_preced_expr_passContextAttrs<'input, 'arena, Tok> for Mul_preced_expr_passContext<'input, 'arena, Tok>
{
    fn atom_preced_expr(&self) -> Option<&'arena Atom_preced_exprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Mul_preced_expr_passContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Mul_preced_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Mul_preced_expr_passContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Mul_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_mul_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Mul_preced_expr_passContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Mul_preced_exprContextAll::Mul_preced_expr_passContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Mul_preced_expr_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Mul_preced_exprContextAll<'input, 'arena, Tok>) {
                Mul_preced_exprContextAll::Mul_preced_expr_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Mul_preced_expr_passContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Mul_preced_exprContextAll<'input, 'arena, Tok>) {
                Mul_preced_exprContextAll::Mul_preced_expr_passContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Mul_preced_exprContextAttrs<'input, 'arena, Tok> for Mul_preced_expr_passContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Mul_preced_expr_passContextExt<'input, 'arena, Tok> {
	fn new(base: Mul_preced_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Mul_preced_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Mul_preced_exprContextAll::Mul_preced_expr_passContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Mul_preced_exprContextAll<'input, 'arena, Tok>) = ctx;
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
	pub fn mul_preced_expr(&mut self,) -> Result<&'arena Mul_preced_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Mul_preced_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 58, RULE_mul_preced_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Mul_preced_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(263);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(19,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Mul_preced_expr_useContextExt::copy_from(ctx);
					    ctx.set_alt_number(1);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Mul_preced_expr_useContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule atom_preced_expr*/
					recog.base.set_state(258);
					recog.atom_preced_expr()?;
					/*InvokeRule mul_preced_op*/
					recog.base.set_state(259);
					recog.mul_preced_op()?;
					/*InvokeRule mul_preced_expr*/
					recog.base.set_state(260);
					recog.mul_preced_expr()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					recog.base.with_mut_ctx(|ctx| {
					    Mul_preced_expr_passContextExt::copy_from(ctx);
					    ctx.set_alt_number(2);
					});
					let _local_ctx_fn = |recog: &Self| -> &'arena Mul_preced_expr_passContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
					{
					/*InvokeRule atom_preced_expr*/
					recog.base.set_state(262);
					recog.atom_preced_expr()?;
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
#[derive(Debug)]
#[repr(C, u16)]
pub enum Atom_preced_exprContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	Atom_preced_expr_apply_listContext(Atom_preced_expr_apply_listContext<'input, 'arena, Tok>),
	Atom_preced_expr_varContext(Atom_preced_expr_varContext<'input, 'arena, Tok>),
	Atom_preced_expr_litContext(Atom_preced_expr_litContext<'input, 'arena, Tok>),
    Error(Atom_preced_exprContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { Atom_preced_exprContextAll { } { Atom_preced_expr_apply_listContext, Atom_preced_expr_varContext, Atom_preced_expr_litContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { Atom_preced_exprContextAll { } { Atom_preced_expr_apply_listContext, Atom_preced_expr_varContext, Atom_preced_expr_litContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { CFoodParserNodeKind::Atom_preced_exprContextAll { Atom_preced_expr_apply_listContext, Atom_preced_expr_varContext, Atom_preced_expr_litContext, Error, } }
dbt_antlr4::impl_node_inner! { CFoodParserNodeKind::Atom_preced_exprContext::Atom_preced_exprContextAll { Atom_preced_expr_apply_listContext, Atom_preced_expr_varContext, Atom_preced_expr_litContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { CFoodListener::CFoodParserNodeKind::Atom_preced_exprContextAll { Atom_preced_expr_apply_listContext(enter_atom_preced_expr_apply_list, exit_atom_preced_expr_apply_list), Atom_preced_expr_varContext(enter_atom_preced_expr_var, exit_atom_preced_expr_var), Atom_preced_expr_litContext(enter_atom_preced_expr_lit, exit_atom_preced_expr_lit), } }
dbt_antlr4::impl_visitable! { CFoodVisitor::Atom_preced_exprContextAll { Atom_preced_expr_apply_listContext(visit_atom_preced_expr_apply_list), Atom_preced_expr_varContext(visit_atom_preced_expr_var), Atom_preced_expr_litContext(visit_atom_preced_expr_lit), } }

impl<'input, 'arena, Tok> Deref for Atom_preced_exprContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn Atom_preced_exprContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use Atom_preced_exprContextAll::*;
		match self{
			Atom_preced_expr_apply_listContext(inner) => inner,
			Atom_preced_expr_varContext(inner) => inner,
			Atom_preced_expr_litContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type Atom_preced_exprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Atom_preced_exprContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Atom_preced_exprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
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
        arena.alloc_labeled_node(Atom_preced_exprContextAll::Error(ctx))
    }
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
        BaseParserRuleContext::create(arena, parent, invoking_state, Atom_preced_exprContextExt {
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
}

impl<'input, 'arena, Tok: Token + 'input> Atom_preced_exprContextAttrs<'input, 'arena, Tok> for Atom_preced_exprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type Atom_preced_expr_apply_listContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Atom_preced_expr_apply_listContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Atom_preced_expr_apply_listContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn apply_list(&self) -> Option<&'arena Apply_listContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_preced_expr_apply_listContextAttrs<'input, 'arena, Tok> for Atom_preced_expr_apply_listContext<'input, 'arena, Tok>
{
    fn apply_list(&self) -> Option<&'arena Apply_listContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Atom_preced_expr_apply_listContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Atom_preced_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Atom_preced_expr_apply_listContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Atom_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_atom_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Atom_preced_expr_apply_listContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Atom_preced_exprContextAll::Atom_preced_expr_apply_listContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Atom_preced_expr_apply_listContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Atom_preced_exprContextAll<'input, 'arena, Tok>) {
                Atom_preced_exprContextAll::Atom_preced_expr_apply_listContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Atom_preced_expr_apply_listContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Atom_preced_exprContextAll<'input, 'arena, Tok>) {
                Atom_preced_exprContextAll::Atom_preced_expr_apply_listContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Atom_preced_exprContextAttrs<'input, 'arena, Tok> for Atom_preced_expr_apply_listContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_preced_expr_apply_listContextExt<'input, 'arena, Tok> {
	fn new(base: Atom_preced_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Atom_preced_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Atom_preced_exprContextAll::Atom_preced_expr_apply_listContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Atom_preced_exprContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Atom_preced_expr_varContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Atom_preced_expr_varContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Atom_preced_expr_varContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_preced_expr_varContextAttrs<'input, 'arena, Tok> for Atom_preced_expr_varContext<'input, 'arena, Tok>
{
    fn var(&self) -> Option<&'arena VarContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Atom_preced_expr_varContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Atom_preced_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Atom_preced_expr_varContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Atom_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_atom_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Atom_preced_expr_varContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Atom_preced_exprContextAll::Atom_preced_expr_varContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Atom_preced_expr_varContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Atom_preced_exprContextAll<'input, 'arena, Tok>) {
                Atom_preced_exprContextAll::Atom_preced_expr_varContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Atom_preced_expr_varContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Atom_preced_exprContextAll<'input, 'arena, Tok>) {
                Atom_preced_exprContextAll::Atom_preced_expr_varContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Atom_preced_exprContextAttrs<'input, 'arena, Tok> for Atom_preced_expr_varContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_preced_expr_varContextExt<'input, 'arena, Tok> {
	fn new(base: Atom_preced_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Atom_preced_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Atom_preced_exprContextAll::Atom_preced_expr_varContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Atom_preced_exprContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type Atom_preced_expr_litContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Atom_preced_expr_litContextExt<'input, 'arena, Tok>, CFoodParserNodeKind, Tok>;

pub trait Atom_preced_expr_litContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn lit(&self) -> Option<&'arena LitContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_preced_expr_litContextAttrs<'input, 'arena, Tok> for Atom_preced_expr_litContext<'input, 'arena, Tok>
{
    fn lit(&self) -> Option<&'arena LitContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct Atom_preced_expr_litContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: Atom_preced_exprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for Atom_preced_expr_litContextExt<'input, 'arena, Tok>
{
	type NodeKind = CFoodParserNodeKind;
    fn node_tag() -> CFoodParserNodeKind { CFoodParserNodeKind::Atom_preced_exprContext }
	fn get_rule_index(&self) -> usize { RULE_atom_preced_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: Atom_preced_expr_litContext<'input, 'arena, Tok>,
    ) -> *mut CFoodParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(Atom_preced_exprContextAll::Atom_preced_expr_litContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Atom_preced_expr_litContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => Atom_preced_exprContextAll<'input, 'arena, Tok>) {
                Atom_preced_exprContextAll::Atom_preced_expr_litContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut CFoodParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Atom_preced_expr_litContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut Atom_preced_exprContextAll<'input, 'arena, Tok>) {
                Atom_preced_exprContextAll::Atom_preced_expr_litContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> Atom_preced_exprContextAttrs<'input, 'arena, Tok> for Atom_preced_expr_litContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Atom_preced_expr_litContextExt<'input, 'arena, Tok> {
	fn new(base: Atom_preced_exprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut CFoodParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut Atom_preced_exprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            Atom_preced_exprContextAll::Atom_preced_expr_litContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut Atom_preced_exprContextAll<'input, 'arena, Tok>) = ctx;
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
	pub fn atom_preced_expr(&mut self,) -> Result<&'arena Atom_preced_exprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Atom_preced_exprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 60, RULE_atom_preced_expr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Atom_preced_exprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(268);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    CFood_PAREN_L  => {
			        /*------- Outer Most Alt 1 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Atom_preced_expr_apply_listContextExt::copy_from(ctx);
			            ctx.set_alt_number(1);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Atom_preced_expr_apply_listContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule apply_list*/
			        recog.base.set_state(265);
			        recog.apply_list()?;
			        }}
			    CFood_IDENT  => {
			        /*------- Outer Most Alt 2 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Atom_preced_expr_varContextExt::copy_from(ctx);
			            ctx.set_alt_number(2);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Atom_preced_expr_varContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule var*/
			        recog.base.set_state(266);
			        recog.var()?;
			        }}
			    CFood_INT |CFood_FLOAT |CFood_CONSTR  => {
			        /*------- Outer Most Alt 3 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            Atom_preced_expr_litContextExt::copy_from(ctx);
			            ctx.set_alt_number(3);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena Atom_preced_expr_litContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        /*InvokeRule lit*/
			        recog.base.set_state(267);
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
        recog.base.enter_rule(Cmp_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 62, RULE_cmp_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Cmp_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(270);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 8257536) != 0)) } {
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
        recog.base.enter_rule(Add_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 64, RULE_add_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Add_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(272);
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
        recog.base.enter_rule(Mul_preced_opContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 66, RULE_mul_preced_op)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Mul_preced_opContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(274);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 503316480) != 0)) } {
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
    fn args(&self) -> Option<&'arena ArgsContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
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
			recog.base.set_state(276);
			recog.base.match_token(CFood_PAREN_L,&mut recog.err_handler)?;
			/*InvokeRule args*/
			recog.base.set_state(277);
			recog.args()?;
			recog.base.set_state(278);
			recog.base.match_token(CFood_PAREN_R,&mut recog.err_handler)?;
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
        recog.base.enter_rule(ArgsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 70, RULE_args)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ArgsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(286);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(21,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule expr*/
					recog.base.set_state(280);
					recog.expr()?;
					recog.base.set_state(281);
					recog.base.match_token(CFood_COMMA,&mut recog.err_handler)?;
					/*InvokeRule args*/
					recog.base.set_state(282);
					recog.args()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule expr*/
					recog.base.set_state(284);
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
    4, 1, 39, 289, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 
    7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 
    10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 
    15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 
    20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 
    25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 
    30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 
    35, 7, 35, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 79, 8, 1, 1, 2, 
    1, 2, 1, 2, 3, 2, 84, 8, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 1, 4, 
    1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 3, 4, 99, 8, 4, 1, 5, 1, 5, 1, 5, 
    3, 5, 104, 8, 5, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 
    7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 
    1, 8, 1, 8, 3, 8, 128, 8, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 3, 9, 135, 
    8, 9, 1, 10, 1, 10, 1, 11, 1, 11, 1, 11, 3, 11, 142, 8, 11, 1, 12, 1, 
    12, 1, 12, 1, 12, 1, 12, 3, 12, 149, 8, 12, 1, 13, 1, 13, 1, 13, 1, 
    13, 1, 13, 3, 13, 156, 8, 13, 1, 14, 1, 14, 1, 14, 1, 14, 3, 14, 162, 
    8, 14, 1, 15, 1, 15, 1, 15, 1, 15, 1, 16, 1, 16, 1, 16, 1, 16, 3, 16, 
    172, 8, 16, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 
    3, 17, 182, 8, 17, 1, 18, 1, 18, 1, 18, 1, 19, 1, 19, 1, 19, 1, 19, 
    1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 
    3, 19, 201, 8, 19, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 21, 
    1, 21, 1, 21, 1, 21, 1, 21, 1, 21, 3, 21, 215, 8, 21, 1, 22, 1, 22, 
    1, 22, 1, 22, 1, 22, 1, 22, 1, 23, 1, 23, 3, 23, 225, 8, 23, 1, 24, 
    1, 24, 1, 24, 1, 24, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 3, 25, 
    237, 8, 25, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 3, 26, 244, 8, 26, 1, 
    27, 1, 27, 1, 27, 1, 27, 3, 27, 250, 8, 27, 1, 28, 1, 28, 1, 28, 1, 
    28, 1, 28, 3, 28, 257, 8, 28, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 3, 
    29, 264, 8, 29, 1, 30, 1, 30, 1, 30, 3, 30, 269, 8, 30, 1, 31, 1, 31, 
    1, 32, 1, 32, 1, 33, 1, 33, 1, 34, 1, 34, 1, 34, 1, 34, 1, 35, 1, 35, 
    1, 35, 1, 35, 1, 35, 1, 35, 3, 35, 287, 8, 35, 1, 35, 0, 0, 36, 0, 2, 
    4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 
    40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 0, 3, 
    1, 0, 17, 22, 1, 0, 23, 24, 1, 0, 25, 28, 287, 0, 72, 1, 0, 0, 0, 2, 
    78, 1, 0, 0, 0, 4, 83, 1, 0, 0, 0, 6, 85, 1, 0, 0, 0, 8, 98, 1, 0, 0, 
    0, 10, 103, 1, 0, 0, 0, 12, 105, 1, 0, 0, 0, 14, 110, 1, 0, 0, 0, 16, 
    127, 1, 0, 0, 0, 18, 134, 1, 0, 0, 0, 20, 136, 1, 0, 0, 0, 22, 141, 
    1, 0, 0, 0, 24, 148, 1, 0, 0, 0, 26, 155, 1, 0, 0, 0, 28, 161, 1, 0, 
    0, 0, 30, 163, 1, 0, 0, 0, 32, 171, 1, 0, 0, 0, 34, 181, 1, 0, 0, 0, 
    36, 183, 1, 0, 0, 0, 38, 200, 1, 0, 0, 0, 40, 202, 1, 0, 0, 0, 42, 214, 
    1, 0, 0, 0, 44, 216, 1, 0, 0, 0, 46, 224, 1, 0, 0, 0, 48, 226, 1, 0, 
    0, 0, 50, 236, 1, 0, 0, 0, 52, 243, 1, 0, 0, 0, 54, 249, 1, 0, 0, 0, 
    56, 256, 1, 0, 0, 0, 58, 263, 1, 0, 0, 0, 60, 268, 1, 0, 0, 0, 62, 270, 
    1, 0, 0, 0, 64, 272, 1, 0, 0, 0, 66, 274, 1, 0, 0, 0, 68, 276, 1, 0, 
    0, 0, 70, 286, 1, 0, 0, 0, 72, 73, 3, 2, 1, 0, 73, 1, 1, 0, 0, 0, 74, 
    75, 3, 4, 2, 0, 75, 76, 3, 2, 1, 0, 76, 79, 1, 0, 0, 0, 77, 79, 1, 0, 
    0, 0, 78, 74, 1, 0, 0, 0, 78, 77, 1, 0, 0, 0, 79, 3, 1, 0, 0, 0, 80, 
    84, 3, 6, 3, 0, 81, 84, 3, 14, 7, 0, 82, 84, 3, 12, 6, 0, 83, 80, 1, 
    0, 0, 0, 83, 81, 1, 0, 0, 0, 83, 82, 1, 0, 0, 0, 84, 5, 1, 0, 0, 0, 
    85, 86, 3, 8, 4, 0, 86, 87, 3, 10, 5, 0, 87, 88, 5, 31, 0, 0, 88, 7, 
    1, 0, 0, 0, 89, 90, 3, 26, 13, 0, 90, 91, 5, 33, 0, 0, 91, 99, 1, 0, 
    0, 0, 92, 93, 3, 26, 13, 0, 93, 94, 5, 33, 0, 0, 94, 95, 5, 15, 0, 0, 
    95, 96, 3, 22, 11, 0, 96, 97, 5, 16, 0, 0, 97, 99, 1, 0, 0, 0, 98, 89, 
    1, 0, 0, 0, 98, 92, 1, 0, 0, 0, 99, 9, 1, 0, 0, 0, 100, 101, 5, 29, 
    0, 0, 101, 104, 3, 46, 23, 0, 102, 104, 1, 0, 0, 0, 103, 100, 1, 0, 
    0, 0, 103, 102, 1, 0, 0, 0, 104, 11, 1, 0, 0, 0, 105, 106, 3, 26, 13, 
    0, 106, 107, 5, 33, 0, 0, 107, 108, 3, 16, 8, 0, 108, 109, 3, 30, 15, 
    0, 109, 13, 1, 0, 0, 0, 110, 111, 5, 5, 0, 0, 111, 112, 5, 32, 0, 0, 
    112, 113, 5, 29, 0, 0, 113, 114, 5, 11, 0, 0, 114, 115, 3, 24, 12, 0, 
    115, 116, 5, 12, 0, 0, 116, 117, 5, 31, 0, 0, 117, 15, 1, 0, 0, 0, 118, 
    119, 5, 11, 0, 0, 119, 120, 3, 18, 9, 0, 120, 121, 5, 12, 0, 0, 121, 
    128, 1, 0, 0, 0, 122, 123, 5, 11, 0, 0, 123, 124, 5, 9, 0, 0, 124, 128, 
    5, 12, 0, 0, 125, 126, 5, 11, 0, 0, 126, 128, 5, 12, 0, 0, 127, 118, 
    1, 0, 0, 0, 127, 122, 1, 0, 0, 0, 127, 125, 1, 0, 0, 0, 128, 17, 1, 
    0, 0, 0, 129, 130, 3, 20, 10, 0, 130, 131, 5, 30, 0, 0, 131, 132, 3, 
    18, 9, 0, 132, 135, 1, 0, 0, 0, 133, 135, 3, 20, 10, 0, 134, 129, 1, 
    0, 0, 0, 134, 133, 1, 0, 0, 0, 135, 19, 1, 0, 0, 0, 136, 137, 3, 8, 
    4, 0, 137, 21, 1, 0, 0, 0, 138, 142, 5, 34, 0, 0, 139, 142, 5, 35, 0, 
    0, 140, 142, 5, 36, 0, 0, 141, 138, 1, 0, 0, 0, 141, 139, 1, 0, 0, 0, 
    141, 140, 1, 0, 0, 0, 142, 23, 1, 0, 0, 0, 143, 144, 3, 28, 14, 0, 144, 
    145, 5, 30, 0, 0, 145, 146, 3, 24, 12, 0, 146, 149, 1, 0, 0, 0, 147, 
    149, 3, 28, 14, 0, 148, 143, 1, 0, 0, 0, 148, 147, 1, 0, 0, 0, 149, 
    25, 1, 0, 0, 0, 150, 156, 3, 28, 14, 0, 151, 152, 3, 28, 14, 0, 152, 
    153, 5, 10, 0, 0, 153, 154, 3, 26, 13, 0, 154, 156, 1, 0, 0, 0, 155, 
    150, 1, 0, 0, 0, 155, 151, 1, 0, 0, 0, 156, 27, 1, 0, 0, 0, 157, 162, 
    5, 7, 0, 0, 158, 162, 5, 8, 0, 0, 159, 162, 5, 9, 0, 0, 160, 162, 5, 
    32, 0, 0, 161, 157, 1, 0, 0, 0, 161, 158, 1, 0, 0, 0, 161, 159, 1, 0, 
    0, 0, 161, 160, 1, 0, 0, 0, 162, 29, 1, 0, 0, 0, 163, 164, 5, 13, 0, 
    0, 164, 165, 3, 32, 16, 0, 165, 166, 5, 14, 0, 0, 166, 31, 1, 0, 0, 
    0, 167, 168, 3, 34, 17, 0, 168, 169, 3, 32, 16, 0, 169, 172, 1, 0, 0, 
    0, 170, 172, 1, 0, 0, 0, 171, 167, 1, 0, 0, 0, 171, 170, 1, 0, 0, 0, 
    172, 33, 1, 0, 0, 0, 173, 182, 3, 6, 3, 0, 174, 182, 3, 44, 22, 0, 175, 
    182, 3, 36, 18, 0, 176, 182, 3, 38, 19, 0, 177, 182, 3, 40, 20, 0, 178, 
    182, 3, 42, 21, 0, 179, 182, 3, 30, 15, 0, 180, 182, 5, 31, 0, 0, 181, 
    173, 1, 0, 0, 0, 181, 174, 1, 0, 0, 0, 181, 175, 1, 0, 0, 0, 181, 176, 
    1, 0, 0, 0, 181, 177, 1, 0, 0, 0, 181, 178, 1, 0, 0, 0, 181, 179, 1, 
    0, 0, 0, 181, 180, 1, 0, 0, 0, 182, 35, 1, 0, 0, 0, 183, 184, 3, 46, 
    23, 0, 184, 185, 5, 31, 0, 0, 185, 37, 1, 0, 0, 0, 186, 187, 5, 2, 0, 
    0, 187, 188, 5, 11, 0, 0, 188, 189, 3, 46, 23, 0, 189, 190, 5, 12, 0, 
    0, 190, 191, 3, 34, 17, 0, 191, 201, 1, 0, 0, 0, 192, 193, 5, 2, 0, 
    0, 193, 194, 5, 11, 0, 0, 194, 195, 3, 46, 23, 0, 195, 196, 5, 12, 0, 
    0, 196, 197, 3, 34, 17, 0, 197, 198, 5, 3, 0, 0, 198, 199, 3, 34, 17, 
    0, 199, 201, 1, 0, 0, 0, 200, 186, 1, 0, 0, 0, 200, 192, 1, 0, 0, 0, 
    201, 39, 1, 0, 0, 0, 202, 203, 5, 1, 0, 0, 203, 204, 5, 11, 0, 0, 204, 
    205, 3, 46, 23, 0, 205, 206, 5, 12, 0, 0, 206, 207, 3, 34, 17, 0, 207, 
    41, 1, 0, 0, 0, 208, 209, 5, 4, 0, 0, 209, 215, 5, 31, 0, 0, 210, 211, 
    5, 4, 0, 0, 211, 212, 3, 46, 23, 0, 212, 213, 5, 31, 0, 0, 213, 215, 
    1, 0, 0, 0, 214, 208, 1, 0, 0, 0, 214, 210, 1, 0, 0, 0, 215, 43, 1, 
    0, 0, 0, 216, 217, 5, 6, 0, 0, 217, 218, 5, 33, 0, 0, 218, 219, 5, 29, 
    0, 0, 219, 220, 3, 46, 23, 0, 220, 221, 5, 31, 0, 0, 221, 45, 1, 0, 
    0, 0, 222, 225, 3, 48, 24, 0, 223, 225, 3, 52, 26, 0, 224, 222, 1, 0, 
    0, 0, 224, 223, 1, 0, 0, 0, 225, 47, 1, 0, 0, 0, 226, 227, 3, 50, 25, 
    0, 227, 228, 5, 29, 0, 0, 228, 229, 3, 46, 23, 0, 229, 49, 1, 0, 0, 
    0, 230, 237, 5, 33, 0, 0, 231, 232, 5, 33, 0, 0, 232, 233, 5, 15, 0, 
    0, 233, 234, 3, 46, 23, 0, 234, 235, 5, 16, 0, 0, 235, 237, 1, 0, 0, 
    0, 236, 230, 1, 0, 0, 0, 236, 231, 1, 0, 0, 0, 237, 51, 1, 0, 0, 0, 
    238, 239, 3, 54, 27, 0, 239, 240, 3, 62, 31, 0, 240, 241, 3, 54, 27, 
    0, 241, 244, 1, 0, 0, 0, 242, 244, 3, 54, 27, 0, 243, 238, 1, 0, 0, 
    0, 243, 242, 1, 0, 0, 0, 244, 53, 1, 0, 0, 0, 245, 246, 3, 56, 28, 0, 
    246, 247, 3, 54, 27, 0, 247, 250, 1, 0, 0, 0, 248, 250, 3, 56, 28, 0, 
    249, 245, 1, 0, 0, 0, 249, 248, 1, 0, 0, 0, 250, 55, 1, 0, 0, 0, 251, 
    252, 3, 58, 29, 0, 252, 253, 3, 64, 32, 0, 253, 254, 3, 56, 28, 0, 254, 
    257, 1, 0, 0, 0, 255, 257, 3, 58, 29, 0, 256, 251, 1, 0, 0, 0, 256, 
    255, 1, 0, 0, 0, 257, 57, 1, 0, 0, 0, 258, 259, 3, 60, 30, 0, 259, 260, 
    3, 66, 33, 0, 260, 261, 3, 58, 29, 0, 261, 264, 1, 0, 0, 0, 262, 264, 
    3, 60, 30, 0, 263, 258, 1, 0, 0, 0, 263, 262, 1, 0, 0, 0, 264, 59, 1, 
    0, 0, 0, 265, 269, 3, 68, 34, 0, 266, 269, 3, 50, 25, 0, 267, 269, 3, 
    22, 11, 0, 268, 265, 1, 0, 0, 0, 268, 266, 1, 0, 0, 0, 268, 267, 1, 
    0, 0, 0, 269, 61, 1, 0, 0, 0, 270, 271, 7, 0, 0, 0, 271, 63, 1, 0, 0, 
    0, 272, 273, 7, 1, 0, 0, 273, 65, 1, 0, 0, 0, 274, 275, 7, 2, 0, 0, 
    275, 67, 1, 0, 0, 0, 276, 277, 5, 11, 0, 0, 277, 278, 3, 70, 35, 0, 
    278, 279, 5, 12, 0, 0, 279, 69, 1, 0, 0, 0, 280, 281, 3, 46, 23, 0, 
    281, 282, 5, 30, 0, 0, 282, 283, 3, 70, 35, 0, 283, 287, 1, 0, 0, 0, 
    284, 287, 3, 46, 23, 0, 285, 287, 1, 0, 0, 0, 286, 280, 1, 0, 0, 0, 
    286, 284, 1, 0, 0, 0, 286, 285, 1, 0, 0, 0, 287, 71, 1, 0, 0, 0, 22, 
    78, 83, 98, 103, 127, 134, 141, 148, 155, 161, 171, 181, 200, 214, 224, 
    236, 243, 249, 256, 263, 268, 286
]);