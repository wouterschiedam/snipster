use std::process;

use clap::{Parser, Subcommand};
use commands::commands::StripsterCommand;
use error::StripsterError;

mod clipboard;
mod commands;
mod config;
mod error;
mod storage;

#[derive(Parser)]
#[command(name = "Snippet Manager")]
#[command(about = "Manage and organize your code snippets.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short = 'n', long)]
        name: String,
        #[arg(short = 'c', long)]
        content: String,
        #[arg(short = 't', long)]
        note: String,
    },
    List {
        #[arg(long)]
        copy: bool,
    },
    Write,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[91m\rerror:\x1b[0m {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), StripsterError> {
    let cli = Cli::parse();

    let _ = match &cli.command {
        Some(Commands::Add {
            name,
            content,
            note,
        }) => StripsterCommand::add_snip(name, content, note),
        Some(Commands::List { copy }) => StripsterCommand::get_snip_with_fzf(*copy),
        Some(Commands::Write) | None => match StripsterCommand::get_snip_with_fzf(false) {
            Ok(snip) => {
                let output = StripsterCommand::edit_command_with_input(snip.content.as_str());
                Ok(snip)
            }
            Err(e) => {
                return Err(StripsterError::OutputParsingError(format!(
                    "Failed to get snippet: {}",
                    e
                )))
            }
        },
    };

    Ok(())
}
