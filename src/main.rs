use std::io;
use std::io::Write;
use std::process;

mod calc;

use calc::eval::eval;
use calc::lexer::Lexer;
use calc::parser::Parser;
use calc::token::Token;
use calc::value::Value;


fn main() {
    loop {
        let input = read_user_input();
        if input.trim().is_empty() {
            continue;
        }
        let lexer = Lexer::new(input);
        let expr = match Parser::new(lexer).parse() {
            Result::Ok(t) => t,
            Result::Err(e) => {
                eprintln!("ParserError: {:?}", e);
                continue;
            }
        };
        let value = match eval(&expr) {
            Result::Ok(v) => v,
            Result::Err(e) => {
                eprintln!("EvalError: {:?}", e);
                continue;
            },
        };
        match value {
            Value::Int(i) => println!("{}", i),
            Value::Float(f) => println!("{}", f),
        };
    }
}

fn read_user_input() -> String {
    let mut input = String::new();

    print!("> ");
    match io::stdout().flush() {
        Ok(x) => x,
        Err(e) => {
            println!("failed to flush stdout: {}", e);
            process::exit(1);
        }
    };

    match io::stdin().read_line(&mut input) {
        Ok(x) => x,
        Err(e) => {
            println!("failed to read from stdin: {}", e);
            process::exit(1);
        }
    };

    String::from(input.trim_end_matches("\n"))
}
