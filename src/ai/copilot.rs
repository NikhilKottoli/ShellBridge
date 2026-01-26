use std::process::Command;
use anyhow::{Result, anyhow};
use crate::core::os_detector::Platform;

pub struct CopilotWrapper;

impl CopilotWrapper {
    pub fn new() -> Self {
        Self
    }

    pub fn translate(&self, cmd: &str, target_os: &Platform) -> Result<String> {
        let prompt = format!(
            "Translate '{}' to {} command line. Output ONLY the raw command string. Do not use markdown code blocks. Do not provide any explanation.",
            cmd, target_os
        );

        let output = self.run_copilot(&prompt)?;
        
        // Clean up the output
        let clean = output.trim();
        // Remove markdown code ticks if present despite instructions
        let clean = clean.trim_matches('`').trim();
        
        Ok(clean.to_string())
    }

    pub fn explain(&self, cmd: &str) -> Result<String> {
        let prompt = format!("Explain the shell command '{}'. Keep it concise.", cmd);
        self.run_copilot(&prompt)
    }

    fn run_copilot(&self, prompt: &str) -> Result<String> {
        let output = Command::new("copilot")
            .arg("-p")
            .arg(prompt)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Copilot CLI failed: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Parse out the actual content from the usage stats
        // Usage stats typically start with "Total usage est:"
        let content = if let Some(idx) = stdout.find("\n\nTotal usage est:") {
            &stdout[..idx]
        } else if let Some(idx) = stdout.find("Total usage est:") {
             &stdout[..idx]
        } else {
            &stdout
        };

        Ok(content.trim().to_string())
    }
}
