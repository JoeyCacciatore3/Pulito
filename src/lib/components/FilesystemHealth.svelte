<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke, formatBytes } from '$lib/utils/tauri';
	import { confirmation } from '$lib/stores/confirmation.svelte';
	import { notifyCleanupSuccess, notifyOperationError } from '$lib/utils/notification-helpers';
	import { logger } from '$lib/utils/logger';
	import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte';
	import type { FilesystemHealthResults, ScanItem } from '$lib/generated/types';

	// FilesystemHealthResults type is now imported from generated types
	// Removed duplicate interface definition

	let results = $state<FilesystemHealthResults | null>(null);
	let loading = $state(false);

	async function runFilesystemHealthCheck() {
		loading = true;
		try {
			// 5 minute timeout for filesystem health scan
			results = await invoke<FilesystemHealthResults>('scan_filesystem_health', undefined, 300000);
		} catch (e) {
			logger.error('Failed to run filesystem health check', { component: 'FilesystemHealth', action: 'run_scan', operation: 'start_filesystem_health_check' }, e);
			const errorMessage = e instanceof Error && e.message.includes('timed out')
				? 'Filesystem health check timed out. The scan took too long to complete.'
				: 'Could not complete scan';
			notifyOperationError('Filesystem Health Check', errorMessage);
		} finally {
			loading = false;
		}
	}

	async function cleanAllSafeItems() {
		if (!results || results.total_items === 0) return;

		const confirmed = await confirmation.show({
			title: 'Clean All Safe Items',
			message: `This will remove ${results.total_items} safe filesystem items (${formatBytes(results.total_size)}), including empty directories, broken symlinks, and orphaned temp files. These items are safe to remove.`,
			confirmText: 'Clean All Safe Items',
			cancelText: 'Cancel',
			type: 'info'
		});

		if (!confirmed) return;

		try {
			// Clean empty directories
			if (results.empty_directories.length > 0) {
				const emptyDirPaths = results.empty_directories.map(item => item.path);
				await invoke('clean_items', {
					item_ids: results.empty_directories.map(item => item.id),
					item_paths: emptyDirPaths,
					use_trash: false, // Direct deletion for empty dirs
					retention_days: 3
				});
			}

			// Clean broken symlinks
			if (results.broken_symlinks.length > 0) {
				const brokenLinkPaths = results.broken_symlinks.map(item => item.path);
				await invoke('clean_items', {
					item_ids: results.broken_symlinks.map(item => item.id),
					item_paths: brokenLinkPaths,
					use_trash: false, // Direct deletion for broken links
					retention_days: 3
				});
			}

			// Clean orphaned temp files
			if (results.orphaned_temp_files.length > 0) {
				const tempFilePaths = results.orphaned_temp_files.map(item => item.path);
				await invoke('clean_items', {
					item_ids: results.orphaned_temp_files.map(item => item.id),
					item_paths: tempFilePaths,
					use_trash: true, // Use trash for temp files as safety measure
					retention_days: 3
				});
			}

			notifyCleanupSuccess('Filesystem Cleanup', `Removed ${results.total_items} filesystem items`);
			results = null; // Clear results after successful cleanup
		} catch (e) {
			logger.error('Failed to clean items', { component: 'FilesystemHealth', action: 'clean_items', operation: 'clean_selected_items' }, e);
			notifyOperationError('Filesystem Cleanup', 'Some items could not be removed');
		}
	}

	function getCategoryIcon(category: string): string {
		switch (category) {
			case 'empty_directory': return '📁';
			case 'broken_symlink': return '🔗';
			case 'orphaned_temp': return '🗑️';
			default: return '📄';
		}
	}

	function getCategoryLabel(category: string): string {
		switch (category) {
			case 'empty_directory': return 'Empty Directories';
			case 'broken_symlink': return 'Broken Symlinks';
			case 'orphaned_temp': return 'Orphaned Temp Files';
			default: return category;
		}
	}


	// Auto-run scan on mount
	onMount(async () => {
		await runFilesystemHealthCheck();
	});
</script>

