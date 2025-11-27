use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "calc")]
#[command(about = "A command-line calculator powered by Rust evaluator engine.")]
pub struct Args {

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
