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

### SmartCleanup.svelte

System analysis and cleanup recommendations interface.

**Location:** `src/lib/components/SmartCleanup.svelte`

**Props:** None

**State:**
- `recommendations`: `CleanupRecommendation | null` - Cleanup recommendations
- `loading`: `boolean` - Initial loading state
- `scanning`: `boolean` - Analysis in progress
- `cleaning`: `boolean` - Cleanup in progress
- `progress`: `number` - Progress percentage (0-100)
- `currentOperation`: `string` - Current operation description
- `previewData`: `CleanupPreview | null` - Preview dialog data
- `loadingPreview`: `boolean` - Preview loading state

**Features:**
- System analysis with usage patterns
- Cleanup recommendations with risk assessment
- Impact visualization (before/after sizes)
- Usage pattern analysis
- Preview mode for safe review
- Category-based organization (cache, packages, logs, filesystem, storage)
- Estimated time and space savings

**Usage:**
```svelte
<SmartCleanup />
```

**Dependencies:**
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `confirmation` store
- `notificationStore`
- `logger` utility
- `PreviewDialog` component

**Note:** Uses system analysis (not AI-powered) to generate recommendations based on usage patterns and file access times.

---

### EnhancedTreeView.svelte

Advanced file browser tree view with usage analysis and treemap visualization.

**Location:** `src/lib/components/EnhancedTreeView.svelte`

**Props:** None

**State:**
- `treeData`: `TreeNode[]` - Tree structure data
- `loading`: `boolean` - Loading state
- `searchQuery`: `string` - Search filter
- `viewMode`: `'tree' | 'list' | 'treemap'` - Display mode
- `sortBy`: `'name' | 'size' | 'date' | 'risk'` - Sort field
- `sortOrder`: `'asc' | 'desc'` - Sort direction
- `options`: `TreeViewOptions` - View options (rootPath, maxDepth, etc.)

**Features:**
- Hierarchical file tree display
- Multiple view modes (tree, list, treemap)
- Treemap visualization with:
  - Integrated TreemapLegend component
  - Integrated TreemapStatsPanel component
  - Fullscreen mode
  - Maximized container size
  - Pan and zoom functionality
  - All color modes (size, risk, usage, file type)
- Usage pattern analysis (frequent, occasional, rare, never)
- File size and date sorting
- Search and filter functionality
- Risk level indicators
- Expandable/collapsible directories
- File selection
- Color mode selection for treemap (size, risk, usage, filetype)
- Real-time statistics

**Usage:**
```svelte
<EnhancedTreeView />
```

**Dependencies:**
- `TreeNode`, `TreemapView`, `TreemapLegend`, `TreemapStatsPanel` components
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `notificationStore`
- `logger` utility

---

### TreemapView.svelte

Enhanced treemap visualization component for hierarchical file system data with pan, zoom, and improved interactivity.

**Location:** `src/lib/components/TreemapView.svelte`

**Props:**
- `nodes`: `GeneratedTreeNode[]` - Array of tree nodes to visualize
- `onNodeClick?`: `(node: GeneratedTreeNode) => void` - Click handler
- `onNodeHover?`: `(node: GeneratedTreeNode | null) => void` - Hover handler
- `colorMode?`: `'size' | 'risk' | 'usage' | 'filetype'` - Color encoding mode (default: 'size')
- `minNodeSize?`: `number` - Minimum node size in pixels (default: 8)
- `padding?`: `number` - Padding between nodes (default: 2)

**State:**
- `containerRef`: `HTMLDivElement | null` - Container reference
- `svgRef`: `SVGSVGElement | null` - SVG element reference
- `hoveredNode`: `GeneratedTreeNode | null` - Currently hovered node
- `tooltipPosition`: `{ x: number; y: number }` - Tooltip position
- `zoomStack`: `GeneratedTreeNode[]` - Stack of zoomed nodes
- `isDark`: `boolean` - Dark mode state
- `containerSize`: `{ width: number; height: number }` - Container dimensions
- `isPanning`: `boolean` - Pan state
- `panOffset`: `{ x: number; y: number }` - Pan offset
- `zoomLevel`: `number` - Current zoom level

