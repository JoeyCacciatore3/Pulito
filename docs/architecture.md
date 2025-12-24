# Pulito Architecture Documentation

## Overview

Pulito is a desktop application built with Tauri 2.2+, combining a Rust backend with a Svelte 5 frontend. The architecture follows a clean separation of concerns with IPC communication between frontend and backend.

## Technology Stack

### Frontend
- **Framework**: Svelte 5.46.0 with SvelteKit 2.49.2
- **State Management**: Svelte 5 runes (`$state`, `$derived`, `$effect`)
- **Styling**: Tailwind CSS 4.1.18
- **Build Tool**: Vite 7.3.0
- **Type Safety**: TypeScript 5.9.3
- **Testing**: Vitest 4.0.16

### Backend
- **Runtime**: Rust 1.80+ with Tauri 2.2+
- **Database**: SQLite via rusqlite 0.33 (bundled)
- **System Info**: sysinfo 0.33
- **Async Runtime**: Tokio 1.42
- **Logging**: tracing + tracing-subscriber

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (Svelte 5)                   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │Components│  │  Stores  │  │  Utils   │              │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘              │
│       │             │             │                     │
│       └─────────────┴─────────────┘                     │
│                    │                                     │
│                    │ IPC (Tauri invoke)                  │
└────────────────────┼─────────────────────────────────────┘
                     │
┌────────────────────┼─────────────────────────────────────┐
│                    │         Backend (Rust/Tauri)        │
│                    ▼                                     │
│          ┌──────────────────┐                           │
│          │  Command Handlers │                           │
│          └────────┬─────────┘                           │
│                   │                                     │
│    ┌──────────────┼──────────────┐                     │
│    │              │              │                     │
│    ▼              ▼              ▼                     │
│ ┌──────┐    ┌──────────┐   ┌──────────┐               │
│ │  DB  │    │ Scanner  │   │ Packages │               │
│ └──────┘    └──────────┘   └──────────┘               │
│    │              │              │                     │
│    │              │              │                     │
│    └──────────────┴──────────────┘                     │
│                   │                                     │
│                   ▼                                     │
│            ┌──────────┐                                │
│            │  Trash   │                                │
│            └──────────┘                                │
└─────────────────────────────────────────────────────────┘
```

## Frontend Architecture

### Directory Structure

```
src/
├── lib/
│   ├── components/          # Reusable UI components
│   │   ├── ui/              # Base UI components (InfoCard, LoadingSpinner, ProgressBar)
│   │   ├── Dashboard.svelte
│   │   ├── SystemHealthMonitor.svelte
│   │   ├── FilesystemHealth.svelte
│   │   ├── StorageRecovery.svelte
│   │   ├── CacheOptimization.svelte
│   │   ├── DiskPulse.svelte
│   │   ├── TrashView.svelte
│   │   ├── TreeView.svelte
│   │   ├── Settings.svelte
│   │   ├── Header.svelte
│   │   ├── Sidebar.svelte
│   │   ├── ConfirmationDialog.svelte
│   │   └── NotificationToast.svelte
│   ├── stores/              # Svelte 5 rune-based state management
│   │   ├── confirmation.svelte.ts
│   │   ├── navigation.svelte.ts
│   │   ├── notifications.svelte.ts
│   │   ├── scanner.svelte.ts
│   │   ├── settings.svelte.ts
│   │   └── theme.svelte.ts
│   └── utils/               # Utility functions
│       ├── tauri.ts         # Tauri IPC wrapper with timeout protection
│       ├── logger.ts        # Structured logging
│       ├── color-utils.ts   # Color/theme utilities
│       ├── confirmations.ts # Confirmation dialog helpers
│       └── notification-helpers.ts
├── routes/                  # SvelteKit file-based routing
│   ├── +page.svelte         # Marketing landing page
│   ├── app/                 # Desktop app interface
│   │   ├── +layout.svelte
│   │   └── +page.svelte
│   ├── download/
│   ├── features/
│   ├── faq/
│   ├── privacy/
│   └── terms/
└── test/                    # Test setup
    └── setup.ts
```

### State Management with Svelte 5 Runes

Pulito uses Svelte 5 runes for reactive state management:

- **`$state`**: Declares reactive state variables
- **`$derived`**: Computed values that automatically update when dependencies change
- **`$effect`**: Side effects that run when dependencies change

#### Store Pattern

Stores are implemented using runes with a functional API:

```typescript
// Example: confirmation.svelte.ts
export const confirmation = {
  show: (options: ConfirmationOptions) => Promise<boolean>,
  // ...
};
```

### Component Hierarchy

```
App Layout
├── Header (scan trigger, navigation)
├── Sidebar (navigation, selected items)
└── Main Content
    ├── Dashboard (overview, categories)
    ├── SystemHealthMonitor (real-time monitoring)
    ├── FilesystemHealth (health scan results)
    ├── StorageRecovery (duplicates, large files, old downloads)
    ├── CacheOptimization (cache analytics and cleanup)
    ├── DiskPulse (disk usage monitoring)
    ├── TreeView (file browser with preview)
    ├── TrashView (trash management)
    └── Settings (application settings)
```

## Backend Architecture

### Directory Structure

```
src-tauri/src/
├── main.rs              # Application entry point, Tauri setup
├── commands/            # IPC command handlers
│   └── mod.rs          # All 26 Tauri commands
├── db/                  # Database management
│   └── mod.rs          # SQLite schema and operations
├── scanner/             # File system scanning logic
│   └── mod.rs          # Scan operations and analysis
├── packages/            # Package manager integration
│   └── mod.rs          # apt/dpkg, pip, npm support
└── trash/               # Recoverable deletion system
    └── mod.rs          # Trash operations
