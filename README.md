# Pulito - Advanced Linux System Monitoring & Cleanup

A production-ready, enterprise-grade Linux system monitoring and cleanup tool built with modern technologies following December 2025 best practices. Features real-time hardware monitoring comparable to leading system optimization apps.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Tauri](https://img.shields.io/badge/Tauri-2.2+-orange.svg)
![Svelte](https://img.shields.io/badge/Svelte-5.46-red.svg)
![Rust](https://img.shields.io/badge/Rust-1.80+-brown.svg)
![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)
![Security](https://img.shields.io/badge/security-hardened-blue.svg)

## ✨ Key Features

### 🔬 **Advanced Real-Time System Monitoring (2025 Standards)**
- **Comprehensive Hardware Monitoring**: CPU, GPU, memory, disk I/O, network, and temperature tracking
- **Enterprise-Grade GPU Monitoring**: NVIDIA NVML integration with real-time utilization and VRAM tracking
- **Battery Intelligence**: Power consumption and time-to-charge/discharge calculations (laptops)
- **Process-Level Monitoring**: Top resource consumers with detailed CPU/memory usage
- **Network Connection Tracking**: Active TCP/UDP connections with process association
- **Disk I/O Performance**: Real-time read/write speeds and IOPS monitoring
- **Load Average Tracking**: System load monitoring (1m, 5m, 15m averages)
- **Smart Monitoring Intervals**: Adaptive polling based on system activity and battery status

### 🛡️ **Security & Safety**
- **Path Traversal Protection**: Comprehensive input validation prevents directory traversal attacks
- **System File Protection**: Blocks deletion of critical system directories (`/bin`, `/etc`, `/usr`, etc.)
- **User Home Restriction**: Only allows operations within user's home directory
- **6-Tier Risk Assessment**: Intelligent risk evaluation from Safe to Critical
- **Mandatory Confirmations**: All cleanup operations require explicit user confirmation
- **Recoverable Deletions**: Trash system ensures all deletions can be undone

### 🔍 **Smart Analysis & Cleanup**
- **Multi-Format Scanning**: Detects caches, orphaned packages, large files, and system logs
- **Dependency Resolution**: Advanced package dependency analysis prevents accidental breakage
- **Real-time Size Calculation**: Accurate disk space analysis with live updates
- **Cross-Platform Package Support**: apt/dpkg, pip, npm, and system caches
- **Filesystem Health Checks**: Empty directories, broken symlinks, orphaned temp files

### 🗂️ **Advanced Trash Management**
- **Recoverable Deletions**: Custom trash system with configurable retention periods
- **Metadata Tracking**: Complete file history and restoration capabilities
- **Automatic Cleanup**: Configurable expired file removal
- **Size Management**: Trash size limits with intelligent cleanup

### 🎯 **User Experience**
- **Modern UI**: Svelte 5 with Tailwind CSS for responsive, accessible interface
- **System Tray Integration**: Background monitoring with tray status indicators
- **Comprehensive Settings**: Granular control over all application behavior
- **Real-time Feedback**: Live progress updates and detailed operation logs
- **Confirmation Dialogs**: Glassmorphism confirmation dialogs for all cleanup operations
- **Category-Based Organization**: Intuitive cleanup categories for better user experience

## 🌐 Repository Structure

This repository contains **two completely separate parts**:

### 1. Marketing Website
- **Location:** `marketing/` folder (standalone SvelteKit app)
- **Purpose:** Public-facing marketing site
- **Pages:** Landing (`/`), Features, Download, FAQ, Privacy, Terms
- **Development:** `npm run marketing:dev` or `cd marketing && npm run dev`
- **Build:** `npm run marketing:build` → `marketing/build/`
- **Deployment:** Static site (GitHub Pages, Netlify, etc.)
- **Dependencies:** Svelte, SvelteKit, Tailwind (no Tauri)

### 2. Desktop Application
- **Location:** `src/` folder (Tauri app)
- **Route:** `/app`
- **Purpose:** Full-featured desktop application
- **Development:** `npm run tauri:dev` → Desktop window
- **Build:** `npm run tauri:build` → Desktop packages
- **Deployment:** Desktop packages (`.deb`, `.AppImage`)
- **Dependencies:** Svelte, Tauri, Rust backend

**📖 For detailed structure documentation, see [REPOSITORY_STRUCTURE.md](./REPOSITORY_STRUCTURE.md)**

**Important:**
- Marketing site: `npm run marketing:dev` (or `cd marketing && npm run dev`)
- Desktop app: `npm run tauri:dev` (requires Tauri runtime)

## 🧹 **Advanced Cleanup Operations**

Pulito provides comprehensive system cleanup with multiple safety layers, real-time monitoring, and intelligent optimization.

### **Available Cleanup Operations**

#### **Smart Cache Management**
- **Multi-Source Cache Cleaning**: Application caches, browser caches, development tool caches
- **Real-Time Cache Monitoring**: Live tracking of cache growth and cleanup effectiveness
- **Intelligent Cache Limits**: Automatic recommendations for optimal cache sizes

#### **Package Management**
- **Orphan Package Removal**: Identifies and removes unused packages with dependency analysis
- **Package Cache Cleaning**: Clears downloaded .deb files and metadata
- **Package Health Monitoring**: Real-time tracking of package installation status

#### **Storage Optimization**
- **Filesystem Health Scans**: Empty directories, broken symlinks, orphaned temp files
- **Duplicate File Detection**: Advanced duplicate finding with smart keep algorithms
- **Large File Analysis**: Identifies space-consuming files with usage patterns
- **Storage Recovery Suite**: Comprehensive cleanup of temporary and unnecessary files

#### **System Maintenance**
- **Log File Management**: Intelligent cleanup of old system and user logs
- **Temporary File Cleanup**: Safe removal of orphaned temporary files
- **System Junk Removal**: Comprehensive cleanup of system-generated junk files

### **Safety Features**

#### **Confirmation Dialogs**
All cleanup operations require explicit user confirmation via glassmorphism confirmation dialogs that include:
- Operation details and potential impact
- Item count and estimated space savings
- Risk level assessment (Safe, Caution, Warning)
- Clear action buttons ("Clean Selected" vs "Cancel")

#### **Risk Assessment**
- **6-Tier Risk Levels**: From Safe (cache files) to Critical (system files)
- **Path Validation**: Prevents deletion of system-critical directories
- **Trash System**: All deletions are recoverable with configurable retention periods

#### **Smart Organization**
Category-based cleanup suggestions:
- Filesystem Health: Safe items like empty directories and broken symlinks
- Storage Recovery: Duplicates, large files, and old downloads
- Cache Optimization: Application and system cache management

## Requirements

### System Requirements

- **Ubuntu 22.04+** or compatible Linux distribution
- **WebKitGTK 4.1** (required for Tauri 2.x)
- **Rust 1.80+** (for building from source)
- **Node.js 20+** (for frontend development)

### Install System Dependencies (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf
```

## Installation

### From Release (Recommended)

Download the latest `.deb` or `.AppImage` from the [Releases](https://github.com/JoeyCacciatore3/pulito/releases) page.

```bash
# For .deb
sudo dpkg -i pulito_1.1.0_amd64.deb

# For AppImage
chmod +x pulito_1.1.0_amd64.AppImage
./pulito_1.1.0_amd64.AppImage
```

### Quick Install Script

```bash
curl -fsSL https://raw.githubusercontent.com/JoeyCacciatore3/pulito/main/static/install.sh | bash
```

### From Source

```bash
# Clone the repository
git clone https://github.com/JoeyCacciatore3/pulito.git
cd pulito

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js dependencies
npm install

# Run tests to verify setup
npm test

# Run in development mode (REQUIRED - launches desktop app with Tauri backend)
npm run tauri:dev

# Build for production
npm run tauri:build
```

**Important**: This is a desktop application that requires Tauri runtime. Always use `npm run tauri:dev` for development, NOT `npm run dev` (which runs in browser mode without Tauri API access).

## Development

### Building the Marketing Site

```bash
# Build static site for web deployment from the marketing directory
npm run marketing:build

# Preview the built site locally
npm run marketing:preview

# The built site will be in the `marketing/build/` directory
```

### Web Deployment

The marketing site can be deployed to GitHub Pages or any static hosting service.

#### Automatic Deployment (GitHub Pages)

The repository includes GitHub Actions workflows that automatically deploy the marketing site to GitHub Pages:

- **Workflow files**: `.github/workflows/pages.yml` and `.github/workflows/deploy-pages.yml`
- **Trigger**: Automatically runs on pushes to `main` branch
- **Build location**: `marketing/build/` directory
- **Deployment**: Automatically deploys to GitHub Pages

To enable GitHub Pages:
1. Go to repository Settings → Pages
2. Set source to "GitHub Actions"
3. The workflow will automatically deploy on push to `main`

#### Manual Deployment

To deploy manually:

```bash
# Build the marketing site
npm run marketing:build

# The build output is in the `marketing/build/` directory
# Deploy this to your static hosting service (GitHub Pages, Netlify, Vercel, etc.)
```

For GitHub Pages manual deployment:
1. Build the site: `npm run marketing:build`
2. Copy contents of `marketing/build/` to `gh-pages` branch root
3. Push to `gh-pages` branch

## 🏗️ **Architecture & Design**

For complete architecture documentation, see [docs/architecture.md](docs/architecture.md).

### Project Structure

See [docs/architecture.md](docs/architecture.md) for detailed architecture documentation.

```
pulito/
├── src/                          # Svelte 5 frontend for the Desktop Application
│   ├── lib/
│   │   ├── components/           # Reusable UI components
│   │   ├── stores/               # Svelte 5 rune-based state management
│   │   └── utils/                # Type-safe utilities and Tauri IPC
│   ├── routes/                   # SvelteKit file-based routing
│   │   └── app/                  # Desktop app interface (main application route)
│   └── test/                     # Frontend test setup and utilities
├── src-tauri/                    # Rust backend with Tauri 2.x
│   ├── src/
│   │   ├── commands/             # IPC command handlers with timeout protection
│   │   ├── db/                   # SQLite database management
│   │   ├── scanner/              # Advanced file system scanning logic
│   │   ├── packages/             # Package manager integration
│   │   ├── trash/                # Recoverable deletion system
│   │   └── main.rs               # Application entry point with tray integration
│   └── icons/                    # Application icons
├── static/                       # Static assets for web
│   ├── install.sh                # Universal installer script
│   ├── screenshots/              # Marketing screenshots
│   └── icons/                    # Web icons
├── snap/                         # Snap package configuration
├── flatpak/                      # Flatpak package configuration
└── .github/workflows/            # CI/CD pipelines
```

## 🚀 **Current Implementation Status**

### ✅ **Implemented Features (2025 Standards)**
- **Advanced System Health Monitor**: Real-time CPU, GPU, memory, network, and temperature monitoring
- **GPU Monitoring Suite**: NVIDIA NVML integration with utilization and VRAM tracking
- **Process Monitor**: Top resource consumers with detailed CPU/memory usage
- **Network Connection Tracker**: Active TCP/UDP connections with process association
- **Battery Intelligence**: Power consumption and charging status (laptops)
- **Disk I/O Performance**: Real-time read/write speeds and IOPS monitoring
- **Load Average Tracking**: System load monitoring with trend analysis
- **Filesystem Health Check**: Empty directories, broken symlinks, orphaned temp files
- **Storage Recovery Suite**: Duplicate detection, large files, old downloads
- **Cache Optimization Suite**: Multi-source cache analytics and cleanup
- **Category-Based UI**: Organized cleanup cards on dashboard
- **Smart Preview Pane**: Visual file previews in TreeView
- **Timeout Protection**: All operations protected against hanging
- **System Tray Integration**: Background monitoring with status indicators

### 🔄 **Future Enhancements**
- **Predictive Analytics**: AI-driven system health predictions and recommendations
- **Performance Profiling**: Advanced bottleneck identification and optimization suggestions
- **Automated Maintenance**: Scheduled cleanup operations with smart scheduling
- **Cloud Integration**: Sync settings and data across devices
- **Advanced Reporting**: Detailed system health reports and historical trends

## 🧪 **Testing & Quality Assurance**

### Test Coverage

- **Frontend Type Checking**: Svelte 5 and TypeScript validation
- **Rust Compilation**: Backend code compilation and dependency checks
- **Integration Tests**: IPC communication and data flow
- **Build System**: Complete application builds for all platforms

### Running Tests

```bash
# Frontend tests
npm test                    # Run all frontend tests
npm run test:ui            # Run tests with UI
npm run test:coverage      # Generate coverage report

# Rust tests
cd src-tauri && cargo test  # Run Rust unit tests

# Linting and type checking
npm run lint               # ESLint check
npm run lint:fix           # Auto-fix ESLint issues
npm run check              # TypeScript + Svelte checking

# Full quality check
npm run check && npm test && cargo test
```

### Development Commands

```bash
# Development
# IMPORTANT: Use tauri:dev to run the desktop app with full functionality
npm run tauri:dev          # Start Tauri in dev mode (full desktop app - REQUIRED)
npm run dev                # Start Vite dev server (browser mode only - Tauri API unavailable)

# Building
npm run build              # Build frontend for production
npm run tauri build        # Build complete desktop app

# Code Quality
npm run format             # Format code with Prettier
npm run check              # TypeScript/Svelte type checking
npm run lint               # ESLint code quality
cargo clippy               # Rust linting
cargo fmt                  # Rust formatting
```

### Desktop Launcher for Development Testing

To install a desktop launcher for easy testing during development:

```bash
# Install desktop launcher (uses debug build by default)
./install-desktop-launcher.sh

# Force use of debug build
./install-desktop-launcher.sh --dev

# Force use of release build
./install-desktop-launcher.sh --release

# Quick rebuild and reinstall (development helper)
./dev-launcher.sh
```

**Development Workflow:**
1. Make code changes
2. Rebuild: `npm run tauri:dev` or `./dev-launcher.sh`
3. Reinstall launcher: `./install-desktop-launcher.sh --dev`
4. Test from application menu

The desktop launcher will appear in your application menu with the Pulito logo. After rebuilding, simply reinstall the launcher to update it with the latest binary.

## 🔒 **Security & Error Handling**

### Security Hardening

- **Input Validation**: All file paths validated against directory traversal attacks
- **Path Sanitization**: Absolute path requirements with canonical resolution
- **System Protection**: Critical directories are protected from deletion
- **User Isolation**: Operations restricted to user's home directory
- **Risk Assessment**: Multi-tier risk levels for safe decision making

### Error Handling

- **Comprehensive Logging**: Structured logging with `tracing` and `tracing-subscriber`
- **Error Propagation**: Proper error types with `anyhow` and `thiserror`
- **User Feedback**: Clear error messages and recovery suggestions
- **Graceful Degradation**: Fallback behaviors when operations fail

### **Confirmation Dialog Patterns**

All cleanup operations follow consistent confirmation dialog patterns based on Nielsen Norman Group guidelines:

```typescript
// Basic confirmation pattern
const confirmed = await confirmation.show({
    title: 'Operation Title',
    message: 'Detailed description with impact information',
    confirmText: 'Action Button', // Specific to the operation
    cancelText: 'Cancel',
    type: 'info' | 'warning' | 'danger'  // Based on risk level
});

if (!confirmed) return; // Always check result
```

#### **Dialog Types**
- **Info**: Safe operations (cache cleaning, log cleanup)
- **Warning**: Moderate risk operations (package removal, bulk file cleanup)
- **Danger**: High-risk operations (permanent deletions, system modifications)

#### **Message Guidelines**
- Include item count and estimated space savings
- Explain operation impact and risk level
- Mention recoverability (trash system)
- Use user-centric language

## 📦 **Distribution Formats**

Pulito is available in multiple formats:

- **.deb**: Native Debian/Ubuntu package (recommended for Debian-based distros)
- **AppImage**: Universal Linux package (works on all distributions)
- **Snap**: Coming soon to Snap Store
- **Flatpak**: Coming soon to Flathub

## 🚀 **Performance Optimizations**

### Frontend Optimizations
- **Svelte 5 Runes**: Zero-overhead reactivity system
- **Tree Shaking**: Unused code elimination
- **Code Splitting**: Lazy-loaded components
- **Static Site Generation**: Prerendered marketing pages for fast loading

### Backend Optimizations
- **Async/Await**: Non-blocking I/O operations
- **Connection Pooling**: Efficient database connections
- **Memory Mapping**: Large file handling without full loading
- **Parallel Processing**: Concurrent file operations where safe

### Database Optimizations
- **Indexes**: Optimized queries for scan history
- **Transactions**: Atomic operations with rollback
- **Prepared Statements**: SQL injection prevention and performance
- **WAL Mode**: Concurrent read/write operations

## Known Issues

1. **WebKitGTK Version**: Requires webkit2gtk-4.1 (Ubuntu 22.04+). Earlier Ubuntu versions are not supported.
2. **GTK3 Warning**: Tauri uses GTK3 bindings which show an "unmaintained" warning - this is expected and doesn't affect functionality.
3. **Package Queries**: Some package manager queries require elevated privileges for full system scanning.

## 📚 Documentation

Comprehensive documentation is available in the [`docs/`](docs/) directory.

**Quick Links:**
- [Documentation Index](docs/README.md) - Start here for navigation
- [Architecture](docs/architecture.md) - System design and data flow
- [API Reference](docs/api.md) - Complete Tauri IPC API
- [Contributing](docs/contributing.md) - How to contribute
- [Development Guide](docs/development.md) - Development workflow

See [docs/README.md](docs/README.md) for complete documentation index.

## Contributing

See [docs/contributing.md](docs/contributing.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [Tauri](https://tauri.app/) - Desktop app framework
- [Svelte](https://svelte.dev/) - Frontend framework
- [Tailwind CSS](https://tailwindcss.com/) - Styling
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite bindings
