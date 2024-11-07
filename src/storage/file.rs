use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Read;

use crate::error::StripsterError;

const SNIPPET_LOCATION: &str = "./snippets.json";

pub type SnippetCollection = HashMap<String, Snippet>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snippet {
    pub name: String,
    pub content: String,
    pub note: String,
}

pub fn load_snippets() -> Result<HashMap<String, Snippet>, StripsterError> {
    if !std::path::Path::new(SNIPPET_LOCATION).exists() {
        return Ok(HashMap::new());
    }

    let mut file = fs::File::open(SNIPPET_LOCATION)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let snippets = serde_json::from_str(&contents)?;

    Ok(snippets)
}

pub fn write_snippet(snip: Snippet) -> Result<(), StripsterError> {
    let mut snippets = load_snippets()?;

    // Insert the new snippet
    snippets.insert(snip.name.to_string(), snip);

    // Open the file for writing
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(SNIPPET_LOCATION)?;

    serde_json::to_writer_pretty(file, &snippets)?;

    Ok(())
}