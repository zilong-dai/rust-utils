use clap::command;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    G1(G1Args),
    G2(G2Args),
}

#[derive(Clone, Args)]
pub struct G1Args {
    #[clap(long, short)]
    pub x: String,
    #[clap(long, short)]
    pub y: String,
}

#[derive(Clone, Args)]
pub struct G2Args {
    #[clap(long, short)]
    pub x: String,
    #[clap(long, short)]
    pub y: String,
}
