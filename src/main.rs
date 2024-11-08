use clipboard::copy_to_clipboard;
use std::process;

use clap::{Parser, Subcommand};
use commands::commands::SnipsterCommand;
use error::SnipsterError;
use storage::{
    file::{Snippet, Snipster},
    placeholder::PlaceHolder,
};

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
    // Add {
    //     #[arg(short = 'n', long)]
    //     name: String,
    //     #[arg(short = 'c', long)]
    //     content: String,
    //     #[arg(short = 't', long)]
    //     note: String,
    // },
    List,
    Write,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[91m\rerror:\x1b[0m {e}");
        process::exit(1);
    }
}

fn run() -> Result<Snipster, SnipsterError> {
    let cli = Cli::parse();

    let result = match &cli.command {
        // Some(Commands::Add {
        //     name,
        //     content,
        //     note,
        // }) => SnipsterCommand::add_snip(name, content, note),
        Some(Commands::List) => SnipsterCommand::get_snip_with_fzf(),
        Some(Commands::Write) | None => match SnipsterCommand::get_snip_with_fzf() {
            Ok(snip) => {
                if let Some(snippet) = snip.snippet {
                    let output = SnipsterCommand::edit_command_with_input(snippet.content.as_str());
                    let command = match output {
                        Ok(output) => PlaceHolder::replace_with_value(&snippet, &output),
                        Err(e) => Err(e),
                    };
                    match command {
                        Ok(cmd) => copy_to_clipboard(&cmd),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(SnipsterError::CommandError("ERROR".to_string()))
                }
            }
            Err(e) => Err(e),
        },
    };

    Ok(result?)
}
