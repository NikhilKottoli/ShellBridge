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

        let translated = self.translate_single(cmd, target_os);
        
        if let Some(t_cmd) = translated {
             // Post-processing
             let t_cmd = self.normalize_paths(&t_cmd, target_os);
             let t_cmd = self.translate_env_vars(&t_cmd, target_os);
             Some(t_cmd)
        } else {
            None
        }
    }

    fn translate_single(&self, cmd: &str, target_os: &Platform) -> Option<String> {
        // Separate command from arguments
        // This is a basic split by space. It respects quotes for the FIRST token using a simple check.
        // A robust parser would be better, but for now we try to find the first word.
        
        let (_base_cmd, _args) = match cmd.split_once(' ') {
            Some((first, rest)) => (first, Some(rest)),
            None => (cmd, None),
        };

        // 1. Try Local DB Lookup with the base command
        for entry in &self.commands {
            // Check if input matches Linux command
            if let Some(linux_cmd) = &entry.linux {
                // Check if the DB entry matches the base command
                // NOTE: The DB might contain flags (e.g. "ls -la"). 
                // We should probably check if the USER input STARTS with the DB entry?
                // OR check if the DB entry is just the command.
                // Case 1: DB="ls -la", User="ls -la /tmp" -> simple split won't work perfectly if we strip args.
                // Let's keep it simple: exact match of base command OR check if input starts with DB entry.
                
                // Let's try exact match of parts to keys in DB.
                // Actually, if DB has "ls -la", that's specific. 
                // If user types "ls -la /tmp", base is "ls". "ls" != "ls -la".
                // We need smarter matching. 
                
                // Iterated approach:
                // Check if the full user command STARTS with the db command + space or is equal.
                if cmd == linux_cmd || cmd.starts_with(&format!("{} ", linux_cmd)) {
                     if let Some(translated_base) = self.get_command_for_platform(entry, target_os) {
                         // We found a match!
                         // If we matched the prefix, we need to preserve the suffix (args).
                         if cmd.len() > linux_cmd.len() {
                             let suffix = &cmd[linux_cmd.len()..];
                             return Some(format!("{}{}", translated_base, suffix));
                         } else {
                             return Some(translated_base);
                         }
                     }
                }
            }
            // Check matches MacOS
             if let Some(macos_cmd) = &entry.macos {
                if cmd == macos_cmd || cmd.starts_with(&format!("{} ", macos_cmd)) {
                     if let Some(translated_base) = self.get_command_for_platform(entry, target_os) {
                         if cmd.len() > macos_cmd.len() {
                             let suffix = &cmd[macos_cmd.len()..];
                             return Some(format!("{}{}", translated_base, suffix));
                         } else {
                             return Some(translated_base);
                         }
                     }
                }
            }
            // Check matches Windows
             if let Some(windows_cmd) = &entry.windows {
                if cmd == windows_cmd || cmd.starts_with(&format!("{} ", windows_cmd)) {
                     if let Some(translated_base) = self.get_command_for_platform(entry, target_os) {
                         if cmd.len() > windows_cmd.len() {
                             let suffix = &cmd[windows_cmd.len()..];
                             return Some(format!("{}{}", translated_base, suffix));
                         } else {
                             return Some(translated_base);
                         }
                     }
                }
            }
        }
        
        // 2. Fallback to Copilot
        eprintln!("  (No local match found for '{}'. Asking Copilot...)", cmd);
        match self.copilot.translate(cmd, target_os) {
            Ok(translation) => Some(translation),
            Err(e) => {
                eprintln!("Copilot translation failed: {}", e);
                None
            }
        }
    }

    fn normalize_paths(&self, cmd: &str, target_os: &Platform) -> String {
        match target_os {
            Platform::Windows => cmd.replace('/', "\\"),
            Platform::Linux | Platform::MacOS | Platform::WSL | Platform::Docker | Platform::GitBash => cmd.replace('\\', "/"),
            Platform::Unknown => cmd.to_string(),
        }
    }

    fn translate_env_vars(&self, cmd: &str, target_os: &Platform) -> String {
        // Simple heuristic regex-like replacement
        match target_os {
            Platform::Windows => {
                // Convert $VAR to %VAR%
                // This is a naive implementation. For robust parsing, we'd need a tokenizer.
                // We'll look for $ followed by word characters.
                let mut result = String::new();
                let mut chars = cmd.chars().peekable();
                
                while let Some(c) = chars.next() {
                    if c == '$' {
                        let mut var_name = String::new();
                        while let Some(&next_c) = chars.peek() {
                            if next_c.is_alphanumeric() || next_c == '_' {
                                var_name.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        if !var_name.is_empty() {
                            result.push('%');
                            result.push_str(&var_name);
                            result.push('%');
                        } else {
                             result.push(c);
                        }
                    } else {
                        result.push(c);
                    }
                }
                result
            }
            Platform::Linux | Platform::MacOS | Platform::WSL | Platform::Docker | Platform::GitBash => {
                 // Convert %VAR% to $VAR
                 let mut result = String::new();
                 let mut chars = cmd.chars().peekable();
                 
                 while let Some(c) = chars.next() {
                     if c == '%' {
                         let mut var_name = String::new();
                         while let Some(&next_c) = chars.peek() {
                             if next_c != '%' {
                                 var_name.push(chars.next().unwrap());
                             } else {
                                 chars.next(); // Consume closing %
                                 break;
                             }
                         }
                         if !var_name.is_empty() {
                             result.push('$');
                             result.push_str(&var_name);
                         } else {
                             // Was not a pair of %, just a single one or empty
                             result.push('%');
                             result.push_str(&var_name);
                         }
                     } else {
                         result.push(c);
                     }
                 }
                 result
            }
            Platform::Unknown => cmd.to_string(),
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
            Platform::Linux | Platform::WSL | Platform::Docker | Platform::GitBash => entry.linux.clone(),
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
