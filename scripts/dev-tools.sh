#!/bin/bash

# Pulito Development Tools
# Unified script for all development operations: building, testing, launching, and deployment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ICON_DIR="$SCRIPT_DIR/src-tauri/icons"
DESKTOP_DIR="$HOME/.local/share/applications"
ICON_INSTALL_DIR="$HOME/.local/share/icons/hicolor"

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

# Check if we're in the right directory
check_project() {
    if [ ! -f "$SCRIPT_DIR/package.json" ] || [ ! -d "$SCRIPT_DIR/src-tauri" ]; then
        log_error "This script must be run from the Pulito project root"
        exit 1
    fi
}

# Build debug binary
build_debug() {
    log_info "Building debug binary..."
    cd "$SCRIPT_DIR/src-tauri"
    if cargo build 2>&1 | tee /tmp/pulito-build.log; then
        log_success "Build completed successfully"
        return 0
    else
        log_error "Build failed. Check /tmp/pulito-build.log for details"
        return 1
    fi
}

# Build release binary
build_release() {
    log_info "Building release binary..."
    cd "$SCRIPT_DIR/src-tauri"
    if cargo build --release 2>&1 | tee /tmp/pulito-release-build.log; then
        log_success "Release build completed successfully"
        return 0
    else
        log_error "Release build failed. Check /tmp/pulito-release-build.log for details"
        return 1
    fi
}

# Check if dev server is running
check_dev_server() {
    lsof -Pi :5173 -sTCP:LISTEN -t >/dev/null 2>&1
}

# Start dev server
start_dev_server() {
    if check_dev_server; then
        log_info "Dev server is already running on port 5173"
        return 0
    fi

    log_info "Starting Vite dev server..."
    npm run dev > /tmp/pulito-vite.log 2>&1 &
    VITE_PID=$!

    log_info "Waiting for dev server to start..."
    for i in {1..30}; do
        if check_dev_server; then
            log_success "Dev server is ready"
            return 0
        fi
        sleep 1
    done

    log_error "Timeout waiting for dev server after 30 seconds"
    log_info "Check /tmp/pulito-vite.log for errors"
    return 1
}

# Install icons
install_icons() {
    log_info "Installing icons..."
    mkdir -p "$ICON_INSTALL_DIR"

    for size in 32 128 256 512; do
        icon_size_dir="$ICON_INSTALL_DIR/${size}x${size}/apps"
        mkdir -p "$icon_size_dir"

        if [ -f "$ICON_DIR/${size}x${size}.png" ]; then
            cp "$ICON_DIR/${size}x${size}.png" "$icon_size_dir/pulito.png"
            log_info "  Installed ${size}x${size} icon"
        else
            log_warning "  ${size}x${size}.png not found"
        fi
    done

    # Update icon cache
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -f -t "$ICON_INSTALL_DIR" 2>/dev/null || true
    fi
}

# Install desktop entry
install_desktop_entry() {
    local build_type="$1"
    local binary_path=""

    log_info "Installing desktop entry for $build_type build..."

    mkdir -p "$DESKTOP_DIR"
    DESKTOP_FILE="$DESKTOP_DIR/pulito.desktop"

    case "$build_type" in
        "debug")
            if [ -f "$SCRIPT_DIR/src-tauri/target/debug/pulito" ]; then
                binary_path="$SCRIPT_DIR/src-tauri/target/debug/pulito"
            else
                log_error "Debug binary not found"
                return 1
            fi
            ;;
        "release")
            if [ -f "$SCRIPT_DIR/src-tauri/target/release/pulito" ]; then
                binary_path="$SCRIPT_DIR/src-tauri/target/release/pulito"
            else
                log_error "Release binary not found"
                return 1
            fi
            ;;
        "dev-server")
            if [ -f "$SCRIPT_DIR/../scripts/launch-dev.sh" ]; then
                binary_path="$SCRIPT_DIR/../scripts/launch-dev.sh"
            else
                log_error "Dev launcher script not found"
                return 1
            fi
            ;;
        *)
            log_error "Unknown build type: $build_type"
            return 1
            ;;
    esac

    # Make binary executable
    if [ ! -x "$binary_path" ]; then
        chmod +x "$binary_path"
    fi

    cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Name=Pulito