**Features:**
- D3.js treemap layout with squarified tiling
- Interactive hover with enhanced tooltips (size percentage, file type, last modified, children count)
- Click to select nodes
- Double-click to zoom into directories
- Pan functionality (drag to move view)
- Mouse wheel zoom (0.5x to 5x)
- Breadcrumb navigation for zoomed views
- Zoom level indicator
- Enhanced labels with text shadows for readability
- Percentage labels on rectangles >120px
- Improved font sizing algorithm (10-14px)
- Label threshold increased to 100px (from 60px)
- Theme-aware colors
- Responsive layout
- Smooth transitions (200ms)
- Better visual hierarchy (stronger borders, depth indicators)
- Keyboard navigation support (Enter/Space to select)
- ARIA labels for accessibility
- Minimum size filtering (hides nodes smaller than minNodeSize)

**Usage:**
```svelte
<TreemapView
	nodes={sortedNodes}
	onNodeClick={(node) => handleClick(node)}
	colorMode="size"
	minNodeSize={8}
	padding={2}
/>
```

**Dependencies:**
- `treemap-utils` for layout and color calculations
- `file-types` for file type detection
- `formatBytes` from `$lib/utils/tauri`
- D3.js hierarchy library

**Technical Details:**
- Uses d3.treemapSquarify for layout algorithm
- Sorts nodes by descending size for better visual organization
- Filters nodes smaller than minNodeSize (8px default) to improve performance
- Debounced resize handler (150ms) for smooth resizing
- Progressive rendering for large datasets
- SVG transform for pan/zoom operations

---

### StartupManager.svelte

Startup program management interface.

**Location:** `src/lib/components/StartupManager.svelte`

**Props:** None

**State:**
- `programs`: `StartupProgram[]` - List of startup programs
- `loading`: `boolean` - Loading state
- `toggling`: `Set<string>` - Programs currently being toggled

**Features:**
- Lists all startup programs from XDG autostart and systemd
- Shows program name, description, location, and impact
- Enable/disable startup programs
- Confirmation dialogs for toggling
- Impact level indicators (low, medium, high)
- Location badges (xdg_autostart, systemd_user)

**Usage:**
```svelte
<StartupManager />
```

**Dependencies:**
- `invoke` from `$lib/utils/tauri`
- `confirmation` store
- `notificationStore`
- `logger` utility
- `LoadingSpinner` component

---

### PreviewDialog.svelte

Dialog component for previewing cleanup items before execution.

**Location:** `src/lib/components/PreviewDialog.svelte`

**Props:**
```typescript
interface PreviewDialogProps {
  preview: CleanupPreview | null;
  onConfirm: (selectedItems: PreviewItem[]) => Promise<void>;
}
```

**State:**
- `selectedItems`: `Set<string>` - Selected item IDs
- `expandedCategories`: `Set<string>` - Expanded category names

**Features:**
- Category-based item organization (cache, logs, filesystem, storage)
- Expandable/collapsible categories
- Individual item selection
- Select all / deselect all per category
- Total size calculation for selected items
- Confirmation button with selected count

**Usage:**
```svelte
<PreviewDialog
  bind:preview={previewData}
  bind:onConfirm={handlePreviewConfirm}
/>
```

**Dependencies:**
- `formatBytes` from `$lib/utils/tauri`
- `notificationStore`

---

### EnhancedTreeView.svelte

Advanced file browser with tree, list, and treemap views, integrated with legend and statistics panels.

**Location:** `src/lib/components/EnhancedTreeView.svelte`

**Props:** None (uses stores and Tauri commands)

**State:**
- `treeData`: `TreeNode[]` - Tree data
- `loading`: `boolean` - Loading state
- `searchQuery`: `string` - Search query
- `viewMode`: `'tree' | 'list' | 'treemap'` - Current view mode
- `sortBy`: `'name' | 'size' | 'date' | 'risk'` - Sort field
- `sortOrder`: `'asc' | 'desc'` - Sort order
- `treemapColorMode`: `'size' | 'risk' | 'usage' | 'filetype'` - Treemap color mode
- `isFullscreen`: `boolean` - Fullscreen mode state
- `options`: `TreeViewOptions` - View options

