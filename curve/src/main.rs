use clap::Parser;

mod subcommand;

use subcommand::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::G1(g1) => {
            println!("x {} y {}", g1.x, g1.y);
        }
        Commands::G2(g2) => {
            println!("x {} y {}", g2.x, g2.y);
        }
    }
}
