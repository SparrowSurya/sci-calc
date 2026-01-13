use clap::{Parser, Subcommand, ArgAction};


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
        long = "ignore-func",
        help = "disallowed builtin functions",
        num_args = 1..,
    )]
    pub ignore_func: Vec<String>,

    #[arg(
        long = "ignore-const",
        help = "disallowed builtin constants",
        num_args = 1..,
    )]
    pub ignore_const: Vec<String>,

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
