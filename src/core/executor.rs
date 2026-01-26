use std::process::Command;
use anyhow::{Result, anyhow};
use dialoguer::Confirm;
use colored::*;

pub fn execute_command(cmd: &str) -> Result<()> {
    if is_dangerous(cmd) {
        println!("{}", "WARNING: This command looks potentially dangerous.".red().bold());
        println!("Command: {}", cmd.yellow());
        
        let confirmed = Confirm::new()
            .with_prompt("Do you want to execute this command?")
            .default(false)
            .interact()?;

        if !confirmed {
            return Err(anyhow!("Execution aborted by user."));
        }
    }

    println!("Executing: {}", cmd);

    // Determine shell based on OS logic or just use sh/cmd
    #[cfg(target_os = "windows")]
    let status = Command::new("cmd")
        .args(["/C", cmd])
        .status()?;

    #[cfg(not(target_os = "windows"))]
    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Command exited with failure status"));
    }

    Ok(())
}

fn is_dangerous(cmd: &str) -> bool {
    let dangerous_patterns = [
        "rm", "dd", "mkfs", ":(){:|:&};:", "> /dev/sda", "format", "del /s", "rd /s"
    ];

    for pattern in &dangerous_patterns {
        // Very basic check, can be improved with regex or tokenization
        if cmd.contains(pattern) {
            return true;
        }
    }
    false
}
