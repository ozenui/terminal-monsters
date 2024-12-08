#!/bin/bash

# Configuration
INSTALL_DIR="$HOME/.terminal-monsters"
BIN_DIR="$HOME/.local/bin"
SHELL_CONFIG="$HOME/.bashrc"
GITHUB_REPO="enzo-rma/terminal-monsters"
VERSION="latest"

# Determine OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
case $ARCH in
    x86_64) ARCH="x86_64" ;;
    aarch64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Determine shell configuration file
if [ "$(basename "$SHELL")" = "zsh" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
fi

# Create directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$BIN_DIR"

# Download binaries
echo "Downloading Terminal Monsters..."
DOWNLOAD_URL="https://github.com/$GITHUB_REPO/releases/$VERSION/download/terminal-monsters-$OS-$ARCH.tar.gz"
curl -L "$DOWNLOAD_URL" | tar xz -C "$INSTALL_DIR"

# Install binaries
chmod +x "$INSTALL_DIR/terminal-monsters-app"
chmod +x "$INSTALL_DIR/terminal-monsters-worker"
ln -sf "$INSTALL_DIR/terminal-monsters-app" "$BIN_DIR/tm"
ln -sf "$INSTALL_DIR/terminal-monsters-worker" "$BIN_DIR/tm-worker"

# Add shell configuration
cat << EOF >> "$SHELL_CONFIG"

# Terminal Monsters configuration
export PATH="\$PATH:$BIN_DIR"
precmd() {
    eval \$(tm-worker)
}
alias tm='terminal-monsters-app'
EOF

# Notify success
printf "\n\033[32m✓ Terminal Monsters installed successfully!\033[0m\n"
printf "Run 'source $SHELL_CONFIG' to complete the installation.\n"
printf "Then type 'tm' to start the game.\n"
