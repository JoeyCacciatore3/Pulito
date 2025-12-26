<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke, formatBytes } from '$lib/utils/tauri';
	import { confirmation } from '$lib/stores/confirmation.svelte';
	import { getStatusColor, getStatusIcon } from '$lib/utils/color-utils';
	import { notifyCleanupSuccess, notifyOperationError, notifyPartialSuccess } from '$lib/utils/notification-helpers';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte';
	import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
	import type { DiskPulseHealth, OldFilesSummary, CacheEvent, CacheItem, SystemStats } from '$lib/generated/types';

	let health = $state<DiskPulseHealth | null>(null);
	let oldFiles = $state<OldFilesSummary | null>(null);
	let cacheEvents = $state<CacheEvent[]>([]);
	let cacheItems = $state<CacheItem[]>([]);
	let systemStats = $state<SystemStats | null>(null);
	let sliderValue = $state(30); // Default 30 days
	let loading = $state(true);
	let scanningFilesystem = $state(false); // Tracks filesystem scanning progress
	let needsInitialScan = $state(false);   // True when no scan data exists yet

	async function loadDiskPulseData() {
		try {
		// Load all data in parallel with appropriate timeouts
		const [healthData, oldFilesData, cacheEventsData, cacheItemsData, statsData] = await Promise.all([
			invoke<DiskPulseHealth>('get_diskpulse_health', undefined, 15000), // 15s timeout
			invoke<OldFilesSummary>('get_old_files_summary', { daysCutoff: sliderValue }, 30000), // 30s timeout
			invoke<CacheEvent[]>('get_recent_cache_events', { limit: 10 }, 15000), // 15s timeout
			invoke<CacheItem[]>('get_cache_items', undefined, 30000), // 30s timeout
			invoke<SystemStats>('get_system_stats', undefined, 15000) // 15s timeout for disk space
		]);

			health = healthData;
			oldFiles = oldFilesData;
			cacheEvents = cacheEventsData;
			cacheItems = cacheItemsData;
			systemStats = statsData;

			// Update tray icon based on health status
			if (health) {
				try {
					await invoke('update_tray_icon', { statusColor: health.status_color });
				} catch (e) {
					// Tray icon updates may not be supported on all platforms
					logger.debug('Tray icon update not supported', { component: 'DiskPulse', action: 'update_tray_icon' }, e);
				}
			}

			// Check if we need an initial filesystem scan
			if (!oldFiles || oldFiles.total_files === 0) {
				needsInitialScan = true;
			}
		} catch (e) {
			logger.error('Failed to load DiskPulse data', { component: 'DiskPulse', action: 'load_data' }, e);
			notifyOperationError('Load Data', 'Could not load disk monitoring information');
		} finally {
			loading = false;
		}
	}

	async function handleSliderChange() {
		try {
			// 30-second timeout matches other old files calls
			oldFiles = await invoke<OldFilesSummary>('get_old_files_summary', { daysCutoff: sliderValue }, 30000);
		} catch (e) {
			logger.error('Failed to update old files summary', { component: 'DiskPulse', action: 'update_old_files_summary' }, e);
			notificationStore.warning('Update Failed', 'Could not update old files summary. Please try again.');
		}
	}

	/**
	 * Performs a filesystem scan to populate the file_access database table.
	 * This is required before DiskPulse can detect unused files since the
	 * old files detection depends on scan data.
	 *
	 * Uses dedicated scan_for_old_files command optimized for finding unused files.
	 */
	async function performFilesystemScan() {
		try {
			scanningFilesystem = true;
			needsInitialScan = false;

			logger.info('Starting filesystem scan for old files detection', { component: 'DiskPulse', action: 'scan_filesystem' });

			// Use dedicated command that populates file_access table
			await invoke('scan_for_old_files', undefined, 600000); // 10 minute timeout

			logger.info('Filesystem scan completed, reloading data', { component: 'DiskPulse', action: 'scan_completed' });

			// Reload the data after scanning to show results
			await loadDiskPulseData();

			notifyCleanupSuccess('Filesystem Scanned', 'Successfully scanned your system for unused files.');

		} catch (error) {
			logger.error('Filesystem scan failed', { component: 'DiskPulse', action: 'scan_filesystem' }, error);
			const errorMessage = error instanceof Error && error.message.includes('timed out')
				? 'Filesystem scan timed out. The scan took too long to complete.'
				: 'Failed to scan filesystem for unused files.';
			notifyOperationError('Filesystem Scan', errorMessage);
			needsInitialScan = true; // Reset flag so user can retry
		} finally {
			scanningFilesystem = false;
		}
	}

	async function cleanupOldFiles() {
		if (!oldFiles || oldFiles.total_files === 0) return;

		const confirmed = await confirmation.show({
			title: 'Clean Old Files',
			message: `Remove ${oldFiles.total_files} files older than ${oldFiles.cutoff_days} days (${formatBytes(oldFiles.total_size)})? This action moves files to trash.`,
			confirmText: 'Clean Old Files',
			cancelText: 'Cancel',
			type: 'warning'
		});

		if (!confirmed) return;

		try {
			const result = await invoke<{ cleaned: number; failed: number; total_size: number }>('cleanup_old_files', { daysCutoff: sliderValue });
			await loadDiskPulseData(); // Refresh data

			if (result.failed > 0) {
				notifyPartialSuccess('Cleanup', result.cleaned, result.failed);
			} else {
				notifyCleanupSuccess('Cleanup', `Removed ${result.cleaned} old files`);
			}
		} catch (e) {
			logger.error('Failed to cleanup old files', { component: 'DiskPulse', action: 'cleanup_old_files', operation: 'cleanup_old_files' }, e);
			notifyOperationError('Cleanup', 'Could not remove old files');
		}
	}

	async function clearCacheItem(item: CacheItem) {
		const confirmed = await confirmation.show({
			title: 'Clear Cache',
			message: `Clear ${formatBytes(item.size)} from ${item.name}? This is generally safe but may require applications to rebuild caches.`,
			confirmText: 'Clear Cache',
			cancelText: 'Cancel',
			type: 'info'
		});

		if (!confirmed) return;

		try {
			const result = await invoke<{ cleaned: number; failed: number; total_size: number }>('clear_cache_item', { itemName: item.name });
			await loadDiskPulseData(); // Refresh data

			if (result.failed > 0) {
				notifyPartialSuccess('Cache Clear', result.cleaned, result.failed);
			} else {
				notifyCleanupSuccess('Cache Clear', `Freed ${formatBytes(result.total_size)}`);
			}
		} catch (e) {
			logger.error('Failed to clear cache', { component: 'DiskPulse', action: 'clear_cache', operation: 'clear_cache' }, e);
			notifyOperationError('Cache Clear', 'Could not clear cache');
		}
	}


	// Auto-refresh data every 30 seconds and start background monitoring
	onMount(() => {
		let interval: ReturnType<typeof setInterval>;

		(async () => {
			// Start background monitoring for cache events and file access tracking
			try {
				await invoke('start_diskpulse_monitoring');
				logger.info('DiskPulse background monitoring started', { component: 'DiskPulse', action: 'start_monitoring' });
			} catch (e) {
				logger.error('Failed to start background monitoring', { component: 'DiskPulse', action: 'start_monitoring', operation: 'start_background_monitoring' }, e);
			}

			loadDiskPulseData();

			interval = setInterval(() => {
				loadDiskPulseData();
			}, 30000);
		})();

		return () => {
			if (interval) clearInterval(interval);
		};
	});

	// Watch slider changes
	$effect(() => {
		handleSliderChange();
	});
