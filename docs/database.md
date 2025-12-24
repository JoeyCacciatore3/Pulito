# Database Schema Documentation

## Overview

Pulito uses SQLite (via rusqlite 0.33) with bundled SQLite 3.48 for data persistence. The database stores scan history, trash metadata, settings, cache events, disk usage history, monitoring state, file access tracking, and scan results.

## Database Location

The database is stored in the application data directory (platform-specific):
- **Linux**: `~/.local/share/pulito/database.db`
- Managed by Tauri's `app_data_dir()` API

## Schema Overview

The database consists of 8 tables:

1. `scan_history` - Historical scan results
2. `trash_items` - Trash metadata (actual files stored separately)
3. `settings` - Application settings (key-value store)
4. `cache_events` - Cache growth events for DiskPulse
5. `disk_history` - Disk usage history
6. `monitoring_state` - DiskPulse monitoring state
7. `file_access` - File access tracking for old files detection
8. `last_scan_results` - Last scan results for Dashboard display

## Table Definitions

### `scan_history`

Stores historical scan results for tracking cleanup effectiveness.

```sql
CREATE TABLE IF NOT EXISTS scan_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    total_size INTEGER NOT NULL,
    total_items INTEGER NOT NULL,
    scan_time_ms INTEGER NOT NULL,
    cleaned_size INTEGER DEFAULT 0
);
```

**Columns:**
- `id`: Primary key (auto-increment)
- `timestamp`: ISO 8601 timestamp of the scan
- `total_size`: Total size of items found (bytes)
- `total_items`: Number of items found
- `scan_time_ms`: Time taken for scan (milliseconds)
- `cleaned_size`: Amount cleaned after scan (bytes)

**Usage**: Historical tracking of scans and cleanup effectiveness.

### `trash_items`

Metadata for items in the trash system. Actual files are stored in the trash directory.

```sql
CREATE TABLE IF NOT EXISTS trash_items (
    id TEXT PRIMARY KEY,
    original_path TEXT NOT NULL,
    trash_path TEXT NOT NULL,
    deleted_at TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    size INTEGER NOT NULL,
    item_type TEXT NOT NULL,
    metadata TEXT
);
```

**Columns:**
- `id`: UUID primary key
- `original_path`: Original file/directory path before deletion
- `trash_path`: Path in trash directory
- `deleted_at`: ISO 8601 timestamp when deleted
- `expires_at`: ISO 8601 timestamp when item expires
- `size`: Size in bytes
- `item_type`: Type ("file" or "directory")
- `metadata`: JSON string with TrashMetadata (category, risk_level, reason)

**Indexes:**
- `idx_trash_items_expires` on `expires_at` for efficient expiration queries

**Usage**: Trash management, restoration, and expiration cleanup.

**Note**: Actual file storage is in `~/.local/share/linux-cleaner/trash/` directory structure.

### `settings`

Application settings stored as key-value pairs.

