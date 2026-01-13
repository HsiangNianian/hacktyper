#!/usr/bin/env bash
# One-click install script for hacktyper
# Supports Linux (x86_64, aarch64, armv7) and macOS (x86_64, aarch64)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="HsiangNianian/hacktyper"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
TEMP_DIR=$(mktemp -d)

# Cleanup on exit
trap 'rm -rf "$TEMP_DIR"' EXIT

# Helper functions
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)
            OS="linux"
            ;;
        Darwin*)
            OS="macos"
            ;;
        *)
            error "Unsupported operating system: $(uname -s)"
            ;;
    esac
    info "Detected OS: $OS"
}

# Detect architecture
detect_arch() {
    ARCH=$(uname -m)
    case "$ARCH" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        armv7l|armhf)
            ARCH="armv7hl"
            ;;
        *)
            error "Unsupported architecture: $ARCH"
            ;;
    esac
    info "Detected architecture: $ARCH"
}

# Detect Linux distribution for package format preference
detect_distro() {
    if [ "$OS" != "linux" ]; then
        return
    fi

    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO_ID="$ID"
        DISTRO_ID_LIKE="$ID_LIKE"
    elif [ -f /etc/redhat-release ]; then
        DISTRO_ID="rhel"
    elif [ -f /etc/debian_version ]; then
        DISTRO_ID="debian"
    else
        DISTRO_ID="unknown"
    fi

    # Determine package format preference
    case "$DISTRO_ID" in
        ubuntu|debian|linuxmint|pop|elementary)
            PREFERRED_FORMAT="deb"
            ;;
        fedora|rhel|centos|rocky|alma|opensuse*)
            PREFERRED_FORMAT="rpm"
            ;;
        arch|manjaro)
            PREFERRED_FORMAT="tar.gz"
            ;;
        *)
            # Check ID_LIKE for better detection
            case "$DISTRO_ID_LIKE" in
                *debian*)
                    PREFERRED_FORMAT="deb"
                    ;;
                *rhel*|*fedora*)
                    PREFERRED_FORMAT="rpm"
                    ;;
                *)
                    PREFERRED_FORMAT="tar.gz"
                    ;;
            esac
            ;;
    esac

    info "Detected Linux distribution: $DISTRO_ID (preferred format: $PREFERRED_FORMAT)"
}

# Get latest release version
get_latest_version() {
    info "Fetching latest release version..."
    
    # Set up curl headers for GitHub API
    local curl_args=(-sSf)
    curl_args+=(-H "Accept: application/vnd.github+json")
    
    # Add User-Agent header to avoid rate limiting
    curl_args+=(-H "User-Agent: hacktyper-installer")
    
    # Use GitHub token if available (for CI environments)
    if [ -n "${GITHUB_TOKEN:-}" ]; then
        curl_args+=(-H "Authorization: Bearer $GITHUB_TOKEN")
    fi
    
    # Try jq first for more reliable JSON parsing, fall back to grep/sed
    if command -v jq &> /dev/null; then
        LATEST_VERSION=$(curl "${curl_args[@]}" "https://api.github.com/repos/$REPO/releases/latest" | jq -r .tag_name)
    else
        LATEST_VERSION=$(curl "${curl_args[@]}" "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    fi
    
    if [ -z "$LATEST_VERSION" ]; then
        error "Failed to fetch latest version"
    fi
    
    info "Latest version: $LATEST_VERSION"
}

# Construct download URL and filename
construct_download_info() {
    local base_name="hacktyper-${OS}-${ARCH}"
    
    if [ "$OS" = "linux" ]; then
        # Determine which format to use
        case "$PREFERRED_FORMAT" in
            deb)
                # Map architecture names for .deb packages
                case "$ARCH" in
                    x86_64)
                        DOWNLOAD_FILE="${base_name/x86_64/amd64}.deb"
                        ;;
                    aarch64)
                        DOWNLOAD_FILE="${base_name/aarch64/arm64}.deb"
                        ;;
                    armv7hl)
                        DOWNLOAD_FILE="${base_name/armv7hl/armhf}.deb"
                        ;;
                esac
                ;;
            rpm)
                DOWNLOAD_FILE="${base_name}.rpm"
                ;;
            *)
                DOWNLOAD_FILE="${base_name}.tar.gz"
                ;;
        esac
    elif [ "$OS" = "macos" ]; then
        DOWNLOAD_FILE="${base_name}.tar.gz"
    fi
    
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_VERSION/$DOWNLOAD_FILE"
    info "Download URL: $DOWNLOAD_URL"
}

