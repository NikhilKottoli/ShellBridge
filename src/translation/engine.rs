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
        // 0. Split piped commands
        let parts = self.split_cmd(cmd);
        if parts.len() > 1 {
            let mut translated_parts = Vec::new();
            for part in parts {
                // Recursively translate each part
                if let Some(t_part) = self.translate(&part, target_os) {
                    translated_parts.push(t_part);
                } else {        
                    translated_parts.push(part);
                }
            }
            return Some(translated_parts.join(" | "));
        }

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
        println!("  (No local match found for '{}'. Asking Copilot...)", cmd);
        match self.copilot.translate(cmd, target_os) {
            Ok(translation) => Some(translation),
            Err(e) => {
                eprintln!("Copilot translation failed: {}", e);
                None
            }
        }
    }

    fn split_cmd(&self, cmd: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current_part = String::new();
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut escaped = false;

        for c in cmd.chars() {
            if escaped {
                current_part.push(c);
                escaped = false;
                continue;
            }

            match c {
                '\\' => {
                    escaped = true;
                    current_part.push(c);
                }
                '\'' if !in_double_quote => {
                    in_single_quote = !in_single_quote;
                    current_part.push(c);
                }
                '"' if !in_single_quote => {
                    in_double_quote = !in_double_quote;
                    current_part.push(c);
                }
                '|' if !in_single_quote && !in_double_quote => {
                    parts.push(current_part.trim().to_string());
                    current_part.clear();
                }
                _ => {
                    current_part.push(c);
                }
            }
        }
        if !current_part.trim().is_empty() {
            parts.push(current_part.trim().to_string());
        }
        parts
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
