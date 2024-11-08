use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{
    env,
    io::{self, Write},
    process::{Command, Stdio},
};
use std::{path::PathBuf, process};

use clap::{Parser, Subcommand};
use commands::commands::SnipsterCommand;
use error::SnipsterError;
use storage::placeholder::PlaceHolder;

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
    // List {
    //     #[arg(short = 'c', long)]
    //     copy: bool,
    // },
    Write,
}

pub fn execute_command(cmd: String) -> Result<(), SnipsterError> {
    let current_dir = env::current_dir().map_err(|e| SnipsterError::IoError(e))?;

    let script_path: PathBuf = current_dir.join("sh").join("write_command.sh");

    // dbg!("{}", &cmd);

    let output = Command::new(script_path)
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| {
            SnipsterError::OutputParsingError(format!("Failed to execute script: {}", e))
        })?;

    // Check the output status
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SnipsterError::CommandError(format!("{}", stderr.trim())));
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[91m\rerror:\x1b[0m {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), SnipsterError> {
    let cli = Cli::parse();

    let result = match &cli.command {
        // Some(Commands::Add {
        //     name,
        //     content,
        //     note,
        // }) => SnipsterCommand::add_snip(name, content, note),
        // Some(Commands::List { copy }) => SnipsterCommand::get_snip_with_fzf(*copy),
        Some(Commands::Write) | None => match SnipsterCommand::get_snip_with_fzf(false) {
            Ok(snip) => {
                let output = SnipsterCommand::edit_command_with_input(snip.content.as_str());

                let command = match output {
                    Ok(output) => PlaceHolder::replace_with_value(&snip, &output),
                    Err(e) => Err(e),
                };

                match command {
                    Ok(cmd) => execute_command(cmd),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        },
    };

    Ok(result?)
}
