use sys_info;

#[derive(Debug, PartialEq, Clone)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
    Unknown,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Linux => write!(f, "linux"),
            Platform::MacOS => write!(f, "macos"),
            Platform::Windows => write!(f, "windows"),
            Platform::Unknown => write!(f, "unknown"),
        }
    }
}

pub fn detect_os() -> Platform {
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
