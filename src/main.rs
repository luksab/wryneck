#[macro_use]
extern crate lalrpop_util;
pub mod ast;
use colored::*;
use lalrpop_util::{lexer::Token, ErrorRecovery, ParseError};
use structopt::StructOpt;

lalrpop_mod!(pub wryneck);

fn print_errs(err: &Vec<lalrpop_util::ErrorRecovery<usize, lalrpop_util::lexer::Token, &str>>) {
    for err in err {
        println!("{}", format!("Parse error: {}", err.error).red());
    }
}

fn parse(
    input: &str,
) -> Result<
    (Vec<ast::Function>, Vec<ErrorRecovery<usize, Token, &str>>),
    ParseError<usize, Token, &str>,
> {
    let mut errors = Vec::new();
    let ast = wryneck::ProgramParser::new().parse(&mut errors, input);
    let ast = match ast {
        Ok(ast) => ast,
        Err(err) => {
            return Err(err);
        }
    };

    Ok((ast, errors))
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// The input file
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let input = std::fs::read_to_string(opt.input).unwrap();
    match parse(&input) {
        Ok(ast) => println!("{:#?}", ast),
        Err(err) => {
            println!("{}", format!("{}", err).red());
            err.map_location(|char| {
                let mut input = input.clone();
                // replace the character with a space
                input.insert_str(
                    char,
                    "\x1B[31m",
                );
                input.insert_str(
                    char + 6,
                    "\x1B[0m",
                );
                println!("{}", input);
            });
        }
    }
}
