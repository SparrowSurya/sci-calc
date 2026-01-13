use clap::{Parser, Subcommand, ArgAction};

use crate::calc::value::Value;


#[derive(Parser)]
#[command(name = "calc")]
#[command(about = "A command-line calculator powered by Rust evaluator engine.")]
pub struct Args {

    #[arg(
        short = 'B',
        long,
        help = "allow type conversion when using bitwise with floating point number",
        action = ArgAction::SetTrue,
        default_value_t = false,
    )]
    pub allow_floating_bitwise_operation: bool,

    #[arg(
        long = "ignore-funcs",
        help = "disallowed builtin functions",
        num_args = 1..,
    )]
    pub ignore_funcs: Vec<String>,

    #[arg(
        long = "ignore-consts",
        help = "disallowed builtin constants",
        num_args = 1..,
    )]
    pub ignore_consts: Vec<String>,

    #[arg(
        short = 'O',
        long = "overwrite",
        help = "overwrite existing constants if provided",
    )]
    pub overwrite: bool,

    #[arg(
        long = "extra-consts",
        help = "external constant values",
        num_args = 1..,
        value_parser = parse_extra_consts,
    )]
    pub extra_consts: Vec<(String, Value)>,

    #[arg(help = "expression to evaluate (omit this to open REPL)")]
    pub expr: Option<String>,

    #[command(subcommand)]
    #[command(help = "Available subcommands")]
    pub command: Option<CalcCommand>,
}


#[derive(Subcommand)]
pub enum CalcCommand {

    #[command(about = "Built-in constants")]
    Const(ConstCmd),

    #[command(about = "Built-in functions")]
    Func(FuncCmd),
}


#[derive(Parser)]
pub struct ConstCmd {
    #[command(subcommand)]
    pub sub: ConstSub,
}

#[derive(Subcommand)]
pub enum ConstSub {

    #[command(about = "List all available constants")]
    List,

    #[command(about = "Get value of the constant")]
    Get {
        #[arg(help = "Name of the constant")]
        name: String,
    },
}

#[derive(Parser)]
pub struct FuncCmd {
    #[command(subcommand)]
    pub sub: FuncSub,
}

#[derive(Subcommand)]
pub enum FuncSub {

    #[command(about = "List all available functions")]
    List,
}


fn parse_extra_consts(s: &str) -> Result<(String, Value), String> {
    let (k, v) = s
        .split_once('=')
        .ok_or("expected KEY=VALUE")?;
    match Value::from_string(v.to_string()) {
        Option::Some(v) => Result::Ok((k.to_string(), v)),
        Option::None => Result::Err(format!("failed to parse {}", v)),
    }
}
