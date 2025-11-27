use std::collections::HashMap;

mod calc;
mod cli;

use calc::context::Context;
use calc::eval::{eval, EvalErr};
use calc::functions as func;
use calc::lexer::Lexer;
use calc::parser::Parser;
use calc::token::Token;
use calc::value::Value;

use clap::Parser as _;
use rustyline::DefaultEditor;


fn main() {
    let args = cli::Args::parse();
    let ctx = create_default_context();

    match args.command {
        Some(cli::CalcCommand::Const(cmd)) => match cmd.sub {
            cli::ConstSub::List => {
                let values = ctx.consts
                    .keys()
                    .map(|k| k.to_string().to_uppercase())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("Constants: {}.", values);
                return;
            },
            cli::ConstSub::Get { name } => {
                let name = name.trim().to_lowercase();
                let msg = match ctx.consts.get(name.as_str()) {
                    Option::Some(value) => format!("{}", value),
                    Option::None => format!("Constant '{}' is not defined.", name),
                };
                println!("{}", msg);
                return;
            },
        },
        Some(cli::CalcCommand::Func(cmd)) => match cmd.sub {
            cli::FuncSub::List => {
                let values = ctx.funcs
                    .keys()
                    .map(|k| k.to_string().to_lowercase())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("Functions: {}.", values);
                return;
            },
        },
        _ => {},
    }

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
