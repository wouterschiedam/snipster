use clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(content: &str) -> Result<(), String> {
    // Create a clipboard context
    let mut ctx: ClipboardContext =
        ClipboardProvider::new().map_err(|e| format!("Failed to initialize clipboard: {}", e))?;

    // Set the content to the clipboard
    ctx.set_contents(content.to_string())
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;

    Ok(())
}
