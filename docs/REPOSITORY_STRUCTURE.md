# Repository Structure

This document explains the organization of the Pulito repository, clearly separating the **Landing Page/Marketing Site** from the **Desktop Application**.

## Overview

This repository contains two completely separate parts:

1. **Marketing Website** - Standalone SvelteKit app in `marketing/` folder (deployed to GitHub Pages)
2. **Desktop Application** - Tauri-based desktop app in `src/` folder (bundled as desktop packages)

---

## Directory Structure

```
pulito/
├── marketing/                    # 🎯 SEPARATE MARKETING SITE
│   ├── src/
│   │   ├── routes/
│   │   │   ├── +page.svelte      # Landing page (/)
│   │   │   ├── download/         # Download page (/download)
│   │   │   ├── features/         # Features page (/features)
│   │   │   ├── faq/              # FAQ page (/faq)
│   │   │   ├── privacy/          # Privacy policy (/privacy)
│   │   │   └── terms/            # Terms of service (/terms)
│   │   ├── lib/
│   │   │   └── utils/
│   │   │       └── logger.ts     # Marketing utilities
│   │   ├── app.css               # Marketing styles
│   │   └── app.html              # Marketing HTML template
│   ├── static/                   # Marketing static assets
│   ├── package.json              # Marketing dependencies (no Tauri)
│   ├── svelte.config.js          # Marketing SvelteKit config
│   ├── vite.config.ts            # Marketing Vite config
│   ├── tailwind.config.js        # Marketing Tailwind config
│   └── README.md                 # Marketing-specific docs
├── src/                          # 🎯 TAURI DESKTOP APP
│   ├── routes/
│   │   └── app/                  # Desktop app routes
│   │       ├── +page.svelte      # Main app interface
│   │       └── +layout.svelte    # App-specific layout
│   ├── lib/                      # App components and utilities
│   │   ├── components/           # App components (Dashboard, etc.)
│   │   ├── stores/               # State management
│   │   └── utils/                # App utilities (Tauri IPC, etc.)
│   └── app.css                   # App styles
├── src-tauri/                    # Rust backend (desktop app only)
├── static/                       # Shared static assets
├── package.json                  # Root package.json (app dependencies)
└── build/                        # App build output (Tauri uses this)
```

---

## 1. Marketing Website

**Purpose:** Public-facing marketing website to promote Pulito

**Location:** `marketing/` folder (completely separate from app)

**Routes:**
- `/` - Landing page (hero, features, testimonials)
- `/features` - Detailed features page
- `/download` - Download instructions and links
- `/faq` - Frequently asked questions
- `/privacy` - Privacy policy
- `/terms` - Terms of service

**Access:**
- **Development:** `cd marketing && npm run dev` → `http://localhost:5174/`
- **Production:** Deploy `marketing/build/` to GitHub Pages or any static host

**Key Files:**
- `marketing/src/routes/+page.svelte` - Main landing page
- `marketing/src/routes/features/+page.svelte` - Features showcase
- `marketing/src/routes/download/+page.svelte` - Download page
- `marketing/src/routes/faq/+page.svelte` - FAQ page

**Characteristics:**
- ✅ Completely independent from desktop app
- ✅ Public-facing, no authentication required
- ✅ SEO optimized with meta tags
- ✅ Responsive design for all devices
- ✅ Static content (prerendered)
- ✅ Zero Tauri dependencies
- ❌ Does NOT use Tauri APIs
- ❌ Does NOT access system resources

**Build & Deploy:**
```bash
cd marketing
npm install
npm run build
# Output: marketing/build/ → Deploy to GitHub Pages
```

---

## 2. Desktop Application

**Purpose:** Full-featured desktop application for Linux system cleanup

**Route:** `/app`

**Location:** `src/routes/app/`

**Access:**
- **Tauri Dev:** `npm run tauri:dev` → Desktop window at `/app`
- **Production:** Bundled as desktop app (`.deb`, `.AppImage`)

**Key Files:**
- `src/routes/app/+page.svelte` - Main app interface
- `src/routes/app/+layout.svelte` - App layout with Header/Sidebar
- `src/lib/components/` - All app components (Dashboard, DiskPulse, etc.)
- `src-tauri/` - Rust backend with system access

**Components:**
- `Dashboard.svelte` - Main dashboard view
- `SmartCleanup.svelte` - System analysis and cleanup recommendations
- `EnhancedTreeView.svelte` - Advanced file browser with usage analysis
- `SystemHealthMonitor.svelte` - Real-time system health monitoring
- `DiskPulse.svelte` - Disk usage monitoring and old files management
- `FilesystemHealth.svelte` - Filesystem health analysis
- `StorageRecovery.svelte` - Storage cleanup tools (duplicates, large files, old downloads)
- `CacheOptimization.svelte` - Cache analytics and optimization
- `StartupManager.svelte` - Startup program management
- `TrashView.svelte` - Trash management
- `Settings.svelte` - Application settings
- `PreviewDialog.svelte` - Cleanup preview dialog
- `TreeNode.svelte` - Recursive tree node component
- `Header.svelte` - Application header
- `Sidebar.svelte` - Navigation sidebar
- `ConfirmationDialog.svelte` - Confirmation dialog component
- `NotificationToast.svelte` - Toast notification component
- `charts/` - Chart components (8 components: CPU, Memory, Network, Disk I/O, Temperature, Storage, Sparkline, Performance Gauge)
- `health/` - Health monitor components (9 components: CPU, Memory, GPU, Network, Disk, Temperature, Battery, Process, Performance Summary)
- `ui/` - Base UI components (InfoCard, LoadingSpinner, ProgressBar)

