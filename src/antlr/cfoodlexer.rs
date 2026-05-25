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
pub const KW_if:i32=2; 
pub const KW_else:i32=3; 
pub const KW_return:i32=4; 
pub const KW_type:i32=5; 
pub const KW_let:i32=6; 
pub const TY_int:i32=7; 
pub const TY_float:i32=8; 
pub const TY_str:i32=9; 
pub const TY_void:i32=10; 
pub const MAGIC_printf:i32=11; 
pub const MAGIC_scanf:i32=12; 
pub const ARROW:i32=13; 
pub const PAREN_L:i32=14; 
pub const PAREN_R:i32=15; 
pub const BRACE_L:i32=16; 
pub const BRACE_R:i32=17; 
pub const NE:i32=18; 
pub const EQ:i32=19; 
pub const LT:i32=20; 
pub const GT:i32=21; 
pub const LE:i32=22; 
pub const GE:i32=23; 
pub const PLUS:i32=24; 
pub const SUB:i32=25; 
pub const MOD:i32=26; 
pub const MUL:i32=27; 
pub const DIV:i32=28; 
pub const PEO:i32=29; 
pub const ASSIGN:i32=30; 
pub const COMMA:i32=31; 
pub const SEMICOLON:i32=32; 
pub const TYPE:i32=33; 
pub const IDENT:i32=34; 
pub const INT:i32=35; 
pub const FLOAT:i32=36; 
pub const CONSTR:i32=37; 
pub const LINE_COMMENT:i32=38; 
pub const COMMENT:i32=39; 
pub const WS:i32=40;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;40] = [
    "KW_while", "KW_if", "KW_else", "KW_return", "KW_type", "KW_let", "TY_int", 
    "TY_float", "TY_str", "TY_void", "MAGIC_printf", "MAGIC_scanf", "ARROW", 
    "PAREN_L", "PAREN_R", "BRACE_L", "BRACE_R", "NE", "EQ", "LT", "GT", 
    "LE", "GE", "PLUS", "SUB", "MOD", "MUL", "DIV", "PEO", "ASSIGN", "COMMA", 
    "SEMICOLON", "TYPE", "IDENT", "INT", "FLOAT", "CONSTR", "LINE_COMMENT", 
    "COMMENT", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;33] = [
	None, Some("'while'"), Some("'if'"), Some("'else'"), Some("'return'"), 
	Some("'type'"), Some("'let'"), Some("'int'"), Some("'float'"), Some("'str'"), 
	Some("'void'"), Some("'printf'"), Some("'scanf'"), Some("'->'"), Some("'('"), 
	Some("')'"), Some("'{'"), Some("'}'"), Some("'!='"), Some("'=='"), Some("'<'"), 
	Some("'>'"), Some("'<='"), Some("'>='"), Some("'+'"), Some("'-'"), Some("'%'"), 
	Some("'*'"), Some("'/'"), Some("'##'"), Some("'='"), Some("','"), Some("';'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;41]  = [
	None, Some("KW_while"), Some("KW_if"), Some("KW_else"), Some("KW_return"), 
	Some("KW_type"), Some("KW_let"), Some("TY_int"), Some("TY_float"), Some("TY_str"), 
	Some("TY_void"), Some("MAGIC_printf"), Some("MAGIC_scanf"), Some("ARROW"), 
	Some("PAREN_L"), Some("PAREN_R"), Some("BRACE_L"), Some("BRACE_R"), Some("NE"), 
	Some("EQ"), Some("LT"), Some("GT"), Some("LE"), Some("GE"), Some("PLUS"), 
	Some("SUB"), Some("MOD"), Some("MUL"), Some("DIV"), Some("PEO"), Some("ASSIGN"), 
	Some("COMMA"), Some("SEMICOLON"), Some("TYPE"), Some("IDENT"), Some("INT"), 
	Some("FLOAT"), Some("CONSTR"), Some("LINE_COMMENT"), Some("COMMENT"), Some("WS")
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
    4, 0, 40, 262, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 
    14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 
    19, 2, 20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 
    24, 2, 25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 
    29, 2, 30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 
    34, 2, 35, 7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 
    39, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 
    1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 
    4, 1, 4, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 
    1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 8, 1, 8, 1, 8, 1, 8, 1, 9, 1, 
    9, 1, 9, 1, 9, 1, 9, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 
    1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 12, 1, 12, 1, 12, 1, 13, 
    1, 13, 1, 14, 1, 14, 1, 15, 1, 15, 1, 16, 1, 16, 1, 17, 1, 17, 1, 17, 
    1, 18, 1, 18, 1, 18, 1, 19, 1, 19, 1, 20, 1, 20, 1, 21, 1, 21, 1, 21, 
    1, 22, 1, 22, 1, 22, 1, 23, 1, 23, 1, 24, 1, 24, 1, 25, 1, 25, 1, 26, 
    1, 26, 1, 27, 1, 27, 1, 28, 1, 28, 1, 28, 1, 29, 1, 29, 1, 30, 1, 30, 
    1, 31, 1, 31, 1, 32, 1, 32, 5, 32, 192, 8, 32, 10, 32, 12, 32, 195, 
    9, 32, 1, 33, 1, 33, 5, 33, 199, 8, 33, 10, 33, 12, 33, 202, 9, 33, 
    1, 34, 4, 34, 205, 8, 34, 11, 34, 12, 34, 206, 1, 35, 4, 35, 210, 8, 
    35, 11, 35, 12, 35, 211, 1, 35, 1, 35, 4, 35, 216, 8, 35, 11, 35, 12, 
    35, 217, 1, 36, 1, 36, 5, 36, 222, 8, 36, 10, 36, 12, 36, 225, 9, 36, 
    1, 36, 1, 36, 1, 37, 1, 37, 1, 37, 1, 37, 5, 37, 233, 8, 37, 10, 37, 
    12, 37, 236, 9, 37, 1, 37, 1, 37, 1, 37, 1, 37, 1, 38, 1, 38, 1, 38, 
    1, 38, 5, 38, 246, 8, 38, 10, 38, 12, 38, 249, 9, 38, 1, 38, 1, 38, 
    1, 38, 1, 38, 1, 38, 1, 39, 4, 39, 257, 8, 39, 11, 39, 12, 39, 258, 
    1, 39, 1, 39, 3, 223, 234, 247, 0, 40, 1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 
    11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 11, 23, 12, 25, 13, 27, 14, 
    29, 15, 31, 16, 33, 17, 35, 18, 37, 19, 39, 20, 41, 21, 43, 22, 45, 
    23, 47, 24, 49, 25, 51, 26, 53, 27, 55, 28, 57, 29, 59, 30, 61, 31, 
    63, 32, 65, 33, 67, 34, 69, 35, 71, 36, 73, 37, 75, 38, 77, 39, 79, 
    40, 1, 0, 5, 1, 0, 65, 90, 4, 0, 48, 57, 65, 90, 95, 95, 97, 122, 2, 
    0, 95, 95, 97, 122, 1, 0, 48, 57, 3, 0, 9, 10, 13, 13, 32, 32, 270, 
    0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 
    0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 0, 0, 15, 1, 
    0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 0, 0, 0, 0, 
    23, 1, 0, 0, 0, 0, 25, 1, 0, 0, 0, 0, 27, 1, 0, 0, 0, 0, 29, 1, 0, 0, 
    0, 0, 31, 1, 0, 0, 0, 0, 33, 1, 0, 0, 0, 0, 35, 1, 0, 0, 0, 0, 37, 1, 
    0, 0, 0, 0, 39, 1, 0, 0, 0, 0, 41, 1, 0, 0, 0, 0, 43, 1, 0, 0, 0, 0, 
    45, 1, 0, 0, 0, 0, 47, 1, 0, 0, 0, 0, 49, 1, 0, 0, 0, 0, 51, 1, 0, 0, 
    0, 0, 53, 1, 0, 0, 0, 0, 55, 1, 0, 0, 0, 0, 57, 1, 0, 0, 0, 0, 59, 1, 
    0, 0, 0, 0, 61, 1, 0, 0, 0, 0, 63, 1, 0, 0, 0, 0, 65, 1, 0, 0, 0, 0, 
    67, 1, 0, 0, 0, 0, 69, 1, 0, 0, 0, 0, 71, 1, 0, 0, 0, 0, 73, 1, 0, 0, 
    0, 0, 75, 1, 0, 0, 0, 0, 77, 1, 0, 0, 0, 0, 79, 1, 0, 0, 0, 1, 81, 1, 
    0, 0, 0, 3, 87, 1, 0, 0, 0, 5, 90, 1, 0, 0, 0, 7, 95, 1, 0, 0, 0, 9, 
    102, 1, 0, 0, 0, 11, 107, 1, 0, 0, 0, 13, 111, 1, 0, 0, 0, 15, 115, 
    1, 0, 0, 0, 17, 121, 1, 0, 0, 0, 19, 125, 1, 0, 0, 0, 21, 130, 1, 0, 
    0, 0, 23, 137, 1, 0, 0, 0, 25, 143, 1, 0, 0, 0, 27, 146, 1, 0, 0, 0, 
    29, 148, 1, 0, 0, 0, 31, 150, 1, 0, 0, 0, 33, 152, 1, 0, 0, 0, 35, 154, 
    1, 0, 0, 0, 37, 157, 1, 0, 0, 0, 39, 160, 1, 0, 0, 0, 41, 162, 1, 0, 
    0, 0, 43, 164, 1, 0, 0, 0, 45, 167, 1, 0, 0, 0, 47, 170, 1, 0, 0, 0, 
    49, 172, 1, 0, 0, 0, 51, 174, 1, 0, 0, 0, 53, 176, 1, 0, 0, 0, 55, 178, 
    1, 0, 0, 0, 57, 180, 1, 0, 0, 0, 59, 183, 1, 0, 0, 0, 61, 185, 1, 0, 
    0, 0, 63, 187, 1, 0, 0, 0, 65, 189, 1, 0, 0, 0, 67, 196, 1, 0, 0, 0, 
    69, 204, 1, 0, 0, 0, 71, 209, 1, 0, 0, 0, 73, 219, 1, 0, 0, 0, 75, 228, 
    1, 0, 0, 0, 77, 241, 1, 0, 0, 0, 79, 256, 1, 0, 0, 0, 81, 82, 5, 119, 
    0, 0, 82, 83, 5, 104, 0, 0, 83, 84, 5, 105, 0, 0, 84, 85, 5, 108, 0, 
    0, 85, 86, 5, 101, 0, 0, 86, 2, 1, 0, 0, 0, 87, 88, 5, 105, 0, 0, 88, 
    89, 5, 102, 0, 0, 89, 4, 1, 0, 0, 0, 90, 91, 5, 101, 0, 0, 91, 92, 5, 
    108, 0, 0, 92, 93, 5, 115, 0, 0, 93, 94, 5, 101, 0, 0, 94, 6, 1, 0, 
    0, 0, 95, 96, 5, 114, 0, 0, 96, 97, 5, 101, 0, 0, 97, 98, 5, 116, 0, 
    0, 98, 99, 5, 117, 0, 0, 99, 100, 5, 114, 0, 0, 100, 101, 5, 110, 0, 
    0, 101, 8, 1, 0, 0, 0, 102, 103, 5, 116, 0, 0, 103, 104, 5, 121, 0, 
    0, 104, 105, 5, 112, 0, 0, 105, 106, 5, 101, 0, 0, 106, 10, 1, 0, 0, 
    0, 107, 108, 5, 108, 0, 0, 108, 109, 5, 101, 0, 0, 109, 110, 5, 116, 
    0, 0, 110, 12, 1, 0, 0, 0, 111, 112, 5, 105, 0, 0, 112, 113, 5, 110, 
    0, 0, 113, 114, 5, 116, 0, 0, 114, 14, 1, 0, 0, 0, 115, 116, 5, 102, 
    0, 0, 116, 117, 5, 108, 0, 0, 117, 118, 5, 111, 0, 0, 118, 119, 5, 97, 
    0, 0, 119, 120, 5, 116, 0, 0, 120, 16, 1, 0, 0, 0, 121, 122, 5, 115, 
    0, 0, 122, 123, 5, 116, 0, 0, 123, 124, 5, 114, 0, 0, 124, 18, 1, 0, 
    0, 0, 125, 126, 5, 118, 0, 0, 126, 127, 5, 111, 0, 0, 127, 128, 5, 105, 
    0, 0, 128, 129, 5, 100, 0, 0, 129, 20, 1, 0, 0, 0, 130, 131, 5, 112, 
    0, 0, 131, 132, 5, 114, 0, 0, 132, 133, 5, 105, 0, 0, 133, 134, 5, 110, 
    0, 0, 134, 135, 5, 116, 0, 0, 135, 136, 5, 102, 0, 0, 136, 22, 1, 0, 
    0, 0, 137, 138, 5, 115, 0, 0, 138, 139, 5, 99, 0, 0, 139, 140, 5, 97, 
    0, 0, 140, 141, 5, 110, 0, 0, 141, 142, 5, 102, 0, 0, 142, 24, 1, 0, 
    0, 0, 143, 144, 5, 45, 0, 0, 144, 145, 5, 62, 0, 0, 145, 26, 1, 0, 0, 
    0, 146, 147, 5, 40, 0, 0, 147, 28, 1, 0, 0, 0, 148, 149, 5, 41, 0, 0, 
    149, 30, 1, 0, 0, 0, 150, 151, 5, 123, 0, 0, 151, 32, 1, 0, 0, 0, 152, 
    153, 5, 125, 0, 0, 153, 34, 1, 0, 0, 0, 154, 155, 5, 33, 0, 0, 155, 
    156, 5, 61, 0, 0, 156, 36, 1, 0, 0, 0, 157, 158, 5, 61, 0, 0, 158, 159, 
    5, 61, 0, 0, 159, 38, 1, 0, 0, 0, 160, 161, 5, 60, 0, 0, 161, 40, 1, 
    0, 0, 0, 162, 163, 5, 62, 0, 0, 163, 42, 1, 0, 0, 0, 164, 165, 5, 60, 
    0, 0, 165, 166, 5, 61, 0, 0, 166, 44, 1, 0, 0, 0, 167, 168, 5, 62, 0, 
    0, 168, 169, 5, 61, 0, 0, 169, 46, 1, 0, 0, 0, 170, 171, 5, 43, 0, 0, 
    171, 48, 1, 0, 0, 0, 172, 173, 5, 45, 0, 0, 173, 50, 1, 0, 0, 0, 174, 
    175, 5, 37, 0, 0, 175, 52, 1, 0, 0, 0, 176, 177, 5, 42, 0, 0, 177, 54, 
    1, 0, 0, 0, 178, 179, 5, 47, 0, 0, 179, 56, 1, 0, 0, 0, 180, 181, 5, 
    35, 0, 0, 181, 182, 5, 35, 0, 0, 182, 58, 1, 0, 0, 0, 183, 184, 5, 61, 
    0, 0, 184, 60, 1, 0, 0, 0, 185, 186, 5, 44, 0, 0, 186, 62, 1, 0, 0, 
    0, 187, 188, 5, 59, 0, 0, 188, 64, 1, 0, 0, 0, 189, 193, 7, 0, 0, 0, 
    190, 192, 7, 1, 0, 0, 191, 190, 1, 0, 0, 0, 192, 195, 1, 0, 0, 0, 193, 
    191, 1, 0, 0, 0, 193, 194, 1, 0, 0, 0, 194, 66, 1, 0, 0, 0, 195, 193, 
    1, 0, 0, 0, 196, 200, 7, 2, 0, 0, 197, 199, 7, 1, 0, 0, 198, 197, 1, 
    0, 0, 0, 199, 202, 1, 0, 0, 0, 200, 198, 1, 0, 0, 0, 200, 201, 1, 0, 
    0, 0, 201, 68, 1, 0, 0, 0, 202, 200, 1, 0, 0, 0, 203, 205, 7, 3, 0, 
    0, 204, 203, 1, 0, 0, 0, 205, 206, 1, 0, 0, 0, 206, 204, 1, 0, 0, 0, 
    206, 207, 1, 0, 0, 0, 207, 70, 1, 0, 0, 0, 208, 210, 7, 3, 0, 0, 209, 
    208, 1, 0, 0, 0, 210, 211, 1, 0, 0, 0, 211, 209, 1, 0, 0, 0, 211, 212, 
    1, 0, 0, 0, 212, 213, 1, 0, 0, 0, 213, 215, 5, 46, 0, 0, 214, 216, 7, 
    3, 0, 0, 215, 214, 1, 0, 0, 0, 216, 217, 1, 0, 0, 0, 217, 215, 1, 0, 
    0, 0, 217, 218, 1, 0, 0, 0, 218, 72, 1, 0, 0, 0, 219, 223, 5, 34, 0, 
    0, 220, 222, 9, 0, 0, 0, 221, 220, 1, 0, 0, 0, 222, 225, 1, 0, 0, 0, 
    223, 224, 1, 0, 0, 0, 223, 221, 1, 0, 0, 0, 224, 226, 1, 0, 0, 0, 225, 
    223, 1, 0, 0, 0, 226, 227, 5, 34, 0, 0, 227, 74, 1, 0, 0, 0, 228, 229, 
    5, 47, 0, 0, 229, 230, 5, 47, 0, 0, 230, 234, 1, 0, 0, 0, 231, 233, 
    9, 0, 0, 0, 232, 231, 1, 0, 0, 0, 233, 236, 1, 0, 0, 0, 234, 235, 1, 
    0, 0, 0, 234, 232, 1, 0, 0, 0, 235, 237, 1, 0, 0, 0, 236, 234, 1, 0, 
    0, 0, 237, 238, 5, 10, 0, 0, 238, 239, 1, 0, 0, 0, 239, 240, 6, 37, 
    0, 0, 240, 76, 1, 0, 0, 0, 241, 242, 5, 47, 0, 0, 242, 243, 5, 42, 0, 
    0, 243, 247, 1, 0, 0, 0, 244, 246, 9, 0, 0, 0, 245, 244, 1, 0, 0, 0, 
    246, 249, 1, 0, 0, 0, 247, 248, 1, 0, 0, 0, 247, 245, 1, 0, 0, 0, 248, 
    250, 1, 0, 0, 0, 249, 247, 1, 0, 0, 0, 250, 251, 5, 42, 0, 0, 251, 252, 
    5, 47, 0, 0, 252, 253, 1, 0, 0, 0, 253, 254, 6, 38, 0, 0, 254, 78, 1, 
    0, 0, 0, 255, 257, 7, 4, 0, 0, 256, 255, 1, 0, 0, 0, 257, 258, 1, 0, 
    0, 0, 258, 256, 1, 0, 0, 0, 258, 259, 1, 0, 0, 0, 259, 260, 1, 0, 0, 
    0, 260, 261, 6, 39, 0, 0, 261, 80, 1, 0, 0, 0, 10, 0, 193, 200, 206, 
    211, 217, 223, 234, 247, 258, 1, 6, 0, 0
]);