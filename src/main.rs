use std::io;
use std::io::Write;
use std::process;

mod calc;

use calc::lexer::{Lexer, LexerErr};
use calc::parser::Parser;
use calc::token::Token;

fn main() {
    parser_main();
}

fn get_expression() -> String {
    let mut input = String::new();

    print!("> ");
    match io::stdout().flush() {
        Ok(x) => x,
        Err(e) => {
            println!("Unable to flush stdout: {}", e);
            process::exit(1);
        }
    };

    match io::stdin().read_line(&mut input) {
        Ok(x) => x,
        Err(e) => {
            println!("Unable to read from stdin: {}", e);
            process::exit(1);
        }
    };

    String::from(input.trim_end_matches("\n"))
}

#[allow(dead_code)]
fn lexer_main() {
    loop {
        let input = get_expression();
        if input == String::from("exit") {
            break;
        }

        let mut lexer = Lexer::new(input.as_str());
        loop {
            let result = lexer.next_token();
            match result {
                Ok(token) => {
                    println!("{:?}", token);
                    match token {
                        Token::EOF(_) => break,
                        _ => {}
                    };
                }
                Err(error) => match error {
                    LexerErr::IllegalChar(ch, pos) => {
                        eprintln!("Error: Illegal character '{}' at {}", ch, pos);
                        break;
                    }
                },
            };
        }
    }
}

#[allow(dead_code)]
fn parser_main() {
    loop {
        let input = get_expression();
        let expr = input.as_str();
        let tokens = match Lexer::new(expr).tokenise() {
            Result::Ok(t) => t,
            Result::Err(e) => {
                eprintln!("LexerError: {:?}", e);
                continue;
            }
        };
        dbg!(&tokens);
        let tree = match Parser::new(tokens).parse() {
            Result::Ok(t) => t,
            Result::Err(e) => {
                eprintln!("ParserError: {:?}", e);
                continue;
            }
        };
        dbg!(tree);
    }
}
