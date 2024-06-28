use std::result::Result;

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
    Rev(RevArgs),
}

#[derive(Clone, Args)]
pub struct RevArgs {
    pub arg: String,
    #[arg(short, long)]
    pub num: Option<usize>,
}

#[derive(Debug, thiserror::Error)]
pub enum ByteReverseError {
    #[error("num must be even")]
    NumNotEvenError,
    #[error("string too long")]
    StringTooLongError,
}

fn byte_reverse(s: &str, num: usize) -> Result<String, ByteReverseError> {
    // Check if the length of the string is greater than the specified num.
    if s.len() > num {
        return Err(ByteReverseError::StringTooLongError);
    }

    // Check if num is even.
    if num % 2 != 0 {
        return Err(ByteReverseError::NumNotEvenError);
    }

    let pad_num = num - s.len();
    let s: String = format!("{}{}", "0".repeat(pad_num), s);

    let mut res = String::with_capacity(s.len());
    for chunk in s.as_bytes().chunks(2).rev() {
        //the chunk mut has a length of 2.
        assert_eq!(chunk.len(), 2);
        res.push(chunk[0].into());
        res.push(chunk[1].into());
    }

    Ok(res)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Rev(rev) => {
            let num = rev.num.unwrap_or(if rev.arg.len() % 2 == 0 {
                rev.arg.len()
            } else {
                rev.arg.len() + 1
            });
            let result = byte_reverse(&rev.arg, num).unwrap_or_else(|e| e.to_string());
            println!("{}", result);
        }
        // _ => {
        //     eprintln!("invalid command");
        //     std::process::exit(1);
        // }
    }
    Ok(())
}
