use std::process::{Command, Stdio};

use crate::{
    error::SnipsterError,
    storage::{
        file::{write_snippet, Snippet, Snipster},
        placeholder::PlaceHolder,
    },
};

use super::fzf_builder::FzfBuilder;

pub struct SnipsterCommand;

impl SnipsterCommand {
    pub fn fzf_with_command(fzf: FzfBuilder, command: &str) -> Result<String, SnipsterError> {
        let full_command = format!(
            r#"
            {} | {}"#,
            command,
            fzf.build()
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(full_command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| {
                SnipsterError::OutputParsingError(format!("Failed to execute script: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let hint = if stderr.contains("fzf: command not found")
                || stderr.contains("jq: command not found")
            {
                Some("Ensure fzf and jq are installed.")
            } else {
                None
            };

            let hint_message = hint.map_or("".to_string(), |hint| format!("Hint: {}", hint));
            return Err(SnipsterError::CommandError(format!(
                "{} {}",
                stderr.trim(),
                hint_message
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(stdout)
    }

    pub fn get_snip_with_fzf() -> Result<Snipster, SnipsterError> {
        // Uhm yes, this is kinda messy.
        let command = r#"
    max_category_len=$(jq -r 'to_entries | .[] | .key | length' snippets.json | sort -nr | head -n 1)
    max_name_len=$(jq -r '.[] | .[] | .name | length' snippets.json | sort -nr | head -n 1)
    max_note_len=$(jq -r '.[] | .[] | .note | length' snippets.json | sort -nr | head -n 1)

    cat snippets.json | \
    jq -r 'to_entries | .[] | .key as $category | .value[] | "\($category)\t\(.name)\t\(.note)\t\(.content)\t\u0001\(. | @json)"' | \
    awk -F '\t' -v max_category_len="$max_category_len" -v max_name_len="$max_name_len" -v max_note_len="$max_note_len" '{
        printf "\033[35m%-*s\033[0m\t\033[32m%-*s\033[0m\t\033[36m%-*s\033[0m\t\033[33m%s\033[0m\t%s\n", max_category_len, $1, max_name_len, $2, max_note_len, $3, $4, $5;
    }' | \
    fzf --ansi --reverse --delimiter="\t" \
        --with-nth=1,2,3 \
        --preview='printf "\033[35m%-15s%s\033[0m\n\033[36m%-15s%s\033[0m\n\033[33m%-15s%s\033[0m\n\033[32m%-15s%s\033[0m\n" \
                  "Category ->" "{1}" "Name ->" "{2}" "Note ->" "{3}" "Command ->" "{4}"' \
        --preview-window=up:4:wrap \
        --bind='enter:become(echo {5})'"#;

        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| {
                SnipsterError::OutputParsingError(format!("Failed to execute script: {}", e))
            })?;

        let stdout = String::from_utf8(output.stdout)?;

        let unescaped_stdout = stdout.trim().trim_start_matches('\u{1}');

        let snippet: Snippet = serde_json::from_str(unescaped_stdout.trim())
            .map_err(|e| SnipsterError::SerdeError(e))?;

        Ok(Snipster {
            snippet: Some(snippet),
        })
    }

    pub fn add_snip(name: &str, content: &str, note: &str) -> Result<Snippet, SnipsterError> {
        let placeholders =
            PlaceHolder::extract_placeholders(content).map_err(SnipsterError::PlaceHolderError)?;

        let snip: Snippet = Snippet {
            name: name.to_string(),
            content: content.to_string(),
            note: note.to_string(),
            placeholders,
        };

        let _ = write_snippet(snip.clone());

        Ok(snip)
    }

    pub fn edit_command_with_input(content: &str) -> Result<String, SnipsterError> {
        let placeholders =
            PlaceHolder::extract_placeholders(content).map_err(SnipsterError::PlaceHolderError);

        match placeholders {
            Ok(result) => result
                .iter()
                .map(|x| x.handle())
                .collect::<Result<String, SnipsterError>>(),
            Err(_) => Err(SnipsterError::CommandError(
                "Failed to parse fzf command".to_string(),
            )),
        }
    }
}
