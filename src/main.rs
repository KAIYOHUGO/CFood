use cfood::{
    antlr::*,
    buildin::{build_buildin_type, buildin_type},
    checker::TLT,
    compiler::{Compiler, LLVMCtx},
    cst::{CstToSexpr, parse_to_cst, visitor::Visitor},
    error::PanicHandler,
};
use clap::Parser;
use inkwell::{
    context::Context,
    targets::{InitializationConfig, Target},
};
use miette::{LabeledSpan, MietteDiagnostic, Report};

use std::{error::Error, fs, io, ops::Range, path::PathBuf};

use anyhow::{Result, anyhow, bail};
use dbt_antlr4::{
    Arena, BailErrorStrategy, InputStream,
    errors::{ANTLRError, ANTLRErrorKind},
    parser::Parser as AntlrParser,
    recognizer::Recognizer,
    token::{TOKEN_EOF, Token},
    token_factory::CommonTokenFactory,
    token_stream::UnbufferedTokenStream,
    tree::NodeInner,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(value_name = "INPUT", required_unless_present = "stdin")]
    input: Option<PathBuf>,

    #[arg(long)]
    stdin: bool,

    #[arg(long)]
    emit_cst: bool,

    #[arg(short, long, default_value = "output.ll")]
    output: PathBuf,

    #[arg(long, default_value = "x86_64-linux-gnu")]
    target: String,
}

fn find_antlr_error(err: &anyhow::Error) -> Option<&ANTLRError> {
    err.chain().find_map(find_antlr_error_in_source)
}

fn find_antlr_error_in_source<'a>(err: &'a (dyn Error + 'static)) -> Option<&'a ANTLRError> {
    find_antlr_error_in_chain(err)
}

fn find_antlr_error_in_chain<'a>(mut err: &'a (dyn Error + 'static)) -> Option<&'a ANTLRError> {
    let mut last_antlr = None;

    loop {
        if let Some(antlr_err) = err.downcast_ref::<ANTLRError>() {
            last_antlr = Some(antlr_err);
        }

        let Some(source) = err.source() else {
            return last_antlr;
        };

        err = source;
    }
}

fn syntax_error_span(err: &ANTLRError, source: &str) -> Range<usize> {
    let source_len = source.len();

    match err.as_ref() {
        ANTLRErrorKind::LexerNoAltError { start_index } => {
            let start = (*start_index).max(0) as usize;
            let start = start.min(source_len);
            let end = if source_len == 0 {
                start
            } else {
                start.saturating_add(1).min(source_len)
            };
            start..end
        }
        _ => {
            let Some(token) = err.get_offending_token() else {
                return 0..0;
            };

            let start = token.get_start_index().max(0) as usize;
            let start = start.min(source_len);
            let end = if token.get_token_type() == TOKEN_EOF {
                start
            } else {
                (token.get_stop_index() + 1).max(token.get_start_index() + 1) as usize
            }
            .min(source_len);

            start..end.max(start)
        }
    }
}

fn syntax_error_label(err: &ANTLRError) -> String {
    match err.as_ref() {
        ANTLRErrorKind::LexerNoAltError { .. } => "invalid token starts here".to_string(),
        _ => {
            let Some(token) = err.get_offending_token() else {
                return "syntax error here".to_string();
            };

            if token.get_token_type() == TOKEN_EOF {
                "unexpected end of file".to_string()
            } else {
                format!("unexpected token `{}`", token.get_text())
            }
        }
    }
}

fn emit_syntax_error_report(err: &ANTLRError, source: &str, expected_help: Option<String>) {
    let mut diag = MietteDiagnostic::new("syntax error");
    let span = syntax_error_span(err, source);
    diag = diag.and_label(LabeledSpan::new_with_span(
        Some(syntax_error_label(err)),
        span,
    ));

    match err.as_ref() {
        ANTLRErrorKind::InputMismatchError(_)
        | ANTLRErrorKind::NoAltError(_)
        | ANTLRErrorKind::PredicateError(_) => {
            diag.help = expected_help.or_else(|| Some(format!("ANTLR reported: {err}")));
        }
        ANTLRErrorKind::LexerNoAltError { .. } => {
            diag.help = Some("The lexer could not recognize this input.".to_string());
        }
        _ => {
            diag.help = Some(format!("ANTLR reported: {err}"));
        }
    }

    let report = Report::new(diag).with_source_code(source.to_string());
    eprintln!("{:?}", report);
}

fn main() -> Result<()> {
    miette::set_panic_hook();
    let _handler = PanicHandler;
    let cli = Cli::parse();

    let data = if cli.stdin {
        io::read_to_string(io::stdin())?
    } else {
        let input = cli.input.ok_or_else(|| anyhow!("You need pass file!"))?;
        fs::read_to_string(input)?
    };

    Target::initialize_all(&InitializationConfig::default());

    let (cst, span_store) = match Arena::with(|arena| {
        let lexer = cfoodlexer::CFoodLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(data.as_str()),
        );
        let ts = UnbufferedTokenStream::new_unbuffered(lexer);
        let mut parser = cfoodparser::CFoodParser::new(arena, ts);
        parser.set_error_strategy(Box::new(BailErrorStrategy::new()));

        let ast = match parser.file() {
            Ok(ast) => ast,
            Err(err) => {
                let err = find_antlr_error_in_source(&err).unwrap_or(&err);
                let expected = parser
                    .get_expected_tokens()
                    .to_token_string(parser.get_vocabulary());
                let expected_help = if expected.is_empty() {
                    None
                } else {
                    Some(format!("expected one of: {expected}"))
                };
                emit_syntax_error_report(err, data.as_str(), expected_help);
                bail!("Compile fail due to syntax error");
            }
        };

        let (cst, span_store) = parse_to_cst(ast.as_node())?;
        Ok((cst, span_store))
    }) {
        Ok(result) => result,
        Err(err) => {
            if let Some(antlr_err) = find_antlr_error(&err) {
                emit_syntax_error_report(antlr_err, &data, None);
                bail!("Compile fail due to syntax error");
            }

            return Err(err);
        }
    };

    let mut tlt = TLT::default();
    let buildin_type = tlt.add_buildin_funcs(buildin_type().into_iter());
    let is_check_err = tlt.check_file(&cst).is_err();

    if cli.emit_cst {
        let mut cst_to_sexpr = CstToSexpr::new(&span_store, vec![&tlt]);
        let s = cst_to_sexpr.visit_file(&cst)?;
        print!("{}", s);
    }

    if is_check_err || !tlt.errors.is_empty() {
        for e in tlt.errors {
            let diag = Report::new(e.to_diagnostic(&span_store)).with_source_code(data.clone());
            eprintln!("{:?}", diag);
        }

        bail!("Compile fail due to type error");
    }

    let context = Context::create();
    let module = context.create_module("cfood");
    let builder = context.create_builder();
    let mut compiler = Compiler::new(
        LLVMCtx {
            context: &context,
            builder: &builder,
            module: &module,
        },
        &cli.target,
        tlt.refer_map,
        tlt.type_store,
    )?;
    build_buildin_type(&mut compiler, buildin_type);

    compiler.compile(&cst, &cli.output)?;
    Ok(())
}