**Characteristics:**
- ✅ Requires Tauri runtime (desktop app)
- ✅ Full system access via Rust backend
- ✅ Real-time system monitoring
- ✅ File system operations
- ✅ Database for scan history and trash
- ❌ Cannot run in browser (Tauri APIs required)
- ❌ Not accessible via web URL in production

**Build & Deploy:**
```bash
npm install
npm run tauri:build
# Output: src-tauri/target/release/bundle/ → Desktop packages
```

---

## Development Workflows

### Running the Marketing Site

```bash
# From root directory
npm run marketing:dev

# Or from marketing directory
cd marketing
npm install
npm run dev
# Opens: http://localhost:5174/
```

**Use Case:** Testing marketing pages, SEO, responsive design

### Running the Desktop App

```bash
npm run tauri:dev
# Opens: Desktop window at /app
# Shows: Full application interface
```

**Use Case:** Developing app features, testing Tauri APIs, system integration

### Building Both

```bash
# Build marketing site
npm run marketing:build
# Output: marketing/build/

# Build desktop app
npm run tauri:build
# Output: Desktop packages
```

---

## Routing Configuration

### Marketing Site

The marketing site is a standalone SvelteKit application:
- All routes are prerendered for static hosting
- No server-side rendering needed
- Deploys as static HTML/CSS/JS

### Desktop App

The desktop app uses SvelteKit routing but runs in Tauri:
- Tauri config points to `/app` route
- No prerendering (SSR disabled)
- Requires Tauri runtime to function

### Tauri Configuration

The Tauri app is configured to open at `/app`:

```json
// src-tauri/tauri.conf.json
{
  "app": {
    "windows": [{
      "url": "/app"  // ← Desktop app opens here
    }]
  }
}
```

---

## Build Output

### Marketing Site Build

```bash
cd marketing
npm run build
# Output: marketing/build/
# Contains: All routes prerendered as static HTML
# Deploy: Can be deployed to GitHub Pages, Netlify, etc.
```

### Desktop App Build

```bash
npm run tauri:build
# Output: src-tauri/target/release/bundle/
# Contains: .deb and .AppImage packages
# Deploy: Distribution via GitHub Releases
```

---

## Important Notes

### ⚠️ Complete Separation

The marketing site and desktop app are **completely separate**:

- **Marketing:** Independent SvelteKit app in `marketing/` folder
- **App:** Tauri app in `src/` folder
- **No shared code:** Marketing has its own dependencies and build process
- **Independent deployment:** Marketing deploys separately from app

### 🔒 Security

- Marketing pages are public and don't require authentication
- Desktop app has full system access (by design)
- Tauri security policies apply to desktop app only
- Marketing site has zero system access

### 📦 Dependencies

- **Marketing:** Only frontend dependencies (Svelte, Tailwind) - no Tauri
- **Desktop App:** Frontend + Tauri + Rust backend dependencies

---

## Quick Reference

| Aspect | Marketing Site | Desktop App |
|--------|---------------|-------------|
| **Location** | `marketing/` | `src/` |
| **Route** | `/` (and marketing pages) | `/app` |
| **Dev Command** | `npm run marketing:dev` | `npm run tauri:dev` |
| **Build Command** | `npm run marketing:build` | `npm run tauri:build` |
| **URL** | `http://localhost:5174/` | Desktop window |
| **Tauri APIs** | ❌ No | ✅ Yes |
| **System Access** | ❌ No | ✅ Yes |
| **Purpose** | Marketing | Application |
| **Deployment** | Static site (GitHub Pages) | Desktop packages |

---

## Benefits of Separate Structure

1. **Zero Risk of Mixing**: Physically impossible to accidentally include app code in marketing
2. **Independent Deployment**: Marketing deploys without any app dependencies
3. **Clear Boundaries**: Obvious where marketing code ends and app code begins
4. **Independent Development**: Can work on marketing without app dependencies
5. **Future Flexibility**: Easy to move marketing to separate repo if needed
6. **Cleaner Dependencies**: Marketing doesn't need Tauri, Rust, or app-specific packages
7. **Better CI/CD**: Separate build and deployment pipelines
8. **Team Collaboration**: Different teams can work independently

---

## Questions?

- **Marketing site issues?** Check `marketing/` directory and `marketing/README.md`
- **App not working?** Ensure you're running `npm run tauri:dev` (not `npm run dev`)
- **Routing problems?** Verify Tauri config points to `/app`
- **Build issues?** Check respective config files (`marketing/svelte.config.js` or root `svelte.config.js`)

---

**Last Updated:** December 2025
