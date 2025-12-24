# Component Library Documentation

## Overview

Pulito uses Svelte 5 with runes for all components. Components are organized in `src/lib/components/` with a `ui/` subdirectory for base UI components.

## Component Architecture

All components use Svelte 5 runes:
- `$state` for reactive state
- `$derived` for computed values
- `$effect` for side effects

## Main Components

### Dashboard.svelte

Main dashboard view displaying system overview and cleanup categories.

**Location:** `src/lib/components/Dashboard.svelte`

**Props:** None (uses stores and Tauri commands)

**State:**
- `stats`: `SystemStats | null` - System statistics
- `loading`: `boolean` - Loading state
- `categories`: `CleanupCategory[]` - Cleanup categories

**Features:**
- Displays disk usage and statistics
- Shows cleanup categories with estimated savings
- Refreshes stats every 30 seconds
- Validates stats data for reasonableness

**Usage:**
```svelte
<Dashboard />
```

**Dependencies:**
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `navigation` store
- `confirmation` store
- `logger` utility

---

### SystemHealthMonitor.svelte

Real-time system health monitoring display.

**Location:** `src/lib/components/SystemHealthMonitor.svelte`

**Props:** None

**State:**
- `healthData`: `SystemHealthData | null`
- `loading`: `boolean`
- `monitoring`: `boolean`
- `updateCount`: `number`

**Features:**
- Real-time CPU, memory, GPU, network, temperature monitoring
- Process list with resource usage
- Network connection tracking
- Battery status (if available)
- Disk I/O performance
- Load average charts
- Adaptive refresh rate (2-30 seconds)

**Usage:**
```svelte
<SystemHealthMonitor />
```

**Dependencies:**
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `getUsageColor`, `getUsageBgColor`, `getTempColor` from `$lib/utils/color-utils`
- `logger` utility

---

### FilesystemHealth.svelte

Filesystem health scan results and cleanup.

**Location:** `src/lib/components/FilesystemHealth.svelte`

**Props:** None

**State:**
- `results`: `FilesystemHealthResults | null`
- `loading`: `boolean`
- `selectedItems`: `Set<string>`

**Features:**
- Displays empty directories
- Shows broken symlinks
- Lists orphaned temp files
- Category-based organization
- Clean all safe items functionality

**Usage:**
```svelte
<FilesystemHealth />
```

**Dependencies:**
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `confirmation` store
- `notificationStore`
- `logger` utility

---

### StorageRecovery.svelte

Storage recovery scan results (duplicates, large files, old downloads).

**Location:** `src/lib/components/StorageRecovery.svelte`

**Props:** None

**State:**
- `results`: `StorageRecoveryResults | null`
- `loading`: `boolean`
- `expandedGroups`: `Set<string>` - Expanded duplicate groups
- `selectedLargeFiles`: `Set<string>`
- `selectedOldDownloads`: `Set<string>`

**Features:**
- Duplicate file detection with grouping
- Large file identification
- Old downloads detection
- Smart cleanup with selection
- Category breakdown

**Usage:**
```svelte
<StorageRecovery />
```

**Dependencies:**
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `confirmation` store
- `notificationStore`
- `logger` utility

---

### CacheOptimization.svelte

Cache analytics and optimization interface.

**Location:** `src/lib/components/CacheOptimization.svelte`

**Props:** None

**State:**
- `analytics`: `CacheAnalytics | null`
- `loading`: `boolean`

**Features:**
- Cache size breakdown by source
- Growth rate analysis
- Growth trend visualization
- Recommended limits
- Per-source cache clearing
- Clean all caches functionality

**Usage:**
```svelte
<CacheOptimization />
```

**Dependencies:**
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `confirmation` store
- `notificationStore`
- `logger` utility

---

### DiskPulse.svelte

Disk usage monitoring and old files management.

**Location:** `src/lib/components/DiskPulse.svelte`

**Props:** None

