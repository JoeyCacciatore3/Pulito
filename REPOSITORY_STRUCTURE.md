# Repository Structure

This document explains the organization of the Pulito repository, clearly separating the **Landing Page/Marketing Site** from the **Desktop Application**.

## Overview

This repository contains two distinct parts:

1. **Landing Page / Marketing Website** - Public-facing marketing site
2. **Desktop Application** - Tauri-based desktop app for Linux system cleanup

---

## Directory Structure

```
pulito/
├── src/
│   ├── routes/
│   │   ├── +page.svelte          # Landing page (marketing site root)
│   │   ├── +layout.svelte         # Root layout (shared for all routes)
│   │   ├── app/                   # 🎯 DESKTOP APPLICATION
│   │   │   ├── +page.svelte       # Main app interface
│   │   │   └── +layout.svelte     # App-specific layout
│   │   ├── download/              # Marketing: Download page
│   │   ├── features/              # Marketing: Features page
│   │   ├── faq/                   # Marketing: FAQ page
│   │   ├── privacy/               # Marketing: Privacy policy
│   │   └── terms/                 # Marketing: Terms of service
│   └── lib/                       # Shared components and utilities
│       ├── components/            # React components (used by app)
│       ├── stores/                # State management
│       └── utils/                 # Utility functions
├── src-tauri/                     # Rust backend (desktop app only)
├── static/                        # Static assets (shared)
└── build/                         # Build output
```

---

## 1. Landing Page / Marketing Website

**Purpose:** Public-facing marketing website to promote Pulito

**Routes:**
- `/` - Landing page (hero, features, testimonials)
- `/features` - Detailed features page
- `/download` - Download instructions and links
- `/faq` - Frequently asked questions
- `/privacy` - Privacy policy
- `/terms` - Terms of service

**Location:** `src/routes/+page.svelte` and related route files

**Access:**
- **Web:** `http://localhost:5174/` (when running `npm run dev`)
- **Production:** Deployed as static site (GitHub Pages, etc.)

**Key Files:**
- `src/routes/+page.svelte` - Main landing page
- `src/routes/features/+page.svelte` - Features showcase
- `src/routes/download/+page.svelte` - Download page
- `src/routes/faq/+page.svelte` - FAQ page

**Characteristics:**
- ✅ Public-facing, no authentication required
- ✅ SEO optimized with meta tags
- ✅ Responsive design for all devices
- ✅ Static content (can be pre-rendered)
- ❌ Does NOT use Tauri APIs
- ❌ Does NOT access system resources

---

## 2. Desktop Application

**Purpose:** Full-featured desktop application for Linux system cleanup

**Route:** `/app`

**Location:** `src/routes/app/`

**Access:**
- **Tauri Dev:** Automatically opens at `/app` when running `npm run tauri:dev`
- **Web Dev:** `http://localhost:5174/app` (limited functionality, Tauri APIs unavailable)
- **Production:** Bundled as desktop app (`.deb`, `.AppImage`)

**Key Files:**
- `src/routes/app/+page.svelte` - Main app interface
- `src/routes/app/+layout.svelte` - App layout with Header/Sidebar
- `src/lib/components/` - All app components (Dashboard, DiskPulse, etc.)
- `src-tauri/` - Rust backend with system access

**Components:**
- `Dashboard.svelte` - Main dashboard view
- `DiskPulse.svelte` - Real-time disk monitoring
- `FilesystemHealth.svelte` - Filesystem analysis
- `StorageRecovery.svelte` - Storage cleanup tools
- `CacheOptimization.svelte` - Cache management
- `TreeView.svelte` - File tree browser
- `TrashView.svelte` - Trash management
- `Settings.svelte` - Application settings
- `SystemHealthMonitor.svelte` - System monitoring

**Characteristics:**
- ✅ Requires Tauri runtime (desktop app)
- ✅ Full system access via Rust backend
- ✅ Real-time system monitoring
- ✅ File system operations
- ✅ Database for scan history and trash
- ❌ Cannot run in browser (Tauri APIs required)
- ❌ Not accessible via web URL in production

---

## Development Workflows

### Running the Landing Page (Web)

```bash
npm run dev
# Opens: http://localhost:5174/
# Shows: Landing page at root (/)
```

**Use Case:** Testing marketing pages, SEO, responsive design

### Running the Desktop App

```bash
npm run tauri:dev
# Opens: Desktop window at /app
# Shows: Full application interface
```

**Use Case:** Developing app features, testing Tauri APIs, system integration

### Running Both (for testing)

```bash
# Terminal 1: Start dev server
npm run dev

# Terminal 2: Start Tauri (uses dev server)
npm run tauri:dev
```

**Note:** Tauri will automatically navigate to `/app` in the desktop window, while the browser can access both `/` (landing) and `/app` (app).

---

## Routing Configuration

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

### SvelteKit Routing

SvelteKit automatically handles routing based on the `src/routes/` directory structure:

- `/` → `src/routes/+page.svelte` (Landing page)
- `/app` → `src/routes/app/+page.svelte` (Desktop app)
- `/features` → `src/routes/features/+page.svelte` (Marketing)
- `/download` → `src/routes/download/+page.svelte` (Marketing)

---

## Build Output

### Landing Page Build

```bash
npm run build
# Output: build/
# Contains: All routes pre-rendered as static HTML
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

### ⚠️ Tauri API Availability

The desktop app (`/app`) **requires** Tauri APIs to function properly. When accessing `/app` in a regular browser:

- ❌ Tauri APIs are unavailable
- ❌ System operations will fail
- ✅ UI will render but show errors
- ✅ Use `npm run tauri:dev` for proper app development

### 🔒 Security

- Landing pages are public and don't require authentication
- Desktop app has full system access (by design)
- Tauri security policies apply to desktop app only

### 📦 Dependencies

- **Landing Page:** Only frontend dependencies (Svelte, Tailwind)
- **Desktop App:** Frontend + Tauri + Rust backend dependencies

---

## Quick Reference

| Aspect | Landing Page | Desktop App |
|--------|-------------|-------------|
| **Route** | `/` | `/app` |
| **Dev Command** | `npm run dev` | `npm run tauri:dev` |
| **URL** | `http://localhost:5174/` | Desktop window |
| **Tauri APIs** | ❌ No | ✅ Yes |
| **System Access** | ❌ No | ✅ Yes |
| **Purpose** | Marketing | Application |
| **Deployment** | Static site | Desktop packages |

---

## Future Improvements

Consider these organizational improvements:

1. **Separate Repositories:**
   - `pulito-website` - Marketing site
   - `pulito-app` - Desktop application

2. **Monorepo Structure:**
   ```
   packages/
     ├── website/     # Landing page
     └── app/         # Desktop app
   ```

3. **Shared Components:**
   - Extract truly shared components to `packages/shared/`
   - Keep app-specific components in app directory

---

## Questions?

- **Landing page issues?** Check `src/routes/+page.svelte`
- **App not working?** Ensure you're running `npm run tauri:dev` (not `npm run dev`)
- **Routing problems?** Verify Tauri config points to `/app`
- **Build issues?** Check `svelte.config.js` and `src-tauri/tauri.conf.json`

---

**Last Updated:** December 21, 2025
