use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    commands::{commands::SnipsterCommand, fzf_builder::FzfBuilder},
    error::SnipsterError,
};

use super::file::Snippet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlaceHolder {
    PID,       // Process ID (for commands like `kill`)
    File,      // File name/path (for commands like `cp`, `mv`, `rm`)
    Directory, // Directory path (for commands like `cd`, `ls`)
    Container, // Docker container ID or name (for `docker stop`, `docker rm`)
    Image,     // Docker image ID or name (for `docker rmi`, `docker pull`)
    Port,      // Network port (for commands like `netstat`, `kill`)
    User,      // Username (for `sudo`, `killall`, `chown`)
    Group,     // Group name (for `chgrp`, `groups`)
    Command,   // Command name (for `man`, `which`, `killall`)
    Package,   // Package name (for `apt-get`, `yum`, `brew`)
    Interface, // Network interface (for `ifconfig`, `ip`)
    Service,   // Service name (for `systemctl`, `service`)
    IPAddress, // IP address (for `ping`, `traceroute`, `ssh`)
    URL,       // URL (for `curl`, `wget`)
    Device,    // Device name (for `mount`, `umount`, `lsblk`)
    Disk,      // Disk name or partition (for `df`, `fsck`)
    Shell,     // Shell type (for `chsh`)
    Date,      // Date string (for `date`, `touch`)
    Time,      // Time string (for scheduling or logging)
    Signal,    // Signal type (e.g., `SIGKILL`, `SIGTERM`, for `kill -s`)
    Unknown(String),
}
impl PlaceHolder {
    pub fn extract_placeholders(content: &str) -> Result<Vec<PlaceHolder>, String> {
        let placeholder_regex =
            Regex::new(r"<[^>]+>").map_err(|e| format!("Invalid regex: {}", e))?;

        let placeholders: Vec<PlaceHolder> = placeholder_regex
            .find_iter(content)
            .map(|m| PlaceHolder::from_string(m.as_str()))
            .collect();

        Ok(placeholders)
    }

    pub fn handle(&self) -> Result<String, SnipsterError> {
        match self {
            PlaceHolder::Container => {
                let fzf = FzfBuilder::new()
                    .ansi()
                    .reverse()
                    .header(1)
                    .bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, "docker ps")
            }
            _ => Err(SnipsterError::CommandError(
                "No handler defined for this placeholder.".into(),
            )),
        }
    }

    pub fn replace_with_value(snippet: &Snippet, value: &String) -> Result<String, SnipsterError> {
        let mut command = snippet.content.clone(); // Start with the content of the snippet

        for placeholder in &snippet.placeholders {
            match placeholder {
                PlaceHolder::Container => {
                    command = command.replace("<container>", value); // Replace the placeholder with the value
                }
                _ => Err(SnipsterError::CommandError(
                    "No handler defined for this placeholder.".into(),
                ))?,
            }
        }

        Ok(command) // Return the modified command
    }

    fn from_string(s: &str) -> Self {
        match s {
            "<PID>" => PlaceHolder::PID,
            "<file>" => PlaceHolder::File,
            "<directory>" => PlaceHolder::Directory,
            "<container>" => PlaceHolder::Container,
            "<image>" => PlaceHolder::Image,
            "<port>" => PlaceHolder::Port,
            "<user>" => PlaceHolder::User,
            "<group>" => PlaceHolder::Group,
            "<command>" => PlaceHolder::Command,
            "<package>" => PlaceHolder::Package,
            "<interface>" => PlaceHolder::Interface,
            "<service>" => PlaceHolder::Service,
            "<ip_address>" => PlaceHolder::IPAddress,
            "<url>" => PlaceHolder::URL,
            "<device>" => PlaceHolder::Device,
            "<disk>" => PlaceHolder::Disk,
            "<shell>" => PlaceHolder::Shell,
            "<date>" => PlaceHolder::Date,
            "<time>" => PlaceHolder::Time,
            "<signal>" => PlaceHolder::Signal,
            _ => PlaceHolder::Unknown(s.to_string()), // Handle unknown placeholders
        }
    }
}