```sql
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

**Columns:**
- `key`: Setting key (e.g., "trash.retention_days")
- `value`: JSON-encoded setting value

**Usage**: Persistent application settings (trash, monitoring, notifications, scan, theme).

**Settings Structure:**
```json
{
  "trash": {
    "retention_days": 3,
    "max_size_mb": 1000
  },
  "monitoring": {
    "enabled": true,
    "interval_hours": 24
  },
  "notifications": {
    "system": true,
    "tray": true,
    "in_app": true
  },
  "scan": {
    "include_hidden": false,
    "large_file_threshold_mb": 100
  },
  "theme": "system"
}
```

### `cache_events`

Cache growth events for DiskPulse cache feed.

```sql
CREATE TABLE IF NOT EXISTS cache_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL,
    size_change INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    source TEXT,
    timestamp INTEGER NOT NULL
);
```

**Columns:**
- `id`: Primary key (auto-increment)
- `path`: File or directory path
- `size_change`: Change in size (bytes, can be negative)
- `event_type`: Type of event (e.g., "created", "modified", "deleted")
- `source`: Cache source identifier (optional)
- `timestamp`: Unix timestamp (seconds since epoch)

**Indexes:**
- `idx_cache_events_timestamp` on `timestamp` for time-based queries
- `idx_cache_events_source` on `source` for source-based queries

**Usage**: Track cache growth patterns for DiskPulse analytics.

### `disk_history`

Historical disk usage data for trending analysis.

```sql
CREATE TABLE IF NOT EXISTS disk_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    used_bytes INTEGER NOT NULL,
    total_bytes INTEGER NOT NULL,
    available_bytes INTEGER NOT NULL
);
```

**Columns:**
- `id`: Primary key (auto-increment)
- `timestamp`: Unix timestamp (seconds since epoch)
- `used_bytes`: Used disk space (bytes)
- `total_bytes`: Total disk space (bytes)
- `available_bytes`: Available disk space (bytes)

**Indexes:**
- `idx_disk_history_timestamp` on `timestamp` for time-based queries

**Usage**: Disk usage trending and projection for DiskPulse.

### `monitoring_state`

DiskPulse monitoring state persistence.

```sql
CREATE TABLE IF NOT EXISTS monitoring_state (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);
```

**Columns:**
- `key`: State key (e.g., "is_running", "last_check")
- `value`: State value (JSON-encoded if needed)
- `updated_at`: Unix timestamp of last update

**Usage**: Persist DiskPulse monitoring state across application restarts.

### `file_access`

File access tracking for old files detection.

```sql
CREATE TABLE IF NOT EXISTS file_access (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    size INTEGER NOT NULL,
    last_access INTEGER NOT NULL
);
```

**Columns:**
- `id`: Primary key (auto-increment)
- `path`: File path (unique)
- `size`: File size (bytes)
- `last_access`: Unix timestamp of last access

**Indexes:**
- `idx_file_access_last_access` on `last_access` for finding old files

**Usage**: Track file access patterns to identify old/unused files.

### `last_scan_results`

Cached results from last scan for Dashboard display.

```sql
CREATE TABLE IF NOT EXISTS last_scan_results (
    scan_type TEXT PRIMARY KEY,
    total_size INTEGER NOT NULL,
    total_items INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,
    scan_data TEXT
);
```

**Columns:**
- `scan_type`: Type of scan ("filesystem_health", "storage_recovery", etc.)
- `total_size`: Total size found (bytes)
- `total_items`: Number of items found
- `timestamp`: Unix timestamp of scan
- `scan_data`: JSON-encoded detailed scan data (optional)

**Usage**: Quick access to last scan results for Dashboard without re-scanning.

## Data Relationships

```
scan_history
    └─→ Stores historical scan data

trash_items
    └─→ Self-contained (references files on filesystem)

settings
    └─→ Self-contained key-value store

cache_events ─┐
              ├─→ Related by timestamp/source for analytics
disk_history ─┤
file_access ──┘

monitoring_state
    └─→ Self-contained state storage

last_scan_results
    └─→ Cached data from scanner operations
```

## Indexes

All indexes are created with `IF NOT EXISTS` to support schema migrations:

1. **idx_trash_items_expires** - Fast expiration queries
2. **idx_cache_events_timestamp** - Time-based cache event queries
3. **idx_cache_events_source** - Source-based cache event filtering
4. **idx_disk_history_timestamp** - Time-based disk history queries
5. **idx_file_access_last_access** - Old file detection queries

## Database Initialization

The database is initialized on application startup in `main.rs`:

```rust
db::initialize_database(&app_handle)
```

The initialization process:
1. Creates database file if it doesn't exist
2. Creates all tables with `IF NOT EXISTS`
3. Creates all indexes
4. Sets up WAL mode for better concurrency

## Migration Strategy

Current approach:
- All schema changes use `CREATE TABLE IF NOT EXISTS`
- All indexes use `CREATE INDEX IF NOT EXISTS`
- No explicit migration system (schema is backward compatible)

**Future Consideration**: Implement proper migrations if schema changes become incompatible.

## Access Patterns

### Read Patterns

1. **Settings**: Frequent reads on app start and settings page
2. **Last Scan Results**: Read on Dashboard load
3. **Trash Items**: Read when opening Trash view
4. **Cache Events**: Read for DiskPulse cache feed
5. **Disk History**: Read for DiskPulse trending

### Write Patterns

1. **Settings**: Writes on settings save (infrequent)
2. **Scan History**: Writes after each scan
3. **Trash Items**: Writes on delete/restore operations
4. **Cache Events**: Writes during file system monitoring
5. **Disk History**: Periodic writes during monitoring
6. **Last Scan Results**: Writes after each scan type

## Performance Considerations

- **WAL Mode**: Enabled for better read/write concurrency
- **Indexes**: Strategic indexes on frequently queried columns
- **Prepared Statements**: Used for repeated queries
- **Transactions**: Used for atomic operations

## Data Retention

- **scan_history**: Unlimited (user can manually clean if needed)
- **trash_items**: Auto-expired based on `expires_at` timestamp
- **cache_events**: Can be cleaned by age (old events)
- **disk_history**: Can be cleaned by age (old history)
- **file_access**: Can be cleaned by age (old access records)
- **last_scan_results**: Updated on each scan (single row per scan type)
- **settings**: Persistent (never expired)

## Backup Recommendations

Users should backup:
- Database file: `~/.local/share/pulito/database.db`
- Trash directory: `~/.local/share/linux-cleaner/trash/`

Backup should include both for complete data recovery.
