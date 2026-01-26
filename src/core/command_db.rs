use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub id: String,
    pub category: String,
    pub description: String,
    pub linux: Option<String>,
    pub macos: Option<String>,
    pub windows: Option<String>,
}

pub fn load_db<P: AsRef<Path>>(path: P) -> Result<Vec<Command>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let commands: Vec<Command> = serde_json::from_reader(reader)?;
    Ok(commands)
}