</script>

<div class="diskpulse-container max-w-4xl mx-auto p-6 space-y-6">
	<!-- Health Bar -->
	<div class="health-card card p-6">
		<div class="flex items-center justify-between mb-4">
			<div class="flex items-center gap-3">
				<div class="w-4 h-4 rounded-full" style="background-color: {getStatusColor(health?.status_color || 'gray')}"></div>
				<h2 class="text-xl font-semibold">Your Disk</h2>
			</div>
			{#if health}
				<span class="text-2xl">{getStatusIcon(health.status_color)}</span>
			{/if}
		</div>

		{#if loading}
			<LoadingSpinner message="Loading disk health..." />
		{:else if health}
			<div class="space-y-4">
				<!-- Usage Bar -->
				<div class="space-y-2">
					<div class="flex justify-between text-sm">
						<span>{Math.round(health.disk_usage_percent)}% used</span>
						{#if systemStats}
							<span>{formatBytes(systemStats.used_disk_space)} / {formatBytes(systemStats.total_disk_space)}</span>
						{:else}
							<span>Loading...</span>
						{/if}
					</div>
					<ProgressBar
						percentage={health.disk_usage_percent}
						color="bg-primary-500"
						height="h-3"
					/>
				</div>

				<!-- Status Message -->
				<p class="text-center text-lg font-medium" style="color: {getStatusColor(health.status_color)}">
					{health.status_message}
				</p>

				{#if health.projected_days_until_full}
					<p class="text-center text-sm text-muted">
						At this rate, you'll have space for about {Math.round(health.projected_days_until_full)} more months.
					</p>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Old Files Timeline -->
	<div class="timeline-card card p-6">
		<h2 class="text-xl font-semibold mb-4">Old Stuff</h2>
		<p class="text-muted text-sm mb-4">Find files you haven't opened in a while</p>

		<div class="space-y-4">
			<!-- Slider -->
			<div class="space-y-2">
				<label for="age-slider" class="text-sm font-medium">
					Show me files I haven't opened since...
				</label>
				<div class="space-y-2">
					<input
						id="age-slider"
						type="range"
						min="7"
						max="365"
						step="7"
						bind:value={sliderValue}
						class="w-full slider"
					/>
					<div class="flex justify-between text-xs text-muted">
						<span>1 week</span>
						<span class="font-medium">{sliderValue} days ago</span>
						<span>1 year</span>
					</div>
				</div>
			</div>

			<!-- Results -->
			{#if scanningFilesystem}
				<div class="results-display p-4 bg-muted/20 rounded-lg">
					<div class="flex items-center justify-center py-8">
						<div class="text-center">
							<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4"></div>
							<p class="text-muted">Scanning filesystem for unused files...</p>
							<p class="text-sm text-muted mt-2">This may take a few minutes</p>
						</div>
					</div>
				</div>
			{:else if needsInitialScan}
				<div class="results-display p-4 bg-muted/20 rounded-lg border-2 border-dashed border-muted">
					<div class="text-center py-8">
						<div class="text-4xl mb-4">🔍</div>
						<h3 class="text-lg font-semibold mb-2">Scan Your System</h3>
						<p class="text-muted mb-4">
							To find unused files, we need to scan your filesystem first.
							This will help us identify files you haven't accessed recently.
						</p>
						<button
							class="btn btn-primary"
							onclick={performFilesystemScan}
							disabled={scanningFilesystem}
						>
							Start Scan
						</button>
						<p class="text-xs text-muted mt-3">
							Scan time depends on your system size. Typically takes 2-5 minutes.
						</p>
					</div>
				</div>
			{:else if oldFiles}
				<div class="results-display p-4 bg-muted/20 rounded-lg">
					<div class="flex items-center justify-between">
						<div>
							<div class="text-2xl font-bold">{oldFiles.total_files.toLocaleString()}</div>
							<div class="text-sm text-muted">files found</div>
						</div>
						<div class="text-right">
							<div class="text-xl font-bold">{formatBytes(oldFiles.total_size)}</div>
							<div class="text-sm text-muted">space reclaimable</div>
						</div>
					</div>

					{#if oldFiles.total_files > 0}
						<button
							class="btn btn-primary w-full mt-4"
							onclick={cleanupOldFiles}
						>
							Review These
						</button>
					{:else}
						<p class="text-center text-muted mt-4">No old files found in this time range</p>
					{/if}
				</div>
			{:else}
				<div class="results-display p-4 bg-muted/20 rounded-lg">
					<div class="flex items-center justify-center py-8">
						<div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary mx-auto mb-2"></div>
						<p class="text-muted">Loading...</p>
					</div>
				</div>
			{/if}
		</div>
	</div>

	<!-- Cache Feed -->
	<div class="cache-card card p-6">
		<h2 class="text-xl font-semibold mb-4">Junk Piling Up</h2>
		<p class="text-muted text-sm mb-4">Clear temporary files from your applications</p>

		{#if cacheItems.length > 0}
			<div class="space-y-3">
				{#each cacheItems as item}
					<div class="cache-item flex items-center justify-between p-3 bg-muted/10 rounded-lg">
						<div>
							<div class="font-medium">{item.name}</div>
							<div class="text-sm text-muted">{formatBytes(item.size)}</div>
						</div>
						<button
							class="btn btn-outline btn-sm"
							onclick={() => clearCacheItem(item)}
							disabled={!item.can_clear}
						>
							Clear
						</button>
					</div>
				{/each}
			</div>
		{:else}
			<p class="text-center text-muted py-8">Nothing to clean right now.</p>
		{/if}

		<!-- Recent Events -->
		{#if cacheEvents.length > 0}
			<div class="mt-6 pt-4 border-t">
				<h3 class="text-sm font-medium text-muted mb-3">Recent Activity</h3>
				<div class="space-y-2">
					{#each cacheEvents.slice(0, 3) as event}
						<div class="text-xs text-muted">
							{event.source || 'Unknown'} {event.event_type} {formatBytes(Math.abs(event.size_change))}
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.diskpulse-container {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	.slider {
		-webkit-appearance: none;
		appearance: none;
		height: 6px;
		border-radius: 3px;
		background: var(--color-border);
		outline: none;
	}

	.slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: var(--color-primary);
		cursor: pointer;
		border: 2px solid white;
		box-shadow: 0 2px 4px rgba(0,0,0,0.2);
	}

	.slider::-moz-range-thumb {
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: var(--color-primary);
		cursor: pointer;
		border: 2px solid white;
		box-shadow: 0 2px 4px rgba(0,0,0,0.2);
	}

	.results-display {
		animation: fadeIn 0.3s ease-in;
	}

	@keyframes fadeIn {
		from { opacity: 0; transform: translateY(10px); }
		to { opacity: 1; transform: translateY(0); }
	}
</style>
