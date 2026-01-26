use crate::core::command_db::{Command, load_db};
use crate::core::os_detector::Platform;
use anyhow::{Result, anyhow};
use std::path::PathBuf;

pub struct TranslationEngine {
    commands: Vec<Command>,
}

impl TranslationEngine {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let commands = load_db(db_path)?;
        Ok(Self { commands })
    }

    pub fn translate(&self, cmd: &str, target_os: &Platform) -> Option<String> {
        // Simple linear search for now. 
        // We check if the input 'cmd' matches any of the fields in any command entry.
        
        for entry in &self.commands {
            // Check if input matches Linux command
            if let Some(linux_cmd) = &entry.linux {
                if linux_cmd == cmd {
                    return self.get_command_for_platform(entry, target_os);
                }
            }
            // Check matches MacOS
            if let Some(macos_cmd) = &entry.macos {
                if macos_cmd == cmd {
                     return self.get_command_for_platform(entry, target_os);
                }
            }
            // Check matches Windows
             if let Some(windows_cmd) = &entry.windows {
                if windows_cmd == cmd {
                     return self.get_command_for_platform(entry, target_os);
                }
            }
        }
        
        None
    }

    fn get_command_for_platform(&self, entry: &Command, target: &Platform) -> Option<String> {
        match target {
            Platform::Linux => entry.linux.clone(),
            Platform::MacOS => entry.macos.clone(),
            Platform::Windows => entry.windows.clone(),
            Platform::Unknown => None,
        }
    }
}
