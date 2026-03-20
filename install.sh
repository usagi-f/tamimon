#!/bin/sh
set -e

REPO="usagi-f/tamimon"
INSTALL_DIR="${TAMIMON_INSTALL_DIR:-/usr/local/bin}"

main() {
    need_cmd curl
    need_cmd uname
    need_cmd chmod
    need_cmd mkdir

    local arch os target

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)
            case "$arch" in
                x86_64) target="x86_64-unknown-linux-gnu" ;;
                *) err "Unsupported architecture: $arch (Linux supports x86_64 only)" ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64) target="x86_64-apple-darwin" ;;
                arm64)  target="aarch64-apple-darwin" ;;
                *) err "Unsupported architecture: $arch" ;;
            esac
            ;;
        *) err "Unsupported OS: $os (macOS and Linux only)" ;;
    esac

    echo "Detected platform: ${os} ${arch}"
    echo "Target: ${target}"
    echo ""

    local latest_tag
    latest_tag="$(curl -sSf "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')"

    if [ -z "$latest_tag" ]; then
        err "Could not determine latest release. Check https://github.com/${REPO}/releases"
    fi

    echo "Latest version: ${latest_tag}"

    local url="https://github.com/${REPO}/releases/download/${latest_tag}/tamimon-${target}"

    echo "Downloading tamimon from ${url}..."
    local tmp
    tmp="$(mktemp)"
    curl -sSfL "$url" -o "$tmp" || err "Download failed. Check the URL: ${url}"

    chmod +x "$tmp"

    echo "Installing to ${INSTALL_DIR}/tamimon..."

    if [ -w "$INSTALL_DIR" ]; then
        mv "$tmp" "${INSTALL_DIR}/tamimon"
    else
        echo "(requires sudo)"
        sudo mv "$tmp" "${INSTALL_DIR}/tamimon"
    fi

    echo ""
    echo "tamimon ${latest_tag} installed successfully!"
    echo "Run 'tamimon' to start."
}

need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1; then
        err "Required command not found: $1"
    fi
}

err() {
    echo "Error: $1" >&2
    exit 1
}

main "$@"