**Features:**
- Three view modes: Tree, List, Treemap
- Search and filter functionality
- Multiple sort options
- Treemap visualization with:
  - Integrated TreemapLegend
  - Integrated TreemapStatsPanel
  - Fullscreen mode
  - Maximized container size
  - All color modes (size, risk, usage, file type)
- Real-time statistics
- Selection management
- Usage pattern analysis

**Usage:**
```svelte
<EnhancedTreeView />
```

**Dependencies:**
- `TreeNode`, `TreemapView`, `TreemapLegend`, `TreemapStatsPanel` components
- `invoke`, `formatBytes` from `$lib/utils/tauri`
- `notificationStore`
- `logger` utility

---

### TreeView.svelte

**Note:** This component does not exist. Use `EnhancedTreeView.svelte` instead.

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

## Chart Components (`charts/`)

Chart components use Chart.js via svelte5-chartjs for data visualization. All charts support dark mode and theme-aware colors.

### CPUUsageChart.svelte

Line chart for displaying CPU usage over time.

**Location:** `src/lib/components/charts/CPUUsageChart.svelte`

**Props:**
```typescript
interface CPUUsageChartProps {
  data: number[];      // Array of CPU usage percentages
  color?: string;      // Optional custom color
  height?: number;     // Chart height in pixels (default: 40)
  width?: string;      // Chart width (default: '100%')
}
```

**Features:**
- Smooth line chart with area fill
- Theme-aware colors
- No animation (updates frequently)
- Auto-adjusts to dark/light theme

**Usage:**
```svelte
<CPUUsageChart data={cpuHistory} height={60} />
```

---

### MemoryUsageChart.svelte

Line chart for displaying memory usage over time.

**Location:** `src/lib/components/charts/MemoryUsageChart.svelte`

**Props:**
```typescript
interface MemoryUsageChartProps {
  data: number[];      // Array of memory usage values
  color?: string;
  height?: number;
  width?: string;
}
```

**Features:**
- Same as CPUUsageChart but for memory data
- Used in SystemHealthMonitor

**Usage:**
```svelte
<MemoryUsageChart data={memoryHistory} />
```

---

### NetworkTrafficChart.svelte

Line chart for displaying network traffic (upload/download) over time.

**Location:** `src/lib/components/charts/NetworkTrafficChart.svelte`

**Props:**
```typescript
interface NetworkTrafficChartProps {
  data: number[];      // Array of network speed values
  color?: string;
  height?: number;
  width?: string;
}
```

**Features:**
- Displays network traffic trends
- Used in SystemHealthMonitor

**Usage:**
```svelte
<NetworkTrafficChart data={networkHistory} />
```

---

### DiskIOChart.svelte

Line chart for displaying disk I/O operations over time.

**Location:** `src/lib/components/charts/DiskIOChart.svelte`

**Props:**
```typescript
interface DiskIOChartProps {
  data: number[];      // Array of disk I/O values
  color?: string;
  height?: number;
  width?: string;
}
```

**Features:**
- Shows disk read/write activity
- Used in SystemHealthMonitor

**Usage:**
```svelte
<DiskIOChart data={diskIOHistory} />
```

---

### TemperatureChart.svelte

Line chart for displaying temperature readings over time.

**Location:** `src/lib/components/charts/TemperatureChart.svelte`

**Props:**
```typescript
interface TemperatureChartProps {
  data: number[];      // Array of temperature values in Celsius
  color?: string;
  height?: number;
  width?: string;
}
```

**Features:**
- Displays temperature trends
- No animation (updates frequently)
- Used in SystemHealthMonitor

**Usage:**
```svelte
<TemperatureChart data={temperatureHistory} />
```

---

### StorageDonutChart.svelte

Donut chart for displaying storage usage breakdown.

**Location:** `src/lib/components/charts/StorageDonutChart.svelte`

**Props:**
```typescript
interface StorageDonutChartProps {
  data: number[];      // Array of storage segment values
  color?: string;
  height?: number;
  width?: string;
}
```

