use cfood::{
    antlr::{cfoodparser::CFoodTreeWalker, cfoodvisitor::CFoodVisitor, *},
    cst::{CstToSexpr, parse_to_cst, visitor::Visitor},
    print::SexprAst,
};

use std::{env, fs, io};

use anyhow::{Ok, Result, anyhow};
use dbt_antlr4::{
    Arena, BailErrorStrategy, InputStream, token_factory::CommonTokenFactory,
    token_stream::UnbufferedTokenStream, tree::NodeInner,
};

fn main() -> Result<()> {
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

    let ast = Arena::with(|arena| {
        let lexer = cfoodlexer::CFoodLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(data.as_str()),
        );
        let ts = UnbufferedTokenStream::new_unbuffered(lexer);
        let mut parser = cfoodparser::CFoodParser::new(arena, ts);
        parser.set_error_strategy(Box::new(BailErrorStrategy::new()));

        let ast = parser.file()?;

        let (file, span_store) = parse_to_cst(ast.as_node())?;
        let mut cst_to_sexpr = CstToSexpr::new(&span_store);
        let s = cst_to_sexpr.visit_file(&file)?;

        // let sexpr = Box::new(SexprAst::new(
        //     &cfoodparser::ruleNames,
        //     &cfoodlexer::ruleNames,
        //     parser.tlt.clone(),
        // ));
        // let sexpr = CFoodTreeWalker::walk(sexpr, ast)?;

        // Ok(sexpr.to_sexpr())
        Ok(s)
    })?;

    if pretty {
        print!("{}", ast);
    }

    Ok(())
}
