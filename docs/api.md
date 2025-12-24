# Tauri IPC API Reference

## Overview

Pulito exposes 26 Tauri IPC commands for communication between the frontend (Svelte) and backend (Rust). All commands are async and return `Result<T, String>` for error handling.

## Type Definitions

All TypeScript types for command return values and parameters are automatically generated from Rust structs using [Specta](https://github.com/oscartbeaumont/specta). The generated types are located in `src/lib/generated/types.ts` and are automatically updated when the application runs in debug mode.

To use these types in your code:

```typescript
import type { SystemStats, ScanResults, TrashItem } from '$lib/generated/types';
```

See the [Specta Implementation Guide](development/specta-implementation.md) for more details.

## Invocation Pattern

All commands are invoked from the frontend using the `invoke` wrapper:

```typescript
import { invoke } from '$lib/utils/tauri';

// Example
const result = await invoke<ReturnType>('command_name', args, timeoutMs);
```

The `invoke` wrapper provides:
- Type safety with TypeScript generics
- Timeout protection (default: 30 seconds, configurable)
- Error handling and logging
- Validation of Tauri environment

## Command Categories

1. **Initialization** (1 command)
2. **System Information** (2 commands)
3. **Scanning Operations** (3 commands)
4. **Cleanup Operations** (4 commands)
5. **Trash Management** (4 commands)
6. **Settings Management** (2 commands)
7. **DiskPulse Monitoring** (7 commands)
8. **Cache Management** (3 commands)

## Command Reference

### Initialization

#### `initialize_app`

Initializes the application on startup.

**Signature:**
```rust
pub async fn initialize_app() -> Result<(), String>
```

**Parameters:** None

**Return Type:** `Result<(), String>`

**Timeout:** No explicit timeout (runs during app startup)

**Description:**
- Cleans up expired trash items
- Called automatically on application startup
- Frontend typically doesn't need to call this directly

**Usage:**
```typescript
await invoke('initialize_app');
```

---

### System Information

#### `get_system_stats`

Gets comprehensive system statistics for the Dashboard.

**Signature:**
```rust
pub async fn get_system_stats(app_handle: tauri::AppHandle) -> Result<SystemStats, String>
```

**Parameters:**
- `app_handle`: Tauri AppHandle (injected automatically)

**Return Type:** `SystemStats`

The `SystemStats` type is automatically generated from the Rust struct. Import it from generated types:

```typescript
import type { SystemStats } from '$lib/generated/types';
```

Type definition (generated from Rust):
```typescript
export type SystemStats = {
  total_disk_space: number;
  used_disk_space: number;
  cleanable_space: number;
  last_scan: string | null;
  packages_installed: number;
  orphan_packages: number;
  cache_size: number;
  filesystem_health_savings?: number;
  storage_recovery_savings?: number;
  orphan_packages_size?: number;
}
```

**Timeout:** 30 seconds (internal blocking task timeout)

**Description:**
- Queries disk space information
- Gets package statistics (total and orphaned packages)
- Calculates cache directory size
- Retrieves last scan results from database
- Combines all data into comprehensive stats

**Usage:**
```typescript
const stats = await invoke<SystemStats>('get_system_stats', undefined, 30000);
```

**Error Handling:**
- Package stats timeout: Uses defaults (0 packages)
- Cache size timeout: Uses 0
- Database errors: Uses None for optional fields

---

#### `get_system_health`

Gets real-time system health monitoring data.

**Signature:**
```rust
pub async fn get_system_health() -> Result<SystemHealthData, String>
```

**Parameters:** None

**Return Type:** `SystemHealthData`

```typescript
interface SystemHealthData {
  cpu_usage: number;              // Percentage (0-100)
  cpu_cores: number;
  cpu_frequency: number;          // MHz
  core_usages: number[];          // Per-core usage percentages
  total_memory: number;           // Bytes
  used_memory: number;            // Bytes
  available_memory: number;       // Bytes
  swap_total: number;             // Bytes
  swap_used: number;              // Bytes
  gpu_info?: GpuInfo;             // Optional GPU information
  network_up: number;             // Bytes per second
  network_down: number;           // Bytes per second
  network_interfaces: NetworkInterfaceInfo[];
  active_connections: NetworkConnection[];
  temperatures: {
    cpu: number;                  // Celsius
    system: number;               // Celsius
    gpu?: number;                 // Celsius (optional)
  };
  disk_read_bytes: number;        // Bytes per second
  disk_write_bytes: number;       // Bytes per second
  disk_read_ops: number;          // IOPS
  disk_write_ops: number;         // IOPS
  battery_info?: BatteryInfo;     // Optional battery info
  top_processes: ProcessInfo[];   // Top resource consumers
  load_average?: {
    one_min: number;
    five_min: number;
    fifteen_min: number;
  };
  timestamp: number;              // Unix timestamp
}
```

**Timeout:** 30 seconds

**Description:**
- Refreshes system information (CPU, memory, processes)
- Gathers GPU information (NVIDIA NVML if available)
- Collects network statistics and connections
- Reads temperature sensors
- Calculates disk I/O statistics
- Enumerates top processes by CPU/memory usage
- Reads load averages (Linux)

**Usage:**
```typescript
const health = await invoke<SystemHealthData>('get_system_health', undefined, 30000);
```

**Notes:**
- GPU information requires NVIDIA GPU with NVML support
- Battery information only available on laptops
- Network connections limited to 50 for performance

---

### Scanning Operations

#### `start_scan`

Performs a comprehensive system scan.

**Signature:**
```rust
pub async fn start_scan(options: ScanOptions) -> Result<ScanResults, String>
```

**Parameters:**
```typescript
interface ScanOptions {
  include_caches?: boolean;
  include_packages?: boolean;
  include_logs?: boolean;
  include_hidden?: boolean;
  large_file_threshold_mb?: number;
}
```

**Return Type:** `ScanResults`

```typescript
interface ScanResults {
  items: ScanItem[];
  total_size: number;
  total_items: number;
  scan_time_ms: number;
  timestamp: string;
}
```

**Timeout:**
- Comprehensive scans (caches + packages): 15 minutes (900 seconds)
- Basic scans: 10 minutes (600 seconds)

**Description:**
- Scans system for cleanable items based on options
- Analyzes caches, packages, logs, and large files
- Calculates sizes and assesses risk levels
- Returns comprehensive scan results

**Usage:**
```typescript
const results = await invoke<ScanResults>('start_scan', {
  include_caches: true,
  include_packages: true,
  include_hidden: false,
  large_file_threshold_mb: 100
}, 900000); // 15 minute timeout
```

---

#### `scan_filesystem_health`

Scans for filesystem health issues (empty directories, broken symlinks, orphaned temp files).

**Signature:**
```rust
pub async fn scan_filesystem_health(app_handle: tauri::AppHandle) -> Result<FilesystemHealthResults, String>
```

**Parameters:**
- `app_handle`: Tauri AppHandle (injected automatically)

**Return Type:** `FilesystemHealthResults`

```typescript
interface FilesystemHealthResults {
  empty_directories: ScanItem[];
  broken_symlinks: ScanItem[];
  orphaned_temp_files: ScanItem[];
  total_size: number;
  total_items: number;
}
```

**Timeout:** 5 minutes (300 seconds)

**Description:**
- Finds empty directories
- Identifies broken symlinks
- Detects orphaned temporary files
- Stores results in database for Dashboard display
- All items are safe to remove (risk level 0)

**Usage:**
```typescript
const results = await invoke<FilesystemHealthResults>('scan_filesystem_health', undefined, 300000);
```

---

#### `scan_storage_recovery`

Scans for storage recovery opportunities (duplicates, large files, old downloads).

**Signature:**
```rust
pub async fn scan_storage_recovery(app_handle: tauri::AppHandle) -> Result<StorageRecoveryResults, String>
```

**Parameters:**
- `app_handle`: Tauri AppHandle (injected automatically)

**Return Type:** `StorageRecoveryResults`

```typescript
interface StorageRecoveryResults {
  duplicates: DuplicateGroup[];
  large_files: ScanItem[];
  old_downloads: ScanItem[];
  total_size: number;
  total_items: number;
  total_recoverable_size: number;
  total_duplicate_size: number;
  total_large_files_size: number;
  total_old_downloads_size: number;
}

interface DuplicateGroup {
  id: string;
  files: ScanItem[];
  total_size: number;
  group_size: number;
  hash?: string;
}
```

**Timeout:** 10 minutes (600 seconds)

**Description:**
- Finds duplicate files using content hashing
- Identifies large files (configurable threshold)
- Detects old downloads in Downloads directory
- Populates file_access table for old file detection
- Stores results in database for Dashboard display

**Usage:**
```typescript
const results = await invoke<StorageRecoveryResults>('scan_storage_recovery', undefined, 600000);
```

---

### Cleanup Operations

#### `clean_items`

Cleans selected items from scan results.

**Signature:**
```rust
pub async fn clean_items(
    item_ids: Vec<String>,
    item_paths: Vec<String>,
    use_trash: bool,
    retention_days: i64,
) -> Result<CleanResult, String>
```

**Parameters:**
```typescript
{
  item_ids: string[];
  item_paths: string[];
  use_trash: boolean;
  retention_days: number;
}
```

**Return Type:** `CleanResult`

```typescript
interface CleanResult {
  cleaned: number;
  failed: number;
  total_size: number;
}
```

**Timeout:** 5 minutes (300 seconds)

**Description:**
- Validates all paths (security checks)
- Moves items to trash (if `use_trash=true`) or permanently deletes
- Processes items sequentially
- Returns count of successful and failed operations

**Security:**
- Path validation prevents directory traversal
- System directory protection
- User home directory restriction

**Usage:**
```typescript
const result = await invoke<CleanResult>('clean_items', {
  item_ids: ['id1', 'id2'],
  item_paths: ['/home/user/file1', '/home/user/file2'],
  use_trash: true,
  retention_days: 3
}, 300000);
```

**Frontend Confirmation Required:**
- Type: 'info' or 'warning' (based on high-risk items)
- Shows item count, total size, and risk warnings

---

#### `clear_cache`

Clears user cache directories (~/.cache).

**Signature:**
```rust
pub async fn clear_cache() -> Result<CleanResult, String>
```

**Parameters:** None

**Return Type:** `CleanResult`

**Timeout:** No explicit timeout (should complete quickly)

**Description:**
- Clears safe cache subdirectories within user's home
- Includes: thumbnails, browser caches, development tool caches
- Only operates on user-specific, non-system-critical caches
- Generally safe but applications may need to rebuild caches

**Usage:**
```typescript
const result = await invoke<CleanResult>('clear_cache');
```

**Frontend Confirmation Required:**
- Type: 'warning'
- Message: "This will clear application caches and temporary files..."

---

#### `clean_packages`

Cleans package manager caches and orphaned packages.

**Signature:**
```rust
pub async fn clean_packages() -> Result<CleanResult, String>
```

**Parameters:** None

**Return Type:** `CleanResult`

**Timeout:** No explicit timeout

**Description:**
- Cleans apt/dpkg package caches
- Removes orphaned packages (with dependency checking)
- Cleans pip and npm caches
- Requires careful dependency analysis

**Usage:**
```typescript
const result = await invoke<CleanResult>('clean_packages');
```

**Frontend Confirmation Required:**
- Type: 'warning' or 'danger' (depending on packages)

---

#### `clear_logs`

Clears old user log files.

**Signature:**
```rust
pub async fn clear_logs() -> Result<CleanResult, String>
```

**Parameters:** None

**Return Type:** `CleanResult`

**Timeout:** No explicit timeout

**Description:**
- Clears old log files from user directories
- Only clears user logs, not system logs
- Safe operation (logs are typically regenerated)

**Usage:**
```typescript
const result = await invoke<CleanResult>('clear_logs');
```

---

### Trash Management

#### `get_trash_items`

Retrieves all items currently in trash.

**Signature:**
```rust
pub async fn get_trash_items() -> Result<TrashData, String>
```

**Parameters:** None

**Return Type:** `TrashData`

```typescript
interface TrashData {
  items: TrashItem[];
  total_size: number;
  total_items: number;
}

interface TrashItem {
  id: string;
  original_path: string;
  trash_path: string;
  deleted_at: string;
  expires_at: string;
  size: number;
  item_type: 'file' | 'directory';
  metadata?: {
    category: string;
    risk_level: number;
    reason: string;
  };
}
```

**Timeout:** 10 seconds

**Description:**
- Loads trash metadata from JSON file
- Calculates total size and item count
- Returns all trash items with metadata

**Usage:**
```typescript
const trash = await invoke<TrashData>('get_trash_items', undefined, 15000);
```

---

#### `restore_from_trash`

Restores an item from trash to its original location.

**Signature:**
```rust
pub async fn restore_from_trash(id: String) -> Result<(), String>
```

**Parameters:**
```typescript
{ id: string }
```

**Return Type:** `Result<(), String>`

**Timeout:** 10 seconds

**Description:**
- Finds item in trash by ID
- Validates trash file still exists
- Restores to original path
- Creates parent directories if needed
- Removes item from trash metadata

**Usage:**
```typescript
await invoke('restore_from_trash', { id: 'uuid-here' });
```

**Errors:**
- "Item not found in trash"
- "Item no longer exists in trash"
- "Cannot restore: path already exists"

---

#### `delete_from_trash`

Permanently deletes an item from trash.

**Signature:**
```rust
pub async fn delete_from_trash(id: String) -> Result<(), String>
```

**Parameters:**
```typescript
{ id: string }
```

**Return Type:** `Result<(), String>`

**Timeout:** 10 seconds

**Description:**
- Permanently deletes file/directory from trash
- Removes metadata entry
- Cannot be undone

**Usage:**
```typescript
await invoke('delete_from_trash', { id: 'uuid-here' });
```

**Frontend Confirmation Required:**
- Type: 'danger'
- Message: "This action cannot be undone..."

---

#### `empty_trash`

Permanently deletes all items in trash.

**Signature:**
```rust
pub async fn empty_trash() -> Result<usize, String>
```

**Parameters:** None

**Return Type:** `Result<number, string>` (number of items deleted)

**Timeout:** 30 seconds

**Description:**
- Permanently deletes all files in trash
- Clears trash metadata
- Returns count of items deleted
- Cannot be undone

**Usage:**
```typescript
const count = await invoke<number>('empty_trash', undefined, 30000);
```

**Frontend Confirmation Required:**
- Type: 'danger'
- Message: "Permanently delete all X items in trash?"

---

### Settings Management

#### `get_settings`

Retrieves application settings from database.

**Signature:**
```rust
pub async fn get_settings(app_handle: tauri::AppHandle) -> Result<AppSettings, String>
```

**Parameters:**
- `app_handle`: Tauri AppHandle (injected automatically)

**Return Type:** `AppSettings`

```typescript
interface AppSettings {
  trash: {
    retention_days: number;
    max_size_mb: number;
  };
  monitoring: {
    enabled: boolean;
    interval_hours: number;
  };
  notifications: {
    system: boolean;
    tray: boolean;
    in_app: boolean;
  };
  scan: {
    include_hidden: boolean;
    large_file_threshold_mb: number;
  };
  theme: 'light' | 'dark' | 'system';
}
```

**Timeout:** 5 seconds

**Description:**
- Reads settings from database
- Returns default settings if none exist
- Used on app start and settings page load

**Usage:**
```typescript
const settings = await invoke<AppSettings>('get_settings');
```

---

#### `save_settings`

Saves application settings to database.

**Signature:**
```rust
pub async fn save_settings(app_handle: tauri::AppHandle, settings: AppSettings) -> Result<(), String>
```

**Parameters:**
```typescript
{
  settings: AppSettings
}
```

**Return Type:** `Result<(), String>`

**Timeout:** 5 seconds

**Description:**
- Serializes settings to JSON
- Stores in database (INSERT OR REPLACE)
- Persists all application configuration

**Usage:**
```typescript
await invoke('save_settings', {
  settings: {
    trash: { retention_days: 7, max_size_mb: 2000 },
    monitoring: { enabled: true, interval_hours: 12 },
    notifications: { system: true, tray: true, in_app: true },
    scan: { include_hidden: false, large_file_threshold_mb: 100 },
    theme: 'dark'
  }
});
```

---

### DiskPulse Monitoring

#### `start_diskpulse_monitoring`

Starts background monitoring for DiskPulse features.

**Signature:**
```rust
pub async fn start_diskpulse_monitoring(app_handle: tauri::AppHandle) -> Result<(), String>
```

**Parameters:**
- `app_handle`: Tauri AppHandle (injected automatically)

**Return Type:** `Result<(), String>`

**Timeout:** No explicit timeout

**Description:**
- Starts disk usage monitoring (every 4 hours)
- Sets up cache directory file watcher
- Updates monitoring state in database
- Idempotent (returns OK if already running)

**Usage:**
```typescript
await invoke('start_diskpulse_monitoring');
```

---

#### `stop_diskpulse_monitoring`

Stops background monitoring.

**Signature:**
```rust
pub async fn stop_diskpulse_monitoring(app_handle: tauri::AppHandle) -> Result<(), String>
```

**Parameters:**
- `app_handle`: Tauri AppHandle (injected automatically)

**Return Type:** `Result<(), String>`

**Timeout:** No explicit timeout

**Description:**
- Stops disk monitoring task
- Stops cache watcher
- Updates monitoring state in database
- Idempotent (returns OK if not running)

**Usage:**
```typescript
await invoke('stop_diskpulse_monitoring');
```

---

#### `get_diskpulse_health`

Gets DiskPulse health status and projections.

**Signature:**
```rust
pub async fn get_diskpulse_health(app_handle: tauri::AppHandle) -> Result<DiskPulseHealth, String>
```

**Parameters:**
- `app_handle`: Tauri AppHandle (injected automatically)

**Return Type:** `DiskPulseHealth`

```typescript
interface DiskPulseHealth {
  disk_usage_percent: number;
  projected_days_until_full?: number;
  status_color: 'green' | 'yellow' | 'red';
  status_message: string;
}
```

**Timeout:** No explicit timeout (calls get_system_stats internally)

**Description:**
- Calculates current disk usage percentage
- Projects days until disk is full using historical data
- Determines status color based on usage thresholds:
  - Green: < 70%
  - Yellow: 70-85%
  - Red: > 85%

**Usage:**
```typescript
const health = await invoke<DiskPulseHealth>('get_diskpulse_health');
```

---

#### `get_old_files_summary`

Gets summary of old files based on access time.

**Signature:**
```rust
pub async fn get_old_files_summary(app_handle: tauri::AppHandle, days_cutoff: u32) -> Result<OldFilesSummary, String>
```

**Parameters:**
```typescript
{ days_cutoff: number }
```

**Return Type:** `OldFilesSummary`

```typescript
interface OldFilesSummary {
  total_files: number;
  total_size: number;
  cutoff_days: number;
}
```

**Timeout:** No explicit timeout

**Description:**
- Queries file_access table for files older than cutoff
- Returns count and total size
- Used for DiskPulse old files feature

**Usage:**
```typescript
const summary = await invoke<OldFilesSummary>('get_old_files_summary', { days_cutoff: 90 });
```

---

#### `get_recent_cache_events`

Gets recent cache growth events.

**Signature:**
```rust
pub async fn get_recent_cache_events(app_handle: tauri::AppHandle, limit: usize) -> Result<Vec<CacheEvent>, String>
```

**Parameters:**
```typescript
{ limit: number }
```

**Return Type:** `CacheEvent[]`

```typescript
interface CacheEvent {
  id: number;
  path: string;
  size_change: number;
  event_type: string;
  source: string | null;
  timestamp: number;
}
```

**Timeout:** No explicit timeout

**Description:**
- Retrieves recent cache events from database
- Ordered by timestamp (newest first)
- Limited by `limit` parameter
- Used for DiskPulse cache feed

**Usage:**
```typescript
const events = await invoke<CacheEvent[]>('get_recent_cache_events', { limit: 50 });
```

---

#### `get_cache_items`

Gets current cache items with sizes.

**Signature:**
```rust
pub async fn get_cache_items() -> Result<Vec<CacheItem>, String>
```

**Parameters:** None

**Return Type:** `CacheItem[]`

```typescript
interface CacheItem {
  name: string;
  size: number;
  category: string;
  can_clear: boolean;
}
```

**Timeout:** No explicit timeout

**Description:**
- Scans common cache directories
- Calculates sizes for each cache type
- Returns list of cache items
- Used for DiskPulse cache optimization

**Usage:**
```typescript
const items = await invoke<CacheItem[]>('get_cache_items');
```

---

#### `cleanup_old_files`

Cleans up old files based on access time.

**Signature:**
```rust
pub async fn cleanup_old_files(app_handle: tauri::AppHandle, days_cutoff: u32) -> Result<CleanResult, String>
```

**Parameters:**
```typescript
{ days_cutoff: number }
```

**Return Type:** `CleanResult`

**Timeout:** 5 minutes (300 seconds)

**Description:**
- Finds files older than cutoff days in file_access table
- Moves them to trash
- Returns cleanup results

**Usage:**
```typescript
const result = await invoke<CleanResult>('cleanup_old_files', { days_cutoff: 90 }, 300000);
```

---

### Cache Management

#### `get_cache_analytics`

Gets comprehensive cache analytics.

**Signature:**
```rust
pub async fn get_cache_analytics(app_handle: tauri::AppHandle) -> Result<CacheAnalytics, String>
```

**Parameters:**
- `app_handle`: Tauri AppHandle (injected automatically)

**Return Type:** `CacheAnalytics`

```typescript
interface CacheAnalytics {
  total_cache_size: number;
  cache_breakdown: CacheContributor[];
  growth_trend: CacheGrowthPoint[];
  recommended_limits: Record<string, number>;
}

interface CacheContributor {
  source: string;
  size: number;
  growth_rate: number;        // MB per day
  last_activity: number;      // Unix timestamp
  recommended_limit?: number;
}

interface CacheGrowthPoint {
  timestamp: number;
  total_size: number;
  sources: Record<string, number>;
}
```

**Timeout:** 30 seconds

**Description:**
- Analyzes cache events from last 30 days
- Calculates growth rates per cache source
- Generates growth trend (last 7 days)
- Provides recommended limits
- Used for Cache Optimization view

**Usage:**
```typescript
const analytics = await invoke<CacheAnalytics>('get_cache_analytics', undefined, 30000);
```

---

#### `clear_cache_item`

Clears a specific cache item by name.

**Signature:**
```rust
pub async fn clear_cache_item(item_name: String) -> Result<CleanResult, String>
```

**Parameters:**
```typescript
{ item_name: string }
```

**Return Type:** `CleanResult`

**Timeout:** Varies (depends on underlying operation)

**Description:**
- Clears specific cache item
- Supported items:
  - "Chrome temporary files" → `clear_cache()`
  - "Firefox cache" → `clear_cache()`
  - "Python packages cache" → `clean_packages()`
- Returns error for unknown items

**Usage:**
```typescript
const result = await invoke<CleanResult>('clear_cache_item', {
  item_name: 'Chrome temporary files'
});
```

---

#### `update_tray_icon`

Updates system tray icon (placeholder implementation).

**Signature:**
```rust
pub async fn update_tray_icon(_app_handle: tauri::AppHandle, status_color: String) -> Result<(), String>
```

**Parameters:**
```typescript
{ status_color: string }  // "green", "yellow", "red"
```

**Return Type:** `Result<(), String>`

**Timeout:** No explicit timeout

**Description:**
- Currently logs status change to application logs
- Tray icon display is handled by system tray setup in `main.rs`
- Future enhancement: Dynamic icon updates based on system status (requires platform-specific icon variants)

**Usage:**
```typescript
await invoke('update_tray_icon', { status_color: 'yellow' });
```

**Note:** This command currently logs status changes for debugging/monitoring purposes. The tray icon itself is statically configured during application startup. Full dynamic icon switching would require platform-specific implementation with multiple icon variants.

---

## Error Handling

All commands return `Result<T, String>`. Errors are:

1. **Timeout Errors**: Commands that exceed their timeout return error string
2. **Validation Errors**: Invalid paths, missing items, etc.
3. **System Errors**: File system errors, database errors, etc.

Frontend error handling pattern:

```typescript
try {
  const result = await invoke<Type>('command', args, timeout);
  // Handle success
} catch (error) {
  logger.error('Command failed:', error);
  notificationStore.error('Operation Failed', error.message);
}
```

## Security Considerations

### Path Validation

All file operations validate paths:
- Canonical path resolution
- Directory traversal protection (`..` blocked)
- System directory protection
- User home directory restriction

### Confirmation Dialogs

Critical operations require frontend confirmation:
- All cleanup operations
- Trash deletions
- Package removals

See [architecture.md](architecture.md) for detailed security architecture.

## Performance Notes

- **Timeout Values**: Based on operation complexity
  - Quick operations: 5-10 seconds
  - Medium operations: 30 seconds
  - Long operations: 5-15 minutes
- **Blocking Operations**: Long-running operations use `spawn_blocking` to avoid blocking async runtime
- **Database Queries**: Use prepared statements and indexes for performance

## Type Definitions

For complete TypeScript type definitions, see:
- `src/lib/stores/scanner.svelte.ts` - Scan-related types
- `src/lib/stores/settings.svelte.ts` - Settings types
- `src/lib/utils/tauri.ts` - Utility types
