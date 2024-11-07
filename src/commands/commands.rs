use regex::Regex;
use std::io::{self, Write};
use std::process::{Command, Stdio};

use crate::{
    clipboard,
    error::StripsterError,
    storage::file::{write_snippet, Snippet},
};

pub struct StripsterCommand;

impl StripsterCommand {
    pub fn get_snip_with_fzf(copy: bool) -> Result<Snippet, StripsterError> {
        // Uhm yes, this is kinda messy.
        let command = r#"
            max_name_len=$(jq -r '.[] | .name | length' snippets.json | sort -nr | head -n 1)
            max_note_len=$(jq -r '.[] | .note | length' snippets.json | sort -nr | head -n 1)

            cat snippets.json | \
            jq -r '.[] | "\(.name)\t\(.note)\t\(.content)\t\u0001\(. | @json)"' | \
            awk -F '\t' -v max_name_len="$max_name_len" -v max_note_len="$max_note_len" '{
                printf "\033[32m%-*s\033[0m\t\033[36m%-*s\033[0m\t\033[33m%s\033[0m\t%s\n", max_name_len, $1, max_note_len, $2, $3, $4;
            }' | \

            fzf --ansi --reverse --delimiter="\t" \
                --with-nth=1,2,3 \
                --preview='echo -e "\033[1;36m{2} [{1}]\033[0m\n\033[1;33m{3}\033[0m"' \
                --preview-window=up:2:wrap \
                --bind='enter:become(echo {4})'"#;

        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8(output.stderr)?;

                    let hint = match &stderr {
                        stderr
                            if stderr.contains("fzf: command not found")
                                || stderr.contains("jq: command not found") =>
                        {
                            Some("List requires fzf and jq")
                        }
                        _ => None,
                    };

                    let hint = match hint {
                        Some(hint) => format!("hint: {}", hint),
                        None => String::new(),
                    };

                    return Err(StripsterError::CommandError(format!("{} {}", stderr, hint)));
                }

                let stdout = String::from_utf8(output.stdout)?;

                let unescaped_stdout = stdout
                    .trim()
                    .trim_start_matches('\u{1}')
                    .trim_end_matches('\"')
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\");

                let snippet: Snippet = serde_json::from_str(unescaped_stdout.trim())
                    .map_err(|e| StripsterError::SerdeError(e))?;

                if copy {
                    let _ = clipboard::copy_to_clipboard(&snippet.content);
                }

                Ok(snippet)
            }
            Err(e) => Err(StripsterError::OutputParsingError(format!(
                "Unexpected format from fzf output: {}",
                e.to_string()
            ))),
        }
    }

    pub fn add_snip(name: &str, content: &str, note: &str) -> Result<Snippet, StripsterError> {
        let snip: Snippet = Snippet {
            name: name.to_string(),
            content: content.to_string(),
            note: note.to_string(),
        };

        let _ = write_snippet(snip.clone());

        Ok(snip)
    }

    pub fn edit_command_with_input(content: &str) -> Result<String, String> {
        let mut final_command = content.to_string();

        // Regex to match placeholders like `<...>`
        let placeholder_regex =
            Regex::new(r"<[^>]+>").map_err(|e| format!("Invalid regex: {}", e))?;

        // Process each placeholder
        while let Some(captures) = placeholder_regex.find(&final_command) {
            let placeholder = captures.as_str(); // Extract placeholder like `<placeholder>`
            println!("Enter value for {}: ", placeholder);

            // Prompt user for input
            let mut input = String::new();
            print!("> "); // Show a prompt symbol
            io::stdout()
                .flush()
                .map_err(|e| format!("Failed to flush stdout: {}", e))?;
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| format!("Failed to read input: {}", e))?;

            let replacement = input.trim(); // Remove any trailing newline or spaces

            // Replace the placeholder in the command
            final_command = final_command.replacen(placeholder, replacement, 1);
        }

        dbg!("{}", &final_command);

        Ok(final_command)
    }
}