**State:**
- `health`: `DiskPulseHealth | null`
- `oldFiles`: `OldFilesSummary | null`
- `cacheEvents`: `CacheEvent[]`
- `cacheItems`: `CacheItem[]`
- `loading`: `boolean`
- `cutoffDays`: `number` - Days cutoff for old files

**Features:**
- Disk usage percentage with color coding
- Projected days until full
- Old files summary with slider
- Recent cache events feed
- Cache items list
- Cleanup old files functionality

**Usage:**
```svelte
<DiskPulse />
```

**Dependencies:**
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `confirmation` store
- `notificationStore`
- `logger` utility

---

### TreeView.svelte

File browser tree view with preview pane.

**Location:** `src/lib/components/TreeView.svelte`

**Props:** None (uses scanner store)

**State:**
- `expandedItems`: `Set<string>` - Expanded item IDs
- `selectedItem`: `ScanItem | null` - Item selected for preview

**Features:**
- Hierarchical file tree display
- Item expansion/collapse
- File preview pane (images)
- Risk level indicators
- Selection and cleanup
- Filter functionality

**Usage:**
```svelte
<TreeView />
```

**Dependencies:**
- `scanner` store
- `formatBytes`, `getRiskLevelInfo`, `invoke` from `$lib/utils/tauri`
- `confirmation` store
- `notificationStore`

---

### TrashView.svelte

Trash management interface.

**Location:** `src/lib/components/TrashView.svelte`

**Props:** None

**State:**
- `trashData`: `TrashData | null`
- `loading`: `boolean`

**Features:**
- List all trash items
- Restore items to original location
- Permanently delete items
- Empty trash functionality
- Item metadata display
- Time until expiry

**Usage:**
```svelte
<TrashView />
```

**Dependencies:**
- `invoke`, `formatBytes`, `formatRelativeTime` from `$lib/utils/tauri`
- `confirmation` store
- `logger` utility

---

### Settings.svelte

Application settings interface.

**Location:** `src/lib/components/Settings.svelte`

**Props:** None

**State:**
- `saving`: `boolean`
- `saved`: `boolean`
- `loading`: `boolean`

**Features:**
- Trash settings (retention days, max size)
- Monitoring settings (enabled, interval)
- Notification settings (system, tray, in-app)
- Scan settings (include hidden, large file threshold)
- Theme selection (light, dark, system)
- Auto-save on change

**Usage:**
```svelte
<Settings />
```

**Dependencies:**
- `settings` store
- `theme` store
- `invoke` from `$lib/utils/tauri`
- `logger` utility

---

### Header.svelte

Application header with navigation and scan trigger.

**Location:** `src/lib/components/Header.svelte`

**Props:** None

**State:**
- None (uses stores)

**Features:**
- Navigation menu
- Scan trigger button
- Scanner results summary
- Logo/branding

**Usage:**
```svelte
<Header />
```

**Dependencies:**
- `scanner` store
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `navigation` store

---

### Sidebar.svelte

Navigation sidebar with selected items summary.

**Location:** `src/lib/components/Sidebar.svelte`

**Props:** None

**State:**
- None (uses stores)

**Features:**
- Navigation links
- Selected items count and size
- Clean selected items button
- Active view indication

**Usage:**
```svelte
<Sidebar />
```

**Dependencies:**
- `navigation` store
- `scanner` store
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `confirmation` store
- `notificationStore`

---

### ConfirmationDialog.svelte

Glassmorphism confirmation dialog component.

**Location:** `src/lib/components/ConfirmationDialog.svelte`

**Props:**
```typescript
interface ConfirmationDialogProps {
  open: boolean;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  type?: 'info' | 'warning' | 'danger';
  size?: 'small' | 'medium' | 'large';
}
```

**Features:**
- Glassmorphism design
- Keyboard shortcuts (Enter, Escape)
- Backdrop click to close
- Type-based styling (info, warning, danger)
- Size variants

