
#!/usr/bin/env bash
set -Eeuo pipefail

REPO="https://github.com/svl10/plsfetch.git"
INSTALL_DIR="${CARGO_HOME:-$HOME/.cargo}/bin"

info() { printf '\033[1;34m[INFO]\033[0m %s\n' "$*"; }
error() { printf '\033[1;31m[ERROR]\033[0m %s\n' "$*" >&2; }

# Check required download utility
if ! command -v curl >/dev/null 2>&1; then
    error "curl is required. Please install curl and try again."
    exit 1
fi

# Install Rust and Cargo if missing
if ! command -v cargo >/dev/null 2>&1; then
    info "Rust/Cargo not found. Installing Rust using rustup..."

    if ! command -v sh >/dev/null 2>&1; then
        error "sh is required to install Rust."
        exit 1
    fi

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        -o "${TMPDIR:-/tmp}/plsfetch-rustup.sh"

    sh "${TMPDIR:-/tmp}/plsfetch-rustup.sh" -y
    rm -f "${TMPDIR:-/tmp}/plsfetch-rustup.sh"

    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Verify Cargo is available
if ! command -v cargo >/dev/null 2>&1; then
    error "Cargo is unavailable. Add ~/.cargo/bin to your PATH."
    exit 1
fi

info "Rust version: $(rustc --version)"
info "Installing plsfetch from GitHub..."

# Build and install the application
cargo install --git "$REPO"

# Ensure the installed binary directory is on PATH
mkdir -p "$INSTALL_DIR"

case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        export PATH="$INSTALL_DIR:$PATH"
        info "Add this to your shell configuration if needed:"
        printf 'export PATH="%s:$PATH"\n' "$INSTALL_DIR"
        ;;
esac

if command -v plsfetch >/dev/null 2>&1; then
    info "plsfetch installed successfully!"
    plsfetch
else
    info "Installation finished. Run this command to start:"
    printf '  %s/plsfetch\n' "$INSTALL_DIR"
fi
