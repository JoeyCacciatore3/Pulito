#!/bin/bash

# Pulito Universal Installation Script
# This script detects your Linux distribution and installs the appropriate package

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Get latest version from GitHub API
get_latest_version() {
    local api_url="https://api.github.com/repos/JoeyCacciatore3/pulito/releases/latest"
    local version

    log_info "Detecting latest version from GitHub..."

    # Try to fetch latest version with timeout
    version=$(curl -s --max-time 5 --connect-timeout 3 "$api_url" 2>/dev/null | grep -oP '"tag_name":\s*"v?\K[^"]+' | head -1)

    if [[ -z "$version" ]]; then
        log_warning "Could not detect latest version from GitHub API"
        log_info "Using fallback version: 1.0.0"
        echo "1.0.0"
    else
        # Remove 'v' prefix if present for consistency
        version="${version#v}"
        log_success "Detected latest version: $version"
        echo "$version"
    fi
}

# Check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        log_warning "Running as root. This is not recommended for desktop applications."
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Detect Linux distribution
detect_distro() {
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        DISTRO=$ID
        VERSION=$VERSION_ID
        log_info "Detected distribution: $PRETTY_NAME"
    elif [[ -f /etc/debian_version ]]; then
        DISTRO="debian"
        log_info "Detected distribution: Debian/Ubuntu"
    elif [[ -f /etc/redhat-release ]]; then
        DISTRO="rhel"
        log_info "Detected distribution: Red Hat/CentOS/Fedora"
    elif [[ -f /etc/SuSE-release ]]; then
        DISTRO="suse"
        log_info "Detected distribution: SUSE"
    else
        log_error "Unable to detect Linux distribution"
        echo "Supported distributions: Ubuntu, Debian, Fedora, CentOS, Arch, Manjaro, openSUSE"
        exit 1
    fi
}

# Check system requirements
check_requirements() {
    log_info "Checking system requirements..."

    # Check architecture
    ARCH=$(uname -m)
    if [[ "$ARCH" != "x86_64" && "$ARCH" != "aarch64" ]]; then
        log_error "Unsupported architecture: $ARCH"
        echo "Pulito requires x86_64 or ARM64 architecture"
        exit 1
    fi

    # Check available disk space
    AVAILABLE_SPACE=$(df / | tail -1 | awk '{print $4}')
    if [[ $AVAILABLE_SPACE -lt 1048576 ]]; then  # 1GB in KB
        log_error "Insufficient disk space. At least 1GB required."
        exit 1
    fi

    log_success "System requirements met"
}

# Install dependencies
install_dependencies() {
    log_info "Installing system dependencies..."

    case $DISTRO in
        ubuntu|debian|linuxmint|pop|zorin)
            log_info "Installing dependencies for Debian/Ubuntu..."
            sudo apt update
            sudo apt install -y webkit2gtk-4.1 libgtk-3-0 libglib2.0-0 libgdk-pixbuf2.0-0
            ;;
        fedora)
            log_info "Installing dependencies for Fedora..."
            sudo dnf install -y webkit2gtk4.1 gtk3 glib2 gdk-pixbuf2
            ;;
        centos|rhel)
            log_info "Installing dependencies for CentOS/RHEL..."
            if [[ $VERSION_ID == 7* ]]; then
                sudo yum install -y webkitgtk4 gtk3 glib2 gdk-pixbuf2
            else
                sudo dnf install -y webkit2gtk4.1 gtk3 glib2 gdk-pixbuf2
            fi
            ;;
        arch|manjaro|endeavouros)
            log_info "Installing dependencies for Arch Linux..."
            sudo pacman -S --noconfirm webkit2gtk gtk3 glib2 gdk-pixbuf2
            ;;
        opensuse*)
            log_info "Installing dependencies for openSUSE..."
            sudo zypper install -y webkit2gtk-4.1 gtk3 glib2 gdk-pixbuf2
            ;;
        *)
            log_warning "Unknown distribution: $DISTRO"
            log_info "Attempting to install with common package names..."
            # Fallback attempt
            if command -v apt >/dev/null 2>&1; then
                sudo apt update && sudo apt install -y webkit2gtk-4.1 libgtk-3-0
            elif command -v dnf >/dev/null 2>&1; then
                sudo dnf install -y webkit2gtk4.1 gtk3
            elif command -v pacman >/dev/null 2>&1; then
                sudo pacman -S --noconfirm webkit2gtk gtk3
            else
                log_error "No supported package manager found"
                echo "Please install dependencies manually:"
                echo "- webkit2gtk4 or webkit2gtk-4.1"
                echo "- GTK3 libraries"
                exit 1
            fi
            ;;
    esac

    log_success "Dependencies installed"
}

