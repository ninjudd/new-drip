#!/bin/sh
set -e

if [ "$1" = "--dev" ]; then
    echo "Building drip (debug)..."
    cargo build
    echo "Linking to /usr/local/bin/drip..."
    sudo ln -sf "$(pwd)/target/debug/drip" /usr/local/bin/drip
else
    echo "Building drip..."
    cargo build --release
    echo "Installing to /usr/local/bin/drip..."
    sudo cp target/release/drip /usr/local/bin/drip
fi

HOOK='# drip shell hook
if [ -n "$DRIP_SESSION" ]; then
  _drip_precmd() { eval "$(drip init)"; }
  if [ -n "$ZSH_VERSION" ]; then
    precmd_functions+=(_drip_precmd)
  elif [ -n "$BASH_VERSION" ]; then
    PROMPT_COMMAND="_drip_precmd${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
  fi
fi'

MARKER="# drip shell hook"

install_hook() {
    file="$1"
    if [ -f "$file" ]; then
        if grep -qF "$MARKER" "$file"; then
            echo "Shell hook already in $file"
            return
        fi
    fi
    printf '\n%s\n' "$HOOK" >> "$file"
    echo "Added shell hook to $file"
}

install_hook "$HOME/.zshrc"
install_hook "$HOME/.bashrc"

echo "Done. Run 'drip enter' to get started."
