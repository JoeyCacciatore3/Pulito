# State Management Documentation

## Overview

Pulito uses Svelte 5 runes for state management. Stores are implemented as rune-based modules with functional APIs, providing reactive state that integrates seamlessly with Svelte components.

## Store Architecture

All stores use Svelte 5 runes pattern:
- `$state` for reactive state variables
- Functional API for state manipulation
- No explicit subscription needed (rune reactivity handles updates)

## Store Modules

### confirmation.svelte.ts

Manages confirmation dialog state and display.

**Location:** `src/lib/stores/confirmation.svelte.ts`

**API:**
```typescript
export const confirmation = {
  show(options: ConfirmationOptions): Promise<boolean>;
  hide(): void;
};
```

**Types:**
```typescript
interface ConfirmationOptions {
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  type?: 'info' | 'warning' | 'danger';
  size?: 'small' | 'medium' | 'large';
}
```

**Usage:**
```typescript
import { confirmation } from '$lib/stores/confirmation.svelte';

const confirmed = await confirmation.show({
  title: 'Confirm Deletion',
  message: 'Are you sure you want to delete this item?',
  confirmText: 'Delete',
  cancelText: 'Cancel',
  type: 'danger'
});

if (confirmed) {
  // Proceed with deletion
}
```

**State:**
- Internal state managed by store
- Returns Promise that resolves when user confirms or cancels
- Used by ConfirmationDialog component

---

### navigation.svelte.ts

Manages navigation state and active view.

**Location:** `src/lib/stores/navigation.svelte.ts`

**API:**
```typescript
export const navigation = {
  navigate(route: string): void;
  get activeView(): string;
};
```

**Types:**
```typescript
type View = 'dashboard' | 'cleanup' | 'treeview' | 'filesystem-health' | 'trash' | 'settings' | 'startup' | 'system-health' | 'disk-pulse' | 'storage-recovery';
```

**Usage:**
```typescript
import { navigation } from '$lib/stores/navigation.svelte';

// Navigate to a view
navigation.set('filesystem-health');

// Check current view
if (navigation.activeView === 'dashboard') {
  // ...
}
```

**State:**
- `activeView`: Current active view identifier
- Updates trigger component re-renders automatically

---

### notifications.svelte.ts

Manages toast notifications.

**Location:** `src/lib/stores/notifications.svelte.ts`

**API:**
```typescript
export const notificationStore = {
  success(message: string, details?: string): void;
  error(message: string, details?: string): void;
  warning(message: string, details?: string): void;
  info(message: string, details?: string): void;
  get notifications(): Notification[];
  remove(id: string): void;
  clear(): void;
};
```

**Types:**
```typescript
interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  details?: string;
  timestamp: number;
}
```

**Usage:**
```typescript
import { notificationStore } from '$lib/stores/notifications.svelte';

// Show notifications
notificationStore.success('Operation completed', 'Files cleaned successfully');
notificationStore.error('Operation failed', error.message);
notificationStore.warning('Low disk space', 'Only 10% remaining');
notificationStore.info('Scan started', 'Scanning system...');

// Access current notifications
const currentNotifications = notificationStore.notifications;

// Remove notification
notificationStore.remove(notificationId);

// Clear all notifications
notificationStore.clear();
```

**State:**
- `notifications`: Array of active notifications
- Auto-dismiss after timeout (configurable)
- Maximum number of notifications displayed (configurable)

---

### scanner.svelte.ts

Manages scanner state and scan results.

**Location:** `src/lib/stores/scanner.svelte.ts`

**API:**
```typescript
export const scanner = {
  startScan(options: ScanOptions): Promise<void>;
  startFilesystemHealthScan(): Promise<void>;
  startStorageRecoveryScan(): Promise<void>;
  selectItem(id: string): void;
  deselectItem(id: string): void;
  selectAll(): void;
  deselectAll(): void;
  cleanSelected(useTrash: boolean): Promise<void>;

  // Getters
  get results(): ScanResults | null;
  get filesystemHealthResults(): FilesystemHealthResults | null;
  get storageRecoveryResults(): StorageRecoveryResults | null;
  get selectedItems(): ScanItem[];
  get selectedSize(): number;
  get isScanning(): boolean;
};
```

**Types:**
```typescript
interface ScanOptions {
  include_caches?: boolean;
  include_packages?: boolean;
  include_logs?: boolean;
  include_hidden?: boolean;
  large_file_threshold_mb?: number;
}
```

**Usage:**
```typescript
import { scanner } from '$lib/stores/scanner.svelte';

// Start a scan
await scanner.startScan({
  include_caches: true,
  include_packages: true,
  include_hidden: false
});

// Access results
const results = scanner.results;
const items = results.items;

// Select items
scanner.selectItem(itemId);
scanner.selectAll();

// Clean selected items
await scanner.cleanSelected(true); // Use trash

// Check scanning state
if (scanner.isScanning) {
  // Show loading indicator
}
```

