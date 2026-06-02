use cfood::{
    antlr::*,
    checker::TLT,
    compiler::{Compiler, LLVMCtx},
    cst::{CstToSexpr, parse_to_cst, visitor::Visitor},
    error::PanicHandler,
};
use inkwell::{
    context::Context,
    targets::{InitializationConfig, Target},
};
use miette::Report;

use std::{env, fs, io};

use anyhow::{Ok, Result, anyhow, bail};
use dbt_antlr4::{
    Arena, BailErrorStrategy, InputStream, token_factory::CommonTokenFactory,
    token_stream::UnbufferedTokenStream, tree::NodeInner,
};

fn main() -> Result<()> {
    miette::set_panic_hook();
    let _handler = PanicHandler;

    let mut args = env::args().skip(1);
    let pretty = env::var("PRETTY")
        .map(|x| !x.is_empty())
        .unwrap_or_default();
    let stdin = env::var("STDIN").map(|x| !x.is_empty()).unwrap_or_default();

    let data = if stdin {
        io::read_to_string(io::stdin())?
    } else {
        let input = args.next().ok_or_else(|| anyhow!("You need pass file!"))?;
        fs::read_to_string(input)?
    };

    Target::initialize_x86(&InitializationConfig::default());

    let (cst, span_store) = Arena::with(|arena| {
        let lexer = cfoodlexer::CFoodLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(data.as_str()),
        );
        let ts = UnbufferedTokenStream::new_unbuffered(lexer);
        let mut parser = cfoodparser::CFoodParser::new(arena, ts);
        parser.set_error_strategy(Box::new(BailErrorStrategy::new()));

        let ast = parser.file()?;

        let (cst, span_store) = parse_to_cst(ast.as_node())?;
        Ok((cst, span_store))
    })?;

    let mut tlt = TLT::default();
    if tlt.check_file(&cst).is_err() || !tlt.errors.is_empty() {
        for e in tlt.errors {
            let diag = Report::new(e.to_diagnostic(&span_store)).with_source_code(data.clone());
            println!("{:?}", diag);
        }

        bail!("Compile fail due to the error");
    }

    if pretty {
        let mut cst_to_sexpr = CstToSexpr::new(&span_store, vec![&tlt]);
        let s = cst_to_sexpr.visit_file(&cst)?;
        print!("{}", s);
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
        "x86_64-linux-gnu",
        tlt.refer_map,
        tlt.type_store,
    )?;

    compiler.compile(&cst, "./output.ll")?;
    Ok(())
}
