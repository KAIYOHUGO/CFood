// Generated from ./CFood.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unused_variables)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::Arena;
use dbt_antlr4::atn::ATN;
use dbt_antlr4::char_stream::CharStream;
use dbt_antlr4::int_stream::IntStream;
use dbt_antlr4::lexer::{BaseLexer, LexerRecog, Lexer as _};
use dbt_antlr4::atn_config_set::LexerATNConfigSet;
use dbt_antlr4::atn_deserializer::ATNDeserializer;
use dbt_antlr4::atn_simulator::BaseATNSimulator;
use dbt_antlr4::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;
use dbt_antlr4::TokenSource;
use dbt_antlr4::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use dbt_antlr4::PredictionContextCache;
use dbt_antlr4::recognizer::Actions;
use dbt_antlr4::token_factory::{CommonTokenFactory, TokenFactory};
use dbt_antlr4::rule_context::{BaseRuleContext,EmptyNodeKind,EmptyCustomRuleContext,EmptyRuleNode};
use dbt_antlr4::vocabulary::{Vocabulary,VocabularyImpl};

use std::ops::{DerefMut, Deref};
use std::sync::LazyLock;

dbt_antlr4::check_version!("1","3");
pub const KW_while:i32=1; 
pub const KW_for:i32=2; 
pub const KW_if:i32=3; 
pub const KW_else:i32=4; 
pub const KW_return:i32=5; 
pub const KW_type:i32=6; 
pub const KW_let:i32=7; 
pub const KW_as:i32=8; 
pub const LIT_true:i32=9; 
pub const LIT_false:i32=10; 
pub const TY_int:i32=11; 
pub const TY_float:i32=12; 
pub const TY_str:i32=13; 
pub const TY_void:i32=14; 
pub const TY_bool:i32=15; 
pub const MAGIC_printf:i32=16; 
pub const MAGIC_scanf:i32=17; 
pub const MAGIC_new:i32=18; 
pub const PAREN_L:i32=19; 
pub const PAREN_R:i32=20; 
pub const BRACE_L:i32=21; 
pub const BRACE_R:i32=22; 
pub const NE:i32=23; 
pub const EQ:i32=24; 
pub const LT:i32=25; 
pub const GT:i32=26; 
pub const LE:i32=27; 
pub const GE:i32=28; 
pub const NOT:i32=29; 
pub const AND:i32=30; 
pub const OR:i32=31; 
pub const PLUS:i32=32; 
pub const SUB:i32=33; 
pub const MOD:i32=34; 
pub const MUL:i32=35; 
pub const DIV:i32=36; 
pub const PEO:i32=37; 
pub const ASSIGN:i32=38; 
pub const COMMA:i32=39; 
pub const REFER:i32=40; 
pub const SEMICOLON:i32=41; 
pub const TYPE:i32=42; 
pub const IDENT:i32=43; 
pub const INT:i32=44; 
pub const FLOAT:i32=45; 
pub const CONSTR:i32=46; 
pub const LINE_COMMENT:i32=47; 
pub const COMMENT:i32=48; 
pub const WS:i32=49;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;49] = [
    "KW_while", "KW_for", "KW_if", "KW_else", "KW_return", "KW_type", "KW_let", 
    "KW_as", "LIT_true", "LIT_false", "TY_int", "TY_float", "TY_str", "TY_void", 
    "TY_bool", "MAGIC_printf", "MAGIC_scanf", "MAGIC_new", "PAREN_L", "PAREN_R", 
    "BRACE_L", "BRACE_R", "NE", "EQ", "LT", "GT", "LE", "GE", "NOT", "AND", 
    "OR", "PLUS", "SUB", "MOD", "MUL", "DIV", "PEO", "ASSIGN", "COMMA", 
    "REFER", "SEMICOLON", "TYPE", "IDENT", "INT", "FLOAT", "CONSTR", "LINE_COMMENT", 
    "COMMENT", "WS"
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

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, CFoodLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct CFoodLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr4::impl_token_source! { CFoodLexer }
dbt_antlr4::impl_deref! { lexer => CFoodLexer }

impl<'input, 'arena, Input, TF> CFoodLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = CFoodLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct CFoodLexerActions {
}

impl CFoodLexerActions {
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseLexerType<'input, 'arena, Input, TF>, TF::Tok>
    for CFoodLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
 {}

impl<'input, 'arena, Input, TF> LexerRecog<'input, 'arena, TF, BaseLexerType<'input, 'arena, Input, TF>>
    for CFoodLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn get_rule_names(&self) -> &'static [&'static str] { &ruleNames }
    fn get_literal_names(&self) -> &[Option<&str>] { &_LITERAL_NAMES }
    fn get_symbolic_names(&self) -> &[Option<&str>] { &_SYMBOLIC_NAMES }
    fn get_grammar_file_name(&self) -> &'static str { "CFoodLexer.g4" }
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: LazyLock<Vec<i32>> = LazyLock::new(|| vec![
    4, 0, 49, 314, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 
    14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 
    19, 2, 20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 
    24, 2, 25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 
    29, 2, 30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 
    34, 2, 35, 7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 
    39, 2, 40, 7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 
    44, 2, 45, 7, 45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 48, 1, 0, 1, 
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 
    1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 
    4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 1, 7, 1, 7, 
    1, 7, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 
    9, 1, 10, 1, 10, 1, 10, 1, 10, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 
    11, 1, 12, 1, 12, 1, 12, 1, 12, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 
    14, 1, 14, 1, 14, 1, 14, 1, 14, 1, 15, 1, 15, 1, 15, 1, 15, 1, 15, 1, 
    15, 1, 15, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 17, 1, 17, 1, 
    17, 1, 17, 1, 18, 1, 18, 1, 19, 1, 19, 1, 20, 1, 20, 1, 21, 1, 21, 1, 
    22, 1, 22, 1, 22, 1, 23, 1, 23, 1, 23, 1, 24, 1, 24, 1, 25, 1, 25, 1, 
    26, 1, 26, 1, 26, 1, 27, 1, 27, 1, 27, 1, 28, 1, 28, 1, 29, 1, 29, 1, 
    29, 1, 30, 1, 30, 1, 30, 1, 31, 1, 31, 1, 32, 1, 32, 1, 33, 1, 33, 1, 
    34, 1, 34, 1, 35, 1, 35, 1, 36, 1, 36, 1, 36, 1, 37, 1, 37, 1, 38, 1, 
    38, 1, 39, 1, 39, 1, 40, 1, 40, 1, 41, 1, 41, 5, 41, 244, 8, 41, 10, 
    41, 12, 41, 247, 9, 41, 1, 42, 1, 42, 5, 42, 251, 8, 42, 10, 42, 12, 
    42, 254, 9, 42, 1, 43, 4, 43, 257, 8, 43, 11, 43, 12, 43, 258, 1, 44, 
    4, 44, 262, 8, 44, 11, 44, 12, 44, 263, 1, 44, 1, 44, 4, 44, 268, 8, 
    44, 11, 44, 12, 44, 269, 1, 45, 1, 45, 5, 45, 274, 8, 45, 10, 45, 12, 
    45, 277, 9, 45, 1, 45, 1, 45, 1, 46, 1, 46, 1, 46, 1, 46, 5, 46, 285, 
    8, 46, 10, 46, 12, 46, 288, 9, 46, 1, 46, 1, 46, 1, 46, 1, 46, 1, 47, 
    1, 47, 1, 47, 1, 47, 5, 47, 298, 8, 47, 10, 47, 12, 47, 301, 9, 47, 
    1, 47, 1, 47, 1, 47, 1, 47, 1, 47, 1, 48, 4, 48, 309, 8, 48, 11, 48, 
    12, 48, 310, 1, 48, 1, 48, 3, 275, 286, 299, 0, 49, 1, 1, 3, 2, 5, 3, 
    7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 11, 23, 12, 25, 
    13, 27, 14, 29, 15, 31, 16, 33, 17, 35, 18, 37, 19, 39, 20, 41, 21, 
    43, 22, 45, 23, 47, 24, 49, 25, 51, 26, 53, 27, 55, 28, 57, 29, 59, 
    30, 61, 31, 63, 32, 65, 33, 67, 34, 69, 35, 71, 36, 73, 37, 75, 38, 
    77, 39, 79, 40, 81, 41, 83, 42, 85, 43, 87, 44, 89, 45, 91, 46, 93, 
    47, 95, 48, 97, 49, 1, 0, 5, 1, 0, 65, 90, 4, 0, 48, 57, 65, 90, 95, 
    95, 97, 122, 2, 0, 95, 95, 97, 122, 1, 0, 48, 57, 3, 0, 9, 10, 13, 13, 
    32, 32, 322, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 
    7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 
    0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 
    0, 0, 0, 0, 23, 1, 0, 0, 0, 0, 25, 1, 0, 0, 0, 0, 27, 1, 0, 0, 0, 0, 
    29, 1, 0, 0, 0, 0, 31, 1, 0, 0, 0, 0, 33, 1, 0, 0, 0, 0, 35, 1, 0, 0, 
    0, 0, 37, 1, 0, 0, 0, 0, 39, 1, 0, 0, 0, 0, 41, 1, 0, 0, 0, 0, 43, 1, 
    0, 0, 0, 0, 45, 1, 0, 0, 0, 0, 47, 1, 0, 0, 0, 0, 49, 1, 0, 0, 0, 0, 
    51, 1, 0, 0, 0, 0, 53, 1, 0, 0, 0, 0, 55, 1, 0, 0, 0, 0, 57, 1, 0, 0, 
    0, 0, 59, 1, 0, 0, 0, 0, 61, 1, 0, 0, 0, 0, 63, 1, 0, 0, 0, 0, 65, 1, 
    0, 0, 0, 0, 67, 1, 0, 0, 0, 0, 69, 1, 0, 0, 0, 0, 71, 1, 0, 0, 0, 0, 
    73, 1, 0, 0, 0, 0, 75, 1, 0, 0, 0, 0, 77, 1, 0, 0, 0, 0, 79, 1, 0, 0, 
    0, 0, 81, 1, 0, 0, 0, 0, 83, 1, 0, 0, 0, 0, 85, 1, 0, 0, 0, 0, 87, 1, 
    0, 0, 0, 0, 89, 1, 0, 0, 0, 0, 91, 1, 0, 0, 0, 0, 93, 1, 0, 0, 0, 0, 
    95, 1, 0, 0, 0, 0, 97, 1, 0, 0, 0, 1, 99, 1, 0, 0, 0, 3, 105, 1, 0, 
    0, 0, 5, 109, 1, 0, 0, 0, 7, 112, 1, 0, 0, 0, 9, 117, 1, 0, 0, 0, 11, 
    124, 1, 0, 0, 0, 13, 129, 1, 0, 0, 0, 15, 133, 1, 0, 0, 0, 17, 136, 
    1, 0, 0, 0, 19, 141, 1, 0, 0, 0, 21, 147, 1, 0, 0, 0, 23, 151, 1, 0, 
    0, 0, 25, 157, 1, 0, 0, 0, 27, 161, 1, 0, 0, 0, 29, 166, 1, 0, 0, 0, 
    31, 171, 1, 0, 0, 0, 33, 178, 1, 0, 0, 0, 35, 184, 1, 0, 0, 0, 37, 188, 
    1, 0, 0, 0, 39, 190, 1, 0, 0, 0, 41, 192, 1, 0, 0, 0, 43, 194, 1, 0, 
    0, 0, 45, 196, 1, 0, 0, 0, 47, 199, 1, 0, 0, 0, 49, 202, 1, 0, 0, 0, 
    51, 204, 1, 0, 0, 0, 53, 206, 1, 0, 0, 0, 55, 209, 1, 0, 0, 0, 57, 212, 
    1, 0, 0, 0, 59, 214, 1, 0, 0, 0, 61, 217, 1, 0, 0, 0, 63, 220, 1, 0, 
    0, 0, 65, 222, 1, 0, 0, 0, 67, 224, 1, 0, 0, 0, 69, 226, 1, 0, 0, 0, 
    71, 228, 1, 0, 0, 0, 73, 230, 1, 0, 0, 0, 75, 233, 1, 0, 0, 0, 77, 235, 
    1, 0, 0, 0, 79, 237, 1, 0, 0, 0, 81, 239, 1, 0, 0, 0, 83, 241, 1, 0, 
    0, 0, 85, 248, 1, 0, 0, 0, 87, 256, 1, 0, 0, 0, 89, 261, 1, 0, 0, 0, 
    91, 271, 1, 0, 0, 0, 93, 280, 1, 0, 0, 0, 95, 293, 1, 0, 0, 0, 97, 308, 
    1, 0, 0, 0, 99, 100, 5, 119, 0, 0, 100, 101, 5, 104, 0, 0, 101, 102, 
    5, 105, 0, 0, 102, 103, 5, 108, 0, 0, 103, 104, 5, 101, 0, 0, 104, 2, 
    1, 0, 0, 0, 105, 106, 5, 102, 0, 0, 106, 107, 5, 111, 0, 0, 107, 108, 
    5, 114, 0, 0, 108, 4, 1, 0, 0, 0, 109, 110, 5, 105, 0, 0, 110, 111, 
    5, 102, 0, 0, 111, 6, 1, 0, 0, 0, 112, 113, 5, 101, 0, 0, 113, 114, 
    5, 108, 0, 0, 114, 115, 5, 115, 0, 0, 115, 116, 5, 101, 0, 0, 116, 8, 
    1, 0, 0, 0, 117, 118, 5, 114, 0, 0, 118, 119, 5, 101, 0, 0, 119, 120, 
    5, 116, 0, 0, 120, 121, 5, 117, 0, 0, 121, 122, 5, 114, 0, 0, 122, 123, 
    5, 110, 0, 0, 123, 10, 1, 0, 0, 0, 124, 125, 5, 116, 0, 0, 125, 126, 
    5, 121, 0, 0, 126, 127, 5, 112, 0, 0, 127, 128, 5, 101, 0, 0, 128, 12, 
    1, 0, 0, 0, 129, 130, 5, 108, 0, 0, 130, 131, 5, 101, 0, 0, 131, 132, 
    5, 116, 0, 0, 132, 14, 1, 0, 0, 0, 133, 134, 5, 97, 0, 0, 134, 135, 
    5, 115, 0, 0, 135, 16, 1, 0, 0, 0, 136, 137, 5, 116, 0, 0, 137, 138, 
    5, 114, 0, 0, 138, 139, 5, 117, 0, 0, 139, 140, 5, 101, 0, 0, 140, 18, 
    1, 0, 0, 0, 141, 142, 5, 102, 0, 0, 142, 143, 5, 97, 0, 0, 143, 144, 
    5, 108, 0, 0, 144, 145, 5, 115, 0, 0, 145, 146, 5, 101, 0, 0, 146, 20, 
    1, 0, 0, 0, 147, 148, 5, 105, 0, 0, 148, 149, 5, 110, 0, 0, 149, 150, 
    5, 116, 0, 0, 150, 22, 1, 0, 0, 0, 151, 152, 5, 102, 0, 0, 152, 153, 
    5, 108, 0, 0, 153, 154, 5, 111, 0, 0, 154, 155, 5, 97, 0, 0, 155, 156, 
    5, 116, 0, 0, 156, 24, 1, 0, 0, 0, 157, 158, 5, 115, 0, 0, 158, 159, 
    5, 116, 0, 0, 159, 160, 5, 114, 0, 0, 160, 26, 1, 0, 0, 0, 161, 162, 
    5, 118, 0, 0, 162, 163, 5, 111, 0, 0, 163, 164, 5, 105, 0, 0, 164, 165, 
    5, 100, 0, 0, 165, 28, 1, 0, 0, 0, 166, 167, 5, 98, 0, 0, 167, 168, 
    5, 111, 0, 0, 168, 169, 5, 111, 0, 0, 169, 170, 5, 108, 0, 0, 170, 30, 
    1, 0, 0, 0, 171, 172, 5, 112, 0, 0, 172, 173, 5, 114, 0, 0, 173, 174, 
    5, 105, 0, 0, 174, 175, 5, 110, 0, 0, 175, 176, 5, 116, 0, 0, 176, 177, 
    5, 102, 0, 0, 177, 32, 1, 0, 0, 0, 178, 179, 5, 115, 0, 0, 179, 180, 
    5, 99, 0, 0, 180, 181, 5, 97, 0, 0, 181, 182, 5, 110, 0, 0, 182, 183, 
    5, 102, 0, 0, 183, 34, 1, 0, 0, 0, 184, 185, 5, 110, 0, 0, 185, 186, 
    5, 101, 0, 0, 186, 187, 5, 119, 0, 0, 187, 36, 1, 0, 0, 0, 188, 189, 
    5, 40, 0, 0, 189, 38, 1, 0, 0, 0, 190, 191, 5, 41, 0, 0, 191, 40, 1, 
    0, 0, 0, 192, 193, 5, 123, 0, 0, 193, 42, 1, 0, 0, 0, 194, 195, 5, 125, 
    0, 0, 195, 44, 1, 0, 0, 0, 196, 197, 5, 33, 0, 0, 197, 198, 5, 61, 0, 
    0, 198, 46, 1, 0, 0, 0, 199, 200, 5, 61, 0, 0, 200, 201, 5, 61, 0, 0, 
    201, 48, 1, 0, 0, 0, 202, 203, 5, 60, 0, 0, 203, 50, 1, 0, 0, 0, 204, 
    205, 5, 62, 0, 0, 205, 52, 1, 0, 0, 0, 206, 207, 5, 60, 0, 0, 207, 208, 
    5, 61, 0, 0, 208, 54, 1, 0, 0, 0, 209, 210, 5, 62, 0, 0, 210, 211, 5, 
    61, 0, 0, 211, 56, 1, 0, 0, 0, 212, 213, 5, 33, 0, 0, 213, 58, 1, 0, 
    0, 0, 214, 215, 5, 38, 0, 0, 215, 216, 5, 38, 0, 0, 216, 60, 1, 0, 0, 
    0, 217, 218, 5, 124, 0, 0, 218, 219, 5, 124, 0, 0, 219, 62, 1, 0, 0, 
    0, 220, 221, 5, 43, 0, 0, 221, 64, 1, 0, 0, 0, 222, 223, 5, 45, 0, 0, 
    223, 66, 1, 0, 0, 0, 224, 225, 5, 37, 0, 0, 225, 68, 1, 0, 0, 0, 226, 
    227, 5, 42, 0, 0, 227, 70, 1, 0, 0, 0, 228, 229, 5, 47, 0, 0, 229, 72, 
    1, 0, 0, 0, 230, 231, 5, 35, 0, 0, 231, 232, 5, 35, 0, 0, 232, 74, 1, 
    0, 0, 0, 233, 234, 5, 61, 0, 0, 234, 76, 1, 0, 0, 0, 235, 236, 5, 44, 
    0, 0, 236, 78, 1, 0, 0, 0, 237, 238, 5, 38, 0, 0, 238, 80, 1, 0, 0, 
    0, 239, 240, 5, 59, 0, 0, 240, 82, 1, 0, 0, 0, 241, 245, 7, 0, 0, 0, 
    242, 244, 7, 1, 0, 0, 243, 242, 1, 0, 0, 0, 244, 247, 1, 0, 0, 0, 245, 
    243, 1, 0, 0, 0, 245, 246, 1, 0, 0, 0, 246, 84, 1, 0, 0, 0, 247, 245, 
    1, 0, 0, 0, 248, 252, 7, 2, 0, 0, 249, 251, 7, 1, 0, 0, 250, 249, 1, 
    0, 0, 0, 251, 254, 1, 0, 0, 0, 252, 250, 1, 0, 0, 0, 252, 253, 1, 0, 
    0, 0, 253, 86, 1, 0, 0, 0, 254, 252, 1, 0, 0, 0, 255, 257, 7, 3, 0, 
    0, 256, 255, 1, 0, 0, 0, 257, 258, 1, 0, 0, 0, 258, 256, 1, 0, 0, 0, 
    258, 259, 1, 0, 0, 0, 259, 88, 1, 0, 0, 0, 260, 262, 7, 3, 0, 0, 261, 
    260, 1, 0, 0, 0, 262, 263, 1, 0, 0, 0, 263, 261, 1, 0, 0, 0, 263, 264, 
    1, 0, 0, 0, 264, 265, 1, 0, 0, 0, 265, 267, 5, 46, 0, 0, 266, 268, 7, 
    3, 0, 0, 267, 266, 1, 0, 0, 0, 268, 269, 1, 0, 0, 0, 269, 267, 1, 0, 
    0, 0, 269, 270, 1, 0, 0, 0, 270, 90, 1, 0, 0, 0, 271, 275, 5, 34, 0, 
    0, 272, 274, 9, 0, 0, 0, 273, 272, 1, 0, 0, 0, 274, 277, 1, 0, 0, 0, 
    275, 276, 1, 0, 0, 0, 275, 273, 1, 0, 0, 0, 276, 278, 1, 0, 0, 0, 277, 
    275, 1, 0, 0, 0, 278, 279, 5, 34, 0, 0, 279, 92, 1, 0, 0, 0, 280, 281, 
    5, 47, 0, 0, 281, 282, 5, 47, 0, 0, 282, 286, 1, 0, 0, 0, 283, 285, 
    9, 0, 0, 0, 284, 283, 1, 0, 0, 0, 285, 288, 1, 0, 0, 0, 286, 287, 1, 
    0, 0, 0, 286, 284, 1, 0, 0, 0, 287, 289, 1, 0, 0, 0, 288, 286, 1, 0, 
    0, 0, 289, 290, 5, 10, 0, 0, 290, 291, 1, 0, 0, 0, 291, 292, 6, 46, 
    0, 0, 292, 94, 1, 0, 0, 0, 293, 294, 5, 47, 0, 0, 294, 295, 5, 42, 0, 
    0, 295, 299, 1, 0, 0, 0, 296, 298, 9, 0, 0, 0, 297, 296, 1, 0, 0, 0, 
    298, 301, 1, 0, 0, 0, 299, 300, 1, 0, 0, 0, 299, 297, 1, 0, 0, 0, 300, 
    302, 1, 0, 0, 0, 301, 299, 1, 0, 0, 0, 302, 303, 5, 42, 0, 0, 303, 304, 
    5, 47, 0, 0, 304, 305, 1, 0, 0, 0, 305, 306, 6, 47, 0, 0, 306, 96, 1, 
    0, 0, 0, 307, 309, 7, 4, 0, 0, 308, 307, 1, 0, 0, 0, 309, 310, 1, 0, 
    0, 0, 310, 308, 1, 0, 0, 0, 310, 311, 1, 0, 0, 0, 311, 312, 1, 0, 0, 
    0, 312, 313, 6, 48, 0, 0, 313, 98, 1, 0, 0, 0, 10, 0, 245, 252, 258, 
    263, 269, 275, 286, 299, 310, 1, 6, 0, 0
]);