```

### Module Responsibilities

#### `main.rs`
- Initializes Tauri application
- Sets up system tray
- Initializes database
- Registers all IPC command handlers
- Configures logging

#### `commands/mod.rs`
- Contains all 26 Tauri IPC command handlers
- Defines data structures (SystemStats, SystemHealthData, etc.)
- Implements timeout protection for all commands
- Handles error propagation

#### `db/mod.rs`
- Database initialization
- Schema definition (8 tables)
- Database access patterns
- Transaction management

#### `scanner/mod.rs`
- File system scanning operations
- Cache detection
- Duplicate file detection
- Large file analysis
- Filesystem health checks

#### `packages/mod.rs`
- Package manager integration (apt/dpkg, pip, npm)
- Dependency resolution
- Orphan package detection
- Package statistics

#### `trash/mod.rs`
- Trash directory management
- File move to trash
- Trash restoration
- Trash cleanup (expired items)
- Metadata management

## IPC Communication Pattern

### Frontend → Backend

All communication uses the `invoke` wrapper from `src/lib/utils/tauri.ts`:

```typescript
import { invoke } from '$lib/utils/tauri';

// Example: Get system stats
const stats = await invoke<SystemStats>('get_system_stats', undefined, 30000);
```

### Backend Command Structure

All commands follow this pattern:

```rust
#[tauri::command]
pub async fn command_name(params: Params) -> Result<ReturnType, String> {
    // Set timeout
    let timeout = Duration::from_secs(30);

    // Execute with timeout protection
    timeout(timeout, async {
        // Command logic
    }).await.map_err(|_| "Timeout".to_string())?
}
```

### Timeout Protection

All commands have timeout protection:
- Quick operations: 5-10 seconds (settings, trash operations)
- Medium operations: 30 seconds (system stats, health monitoring)
- Long operations: 5-10 minutes (scans, cleanup operations)

## Data Flow

### System Stats Flow

```
Frontend Dashboard
    │
    ├─→ invoke('get_system_stats')
    │       │
    │       ├─→ commands::get_system_stats()
    │       │       │
    │       │       ├─→ sysinfo::Disks (disk info)
    │       │       ├─→ packages::get_package_stats() (package info)
    │       │       ├─→ db::get_scan_results() (previous scan data)
    │       │       └─→ Return SystemStats
    │       │
    │       └─→ Update Dashboard UI
    └─→ Display stats and categories
```

### Scan Flow

```
Frontend TreeView/Scanner
    │
    ├─→ invoke('start_scan', options)
    │       │
    │       ├─→ commands::start_scan(options)
    │       │       │
    │       │       ├─→ scanner::scan_files(options)
    │       │       │       ├─→ Walk directory tree
    │       │       │       ├─→ Analyze files (size, type, risk)
    │       │       │       └─→ Return ScanResults
    │       │       │
    │       │       ├─→ db::save_scan_results()
    │       │       └─→ Return ScanResults
    │       │
    │       └─→ Update scanner store
    └─→ Display results in TreeView
```

### Cleanup Flow

```
Frontend Component
    │
    ├─→ Confirmation dialog
    │       │
    │       └─→ User confirms
    │
    ├─→ invoke('clean_items', { item_ids, item_paths, use_trash })
    │       │
    │       ├─→ commands::clean_items()
    │       │       │
    │       │       ├─→ For each item:
    │       │       │       ├─→ Validate path (security check)
    │       │       │       ├─→ Move to trash (if use_trash)
    │       │       │       │   └─→ trash::move_to_trash()
    │       │       │       └─→ Or delete permanently
    │       │       │
    │       │       └─→ Return CleanResult
    │       │
    │       └─→ Update UI, show notification
    └─→ Refresh data
```

## Security Architecture

### Path Validation

All file operations go through path validation:

1. **Canonical Path Resolution**: Resolve symlinks and normalize paths
2. **Directory Traversal Protection**: Block `..` in paths
3. **System Directory Protection**: Block critical directories (`/bin`, `/etc`, `/usr`, etc.)
4. **User Home Restriction**: Only allow operations in user's home directory

### Error Handling

- All commands return `Result<T, String>` for error propagation
- Frontend handles errors gracefully with user-friendly messages
- Structured logging on backend for debugging

## Database Schema

See [database.md](database.md) for complete schema documentation.

## Performance Considerations

### Frontend
- Svelte 5 runes provide efficient reactivity (minimal re-renders)
- Component code splitting for lazy loading
- Optimized bundle size with tree shaking

### Backend
- Async/await for non-blocking I/O
- Timeout protection prevents hanging operations
- Database indexes for fast queries
- Efficient file walking with `walkdir`

## Testing Strategy

### Frontend Tests
- Unit tests for utility functions
- Component tests with Testing Library
- Test setup in `src/test/setup.ts`

### Backend Tests
- Unit tests in each module
- Database tests with temporary databases
- Integration tests for command handlers

## Build Process

### Development
```bash
npm run tauri:dev  # Hot reload for frontend, incremental Rust compilation
```

### Production
```bash
npm run build           # Build frontend
npm run tauri:build     # Build complete desktop app
```

## Deployment

- **Format**: .deb, AppImage
- **Target**: Linux (Ubuntu 22.04+)
- **Dependencies**: Bundled in AppImage, system packages for .deb

## Future Enhancements

See README.md for planned features and enhancements.
