use cfood::{
    antlr::{cfoodparser::CFoodTreeWalker, *},
    print::SexprAst,
};

use std::{env, fs};

use anyhow::{Ok, Result, anyhow};
use dbt_antlr4::{
    Arena, BailErrorStrategy, InputStream, token_factory::CommonTokenFactory,
    token_stream::UnbufferedTokenStream,
};

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let input = args.next().ok_or_else(|| anyhow!("You need pass file!"))?;
    let pretty = env::var("PRETTY")
        .map(|x| !x.is_empty())
        .unwrap_or_default();
    let data = fs::read_to_string(input)?;

    let ast = Arena::with(|arena| {
        let lexer = cfoodlexer::CFoodLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(data.as_str()),
        );
        let ts = UnbufferedTokenStream::new_unbuffered(lexer);
        let mut parser = cfoodparser::CFoodParser::new(arena, ts);
        parser.set_error_strategy(Box::new(BailErrorStrategy::new()));

        let ast = parser.file()?;

        let sexpr = Box::new(SexprAst::new(&cfoodparser::ruleNames, parser.tlt.clone()));
        let sexpr = CFoodTreeWalker::walk(sexpr, ast)?;

        Ok(sexpr.to_sexpr())
    })?;

    if pretty {
        println!("{}", ast);
    }

    Ok(())
}
