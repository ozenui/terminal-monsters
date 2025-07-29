#!/bin/bash

set -euo pipefail

# Configuration
CONFIG_DIR="$HOME/.config/terminal-monsters"
APP_PATH="/Applications/Terminal Monsters.app"
MAIN_BINARY="$APP_PATH/Contents/MacOS/terminal-monsters"
WORKER_BINARY="$APP_PATH/Contents/MacOS/terminal-monsters-worker"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Helper functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

verify_installation() {
    if [ ! -d "$APP_PATH" ]; then
        log_error "Terminal Monsters.app not found in Applications folder"
        exit 1
    fi

    if [ ! -x "$WORKER_BINARY" ]; then
        log_error "Worker binary not found or not executable at $WORKER_BINARY"
        exit 1
    fi

    if [ ! -x "$MAIN_BINARY" ]; then
        log_error "Main binary not found or not executable at $MAIN_BINARY"
        exit 1
    fi
}

get_shell() {
    local shell_path
    shell_path=$(basename "$SHELL")
    case "$shell_path" in
        "zsh")
            echo "$HOME/.zshrc"
            ;;
        "bash")
            echo "$HOME/.bashrc"
            ;;
        *)
            log_warning "Unsupported shell: $shell_path. Defaulting to bash configuration."
            echo "$HOME/.bashrc"
            ;;
    esac
}

configure_shell() {
    local shell_config
    shell_config=$(get_shell)

    if [ ! -f "$shell_config" ]; then
        touch "$shell_config"
    fi

    if grep -q "Terminal Monsters Configuration" "$shell_config"; then
        log_warning "Terminal Monsters configuration already exists in $shell_config"
        return
    fi

    # Escape the paths for the shell configuration
    local escaped_worker_binary=$(printf %q "$WORKER_BINARY")
    local escaped_main_binary=$(printf %q "$MAIN_BINARY")

    log_info "Adding configuration to $shell_config"
    if ! cat >> "$shell_config" << EOF

# Terminal Monsters Configuration
tm_preexec_invoke_exec() {
    [ -n "\$COMP_LINE" ] && return
    local cmd="\$1"
    echo "\$cmd" | $escaped_worker_binary
}
if [ -n "\$BASH_VERSION" ]; then
    trap 'tm_preexec_invoke_exec "\$BASH_COMMAND"' DEBUG
elif [ -n "\$ZSH_VERSION" ]; then
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec tm_preexec_invoke_exec
fi
alias tm='$escaped_main_binary'
EOF
    then
        log_error "Failed to update shell configuration"
        exit 1
    fi
    log_info "Added Terminal Monsters configuration to $shell_config"
}

cleanup() {
    if [ $? -ne 0 ]; then
        log_error "Installation failed"
    fi
}

main() {
    trap cleanup EXIT

    log_info "Starting installation..."

    # Verify app installation
    verify_installation

    # Create config directory
    mkdir -p "$CONFIG_DIR"

    # Configure shell
    configure_shell

    log_info "Installation complete!"
    log_info "Restart your terminal or run 'source $(get_shell)'"
}

main "$@"
