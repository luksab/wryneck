#[macro_use]
extern crate lalrpop_util;
pub mod ast;
pub mod formatter;
use colored::*;
use lalrpop_util::{lexer::Token, ErrorRecovery, ParseError};
use structopt::StructOpt;

lalrpop_mod!(pub wryneck);

fn print_errs(
    err: &Vec<lalrpop_util::ErrorRecovery<usize, lalrpop_util::lexer::Token, &str>>,
    input: &str,
) {
    for err in err {
        // println!("{}", format!("Parse error: {}", err.error).red());
        match err.error.clone() {
            ParseError::InvalidToken { location } => todo!(),
            ParseError::UnrecognizedEOF { location, expected } => todo!(),
            ParseError::UnrecognizedToken { token, expected } => {
                let (start_pos, token, end_pos) = token;

                eprintln!(
                    "{}",
                    format!(
                        "Unrecognized token `{}` found at {}..{}",
                        token,
                        start_pos,
                        end_pos
                    ).red()
                );

                eprintln!("{}", format!("Expected: {}", expected.join(" or ")).red());

                let mut input = input.to_string();
                // replace the character with a space
                input.insert_str(start_pos, "\x1B[31m");
                input.insert_str(start_pos + 5 + (end_pos - start_pos), "\x1B[0m");
                let mut line = 0;
                let mut col = 0;
                let lines = input.lines().collect::<Vec<_>>();
                {
                    // find the line number and the column number
                    let mut pos = 0;
                    for (i, l) in lines.iter().enumerate() {
                        if pos + l.len() >= start_pos {
                            line = i;
                            col = start_pos - pos + 1;
                            break;
                        }
                        pos += l.len() + 1;
                    }
                }

                let line_num_width = (line + 1).to_string().len();
                // print the line and the previous one
                if line > 1 {
                    eprintln!("{:>line_num_width$}: {}", line, lines[line - 1]);
                }
                eprintln!("{:>line_num_width$}: {}", line + 1, lines[line]);
                let num_spaces = col + line_num_width + 1;
                eprintln!(
                    "{}^",
                    "-".repeat(num_spaces),
                );
            }
            ParseError::ExtraToken { token } => todo!(),
            ParseError::User { error } => todo!(),
        }
        // err.error.clone().map_location(|char| {
        //     println!("{char}");
        //     let mut input = input.to_string();
        //     // replace the character with a space
        //     input.insert_str(char, "\x1B[31m");
        //     input.insert_str(char + 6, "\x1B[0m");
        //     let mut line = 0;
        //     let mut col = 0;
        //     let lines = input.lines().collect::<Vec<_>>();
        //     {
        //         // find the line number and the column number
        //         let mut pos = 0;
        //         for (i, l) in lines.iter().enumerate() {
        //             if pos + l.len() >= char {
        //                 line = i;
        //                 col = char - pos;
        //                 break;
        //             }
        //             pos += l.len();
        //         }
        //     }

        //     // print the line and the previous one
        //     if let Some(l) = lines.get(line - 1) {
        //         println!("{}: {}", line, l);
        //     }
        //     println!("{}: {}", line + 1, lines[line]);
        //     println!("{}^", "-".repeat(col - 1));
        // });
    }
}

fn parse(
    input: &str,
) -> Result<
    (ast::Program, Vec<ErrorRecovery<usize, Token, &str>>),
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

    /// Print the AST
    #[structopt(short, long)]
    ast: bool,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let input = std::fs::read_to_string(opt.input).unwrap();
    match parse(&input) {
        Ok(ast) => {
            print_errs(&ast.1, &input);
            if opt.ast {
                println!("{:#?}", ast.0);
            } else {
                print!("{}", formatter::format(&ast.0));
            }
        }
        Err(err) => {
            eprintln!("2{}", format!("{}", err).red());
            err.map_location(|char| {
                let mut input = input.clone();
                // replace the character with a space
                input.insert_str(char, "\x1B[31m");
                input.insert_str(char + 6, "\x1B[0m");
                eprintln!("{}", input);
            });
        }
    }
}
