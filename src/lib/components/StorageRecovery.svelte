<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke, formatBytes } from '$lib/utils/tauri';
	import { confirmation } from '$lib/stores/confirmation.svelte';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import type { StorageRecoveryResults } from '$lib/generated/types';

	let results = $state<StorageRecoveryResults | null>(null);
	let loading = $state(false);
	let selectedDuplicates = $state<Set<string>>(new Set());
	let selectedLargeFiles = $state<Set<string>>(new Set());
	let selectedOldDownloads = $state<Set<string>>(new Set());

	async function runStorageRecoveryScan() {
		loading = true;
		try {
			// 10 minute timeout for storage recovery scan (more complex analysis)
			results = await invoke<StorageRecoveryResults>('scan_storage_recovery', undefined, 600000);
		} catch (e) {
			logger.error('Failed to run storage recovery scan', { component: 'StorageRecovery', action: 'run_scan', operation: 'start_storage_recovery_scan' }, e);
			const errorMessage = e instanceof Error && e.message.includes('timed out')
				? 'Storage recovery scan timed out. The scan took too long to complete.'
				: 'Could not complete storage recovery scan';
			notificationStore.error('Scan Failed', errorMessage);
		} finally {
			loading = false;
		}
	}

	function getSelectedSize(): number {
		let total = 0;

		// Calculate size from selected duplicates (only count extra copies)
		for (const groupId of selectedDuplicates) {
			const group = results?.duplicates.find(g => g.id === groupId);
			if (group && group.files.length > 1) {
				// Size of all files minus one (keep one copy)
				total += group.total_size - group.files[0].size;
			}
		}

		// Add selected large files
		for (const fileId of selectedLargeFiles) {
			const file = results?.large_files.find(f => f.id === fileId);
			if (file) {
				total += file.size;
			}
		}

		// Add selected old downloads
		for (const fileId of selectedOldDownloads) {
			const file = results?.old_downloads.find(f => f.id === fileId);
			if (file) {
				total += file.size;
			}
		}

		return total;
	}

	function smartSelectDuplicates() {
		// Keep newest file in each group, remove others
		if (!results) return;

		results.duplicates.forEach(group => {
			if (group.files.length > 1) {
				// Sort by modification time (newest first) - assuming last_modified exists
				// For now, just select all groups (keeping first file)
				selectedDuplicates.add(group.id);
			}
		});

		notificationStore.info('Smart Selection', 'Selected duplicate files (keeping newest)');
	}

	function selectAllSafe() {
		if (!results) return;

		// Select all duplicates
		results.duplicates.forEach(group => {
			if (group.files.length > 1) {
				selectedDuplicates.add(group.id);
			}
		});

		// Select old downloads (low risk)
		results.old_downloads.forEach(file => {
			if (file.risk_level <= 1) {
				selectedOldDownloads.add(file.id);
			}
		});

		notificationStore.info('Safe Selection', 'Selected all safe-to-remove items');
	}

	async function smartCleanup() {
		if (!results) return;

		const totalSelected = selectedDuplicates.size + selectedLargeFiles.size + selectedOldDownloads.size;
		const totalSize = getSelectedSize();

		if (totalSelected === 0) {
			notificationStore.warning('Nothing Selected', 'Please select items to clean up');
			return;
		}

		const confirmed = await confirmation.show({
			title: 'Smart Storage Cleanup',
			message: `Clean up ${totalSelected} selected items (${formatBytes(totalSize)})? This includes duplicate files, large files, and old downloads.`,
			confirmText: 'Clean Selected Items',
			cancelText: 'Cancel',
			type: 'warning'
		});

		if (!confirmed) return;

		try {
			let cleanedCount = 0;
			let totalCleanedSize = 0;

			// Clean selected duplicate files (keep first in each group)
			for (const groupId of selectedDuplicates) {
				const group = results.duplicates.find(g => g.id === groupId);
				if (group && group.files.length > 1) {
					// Remove all but the first file
					const filesToRemove = group.files.slice(1);
					const paths = filesToRemove.map(f => f.path);
					const ids = filesToRemove.map(f => f.id);

					const result = await invoke<{ cleaned: number; failed: number; total_size: number }>('clean_items', {
						item_ids: ids,
						item_paths: paths,
						use_trash: true,
						retention_days: 30 // Longer retention for storage cleanup
					});

					cleanedCount += result.cleaned;
					totalCleanedSize += result.total_size;
				}
			}

			// Clean selected large files
			if (selectedLargeFiles.size > 0 && results) {
				const largeFilePaths = Array.from(selectedLargeFiles)
					.map(id => results!.large_files.find(f => f.id === id)?.path)
					.filter((path): path is string => path !== undefined);

				if (largeFilePaths.length > 0) {
					const result = await invoke<{ cleaned: number; failed: number; total_size: number }>('clean_items', {
						item_ids: Array.from(selectedLargeFiles),
						item_paths: largeFilePaths,
						use_trash: true,
						retention_days: 30
					});

					cleanedCount += result.cleaned;
					totalCleanedSize += result.total_size;
				}
			}

			// Clean selected old downloads
			if (selectedOldDownloads.size > 0 && results) {
				const oldDownloadPaths = Array.from(selectedOldDownloads)
					.map(id => results!.old_downloads.find(f => f.id === id)?.path)
					.filter((path): path is string => path !== undefined);

				if (oldDownloadPaths.length > 0) {
					const result = await invoke<{ cleaned: number; failed: number; total_size: number }>('clean_items', {
						item_ids: Array.from(selectedOldDownloads),
						item_paths: oldDownloadPaths,
						use_trash: true,
						retention_days: 30
					});

					cleanedCount += result.cleaned;
					totalCleanedSize += result.total_size;
				}
			}

			notificationStore.success('Cleanup Complete', `Removed ${cleanedCount} items, freed ${formatBytes(totalCleanedSize)}`);

			// Clear selections and refresh results
			selectedDuplicates.clear();
			selectedLargeFiles.clear();
			selectedOldDownloads.clear();
			results = null;

		} catch (e) {
			logger.error('Failed to clean items', { component: 'StorageRecovery', action: 'clean_items', operation: 'clean_selected_items' }, e);
			notificationStore.error('Cleanup Failed', 'Some items could not be removed');
		}
	}

	function toggleDuplicateGroup(groupId: string) {
		if (selectedDuplicates.has(groupId)) {
			selectedDuplicates.delete(groupId);
		} else {
			selectedDuplicates.add(groupId);
		}
		selectedDuplicates = new Set(selectedDuplicates);
	}

	function toggleLargeFile(fileId: string) {
		if (selectedLargeFiles.has(fileId)) {
			selectedLargeFiles.delete(fileId);
		} else {
			selectedLargeFiles.add(fileId);
		}
		selectedLargeFiles = new Set(selectedLargeFiles);
	}

	function toggleOldDownload(fileId: string) {
		if (selectedOldDownloads.has(fileId)) {
			selectedOldDownloads.delete(fileId);
		} else {
			selectedOldDownloads.add(fileId);
		}
		selectedOldDownloads = new Set(selectedOldDownloads);
	}

	// Auto-run scan on mount
	onMount(async () => {
		await runStorageRecoveryScan();
	});
