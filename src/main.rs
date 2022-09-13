#[macro_use]
extern crate lalrpop_util;
pub mod base_ast;
pub mod formatter;
pub mod resolved_ast;
use std::ops::Range;

use colored::*;
use lalrpop_util::{lexer::Token, ErrorRecovery, ParseError};
use structopt::StructOpt;

lalrpop_mod!(pub wryneck);

/// Print a parse error to error stream.
fn print_parse_error(error: &ParseError<usize, Token, &str>, input: &str) {
    match error.clone() {
        ParseError::InvalidToken { location } => {
            println!(
                "Parse error: {}",
                format!("Invalid token at {}", location).red()
            );
            print_error_line(input, location..location + 1);
        }
        ParseError::UnrecognizedEOF {
            location: _,
            expected,
        } => {
            println!(
                "{}",
                format!(
                    "Parse error: {}",
                    format!(
                        "Unexpected end of file. Expected one of {}",
                        expected.join(", ")
                    )
                    .red()
                )
                .red()
            );
        }
        ParseError::UnrecognizedToken {
            token: (start_pos, token, end_pos),
            expected,
        } => {
            eprintln!(
                "{}",
                format!(
                    "Unrecognized token `{}` found at {}..{}",
                    token, start_pos, end_pos
                )
                .red()
            );

            eprintln!("{}", format!("Expected: {}", expected.join(" or ")).red());

            print_error_line(input, start_pos..end_pos);
        }
        ParseError::ExtraToken {
            token: (start_pos, token, end_pos),
        } => {
            eprintln!(
                "{}",
                format!(
                    "Extra token `{}` found at {}..{}",
                    token, start_pos, end_pos
                )
                .red()
            );

            print_error_line(input, start_pos..end_pos);
        }
        ParseError::User { error } => {
            eprintln!("{}", format!("{}", error).red());
        }
    }
}

/// prints all errors in the given input
fn print_parse_errs(errs: &Vec<ErrorRecovery<usize, Token, &str>>, input: &str) {
    for err in errs {
        print_parse_error(&err.error, input);
    }
}

/// finds the end of the character at the given position
fn find_end(s: &str, mut end: usize) -> usize {
    // use the following, as soon as round_char_boundary is available
    // let end = input.floor_char_boundary(start_pos + 5 + (end_pos - start_pos));
    assert!(end < s.len());
    while !s.is_char_boundary(end + 1) {
        end += 1;
    }
    end + 1
}

/// prints the line of the given input at the given position
fn print_error_line(input: &str, range: Range<usize>) {
    let (start_pos, end_pos) = (range.start, range.end);
    let mut input = input.to_string();
    // replace the character with a space
    input.insert_str(start_pos, "\x1B[31m");
    let end = find_end(&input, start_pos + 4 + (end_pos - start_pos));
    input.insert_str(end, "\x1B[0m");
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
    eprintln!("{}^", "-".repeat(num_spaces),);
}

fn parse(
    input: &str,
) -> Result<
    (base_ast::Program, Vec<ErrorRecovery<usize, Token, &str>>),
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
    let program = match parse(&input) {
        Ok(ast) => {
            print_parse_errs(&ast.1, &input);
            // if opt.ast {
            //     println!("{:#?}", ast.0);
            // } else {
            //     print!("{}", formatter::format(&ast.0));
            // }
            ast.0
        }
        Err(err) => {
            print_parse_error(&err, &input);
            return;
        }
    };
    let program: resolved_ast::Program = program.into();

    if opt.ast {
        println!("{:#?}", program);
    } else {
        print!("{}", formatter::format(&program));
    }
}

// Test the parser
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"
        egg hatch() {
            // does this work?
            let x = {
                let test = 420; // testing
            };
        
            let str = "hello";
            "hello";
        
            if true {
                if false {
                    let x = 420;
                } else {
                    let x = "hello";
                };
                "world";
            };
        
            let y = count_Pigeons(test);
            *)> (69 + {*)>30 + 1;}) * 3;
        }
        // does things
        🥚 count_Pigeons(pigeons) {
            *)> 1;
            // and stuffs
        }
        [
            2 = 0,
            4 = 0,
        ]
        
        "#;
        let program = match parse(&input) {
            Ok(ast) => {
                print_parse_errs(&ast.1, &input);
                // if opt.ast {
                //     println!("{:#?}", ast.0);
                // } else {
                //     print!("{}", formatter::format(&ast.0));
                // }
                ast.0
            }
            Err(err) => {
                print_parse_error(&err, &input);
                return;
            }
        };
        let program: resolved_ast::Program = program.into();

        let output = r#"🥚 🐣() {
    // does this work?
    let x = {
        let test = 420;
        // testing
    };
    let str = "hello";
    "hello";
    if true {
        if false {
            let x = 420;
        }{
            let x = "hello";
        };
        "world";
    };
    let y = count_Pigeons(test);
    🐔 ((69 + {
        🐔 (30 + 1);
    }) * 3);
}

// does things
🥚 count_Pigeons(pigeons) {
    🐔 1;
    // and stuffs
}[
    2 = 0,
    4 = 0,
]

"#;

        assert_eq!(formatter::format(&program), output);
    }
}