**State:**
- `results`: General scan results
- `filesystemHealthResults`: Filesystem health scan results
- `storageRecoveryResults`: Storage recovery scan results
- `selectedItems`: Set of selected item IDs
- `isScanning`: Whether a scan is in progress
- `scanProgress`: Optional scan progress information

---

### settings.svelte.ts

Manages application settings state.

**Location:** `src/lib/stores/settings.svelte.ts`

**API:**
```typescript
export const settings = {
  get value(): AppSettings;
  update(partial: Partial<AppSettings>): void;
  updateTrash(partial: Partial<AppSettings['trash']>): void;
  updateMonitoring(partial: Partial<AppSettings['monitoring']>): void;
  updateNotifications(partial: Partial<AppSettings['notifications']>): void;
  updateScan(partial: Partial<AppSettings['scan']>): void;
  reset(): void;
  load(saved: AppSettings): void;
  save(): Promise<void>;
};
```

**Types:**
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

**Usage:**
```typescript
import { settings } from '$lib/stores/settings.svelte';

// Access settings
const currentSettings = settings.value;
const retentionDays = settings.value.trash.retention_days;

// Update settings (partial updates)
settings.updateTrash({ retention_days: 7 });
settings.updateMonitoring({ enabled: false });
settings.update({ theme: 'dark' });

// Save to backend
await settings.save();

// Reset to defaults
settings.reset();

// Load from backend (typically done on app start)
const savedSettings = await invoke<AppSettings>('get_settings');
settings.load(savedSettings);
```

**State:**
- `value`: Current settings object
- Auto-syncs with backend on save
- Provides typed accessors for nested settings

---

### theme.svelte.ts

Manages theme state (light/dark/system).

**Location:** `src/lib/stores/theme.svelte.ts`

**API:**
```typescript
export const theme = {
  get current(): 'light' | 'dark' | 'system';
  set current(value: 'light' | 'dark' | 'system'): void;
  get effectiveTheme(): 'light' | 'dark';
  toggle(): void;
  apply(): void;
};
```

**Usage:**
```typescript
import { theme } from '$lib/stores/theme.svelte';

// Get current theme
const current = theme.current; // 'light' | 'dark' | 'system'

// Get effective theme (resolves 'system' to actual theme)
const effective = theme.effectiveTheme; // 'light' | 'dark'

// Set theme
theme.current = 'dark';
theme.current = 'system';

// Toggle between light and dark
theme.toggle();

// Apply theme (updates DOM)
theme.apply();
```

**State:**
- `current`: Selected theme preference
- `effectiveTheme`: Resolved theme (if 'system', resolves to actual OS theme)
- Listens to system theme changes when set to 'system'
- Updates DOM classes automatically

---

### metrics-history.svelte.ts

Historical metrics store for time-series data tracking.

**Location:** `src/lib/stores/metrics-history.svelte.ts`

**Purpose:**
Stores time-series data for CPU, memory, network, temperature, and disk I/O metrics. Used by SystemHealthMonitor for historical trend visualization.

**API:**
```typescript
// Add data points
export function addCPUData(data: Omit<CPUDataPoint, 'timestamp'>): void;
export function addMemoryData(data: Omit<MemoryDataPoint, 'timestamp'>): void;
export function addNetworkData(data: Omit<NetworkDataPoint, 'timestamp'>): void;
export function addTemperatureData(data: Omit<TemperatureDataPoint, 'timestamp'>): void;
export function addDiskIOData(data: Omit<DiskIODataPoint, 'timestamp'>): void;

// Get history (filtered by time range)
export function getCPUHistory(range?: '1h' | '6h' | '24h' | 'all'): CPUDataPoint[];
export function getMemoryHistory(range?: '1h' | '6h' | '24h' | 'all'): MemoryDataPoint[];
export function getNetworkHistory(range?: '1h' | '6h' | '24h' | 'all'): NetworkDataPoint[];
export function getTemperatureHistory(range?: '1h' | '6h' | '24h' | 'all'): TemperatureDataPoint[];
export function getDiskIOHistory(range?: '1h' | '6h' | '24h' | 'all'): DiskIODataPoint[];

// Reactive getters (for use in components)
export function getCPUHistoryReactive(range?: '1h' | '6h' | '24h' | 'all'): CPUDataPoint[];
export function getMemoryHistoryReactive(range?: '1h' | '6h' | '24h' | 'all'): MemoryDataPoint[];
export function getNetworkHistoryReactive(range?: '1h' | '6h' | '24h' | 'all'): NetworkDataPoint[];
export function getTemperatureHistoryReactive(range?: '1h' | '6h' | '24h' | 'all'): TemperatureDataPoint[];
export function getDiskIOHistoryReactive(range?: '1h' | '6h' | '24h' | 'all'): DiskIODataPoint[];

// Utility functions
export function clearHistory(): void;
export function getLatestData(): {
  cpu: CPUDataPoint | null;
  memory: MemoryDataPoint | null;
  network: NetworkDataPoint | null;
  temperature: TemperatureDataPoint | null;
  diskIO: DiskIODataPoint | null;
};
```

