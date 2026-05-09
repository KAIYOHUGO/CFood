// Generated from ./CFood.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unused_variables)]
#![allow(unused_braces)]
#![allow(unused_parens)]

	use crate::ty::*;
	use crate::tlt::*;

use dbt_antlr4::Arena;
use dbt_antlr4::atn::ATN;
use dbt_antlr4::char_stream::CharStream;
use dbt_antlr4::int_stream::IntStream;
use dbt_antlr4::lexer::{BaseLexer, LexerRecog, Lexer as _};
use dbt_antlr4::atn_deserializer::ATNDeserializer;
use dbt_antlr4::dfa::DFA;
use dbt_antlr4::TokenSource;
use dbt_antlr4::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use dbt_antlr4::PredictionContextCache;
use dbt_antlr4::recognizer::Actions;
use dbt_antlr4::token_factory::{CommonTokenFactory, TokenFactory};
use dbt_antlr4::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyRuleNode};
use dbt_antlr4::vocabulary::{Vocabulary,VocabularyImpl};

use std::ops::{DerefMut, Deref};
use std::sync::LazyLock;

pub const KW_while:i32=1; 
pub const KW_if:i32=2; 
pub const KW_else:i32=3; 
pub const KW_return:i32=4; 
pub const KW_type:i32=5; 
pub const KW_let:i32=6; 
pub const TY_int:i32=7; 
pub const TY_float:i32=8; 
pub const TY_void:i32=9; 
pub const ARROW:i32=10; 
pub const PAREN_L:i32=11; 
pub const PAREN_R:i32=12; 
pub const BRACE_L:i32=13; 
pub const BRACE_R:i32=14; 
pub const BRACKET_L:i32=15; 
pub const BRACKET_R:i32=16; 
pub const NE:i32=17; 
pub const EQ:i32=18; 
pub const LT:i32=19; 
pub const GT:i32=20; 
pub const LE:i32=21; 
pub const GE:i32=22; 
pub const PLUS:i32=23; 
pub const SUB:i32=24; 
pub const MOD:i32=25; 
pub const MUL:i32=26; 
pub const DIV:i32=27; 
pub const ASSIGN:i32=28; 
pub const COMMA:i32=29; 
pub const SEMICOLON:i32=30; 
pub const TYPE:i32=31; 
pub const IDENT:i32=32; 
pub const INT:i32=33; 
pub const FLOAT:i32=34; 
pub const LINE_COMMENT:i32=35; 
pub const COMMENT:i32=36; 
pub const WS:i32=37;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;37] = [
    "KW_while", "KW_if", "KW_else", "KW_return", "KW_type", "KW_let", "TY_int", 
    "TY_float", "TY_void", "ARROW", "PAREN_L", "PAREN_R", "BRACE_L", "BRACE_R", 
    "BRACKET_L", "BRACKET_R", "NE", "EQ", "LT", "GT", "LE", "GE", "PLUS", 
    "SUB", "MOD", "MUL", "DIV", "ASSIGN", "COMMA", "SEMICOLON", "TYPE", 
    "IDENT", "INT", "FLOAT", "LINE_COMMENT", "COMMENT", "WS"
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

static _shared_context_cache: LazyLock<PredictionContextCache> = LazyLock::new(|| PredictionContextCache::new());
static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, CFoodLexerActions, Input, TF>;

pub struct CFoodLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr4::impl_token_source! { CFoodLexer }

impl<'input, 'arena, Input, TF> CFoodLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        dbt_antlr4::recognizer::check_version("1","0");
        let token_factory = TF::new(arena);
        let actions = CFoodLexerActions {

                tlt: Default::default(),

        };
        let base = BaseLexerType::new_base_lexer(input, actions, token_factory);
        Self { base }
    }
}

impl<'input, 'arena, Input, TF> Deref for CFoodLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    type Target = BaseLexerType<'input, 'arena, Input, TF>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, 'arena, Input, TF> DerefMut for CFoodLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct CFoodLexerActions {

		pub tlt: TLT,

}

impl CFoodLexerActions {
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseLexerType<'input, 'arena, Input, TF>>
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
    fn get_atn_simulator(&self) -> LexerATNSimulator {
        LexerATNSimulator::new_lexer_atnsimulator(
            &_ATN,
            &_decision_to_DFA,
            &_shared_context_cache,
        )
    }
}