**Features:**
- Visual storage breakdown
- Used in Dashboard and storage views

**Usage:**
```svelte
<StorageDonutChart data={storageSegments} />
```

---

### SparklineChart.svelte

Minimal sparkline chart for compact data visualization.

**Location:** `src/lib/components/charts/SparklineChart.svelte`

**Props:**
```typescript
interface SparklineChartProps {
  data: number[];
  color?: string;
  height?: number;
  width?: string;
}
```

**Features:**
- Minimal design for inline use
- Compact visualization
- Theme-aware

**Usage:**
```svelte
<SparklineChart data={trendData} height={20} />
```

---

### PerformanceGauge.svelte

Gauge/donut chart for displaying performance metrics (0-100%).

**Location:** `src/lib/components/charts/PerformanceGauge.svelte`

**Props:**
```typescript
interface PerformanceGaugeProps {
  value: number;       // 0-100 percentage
  size?: number;       // Gauge size in pixels (default: 200)
  showLabel?: boolean; // Show percentage label (default: true)
}
```

**Features:**
- 3/4 circle gauge design
- Color-coded by value (green/yellow/orange/red)
- Animated transitions
- Used for CPU, memory, disk usage displays

**Usage:**
```svelte
<PerformanceGauge value={cpuUsage} size={150} />
```

---

## Health Monitor Components (`health/`)

Health monitor components display real-time system health metrics. Used within SystemHealthMonitor component.

### CpuMonitor.svelte

CPU usage and core monitoring display.

**Location:** `src/lib/components/health/CpuMonitor.svelte`

**Props:**
```typescript
interface CpuMonitorProps {
  cpuUsage: number;        // Overall CPU usage percentage
  cpuFrequency: number;    // CPU frequency in MHz
  cpuCores: number;       // Number of CPU cores
  coreUsages: number[];   // Per-core usage percentages
  temperature?: number;    // Optional CPU temperature
}
```

**Features:**
- Overall CPU usage with progress bar
- Per-core usage visualization
- CPU frequency display
- Optional temperature display

**Usage:**
```svelte
<CpuMonitor
  cpuUsage={health.cpu_usage}
  cpuFrequency={health.cpu_frequency}
  cpuCores={health.cpu_cores}
  coreUsages={health.core_usages}
  temperature={health.temperatures.cpu}
/>
```

---

### MemoryMonitor.svelte

Memory usage monitoring display.

**Location:** `src/lib/components/health/MemoryMonitor.svelte`

**Props:**
```typescript
interface MemoryMonitorProps {
  totalMemory: number;    // Total memory in bytes
  usedMemory: number;     // Used memory in bytes
  availableMemory: number; // Available memory in bytes
  swapTotal?: number;     // Optional swap total
  swapUsed?: number;      // Optional swap used
}
```

**Features:**
- Memory usage percentage
- Used/available/total display
- Optional swap information
- Progress bar visualization

**Usage:**
```svelte
<MemoryMonitor
  totalMemory={health.total_memory}
  usedMemory={health.used_memory}
  availableMemory={health.available_memory}
  swapTotal={health.swap_total}
  swapUsed={health.swap_used}
/>
```

---

### GpuMonitor.svelte

GPU usage and VRAM monitoring display.

**Location:** `src/lib/components/health/GpuMonitor.svelte`

**Props:**
```typescript
interface GpuMonitorProps {
  gpuInfo?: GpuInfo;  // Optional GPU information
}

interface GpuInfo {
  name: string;
  usage: number;           // GPU usage percentage
  memory_used: number;    // VRAM used in bytes
  memory_total: number;    // Total VRAM in bytes
  temperature?: number;    // Optional GPU temperature
}
```

**Features:**
- GPU name and usage
- VRAM usage visualization
- Temperature display (if available)
- Only displays if GPU is available

**Usage:**
```svelte
<GpuMonitor gpuInfo={health.gpu_info} />
```

---

### NetworkMonitor.svelte

Network interface and connection monitoring.

**Location:** `src/lib/components/health/NetworkMonitor.svelte`

