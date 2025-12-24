<script lang="ts">
	import { scanner } from '$lib/stores/scanner.svelte';
	import type { ScanResults } from '$lib/generated/types';
	import { theme } from '$lib/stores/theme.svelte';
	import { invoke, formatBytes } from '$lib/utils/tauri';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import { listen } from '@tauri-apps/api/event';

	interface ScanProgressEvent {
		category: string;
		progress: number;
		message: string;
		items_found: number;
		current_size: number;
	}

	async function handleScan() {
		scanner.setScanning(true);
		scanner.setProgress(0);

		// Calculate timeout based on scan options (matches backend logic)
		// Comprehensive scans (caches + packages) get 15 minutes, basic scans get 10 minutes
		const isComprehensiveScan = true; // All options enabled
		const scanTimeout = isComprehensiveScan ? 900000 : 600000; // 15 min or 10 min

		// Set up progress event listener
		let unlisten: (() => void) | null = null;

		try {
			// Listen to scan progress events from backend
			unlisten = await listen<ScanProgressEvent>('scan-progress', (event) => {
				const progress = event.payload;
				scanner.setProgress(progress.progress);
				logger.debug('Scan progress update', {
					component: 'Header',
					action: 'scan_progress',
					category: progress.category,
					progress: progress.progress,
					message: progress.message
				});
			});

			// Timeout matches backend: 15 minutes for comprehensive scans, 10 minutes for basic
			const results = await invoke<ScanResults>('start_scan', {
				options: {
					include_caches: true,
					include_packages: true,
					include_large_files: true,
					include_logs: true
				}
			}, scanTimeout);

			// Clean up event listener
			if (unlisten) {
				unlisten();
				unlisten = null;
			}

			scanner.setProgress(100);
			scanner.setResults(results);

			// Check for partial failures
			if (results.failed_categories && results.failed_categories.length > 0) {
				const failedCategories = results.failed_categories.map(fc => fc.category).join(', ');
				notificationStore.warning(
					'Scan Complete with Warnings',
					`Found ${results.total_items} items totaling ${formatBytes(results.total_size)}. Some categories failed to scan: ${failedCategories}`
				);
			} else {
				notificationStore.success('Scan Complete', `Found ${results.total_items} items totaling ${formatBytes(results.total_size)}`);
			}
		} catch (e) {
			logger.error('Scan failed', { component: 'Header', action: 'handle_scan', operation: 'start_scan' }, e);

			// Clean up event listener on error
			if (unlisten) {
				unlisten();
				unlisten = null;
			}

			const errorMessage = e instanceof Error
				? (e.message.includes('timed out')
					? `Scan timed out after ${isComprehensiveScan ? '15' : '10'} minutes. The system scan took too long to complete. Try scanning with fewer options enabled or try again later.`
					: e.message)
				: 'System scan failed. Please try again.';
			notificationStore.error('Scan Failed', errorMessage);
		} finally {
			// Ensure event listener is cleaned up
			if (unlisten) {
				unlisten();
			}
			scanner.setScanning(false);
		}
	}
</script>

<header
	class="flex items-center justify-between px-6 py-3 border-b border-[var(--color-border)] bg-[var(--color-bg)]"
>
	<div class="flex items-center gap-4">
		<h1 class="text-lg font-semibold">Linux System Cleaner</h1>

		{#if scanner.results}
			<span class="text-sm text-[var(--color-text-secondary)]">
				{scanner.results.total_items} items · {formatBytes(scanner.results.total_size)} cleanable
			</span>
		{/if}
	</div>

	<div class="flex items-center gap-3">
		{#if scanner.isScanning}
			<div class="flex items-center gap-2 text-sm text-[var(--color-text-secondary)]">
				<div class="spinner"></div>
				<span>Scanning... {Math.round(scanner.progress)}%</span>
			</div>
		{:else}
			<button class="btn btn-primary" onclick={handleScan}>
				<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<circle cx="11" cy="11" r="8" />
					<path d="m21 21-4.35-4.35" />
				</svg>
				Scan System
			</button>
		{/if}

		<button
			class="btn btn-ghost p-2"
			onclick={() => theme.toggle()}
			aria-label="Toggle theme"
		>
			{#if theme.value === 'dark'}
				<svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<circle cx="12" cy="12" r="5" />
					<path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" />
				</svg>
			{:else}
				<svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
				</svg>
			{/if}
		</button>
	</div>
</header>

{#if scanner.isScanning}
	<div class="progress-bar h-1">
		<div
			class="progress-bar-fill bg-primary-500"
			style="width: {scanner.progress}%"
		></div>
	</div>
{/if}