static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _decision_to_DFA: LazyLock<Vec<DFA>> = LazyLock::new(|| {
    let size = _ATN.decision_to_state.len() as i32;
    let mut dfa = Vec::with_capacity(size as usize);
    for i in 0..size {
        dfa.push(DFA::new(
            &_ATN,
            _ATN.get_decision_state(i),
            i,
        ))
    }
    dfa
});
static _serializedATN: LazyLock<Vec<i32>> = LazyLock::new(|| vec![
    4, 0, 37, 231, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 
    14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 
    19, 2, 20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 
    24, 2, 25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 
    29, 2, 30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 
    34, 2, 35, 7, 35, 2, 36, 7, 36, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 
    1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 
    3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 
    1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 
    8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 10, 1, 10, 1, 11, 1, 
    11, 1, 12, 1, 12, 1, 13, 1, 13, 1, 14, 1, 14, 1, 15, 1, 15, 1, 16, 1, 
    16, 1, 16, 1, 17, 1, 17, 1, 17, 1, 18, 1, 18, 1, 19, 1, 19, 1, 20, 1, 
    20, 1, 20, 1, 21, 1, 21, 1, 21, 1, 22, 1, 22, 1, 23, 1, 23, 1, 24, 1, 
    24, 1, 25, 1, 25, 1, 26, 1, 26, 1, 27, 1, 27, 1, 28, 1, 28, 1, 29, 1, 
    29, 1, 30, 1, 30, 5, 30, 170, 8, 30, 10, 30, 12, 30, 173, 9, 30, 1, 
    31, 1, 31, 5, 31, 177, 8, 31, 10, 31, 12, 31, 180, 9, 31, 1, 32, 4, 
    32, 183, 8, 32, 11, 32, 12, 32, 184, 1, 33, 4, 33, 188, 8, 33, 11, 33, 
    12, 33, 189, 1, 33, 1, 33, 4, 33, 194, 8, 33, 11, 33, 12, 33, 195, 1, 
    34, 1, 34, 1, 34, 1, 34, 5, 34, 202, 8, 34, 10, 34, 12, 34, 205, 9, 
    34, 1, 34, 1, 34, 1, 34, 1, 34, 1, 35, 1, 35, 1, 35, 1, 35, 5, 35, 215, 
    8, 35, 10, 35, 12, 35, 218, 9, 35, 1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 
    1, 36, 4, 36, 226, 8, 36, 11, 36, 12, 36, 227, 1, 36, 1, 36, 2, 203, 
    216, 0, 37, 1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 
    19, 10, 21, 11, 23, 12, 25, 13, 27, 14, 29, 15, 31, 16, 33, 17, 35, 
    18, 37, 19, 39, 20, 41, 21, 43, 22, 45, 23, 47, 24, 49, 25, 51, 26, 
    53, 27, 55, 28, 57, 29, 59, 30, 61, 31, 63, 32, 65, 33, 67, 34, 69, 
    35, 71, 36, 73, 37, 1, 0, 5, 1, 0, 65, 90, 4, 0, 48, 57, 65, 90, 95, 
    95, 97, 122, 2, 0, 95, 95, 97, 122, 1, 0, 48, 57, 3, 0, 9, 10, 13, 13, 
    32, 32, 238, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 
    7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 
    0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 
    0, 0, 0, 0, 23, 1, 0, 0, 0, 0, 25, 1, 0, 0, 0, 0, 27, 1, 0, 0, 0, 0, 
    29, 1, 0, 0, 0, 0, 31, 1, 0, 0, 0, 0, 33, 1, 0, 0, 0, 0, 35, 1, 0, 0, 
    0, 0, 37, 1, 0, 0, 0, 0, 39, 1, 0, 0, 0, 0, 41, 1, 0, 0, 0, 0, 43, 1, 
    0, 0, 0, 0, 45, 1, 0, 0, 0, 0, 47, 1, 0, 0, 0, 0, 49, 1, 0, 0, 0, 0, 
    51, 1, 0, 0, 0, 0, 53, 1, 0, 0, 0, 0, 55, 1, 0, 0, 0, 0, 57, 1, 0, 0, 
    0, 0, 59, 1, 0, 0, 0, 0, 61, 1, 0, 0, 0, 0, 63, 1, 0, 0, 0, 0, 65, 1, 
    0, 0, 0, 0, 67, 1, 0, 0, 0, 0, 69, 1, 0, 0, 0, 0, 71, 1, 0, 0, 0, 0, 
    73, 1, 0, 0, 0, 1, 75, 1, 0, 0, 0, 3, 81, 1, 0, 0, 0, 5, 84, 1, 0, 0, 
    0, 7, 89, 1, 0, 0, 0, 9, 96, 1, 0, 0, 0, 11, 101, 1, 0, 0, 0, 13, 105, 
    1, 0, 0, 0, 15, 109, 1, 0, 0, 0, 17, 115, 1, 0, 0, 0, 19, 120, 1, 0, 
    0, 0, 21, 123, 1, 0, 0, 0, 23, 125, 1, 0, 0, 0, 25, 127, 1, 0, 0, 0, 
    27, 129, 1, 0, 0, 0, 29, 131, 1, 0, 0, 0, 31, 133, 1, 0, 0, 0, 33, 135, 
    1, 0, 0, 0, 35, 138, 1, 0, 0, 0, 37, 141, 1, 0, 0, 0, 39, 143, 1, 0, 
    0, 0, 41, 145, 1, 0, 0, 0, 43, 148, 1, 0, 0, 0, 45, 151, 1, 0, 0, 0, 
    47, 153, 1, 0, 0, 0, 49, 155, 1, 0, 0, 0, 51, 157, 1, 0, 0, 0, 53, 159, 
    1, 0, 0, 0, 55, 161, 1, 0, 0, 0, 57, 163, 1, 0, 0, 0, 59, 165, 1, 0, 
    0, 0, 61, 167, 1, 0, 0, 0, 63, 174, 1, 0, 0, 0, 65, 182, 1, 0, 0, 0, 
    67, 187, 1, 0, 0, 0, 69, 197, 1, 0, 0, 0, 71, 210, 1, 0, 0, 0, 73, 225, 
    1, 0, 0, 0, 75, 76, 5, 119, 0, 0, 76, 77, 5, 104, 0, 0, 77, 78, 5, 105, 
    0, 0, 78, 79, 5, 108, 0, 0, 79, 80, 5, 101, 0, 0, 80, 2, 1, 0, 0, 0, 
    81, 82, 5, 105, 0, 0, 82, 83, 5, 102, 0, 0, 83, 4, 1, 0, 0, 0, 84, 85, 
    5, 101, 0, 0, 85, 86, 5, 108, 0, 0, 86, 87, 5, 115, 0, 0, 87, 88, 5, 
    101, 0, 0, 88, 6, 1, 0, 0, 0, 89, 90, 5, 114, 0, 0, 90, 91, 5, 101, 
    0, 0, 91, 92, 5, 116, 0, 0, 92, 93, 5, 117, 0, 0, 93, 94, 5, 114, 0, 
    0, 94, 95, 5, 110, 0, 0, 95, 8, 1, 0, 0, 0, 96, 97, 5, 116, 0, 0, 97, 
    98, 5, 121, 0, 0, 98, 99, 5, 112, 0, 0, 99, 100, 5, 101, 0, 0, 100, 
    10, 1, 0, 0, 0, 101, 102, 5, 108, 0, 0, 102, 103, 5, 101, 0, 0, 103, 
    104, 5, 116, 0, 0, 104, 12, 1, 0, 0, 0, 105, 106, 5, 105, 0, 0, 106, 
    107, 5, 110, 0, 0, 107, 108, 5, 116, 0, 0, 108, 14, 1, 0, 0, 0, 109, 
    110, 5, 102, 0, 0, 110, 111, 5, 108, 0, 0, 111, 112, 5, 111, 0, 0, 112, 
    113, 5, 97, 0, 0, 113, 114, 5, 116, 0, 0, 114, 16, 1, 0, 0, 0, 115, 
    116, 5, 118, 0, 0, 116, 117, 5, 111, 0, 0, 117, 118, 5, 105, 0, 0, 118, 
    119, 5, 100, 0, 0, 119, 18, 1, 0, 0, 0, 120, 121, 5, 45, 0, 0, 121, 
    122, 5, 62, 0, 0, 122, 20, 1, 0, 0, 0, 123, 124, 5, 40, 0, 0, 124, 22, 
    1, 0, 0, 0, 125, 126, 5, 41, 0, 0, 126, 24, 1, 0, 0, 0, 127, 128, 5, 
    123, 0, 0, 128, 26, 1, 0, 0, 0, 129, 130, 5, 125, 0, 0, 130, 28, 1, 
    0, 0, 0, 131, 132, 5, 91, 0, 0, 132, 30, 1, 0, 0, 0, 133, 134, 5, 93, 
    0, 0, 134, 32, 1, 0, 0, 0, 135, 136, 5, 33, 0, 0, 136, 137, 5, 61, 0, 
    0, 137, 34, 1, 0, 0, 0, 138, 139, 5, 61, 0, 0, 139, 140, 5, 61, 0, 0, 
    140, 36, 1, 0, 0, 0, 141, 142, 5, 60, 0, 0, 142, 38, 1, 0, 0, 0, 143, 
    144, 5, 62, 0, 0, 144, 40, 1, 0, 0, 0, 145, 146, 5, 60, 0, 0, 146, 147, 
    5, 61, 0, 0, 147, 42, 1, 0, 0, 0, 148, 149, 5, 62, 0, 0, 149, 150, 5, 
    61, 0, 0, 150, 44, 1, 0, 0, 0, 151, 152, 5, 43, 0, 0, 152, 46, 1, 0, 
    0, 0, 153, 154, 5, 45, 0, 0, 154, 48, 1, 0, 0, 0, 155, 156, 5, 37, 0, 
    0, 156, 50, 1, 0, 0, 0, 157, 158, 5, 42, 0, 0, 158, 52, 1, 0, 0, 0, 
    159, 160, 5, 47, 0, 0, 160, 54, 1, 0, 0, 0, 161, 162, 5, 61, 0, 0, 162, 
    56, 1, 0, 0, 0, 163, 164, 5, 44, 0, 0, 164, 58, 1, 0, 0, 0, 165, 166, 
    5, 59, 0, 0, 166, 60, 1, 0, 0, 0, 167, 171, 7, 0, 0, 0, 168, 170, 7, 
    1, 0, 0, 169, 168, 1, 0, 0, 0, 170, 173, 1, 0, 0, 0, 171, 169, 1, 0, 
    0, 0, 171, 172, 1, 0, 0, 0, 172, 62, 1, 0, 0, 0, 173, 171, 1, 0, 0, 
    0, 174, 178, 7, 2, 0, 0, 175, 177, 7, 1, 0, 0, 176, 175, 1, 0, 0, 0, 
    177, 180, 1, 0, 0, 0, 178, 176, 1, 0, 0, 0, 178, 179, 1, 0, 0, 0, 179, 
    64, 1, 0, 0, 0, 180, 178, 1, 0, 0, 0, 181, 183, 7, 3, 0, 0, 182, 181, 
    1, 0, 0, 0, 183, 184, 1, 0, 0, 0, 184, 182, 1, 0, 0, 0, 184, 185, 1, 
    0, 0, 0, 185, 66, 1, 0, 0, 0, 186, 188, 7, 3, 0, 0, 187, 186, 1, 0, 
    0, 0, 188, 189, 1, 0, 0, 0, 189, 187, 1, 0, 0, 0, 189, 190, 1, 0, 0, 
    0, 190, 191, 1, 0, 0, 0, 191, 193, 5, 46, 0, 0, 192, 194, 7, 3, 0, 0, 
    193, 192, 1, 0, 0, 0, 194, 195, 1, 0, 0, 0, 195, 193, 1, 0, 0, 0, 195, 
    196, 1, 0, 0, 0, 196, 68, 1, 0, 0, 0, 197, 198, 5, 47, 0, 0, 198, 199, 
    5, 47, 0, 0, 199, 203, 1, 0, 0, 0, 200, 202, 9, 0, 0, 0, 201, 200, 1, 
    0, 0, 0, 202, 205, 1, 0, 0, 0, 203, 204, 1, 0, 0, 0, 203, 201, 1, 0, 
    0, 0, 204, 206, 1, 0, 0, 0, 205, 203, 1, 0, 0, 0, 206, 207, 5, 10, 0, 
    0, 207, 208, 1, 0, 0, 0, 208, 209, 6, 34, 0, 0, 209, 70, 1, 0, 0, 0, 
    210, 211, 5, 47, 0, 0, 211, 212, 5, 42, 0, 0, 212, 216, 1, 0, 0, 0, 
    213, 215, 9, 0, 0, 0, 214, 213, 1, 0, 0, 0, 215, 218, 1, 0, 0, 0, 216, 
    217, 1, 0, 0, 0, 216, 214, 1, 0, 0, 0, 217, 219, 1, 0, 0, 0, 218, 216, 
    1, 0, 0, 0, 219, 220, 5, 42, 0, 0, 220, 221, 5, 47, 0, 0, 221, 222, 
    1, 0, 0, 0, 222, 223, 6, 35, 0, 0, 223, 72, 1, 0, 0, 0, 224, 226, 7, 
    4, 0, 0, 225, 224, 1, 0, 0, 0, 226, 227, 1, 0, 0, 0, 227, 225, 1, 0, 
    0, 0, 227, 228, 1, 0, 0, 0, 228, 229, 1, 0, 0, 0, 229, 230, 6, 36, 0, 
    0, 230, 74, 1, 0, 0, 0, 9, 0, 171, 178, 184, 189, 195, 203, 216, 227, 
    1, 6, 0, 0
]);