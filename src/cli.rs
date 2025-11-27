use clap::Parser;


#[derive(Parser)]
#[command(name = "calc")]
#[command(about = "A command-line calculator powered by Rust evaluator engine.")]
pub struct Args {
    pub expr: Option<String>,
}
