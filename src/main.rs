use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::process;

mod calc;
mod cli;

use calc::eval::{eval, EvalErr};
use calc::functions as func;
use calc::lexer::Lexer;
use calc::parser::Parser;
use calc::token::Token;
use calc::value::Value;
use calc::context::Context;
use cli::Args;

use clap::Parser as _;
use rustyline::DefaultEditor;

fn main() {
    let args = Args::parse();
    let ctx = create_default_context();

    if let Some(expr) = args.expr {
        match evaluate(&ctx, expr) {
            Result::Ok(v) => println!("{}", v),
            Result::Err(e) => eprintln!("{:?}", e),
        };
        return;
    }

    run_repl(&ctx);
}

fn evaluate(ctx: &Context, expr: String) -> Result<Value, String> {
    let lexer = Lexer::new(expr);
    let expr = match Parser::new(lexer).parse() {
            Result::Ok(t) => t,
            Result::Err(e) => {
                let msg = format!("ParserError: {:?}", e);
                return Result::Err(String::from(msg));
            }
        };
    match eval(&ctx, &expr) {
        Result::Ok(v) => Result::Ok(v),
        Result::Err(e) => {
            let msg = format!("EvalError: {:?}", e);
            return Result::Err(String::from(msg));
        }
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

fn run_repl(ctx: &Context) {
    println!("Calc REPL. Use 'exit' to quit.");
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        match rl.readline("> ") {
            Result::Ok(input) if input.trim().eq_ignore_ascii_case("exit") => break,
            Result::Ok(input) => {
                if input.trim().is_empty() {
                    continue;
                }
                rl.add_history_entry(input.as_str()).unwrap();
                match evaluate(ctx, input) {
                    Result::Ok(v) => println!("{}", v),
                    Result::Err(e) => println!("{}", e),
                }
            }
            Result::Err(_) => break,
        }
    }
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