**Usage:**
```svelte
<ConfirmationDialog
  bind:open={dialogOpen}
  title="Confirm Action"
  message="Are you sure?"
  confirmText="Confirm"
  cancelText="Cancel"
  type="warning"
  on:confirm={handleConfirm}
  on:cancel={handleCancel}
/>
```

**Events:**
- `confirm` - Emitted when user confirms
- `cancel` - Emitted when user cancels

---

### NotificationToast.svelte

Toast notification component.

**Location:** `src/lib/components/NotificationToast.svelte`

**Props:**
```typescript
interface NotificationToastProps {
  notification: Notification;
  onClose: () => void;
}
```

**Features:**
- Auto-dismiss after timeout
- Type-based styling (success, error, warning, info)
- Close button
- Smooth animations

**Usage:**
Managed by `notificationStore` - not typically used directly.

---

## Base UI Components (`ui/`)

### InfoCard.svelte

Information card component for displaying stats/metrics.

**Location:** `src/lib/components/ui/InfoCard.svelte`

**Props:**
```typescript
interface InfoCardProps {
  title: string;
  value: string | number;
  icon?: string;
  color?: string;
  subtitle?: string;
}
```

**Usage:**
```svelte
<InfoCard
  title="Disk Usage"
  value="75%"
  icon="💾"
  color="blue"
  subtitle="500 GB / 1 TB"
/>
```

---

### LoadingSpinner.svelte

Loading spinner component.

**Location:** `src/lib/components/ui/LoadingSpinner.svelte`

**Props:** None (or optional size prop)

**Usage:**
```svelte
{#if loading}
  <LoadingSpinner />
{/if}
```

---

### ProgressBar.svelte

Progress bar component.

**Location:** `src/lib/components/ui/ProgressBar.svelte`

**Props:**
```typescript
interface ProgressBarProps {
  value: number;        // 0-100
  max?: number;         // Default: 100
  color?: string;       // Color class
  showLabel?: boolean;  // Show percentage label
}
```

**Usage:**
```svelte
<ProgressBar value={diskUsagePercent} color="blue" showLabel={true} />
```

---

## Component Patterns

### State Management

Components use Svelte 5 runes for reactive state:

```typescript
let data = $state<DataType | null>(null);
let loading = $state(true);

// Derived state
let displayValue = $derived(data?.value ?? 'N/A');

// Effects
$effect(() => {
  if (data) {
    console.log('Data updated:', data);
  }
});
```

### Store Integration

Components access stores directly:

```typescript
import { scanner } from '$lib/stores/scanner.svelte';

// Access store state
const items = scanner.results.items;
const selectedSize = scanner.selectedSize;
```

### Error Handling

Components handle errors consistently:

```typescript
try {
  const result = await invoke<Type>('command', args);
  // Update state
} catch (e) {
  logger.error('Operation failed:', e);
  notificationStore.error('Operation Failed', e.message);
}
```

### Confirmation Dialogs

Components use confirmation store for user confirmations:

```typescript
const confirmed = await confirmation.show({
  title: 'Confirm Action',
  message: 'Are you sure?',
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  type: 'warning'
});

if (!confirmed) return;
// Proceed with action
```

## Styling

All components use Tailwind CSS classes. The design system includes:

- **Colors**: Semantic color classes (safe, caution, danger, critical)
- **Spacing**: Consistent padding and margins
- **Typography**: Standardized font sizes and weights
- **Cards**: Consistent card styling with borders and shadows
- **Buttons**: Standardized button styles with variants

## Accessibility

Components include:
- ARIA labels where appropriate
- Keyboard navigation support
- Semantic HTML elements
- Focus management for dialogs
- Screen reader friendly text

## Testing

Components can be tested using:
- `@testing-library/svelte` for component testing
- Vitest for unit tests
- Mock Tauri commands (see test setup)

See [stores.md](stores.md) for state management documentation.
