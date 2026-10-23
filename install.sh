
#!/usr/bin/env bash
set -Eeuo pipefail

REPO="svl10/plsfetch"
VERSION="v0.1.1"
INSTALL_DIR="${HOME}/.local/bin"
BINARY="plsfetch"

info() { printf '[plsfetch] %s\n' "$*"; }
die() { printf '[plsfetch] Error: %s\n' "$*" >&2; exit 1; }

# Check operating system
OS="$(uname -s)"
ARCH="$(uname -m)"

if [[ "$OS" != "Linux" ]]; then
    die "This installer currently supports Linux only."
fi

case "$ARCH" in
    x86_64|amd64) ;;
    *) die "Unsupported architecture: $ARCH (currently supports x86_64)." ;;
esac

# Check download utility
if command -v curl >/dev/null 2>&1; then
    DOWNLOAD="curl"
elif command -v wget >/dev/null 2>&1; then
    DOWNLOAD="wget"
else
    die "Please install curl or wget first."
fi

URL="https://github.com/${REPO}/releases/download/${VERSION}/plsfetch"
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT

info "Downloading plsfetch ${VERSION}..."

if [[ "$DOWNLOAD" == "curl" ]]; then
    curl -fL --retry 3 "$URL" -o "$TMP" ||
        die "Download failed. Check that the release contains the plsfetch asset."
else
    wget -O "$TMP" "$URL" ||
        die "Download failed. Check that the release contains the plsfetch asset."
fi

# Check that the downloaded file is an executable, not an error page
file_type="$(file -b "$TMP" 2>/dev/null || true)"
if [[ "$file_type" != *ELF* ]]; then
    die "Downloaded file is not a Linux executable: $file_type"
fi

mkdir -p "$INSTALL_DIR"
install -m 755 "$TMP" "${INSTALL_DIR}/${BINARY}"

info "Installed to ${INSTALL_DIR}/${BINARY}"

case ":${PATH}:" in
    *":${INSTALL_DIR}:"*) ;;
    *)
        info "Add this to your shell configuration:"
        printf 'export PATH="%s:$PATH"\n' "$INSTALL_DIR"
        ;;
esac

info "Installation complete!"
info "Run: ${INSTALL_DIR}/${BINARY}"
