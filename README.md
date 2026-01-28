# ShellBridge

ShellBridge is an intelligent CLI tool designed to bridge the gap between different operating system shells. It translates shell commands effectively between Linux, macOS, and Windows, allowing developers to work seamlessly across environments without memorizing platform-specific syntax.

## 🚀 Features

- **Cross-Platform Translation**: Translate commands between Linux, macOS, and Windows.
- **Intelligent Fallback**: Uses a local database for common commands and falls back to GitHub Copilot for complex queries.
- **Piped Command Support**: Automatically splits and translates piped commands (e.g., `ip a | grep "hello"`).
- **Command Explanation**: Get concise explanations for what a command does.
- **Safe Execution**: distinct mode to execute translated commands.
- **OS Detection**: Automatically detects your current operating system.

## 🛠️ Installation

### Prerequisites
- **Rust Toolchain**: Ensure you have Rust and Cargo installed. [Install Rust](https://www.rust-lang.org/tools/install)
- **GitHub Copilot CLI**: For AI-powered translations, you need the Copilot CLI installed and authenticated.

### Build from Source

```bash
git clone https://github.com/NikhilKottoli/ShellBridge.git
cd ShellBridge
cargo build --release
```

The binary will be available in `target/release/shellbridge`.

## 📖 Usage

### Basic Translation

Translate a command to your current OS (default) or a specific target.

```bash
# Translate 'ip a' to macOS (defaults to current OS if not specified)
cargo run -- translate "ip a" --target macos
# Output: ifconfig

# Translate 'dir' from Windows to Linux
cargo run -- translate "dir" --target linux
# Output: ls -la
```

### Piped Commands

ShellBridge intelligently handles pipes, translating each component individually.

```bash
cargo run -- translate "ip a | grep 'inet'" --target macos
# Output: ifconfig | grep 'inet'
```

### Explain a Command

Not sure what a command does? Ask ShellBridge.

```bash
cargo run -- explain "chmod 777"
# Output: Sets read, write, and execute permissions for all users on a file or directory.
```

### Execute Immediately

You can run the translated command directly using the `--execute` (or `-e`) flag.

```bash
cargo run -- translate "ls -la" --execute
```

## 🏗️ Architecture

- **Core**: Handles CLI parsing and OS detection.
- **Translation Engine**:
    1. Checks local JSON database (`data/commands.json`) for exact matches.
    2. If piped, splits and processes recursively.
    3. If no match, queries GitHub Copilot CLI for an AI-generated translation.

## 📝 License

MIT License
