use crate::core::command_db::{Command, load_db};
use crate::core::os_detector::Platform;
use crate::ai::copilot::CopilotWrapper;
use anyhow::{Result};
use std::path::PathBuf;

pub struct TranslationEngine {
    commands: Vec<Command>,
    copilot: CopilotWrapper,
}

impl TranslationEngine {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let commands = load_db(db_path)?;
        let copilot = CopilotWrapper::new();
        Ok(Self { commands, copilot })
    }

    pub fn translate(&self, cmd: &str, target_os: &Platform) -> Option<String> {
        // 1. Try Local DB Lookup
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
        
        // 2. Fallback to Copilot
        println!("  (No local match found. Asking Copilot...)");
        match self.copilot.translate(cmd, target_os) {
            Ok(translation) => Some(translation),
            Err(e) => {
                eprintln!("Copilot translation failed: {}", e);
                None
            }
        }
    }

    fn get_command_for_platform(&self, entry: &Command, target: &Platform) -> Option<String> {
        match target {
            Platform::Linux => entry.linux.clone(),
            Platform::MacOS => entry.macos.clone(),
            Platform::Windows => entry.windows.clone(),
            Platform::Unknown => None,
        }
    }
    
    pub fn explain(&self, cmd: &str) -> Option<String> {
        // For explanation, we can check DB description first, but Copilot is better usually.
        // Let's check DB description first for speed.
        for entry in &self.commands {
             if (entry.linux.as_deref() == Some(cmd)) || 
                (entry.macos.as_deref() == Some(cmd)) || 
                (entry.windows.as_deref() == Some(cmd)) {
                 return Some(entry.description.clone());
             }
        }
        
        // Fallback to copilot
        println!("  (Asking Copilot for explanation...)");
        self.copilot.explain(cmd).ok()
    }
}