**Types:**
```typescript
interface CPUDataPoint {
  timestamp: number;
  usage: number;
  coreUsages?: number[];
}

interface MemoryDataPoint {
  timestamp: number;
  usedMemory: number;
  totalMemory: number;
  swapUsed?: number;
  swapTotal?: number;
  cache?: number;
}

interface NetworkDataPoint {
  timestamp: number;
  upload: number;
  download: number;
}

interface TemperatureDataPoint {
  timestamp: number;
  cpu: number;
  gpu?: number;
  system: number;
}

interface DiskIODataPoint {
  timestamp: number;
  readBytes: number;
  writeBytes: number;
  readOps?: number;
  writeOps?: number;
}
```

**Data Retention:**
- Maximum retention: 24 hours
- Maximum points per metric: 500
- Automatic pruning of old data
- Automatic limiting when max points exceeded

**Usage:**
```typescript
import {
  addCPUData,
  getCPUHistory,
  getLatestData,
  clearHistory
} from '$lib/stores/metrics-history.svelte';

// Add a data point
addCPUData({
  usage: 45.5,
  coreUsages: [40, 50, 45, 47]
});

// Get history (last hour)
const cpuHistory = getCPUHistory('1h');

// Get latest data
const latest = getLatestData();
console.log(latest.cpu?.usage);

// Clear all history
clearHistory();
```

**Usage in Components:**
```typescript
// In SystemHealthMonitor component
import { getCPUHistoryReactive } from '$lib/stores/metrics-history.svelte';

// Use in $derived for reactivity
let cpuHistory = $derived(getCPUHistoryReactive('1h'));
```

**State:**
- Internal state managed by store (not directly accessible)
- Data points automatically timestamped on add
- Automatic pruning based on retention and max points
- Thread-safe (single-threaded JavaScript)

**Notes:**
- Timestamps are in milliseconds (Date.now())
- Data is stored in memory (not persisted)
- History is cleared on app restart
- Used primarily by SystemHealthMonitor for chart data

---

## Store Interaction Patterns

### Component-Specific Stores

Each component typically interacts with multiple stores:

```typescript
import { scanner } from '$lib/stores/scanner.svelte';
import { confirmation } from '$lib/stores/confirmation.svelte';
import { notificationStore } from '$lib/stores/notifications.svelte';

// Use in component
const items = scanner.selectedItems;
const confirmed = await confirmation.show({ ... });
notificationStore.success('Operation completed');
```

### Store-to-Store Communication

Stores can interact with each other:

```typescript
// Example: Settings store might trigger theme store update
import { theme } from '$lib/stores/theme.svelte';

settings.update({ theme: 'dark' });
theme.current = 'dark'; // Sync theme store
```

### State Persistence

- **Settings**: Persisted to database via Tauri commands
- **Scanner Results**: Stored in memory, refreshed on scan
- **Theme**: Stored in localStorage and synced with settings
- **Navigation**: In-memory only (based on route)
- **Notifications**: In-memory only (temporary)
- **Confirmation**: In-memory only (modal state)
- **Metrics History**: In-memory only (24-hour retention, max 500 points per metric)

## Reactive Updates

Stores use Svelte 5 runes, so components automatically re-render when store state changes:

```typescript
// In component
let selectedCount = $derived(scanner.selectedItems.length);
let totalSize = $derived(scanner.selectedSize);

// Component automatically updates when scanner.selectedItems changes
```

## Type Safety

All stores are fully typed with TypeScript:
- Store APIs have explicit return types
- State is typed with interfaces
- Type inference works with rune reactivity

## Best Practices

1. **Direct Access**: Access store properties directly (no `.subscribe()` needed)
2. **Derived State**: Use `$derived` for computed values from stores
3. **Side Effects**: Use `$effect` for side effects based on store changes
4. **Error Handling**: Wrap async store operations in try-catch
5. **State Updates**: Prefer store methods over direct state mutation
6. **Loading States**: Track loading states for async operations

## Migration from Svelte 4 Stores

If migrating from Svelte 4 stores:
- Remove `.subscribe()` calls
- Remove `$:` reactive statements (use `$derived` instead)
- Access store properties directly
- Use `$effect` instead of reactive statements for side effects
