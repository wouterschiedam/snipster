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
    History,
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
            PlaceHolder::PID => {
                let fzf = FzfBuilder::new()
                    .ansi()
                    .reverse()
                    .header(1)
                    .bind("enter:become(echo {2})");
                SnipsterCommand::fzf_with_command(fzf, Some("ps aux"))
            }
            PlaceHolder::File => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("find ~ -type f"))
            }
            PlaceHolder::Directory => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("find ~ -type d"))
            }
            PlaceHolder::Container => {
                let fzf = FzfBuilder::new()
                    .ansi()
                    .reverse()
                    .header(1)
                    .bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("docker ps"))
            }
            PlaceHolder::Image => {
                let fzf = FzfBuilder::new()
                    .ansi()
                    .reverse()
                    .header(1)
                    .bind("enter:become(echo {3})");
                SnipsterCommand::fzf_with_command(fzf, Some("docker images"))
            }
            PlaceHolder::Port => {
                let fzf = FzfBuilder::new()
                    .ansi()
                    .reverse()
                    .header(1)
                    .bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("netstat -tuln"))
            }
            PlaceHolder::User => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("getent passwd"))
            }
            PlaceHolder::Group => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("getent group"))
            }
            PlaceHolder::Command => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("compgen -c"))
            }
            PlaceHolder::Package => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("dpkg --get-selections"))
            }
            PlaceHolder::Interface => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("ip link show"))
            }
            PlaceHolder::Service => {
                let fzf = FzfBuilder::new()
                    .ansi()
                    .reverse()
                    .header(1)
                    .bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("systemctl list-units --type=service"))
            }
            PlaceHolder::IPAddress => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("ip a"))
            }
            PlaceHolder::URL => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("curl --list-only"))
            }
            PlaceHolder::Device => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("lsblk"))
            }
            PlaceHolder::Disk => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("lsblk -o NAME,SIZE,TYPE,MOUNTPOINT"))
            }
            PlaceHolder::Shell => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("cat /etc/shells"))
            }
            PlaceHolder::Date => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("date"))
            }
            PlaceHolder::Time => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("date +'%H:%M:%S'"))
            }
            PlaceHolder::Signal => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("kill -l"))
            }
            PlaceHolder::History => {
                let fzf = FzfBuilder::new().bind("enter:become(echo {1})");
                SnipsterCommand::fzf_with_command(fzf, Some("history"))
            }
            PlaceHolder::Unknown(val) => Err(SnipsterError::CommandError(format!(
                "Unknown placeholder: {}",
                val
            ))),
            // _ => Err(SnipsterError::CommandError(format!(
            //     "No handler defined for this placeholder: {:?}",
            //     self
            // ))),
        }
    }

    pub fn replace_with_value(
        snippet: &Snippet,
        values: &Vec<String>,
    ) -> Result<String, SnipsterError> {
        let mut command = snippet.content.clone(); // Start with the content of the snippet

        if snippet.placeholders.len() != values.len() {
            return Err(SnipsterError::CommandError(
                "Mismatch in the number of placeholders and values".into(),
            ));
        }

        for (i, placeholder) in snippet.placeholders.iter().enumerate() {
            let value = &values[i];
            match placeholder {
                PlaceHolder::Container => {
                    command = command.replace("<container>", value);
                }
                PlaceHolder::PID => {
                    command = command.replace("<PID>", value);
                }
                PlaceHolder::File => {
                    command = command.replace("<file>", value);
                }
                PlaceHolder::Directory => {
                    command = command.replace("<directory>", value);
                }
                PlaceHolder::Image => {
                    command = command.replace("<image>", value);
                }
                PlaceHolder::Port => {
                    command = command.replace("<port>", value);
                }
                PlaceHolder::User => {
                    command = command.replace("<user>", value);
                }
                PlaceHolder::Group => {
                    command = command.replace("<group>", value);
                }
                PlaceHolder::Command => {
                    command = command.replace("<command>", value);
                }
                PlaceHolder::Package => {
                    command = command.replace("<package>", value);
                }
                PlaceHolder::Interface => {
                    command = command.replace("<interface>", value);
                }
                PlaceHolder::Service => {
                    command = command.replace("<service>", value);
                }
                PlaceHolder::IPAddress => {
                    command = command.replace("<ip_address>", value);
                }
                PlaceHolder::URL => {
                    command = command.replace("<url>", value);
                }
                PlaceHolder::Device => {
                    command = command.replace("<device>", value);
                }
                PlaceHolder::Disk => {
                    command = command.replace("<disk>", value);
                }
                PlaceHolder::Shell => {
                    command = command.replace("<shell>", value);
                }
                PlaceHolder::Date => {
                    command = command.replace("<date>", value);
                }
                PlaceHolder::Time => {
                    command = command.replace("<time>", value);
                }
                PlaceHolder::Signal => {
                    command = command.replace("<signal>", value);
                }
                PlaceHolder::History => command = value.to_string(),
                PlaceHolder::Unknown(s) => {
                    command = command.replace(&format!("<{}>", s), value);
                }
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
            "<history>" => PlaceHolder::History,
            "<signal>" => PlaceHolder::Signal,
            _ => PlaceHolder::Unknown(s.to_string()), // Handle unknown placeholders
        }
    }
}