<div class="filesystem-health">
	<div class="flex items-center justify-between mb-6">
		<div>
			<h2 class="text-2xl font-semibold mb-2 flex items-center gap-2">
				<span class="text-2xl">🔧</span>
				Filesystem Health Check
			</h2>
			<p class="text-muted">Find and clean up empty directories, broken links, and orphaned files</p>
		</div>
		<button
			class="btn btn-primary"
			onclick={runFilesystemHealthCheck}
			disabled={loading}
		>
			{#if loading}
				<div class="spinner w-4 h-4 mr-2"></div>
			{/if}
			{loading ? 'Scanning...' : 'Run Health Check'}
		</button>
	</div>

	{#if loading}
		<LoadingSpinner
			message="Scanning your filesystem..."
			size="lg"
			fullscreen={true}
		/>
	{:else if results}
		<!-- Summary Card -->
		<div class="card p-6 mb-6">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-4">
					<div class="w-16 h-16 rounded-full bg-green-100 flex items-center justify-center">
						<span class="text-3xl">✅</span>
					</div>
					<div>
						<h3 class="text-xl font-semibold">Health Check Complete</h3>
						<p class="text-muted">Found {results.total_items} safe items to clean</p>
					</div>
				</div>
				<div class="text-right">
					<div class="text-2xl font-bold text-green-600">{formatBytes(results.total_size)}</div>
					<div class="text-sm text-muted">space reclaimable</div>
				</div>
			</div>

			{#if results.total_items > 0}
				<div class="mt-6">
					<button
						class="btn btn-primary w-full py-3 text-lg"
						onclick={cleanAllSafeItems}
					>
						🧹 Clean All Safe Items ({results.total_items} items)
					</button>
				</div>
			{/if}
		</div>

		<!-- Category Breakdown -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<!-- Empty Directories -->
			<div class="card p-5">
				<div class="flex items-center gap-3 mb-3">
					<div class="w-12 h-12 rounded-lg bg-blue-100 flex items-center justify-center">
						<span class="text-2xl">📁</span>
					</div>
					<div>
						<h3 class="font-semibold">Empty Directories</h3>
						<p class="text-sm text-muted">{results.empty_directories.length} found</p>
					</div>
				</div>
				<p class="text-sm text-muted">Directories with no contents that can be safely removed</p>
			</div>

			<!-- Broken Symlinks -->
			<div class="card p-5">
				<div class="flex items-center gap-3 mb-3">
					<div class="w-12 h-12 rounded-lg bg-orange-100 flex items-center justify-center">
						<span class="text-2xl">🔗</span>
					</div>
					<div>
						<h3 class="font-semibold">Broken Symlinks</h3>
						<p class="text-sm text-muted">{results.broken_symlinks.length} found</p>
					</div>
				</div>
				<p class="text-sm text-muted">Symlinks pointing to files that no longer exist</p>
			</div>

			<!-- Orphaned Temp Files -->
			<div class="card p-5">
				<div class="flex items-center gap-3 mb-3">
					<div class="w-12 h-12 rounded-lg bg-purple-100 flex items-center justify-center">
						<span class="text-2xl">🗑️</span>
					</div>
					<div>
						<h3 class="font-semibold">Orphaned Temp Files</h3>
						<p class="text-sm text-muted">{results.orphaned_temp_files.length} found</p>
					</div>
				</div>
				<p class="text-sm text-muted">Old temporary files that are no longer needed</p>
			</div>
		</div>

		<!-- Detailed Results (Expandable) -->
		{#if results.total_items > 0}
			<div class="mt-6">
				<details class="card">
					<summary class="p-4 cursor-pointer hover:bg-muted/20 transition-colors">
						<span class="font-semibold">View Detailed Results</span>
						<span class="text-sm text-muted ml-2">({results.total_items} items)</span>
					</summary>

					<div class="p-4 border-t space-y-4">
						{#each [results.empty_directories, results.broken_symlinks, results.orphaned_temp_files] as categoryItems}
							{#if categoryItems.length > 0}
								<div>
									<h4 class="font-semibold mb-3 flex items-center gap-2">
										<span class="text-lg">{getCategoryIcon(categoryItems[0].category)}</span>
										{getCategoryLabel(categoryItems[0].category)}
										<span class="text-sm text-muted">({categoryItems.length})</span>
									</h4>

									<div class="space-y-2">
										{#each categoryItems.slice(0, 10) as item}
											<div class="flex items-center justify-between p-3 bg-muted/20 rounded">
												<div class="min-w-0 flex-1">
													<div class="font-medium truncate">{item.name}</div>
													<div class="text-sm text-muted truncate">{item.path}</div>
												</div>
												<div class="text-sm text-muted">
													{formatBytes(item.size)}
												</div>
											</div>
										{/each}

										{#if categoryItems.length > 10}
											<div class="text-center text-muted py-2">
												And {categoryItems.length - 10} more items...
											</div>
										{/if}
									</div>
								</div>
							{/if}
						{/each}
					</div>
				</details>
			</div>
		{/if}
	{:else}
		<div class="card p-12 text-center">
			<div class="w-16 h-16 mx-auto mb-4 rounded-full bg-muted flex items-center justify-center">
				<span class="text-3xl">🔧</span>
			</div>
			<h3 class="text-lg font-medium mb-2">Filesystem Health Check</h3>
			<p class="text-muted mb-4">Run a scan to find empty directories, broken symlinks, and orphaned temp files</p>
			<button
				class="btn btn-primary"
				onclick={runFilesystemHealthCheck}
			>
				Start Health Check
			</button>
		</div>
	{/if}
</div>
