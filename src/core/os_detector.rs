use sys_info;

#[derive(Debug, PartialEq, Clone)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
    WSL,
    Docker,
    GitBash,
    Unknown,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Linux => write!(f, "linux"),
            Platform::MacOS => write!(f, "macos"),
            Platform::Windows => write!(f, "windows"),
            Platform::WSL => write!(f, "wsl"),
            Platform::Docker => write!(f, "docker"),
            Platform::GitBash => write!(f, "gitbash"),
            Platform::Unknown => write!(f, "unknown"),
        }
    }
}

pub fn detect_os() -> Platform {
    // Check for Docker
    if std::path::Path::new("/.dockerenv").exists() {
        return Platform::Docker;
    }

    // Check for WSL
    if let Ok(version) = std::fs::read_to_string("/proc/version") {
        if version.to_lowercase().contains("microsoft") {
            return Platform::WSL;
        }
    }

    // Check for Git Bash
    if std::env::var("MSYSTEM").is_ok() {
        return Platform::GitBash;
    }

    let os_type = sys_info::os_type().unwrap_or("unknown".to_string()).to_lowercase();
    
    if os_type.contains("linux") {
        Platform::Linux
    } else if os_type.contains("darwin") || os_type.contains("macos") {
        Platform::MacOS
    } else if os_type.contains("windows") || os_type.contains("win32") {
        Platform::Windows
    } else {
        Platform::Unknown
    }
}