Comment=Smart Linux system cleanup and optimization tool
Exec=$binary_path
Icon=pulito
Terminal=false
Type=Application
Categories=Utility;
StartupWMClass=pulito
EOF

    chmod +x "$DESKTOP_FILE"

    # Update desktop database
    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
    fi

    log_success "Desktop entry installed: $DESKTOP_FILE"
}

# Run the application
run_app() {
    local build_type="$1"

    case "$build_type" in
        "debug")
            binary="$SCRIPT_DIR/src-tauri/target/debug/pulito"
            ;;
        "release")
            binary="$SCRIPT_DIR/src-tauri/target/release/pulito"
            ;;
        "dev")
            binary="$SCRIPT_DIR/../scripts/launch-dev.sh"
            ;;
        *)
            log_error "Unknown build type for run: $build_type"
            return 1
            ;;
    esac

    if [ -x "$binary" ]; then
        log_info "Launching Pulito ($build_type)..."
        exec "$binary"
    else
        log_error "Binary not found or not executable: $binary"
        return 1
    fi
}

# Run tests
run_tests() {
    log_info "Running test suite..."

    # Frontend tests
    log_info "Running frontend tests..."
    if npm test; then
        log_success "Frontend tests passed"
    else
        log_error "Frontend tests failed"
        return 1
    fi

    # Rust tests
    log_info "Running Rust tests..."
    cd "$SCRIPT_DIR/src-tauri"
    if cargo test; then
        log_success "Rust tests passed"
    else
        log_error "Rust tests failed"
        return 1
    fi

    log_success "All tests passed!"
}

# Show usage
show_usage() {
    echo "Pulito Development Tools"
    echo "Usage: $0 <command> [options]"
    echo ""
    echo "Commands:"
    echo "  build [debug|release]    Build the application (default: debug)"
    echo "  dev                      Start development mode (build + dev server + launch)"
    echo "  install [debug|release]  Install desktop launcher (default: debug)"
    echo "  run [debug|release|dev]  Run the application (default: debug)"
    echo "  test                     Run the complete test suite"
    echo "  setup                    Full development setup (build + install + dev server)"
    echo "  clean                    Clean build artifacts"
    echo "  help                     Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 build                 # Build debug version"
    echo "  $0 dev                   # Start full development environment"
    echo "  $0 install               # Install desktop launcher"
    echo "  $0 test                  # Run all tests"
    echo "  $0 setup                 # Complete development setup"
}

# Clean build artifacts
clean_artifacts() {
    log_info "Cleaning build artifacts..."

    # Clean Rust artifacts
    cd "$SCRIPT_DIR/src-tauri"
    cargo clean

    # Clean Node artifacts
    cd "$SCRIPT_DIR"
    rm -rf node_modules/.vite
    rm -rf build
    rm -f src-tauri/target/debug/pulito
    rm -f src-tauri/target/release/pulito

    log_success "Build artifacts cleaned"
}

# Main command handling
main() {
    check_project

    case "${1:-help}" in
        "build")
            case "${2:-debug}" in
                "debug") build_debug ;;
                "release") build_release ;;
                *) log_error "Invalid build type. Use 'debug' or 'release'"; exit 1 ;;
            esac
            ;;
        "dev")
            log_info "Starting development environment..."
            if build_debug && start_dev_server; then
                install_icons
                install_desktop_entry "dev-server"
                log_success "Development environment ready!"
                log_info "You can now:"
                log_info "  • Launch 'Pulito' from your application menu"
                log_info "  • Run '$0 run dev' to start manually"
            fi
            ;;
        "install")
            install_icons
            case "${2:-debug}" in
                "debug"|"release")
                    install_desktop_entry "$2"
                    ;;
                *) log_error "Invalid install type. Use 'debug' or 'release'"; exit 1 ;;
            esac
            ;;
        "run")
            case "${2:-debug}" in
                "debug"|"release"|"dev")
                    run_app "$2"
                    ;;
                *) log_error "Invalid run type. Use 'debug', 'release', or 'dev'"; exit 1 ;;
            esac
            ;;
        "test")
            run_tests
            ;;
        "setup")
            log_info "Setting up complete development environment..."
            if build_debug && start_dev_server; then
                install_icons
                install_desktop_entry "debug"
                run_tests
                log_success "Development environment fully set up!"
            fi
            ;;
        "clean")
            clean_artifacts
            ;;
        "help"|"-h"|"--help")
            show_usage
            ;;
        *)
            log_error "Unknown command: $1"
            echo ""
            show_usage
            exit 1
            ;;
    esac
}

main "$@"