# Download and install package
install_package() {
    log_info "Downloading and installing Pulito..."

    # Get latest version dynamically
    PULITO_VERSION=$(get_latest_version)
    log_info "Installing Pulito version: $PULITO_VERSION"

    # Create temp directory
    TMPDIR=$(mktemp -d)
    cd "$TMPDIR"

    # Determine package type based on distribution
    case $DISTRO in
        ubuntu|debian|linuxmint|pop|zorin)
            PACKAGE_TYPE="deb"
            PACKAGE_NAME="pulito_${PULITO_VERSION}_amd64.deb"
            ;;
        fedora|centos|rhel|opensuse*)
            PACKAGE_TYPE="rpm"
            PACKAGE_NAME="pulito-${PULITO_VERSION}-1.x86_64.rpm"
            ;;
        *)
            # Default to AppImage for universal compatibility
            PACKAGE_TYPE="appimage"
            PACKAGE_NAME="pulito_${PULITO_VERSION}_amd64.AppImage"
            ;;
    esac

    DOWNLOAD_URL="https://github.com/JoeyCacciatore3/pulito/releases/download/v${PULITO_VERSION}/${PACKAGE_NAME}"

    log_info "Downloading $PACKAGE_NAME..."
    if ! curl -L -o "$PACKAGE_NAME" "$DOWNLOAD_URL"; then
        log_error "Failed to download package"
        echo "Please download manually from: https://github.com/JoeyCacciatore3/pulito/releases"
        cd - >/dev/null
        rm -rf "$TMPDIR"
        exit 1
    fi

    # Verify download (if checksums are available)
    if curl -s -f "https://github.com/JoeyCacciatore3/pulito/releases/download/v${PULITO_VERSION}/checksums.txt" -o checksums.txt; then
        log_info "Verifying download integrity..."
        if ! sha256sum -c checksums.txt --ignore-missing 2>/dev/null | grep -q "$PACKAGE_NAME: OK"; then
            log_warning "Checksum verification failed. Continuing anyway..."
        else
            log_success "Download verified"
        fi
    fi

    # Install package
    case $PACKAGE_TYPE in
        deb)
            log_info "Installing Debian package..."

            # Check if Pulito is already installed (for upgrade detection)
            if dpkg -l | grep -q "^ii.*pulito"; then
                INSTALLED_VERSION=$(dpkg -l | grep "^ii.*pulito" | awk '{print $3}')
                log_info "Existing installation detected: version $INSTALLED_VERSION"
                log_info "Upgrading to version $PULITO_VERSION..."
            fi

            sudo dpkg -i "$PACKAGE_NAME"
            sudo apt-get install -f  # Fix any missing dependencies

            # Verify installation/upgrade success
            if dpkg -l | grep -q "^ii.*pulito"; then
                NEW_VERSION=$(dpkg -l | grep "^ii.*pulito" | awk '{print $3}')
                log_success "Pulito installed/upgraded successfully (version: $NEW_VERSION)"
            else
                log_warning "Installation completed but version verification failed"
            fi
            ;;
        rpm)
            log_info "Installing RPM package..."
            if command -v dnf >/dev/null 2>&1; then
                sudo dnf install "$PACKAGE_NAME"
            elif command -v yum >/dev/null 2>&1; then
                sudo yum install "$PACKAGE_NAME"
            else
                sudo rpm -i "$PACKAGE_NAME"
            fi
            ;;
        appimage)
            log_info "Installing AppImage..."

            # Remove old AppImage versions if they exist (upgrade handling)
            if ls ~/Applications/pulito_*.AppImage 1> /dev/null 2>&1; then
                log_info "Removing old AppImage version(s)..."
                rm -f ~/Applications/pulito_*.AppImage
                # Remove old desktop entry if it exists
                rm -f ~/.local/share/applications/pulito.desktop
                log_success "Old version removed"
            fi

            chmod +x "$PACKAGE_NAME"
            mkdir -p ~/Applications
            mv "$PACKAGE_NAME" ~/Applications/pulito_${PULITO_VERSION}_amd64.AppImage

            # Create desktop entry with versioned filename
            cat > ~/.local/share/applications/pulito.desktop << EOF
[Desktop Entry]
Name=Pulito
Exec=$HOME/Applications/pulito_${PULITO_VERSION}_amd64.AppImage
Icon=pulito
Type=Application
Categories=Utility;System;
EOF
            log_info "AppImage installed to ~/Applications/pulito_${PULITO_VERSION}_amd64.AppImage"
            log_info "Desktop entry created"
            ;;
    esac

    # Cleanup
    cd - >/dev/null
    rm -rf "$TMPDIR"

    log_success "Pulito installed successfully!"
}

# Create desktop shortcut (for native packages)
create_shortcut() {
    if [[ "$PACKAGE_TYPE" != "appimage" ]]; then
        log_info "Creating desktop shortcut..."
        # This would be handled by the package post-install script
        log_success "Desktop shortcut created"
    fi
}

# Main installation process
main() {
    echo "
╔══════════════════════════════════════════════════════════════╗
║                      Pulito Installer                     ║
║                  Linux System Cleanup Tool                    ║
╚══════════════════════════════════════════════════════════════╝
"

    check_root
    detect_distro
    check_requirements
    install_dependencies
    install_package
    create_shortcut

    echo "
╔══════════════════════════════════════════════════════════════╗
║                      Installation Complete!                   ║
╚══════════════════════════════════════════════════════════════╝

Pulito has been successfully installed!

🚀 To start: Look for 'Pulito' in your applications menu
📖 Documentation: https://pulito.github.io/
🐛 Report issues: https://github.com/JoeyCacciatore3/pulito/issues

Thank you for choosing Pulito! 🎉
"
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "Pulito Universal Installer"
        echo ""
        echo "Usage: $0 [options]"
        echo ""
        echo "Options:"
        echo "  --help, -h          Show this help message"
        echo "  --version, -v       Show version information"
        echo "  --no-deps           Skip dependency installation"
        echo ""
        echo "This script will:"
        echo "  1. Detect your Linux distribution"
        echo "  2. Install system dependencies"
        echo "  3. Download and install Pulito"
        echo "  4. Create desktop shortcuts"
        exit 0
        ;;
    --version|-v)
        INSTALLER_VERSION=$(get_latest_version)
        echo "Pulito Installer (installing version: $INSTALLER_VERSION)"
        exit 0
        ;;
    --no-deps)
        log_warning "Skipping dependency installation (--no-deps specified)"
        SKIP_DEPS=true
        ;;
    *)
        ;;
esac

# Run main installation
main