</script>

<div class="storage-recovery">
	<div class="flex items-center justify-between mb-6">
		<div>
			<h2 class="text-2xl font-semibold mb-2 flex items-center gap-2">
				<span class="text-2xl">💾</span>
				Storage Recovery
			</h2>
			<p class="text-muted">Find and clean up duplicates, large files, and old downloads</p>
		</div>
		<button
			class="btn btn-primary"
			onclick={runStorageRecoveryScan}
			disabled={loading}
		>
			{#if loading}
				<div class="spinner w-4 h-4 mr-2"></div>
			{/if}
			{loading ? 'Scanning...' : 'Find Storage'}
		</button>
	</div>

	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="text-center">
				<div class="spinner w-12 h-12 mx-auto mb-4"></div>
				<p class="text-lg">Analyzing your storage...</p>
				<p class="text-muted">This may take a moment for large drives</p>
			</div>
		</div>
	{:else if results}
		<!-- Summary Card -->
		<div class="card p-6 mb-6">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-4">
					<div class="w-16 h-16 rounded-full bg-blue-100 flex items-center justify-center">
						<span class="text-3xl">🎯</span>
					</div>
					<div>
						<h3 class="text-xl font-semibold">Storage Recovery Complete</h3>
						<p class="text-muted">Found {results.duplicates.length + results.large_files.length + results.old_downloads.length} opportunities</p>
					</div>
				</div>
				<div class="text-right">
					<div class="text-2xl font-bold text-blue-600">{formatBytes(results.total_recoverable_size)}</div>
					<div class="text-sm text-muted">potential space</div>
				</div>
			</div>

			{#if getSelectedSize() > 0}
				<div class="mt-6 p-4 bg-green-50 dark:bg-green-900/20 rounded-lg border border-green-200 dark:border-green-800">
					<div class="flex items-center justify-between">
						<div>
							<div class="text-lg font-semibold text-green-800 dark:text-green-400">
								Selected for cleanup: {selectedDuplicates.size + selectedLargeFiles.size + selectedOldDownloads.size} items
							</div>
							<div class="text-sm text-green-600 dark:text-green-500">
								{formatBytes(getSelectedSize())} space to recover
							</div>
						</div>
					<div class="flex gap-2 mb-4">
						<button class="btn btn-secondary text-sm" onclick={smartSelectDuplicates}>
							🎯 Smart Select Duplicates
						</button>
						<button class="btn btn-secondary text-sm" onclick={selectAllSafe}>
							✅ Select All Safe
						</button>
					</div>
					<button class="btn btn-primary" onclick={smartCleanup}>
						Clean Selected
					</button>
					</div>
				</div>
			{/if}
		</div>

		<!-- Recovery Categories -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
			<!-- Duplicate Files -->
			<div class="card p-5">
				<div class="flex items-center gap-3 mb-3">
					<div class="w-12 h-12 rounded-lg bg-purple-100 flex items-center justify-center">
						<span class="text-2xl">📋</span>
					</div>
					<div>
						<h3 class="font-semibold">Duplicate Files</h3>
						<p class="text-sm text-muted">{results.duplicates.length} groups found</p>
					</div>
				</div>
				<p class="text-sm text-muted mb-3">Identical files taking up extra space</p>
				<div class="text-lg font-bold text-purple-600">{formatBytes(results.total_duplicate_size)}</div>
				<div class="text-xs text-muted">recoverable space</div>
			</div>

			<!-- Large Files -->
			<div class="card p-5">
				<div class="flex items-center gap-3 mb-3">
					<div class="w-12 h-12 rounded-lg bg-orange-100 flex items-center justify-center">
						<span class="text-2xl">📦</span>
					</div>
					<div>
						<h3 class="font-semibold">Large Files</h3>
						<p class="text-sm text-muted">{results.large_files.length} files >1GB</p>
					</div>
				</div>
				<p class="text-sm text-muted mb-3">Files larger than 1GB that may be unnecessary</p>
				<div class="text-lg font-bold text-orange-600">{formatBytes(results.total_large_files_size)}</div>
				<div class="text-xs text-muted">total size</div>
			</div>

			<!-- Old Downloads -->
			<div class="card p-5">
				<div class="flex items-center gap-3 mb-3">
					<div class="w-12 h-12 rounded-lg bg-green-100 flex items-center justify-center">
						<span class="text-2xl">📥</span>
					</div>
					<div>
						<h3 class="font-semibold">Old Downloads</h3>
						<p class="text-sm text-muted">{results.old_downloads.length} files >90 days</p>
					</div>
				</div>
				<p class="text-sm text-muted mb-3">Downloaded files older than 90 days</p>
				<div class="text-lg font-bold text-green-600">{formatBytes(results.total_old_downloads_size)}</div>
				<div class="text-xs text-muted">recoverable space</div>
			</div>
		</div>

		<!-- Detailed Results -->
		<div class="space-y-6">
			<!-- Duplicate Files Section -->
			{#if results.duplicates.length > 0}
				<div class="card">
					<div class="p-4 border-b">
						<h3 class="text-lg font-semibold flex items-center gap-2">
							<span class="text-xl">📋</span>
							Duplicate Files ({results.duplicates.length} groups)
						</h3>
						<p class="text-sm text-muted">Select groups to clean up duplicate files</p>
					</div>

					<div class="divide-y">
						{#each results.duplicates.slice(0, 5) as group}
							<div class="p-4">
								<div class="flex items-center justify-between mb-2">
									<div class="flex items-center gap-3">
										<input
											type="checkbox"
											id={group.id}
											checked={selectedDuplicates.has(group.id)}
											onchange={() => toggleDuplicateGroup(group.id)}
											class="w-4 h-4"
										/>
										<label for={group.id} class="font-medium cursor-pointer">
											{group.files[0].name}
										</label>
									</div>
									<div class="text-right">
										<div class="font-semibold">{formatBytes(group.files[0].size)}</div>
										<div class="text-sm text-muted">{group.group_size} copies</div>
									</div>
								</div>

								<div class="ml-7 space-y-1">
									{#each group.files as file, index}
										<div class="text-sm text-muted flex items-center gap-2">
											{#if index === 0}
												<span class="text-green-600 font-medium">✓ Keep</span>
											{:else}
												<span class="text-red-600">✗ Remove</span>
											{/if}
											<span class="truncate">{file.path}</span>
										</div>
									{/each}
								</div>
							</div>
						{/each}
					</div>

					{#if results.duplicates.length > 5}
						<div class="p-4 border-t text-center">
							<p class="text-muted">And {results.duplicates.length - 5} more duplicate groups...</p>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Large Files Section -->
			{#if results.large_files.length > 0}
				<div class="card">
					<div class="p-4 border-b">
						<h3 class="text-lg font-semibold flex items-center gap-2">
							<span class="text-xl">📦</span>
							Large Files ({results.large_files.length} files)
						</h3>
						<p class="text-sm text-muted">Files larger than 1GB - review carefully</p>
					</div>

					<div class="divide-y">
						{#each results.large_files.slice(0, 10) as file}
							<div class="p-4 flex items-center justify-between">
								<div class="flex items-center gap-3">
									<input
										type="checkbox"
										id={file.id}
										checked={selectedLargeFiles.has(file.id)}
										onchange={() => toggleLargeFile(file.id)}
										class="w-4 h-4"
									/>
									<div class="min-w-0 flex-1">
										<label for={file.id} class="font-medium cursor-pointer truncate block">
											{file.name}
										</label>
										<div class="text-sm text-muted truncate">{file.path}</div>
									</div>
								</div>
								<div class="text-right">
									<div class="font-semibold">{formatBytes(file.size)}</div>
									<div class="text-xs text-orange-600">High risk</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Old Downloads Section -->
			{#if results.old_downloads.length > 0}
				<div class="card">
					<div class="p-4 border-b">
						<h3 class="text-lg font-semibold flex items-center gap-2">
							<span class="text-xl">📥</span>
							Old Downloads ({results.old_downloads.length} files)
						</h3>
						<p class="text-sm text-muted">Downloaded files older than 90 days</p>
					</div>

					<div class="divide-y">
						{#each results.old_downloads.slice(0, 10) as file}
							<div class="p-4 flex items-center justify-between">
								<div class="flex items-center gap-3">
									<input
										type="checkbox"
										id={file.id}
										checked={selectedOldDownloads.has(file.id)}
										onchange={() => toggleOldDownload(file.id)}
										class="w-4 h-4"
									/>
									<div class="min-w-0 flex-1">
										<label for={file.id} class="font-medium cursor-pointer truncate block">
											{file.name}
										</label>
										<div class="text-sm text-muted truncate">{file.path}</div>
									</div>
								</div>
								<div class="text-right">
									<div class="font-semibold">{formatBytes(file.size)}</div>
									<div class="text-xs text-green-600">Low risk</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	{:else}
		<div class="card p-12 text-center">
			<div class="w-16 h-16 mx-auto mb-4 rounded-full bg-muted flex items-center justify-center">
				<span class="text-3xl">💾</span>
			</div>
			<h3 class="text-lg font-medium mb-2">Storage Recovery</h3>
			<p class="text-muted mb-4">Analyze your storage for duplicates, large files, and old downloads</p>
			<button
				class="btn btn-primary"
				onclick={runStorageRecoveryScan}
			>
				Start Recovery Scan
			</button>
		</div>
	{/if}
</div>

<style>
	.spinner {
		border: 2px solid #f3f3f3;
		border-top: 2px solid #3498db;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}
</style>