# Download the release
download_release() {
    info "Downloading $DOWNLOAD_FILE..."
    
    if ! curl -sSfL "$DOWNLOAD_URL" -o "$TEMP_DIR/$DOWNLOAD_FILE"; then
        error "Failed to download release from $DOWNLOAD_URL"
    fi
    
    info "Download complete"
}

# Check if user has sudo/root access
check_sudo() {
    if [ "$EUID" -ne 0 ]; then
        if ! command -v sudo &> /dev/null; then
            error "This script requires sudo privileges, but sudo is not available"
        fi
        SUDO="sudo"
        info "sudo will be used for installation"
    else
        SUDO=""
    fi
}

# Install from .deb package
install_deb() {
    info "Installing .deb package..."
    
    if command -v apt &> /dev/null; then
        # Try apt install first, if it fails, use dpkg with dependency fix
        if ! $SUDO apt install -y "$TEMP_DIR/$DOWNLOAD_FILE"; then
            $SUDO dpkg -i "$TEMP_DIR/$DOWNLOAD_FILE"
            $SUDO apt-get install -f -y
        fi
    else
        $SUDO dpkg -i "$TEMP_DIR/$DOWNLOAD_FILE"
    fi
}

# Install from .rpm package
install_rpm() {
    info "Installing .rpm package..."
    
    if command -v dnf &> /dev/null; then
        $SUDO dnf install -y "$TEMP_DIR/$DOWNLOAD_FILE"
    elif command -v yum &> /dev/null; then
        $SUDO yum install -y "$TEMP_DIR/$DOWNLOAD_FILE"
    else
        $SUDO rpm -ivh "$TEMP_DIR/$DOWNLOAD_FILE"
    fi
}

# Install from .tar.gz archive
install_tar_gz() {
    info "Installing from .tar.gz archive..."
    
    # Extract the archive
    tar -xzf "$TEMP_DIR/$DOWNLOAD_FILE" -C "$TEMP_DIR"
    
    # Find the binary
    BINARY=$(find "$TEMP_DIR" -name "hacktyper" -type f ! -path "*/.*" | head -n 1)
    
    if [ -z "$BINARY" ]; then
        error "Could not find hacktyper binary in the archive"
    fi
    
    # Make it executable
    chmod +x "$BINARY"
    
    # Install to the target directory
    if [ ! -d "$INSTALL_DIR" ]; then
        $SUDO mkdir -p "$INSTALL_DIR"
    fi
    
    $SUDO cp "$BINARY" "$INSTALL_DIR/hacktyper"
    info "Installed hacktyper to $INSTALL_DIR/hacktyper"
}

# Perform installation
install() {
    check_sudo
    
    case "$DOWNLOAD_FILE" in
        *.deb)
            install_deb
            ;;
        *.rpm)
            install_rpm
            ;;
        *.tar.gz)
            install_tar_gz
            ;;
        *)
            error "Unsupported file format: $DOWNLOAD_FILE"
            ;;
    esac
}

# Verify installation
verify_installation() {
    info "Verifying installation..."
    
    if command -v hacktyper &> /dev/null; then
        VERSION=$(hacktyper --version 2>&1 || echo "unknown")
        info "✓ hacktyper installed successfully!"
        info "Version: $VERSION"
        echo ""
        echo "You can now run: hacktyper"
    else
        warn "hacktyper was installed but is not in PATH"
        warn "You may need to add $INSTALL_DIR to your PATH"
        warn "Or run it directly: $INSTALL_DIR/hacktyper"
    fi
}

# Main execution
main() {
    echo "╔════════════════════════════════════════╗"
    echo "║  Hacktyper One-Click Installer        ║"
    echo "╚════════════════════════════════════════╝"
    echo ""
    
    detect_os
    detect_arch
    detect_distro
    get_latest_version
    construct_download_info
    download_release
    install
    verify_installation
    
    echo ""
    info "Installation complete! 🎉"
}

main "$@"
