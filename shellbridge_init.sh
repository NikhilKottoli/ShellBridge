#!/bin/bash

# ShellBridge Integration Script
# Source this file in your .bashrc or .zshrc
# example: source /path/to/shellbridge_init.sh

# Function to handle unknown commands
# It attempts to translate the command using ShellBridge using the 'translate' subcommand.
# If a translation is found, it suggests it.

shellbridge_suggest() {
    local cmd="$1"
    # Assuming 'shellbridge' binary is in your PATH.
    # If not, you can hardcode the path: 
    # local SB_BIN="/path/to/shellbridge"
    # echo "ShellBridge: Command '$cmd' not found. Checking for translation..."

    local translation=""
    
    if command -v shellbridge &> /dev/null; then
        echo "ShellBridge: Command '$cmd' not found. Checking for translation..."
        translation=$(shellbridge translate "$cmd")
    elif command -v cargo &> /dev/null; then
        echo "ShellBridge: Command '$cmd' not found. Checking for translation..."
        translation=$(cargo run --quiet -- translate "$cmd")
    else
        return 127
    fi
    
    if [ -n "$translation" ] && [ "$translation" != "No direct translation found." ]; then
        echo -e "\033[0;32mDid you mean:\033[0m"
        echo -e "  \033[1m$translation\033[0m"
        
        # Optional: Ask to execute
        echo -n "Run this command? [y/N] "
        read -r response
        if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            eval "$translation"
            return $?
        fi
    else
        echo "No translation found."
    fi
    
    return 127
}

# Bash Hook
if [ -n "$BASH_VERSION" ]; then
    command_not_found_handle() {
        shellbridge_suggest "$*"
        return $?
    }
fi

# Zsh Hook
if [ -n "$ZSH_VERSION" ]; then
    command_not_found_handler() {
        shellbridge_suggest "$*"
        return $?
    }
fi
