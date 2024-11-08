use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self};

use clipboard::{ClipboardContext, ClipboardProvider};

use crate::{error::SnipsterError, storage::file::Snipster};

pub fn copy_to_clipboard(content: &String) -> Result<Snipster, SnipsterError> {
    execute!(io::stdout(), Clear(ClearType::All))?;

    println!(
        "The following command is copied to the clipboard: \x1b[33m{}",
        content // Make the command text green
    );

    // Create a clipboard context
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| {
        SnipsterError::CommandError(format!("Failed to initialize clipboard: {}", e))
    })?;

    // Set the content to the clipboard
    ctx.set_contents(content.to_string()).map_err(|e| {
        SnipsterError::ClipboardError(format!("Failed to copy to clipboard: {}", e))
    })?;

    Ok(Snipster { snippet: None })
}
