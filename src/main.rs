use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::process;

mod calc;

use calc::eval::{eval, EvalErr};
use calc::functions as func;
use calc::lexer::Lexer;
use calc::parser::Parser;
use calc::token::Token;
use calc::value::Value;
use calc::context::Context;

fn main() {
    let ctx = create_default_context();

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
        let value = match eval(&ctx, &expr) {
            Result::Ok(v) => v,
            Result::Err(e) => {
                eprintln!("EvalError: {:?}", e);
                continue;
            }
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

fn create_default_context() -> Context {
    let mut consts: HashMap<&'static str, Value> = HashMap::new();
    consts.insert("pi", Value::Float(std::f64::consts::PI));
    consts.insert("e", Value::Float(std::f64::consts::E));

    let mut funcs: HashMap<&'static str, fn(&[Value]) -> Result<Value, EvalErr>> = HashMap::new();
    funcs.insert("sin", func::sin);
    funcs.insert("cos", func::cos);
    funcs.insert("tan", func::tan);
    funcs.insert("min", func::min);
    funcs.insert("max", func::max);
    funcs.insert("avg", func::avg);
    funcs.insert("ceil", func::ceil);
    funcs.insert("floor", func::floor);
    funcs.insert("log", func::log);

    Context::new(consts, funcs)
}