**Props:**
```typescript
interface NetworkMonitorProps {
  networkUp: number;              // Upload speed (bytes/sec)
  networkDown: number;            // Download speed (bytes/sec)
  networkInterfaces: NetworkInterfaceInfo[];
  activeConnections: NetworkConnection[];
}
```

**Features:**
- Upload/download speed display
- Network interface statistics
- Active connection list
- Connection process association

**Usage:**
```svelte
<NetworkMonitor
  networkUp={health.network_up}
  networkDown={health.network_down}
  networkInterfaces={health.network_interfaces}
  activeConnections={health.active_connections}
/>
```

---

### DiskMonitor.svelte

Disk I/O performance monitoring.

**Location:** `src/lib/components/health/DiskMonitor.svelte`

**Props:**
```typescript
interface DiskMonitorProps {
  diskReadBytes: number;   // Read speed (bytes/sec)
  diskWriteBytes: number;  // Write speed (bytes/sec)
  diskReadOps: number;     // Read IOPS
  diskWriteOps: number;    // Write IOPS
}
```

**Features:**
- Read/write speed display
- IOPS (Input/Output Operations Per Second)
- Real-time disk activity

**Usage:**
```svelte
<DiskMonitor
  diskReadBytes={health.disk_read_bytes}
  diskWriteBytes={health.disk_write_bytes}
  diskReadOps={health.disk_read_ops}
  diskWriteOps={health.disk_write_ops}
/>
```

---

### TemperatureMonitor.svelte

Temperature monitoring with status indicators.

**Location:** `src/lib/components/health/TemperatureMonitor.svelte`

**Props:**
```typescript
interface TemperatureMonitorProps {
  temperatures: {
    cpu: number;
    cpu_sensors: number;
    system: number;
    gpu?: number;
  };
}
```

**Features:**
- CPU temperature (primary from sensors, fallback to thermal zone)
- System temperature
- Optional GPU temperature
- Temperature status indicators (Excellent, Safe, Normal, Warm, Hot, Very Hot, Critical)
- Color-coded by temperature range
- Modern CPU temperature guidelines

**Usage:**
```svelte
<TemperatureMonitor temperatures={health.temperatures} />
```

---

### BatteryMonitor.svelte

Battery status monitoring (laptops only).

**Location:** `src/lib/components/health/BatteryMonitor.svelte`

**Props:**
```typescript
interface BatteryMonitorProps {
  batteryInfo?: BatteryInfo;
}

interface BatteryInfo {
  percentage: number;
  is_charging: boolean;
  time_to_full?: number;    // seconds
  time_to_empty?: number;   // seconds
  power_consumption?: number; // watts
}
```

**Features:**
- Battery percentage with progress bar
- Charging/discharging status
- Time to full/empty estimates
- Only displays if battery is available

**Usage:**
```svelte
<BatteryMonitor batteryInfo={health.battery_info} />
```

---

### ProcessMonitor.svelte

Top processes by resource usage display.

**Location:** `src/lib/components/health/ProcessMonitor.svelte`

**Props:**
```typescript
interface ProcessMonitorProps {
  topProcesses: ProcessInfo[];
}

interface ProcessInfo {
  pid: number;
  name: string;
  cpu_usage: number;      // percentage
  memory_usage: number;   // bytes
  status: string;
  user_id?: number;
}
```

**Features:**
- Lists top resource-consuming processes
- CPU and memory usage per process
- Process name and PID
- Sorted by resource usage

**Usage:**
```svelte
<ProcessMonitor topProcesses={health.top_processes} />
```

---

### PerformanceSummary.svelte

Summary card displaying key performance metrics.

**Location:** `src/lib/components/health/PerformanceSummary.svelte`

**Props:**
```typescript
interface PerformanceSummaryProps {
  cpuUsage: number;
  memoryUsage: number;
  diskUsage?: number;
  networkUp?: number;
  networkDown?: number;
}
```

**Features:**
- Compact summary of key metrics
- Color-coded status indicators
- Quick overview for dashboard

**Usage:**
```svelte
<PerformanceSummary
  cpuUsage={health.cpu_usage}
  memoryUsage={(health.used_memory / health.total_memory) * 100}
/>
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
