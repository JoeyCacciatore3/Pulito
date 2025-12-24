<script lang="ts">
	import { scanner } from '$lib/stores/scanner.svelte';
	import type { ScanItem } from '$lib/generated/types';
	import { formatBytes, getRiskLevelInfo, invoke } from '$lib/utils/tauri';
	import { confirmation } from '$lib/stores/confirmation.svelte';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';

	let searchQuery = $state('');
	let riskFilter = $state<number | null>(null);
	let expandedIds = $state<Set<string>>(new Set());
	let selectedItem = $state<ScanItem | null>(null);
	let showPreview = $state(true);

	function toggleExpand(id: string) {
		const newSet = new Set(expandedIds);
		if (newSet.has(id)) {
			newSet.delete(id);
		} else {
			newSet.add(id);
		}
		expandedIds = newSet;
	}

	function selectItemForPreview(item: ScanItem) {
		selectedItem = item;
	}

	function closePreview() {
		selectedItem = null;
	}

	function isImageFile(filename: string): boolean {
		const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.webp', '.svg'];
		const lowerName = filename.toLowerCase();
		return imageExtensions.some(ext => lowerName.endsWith(ext));
	}

	function filterItems(items: ScanItem[]): ScanItem[] {
		return items.filter((item) => {
			const matchesSearch =
				!searchQuery ||
				item.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				item.path.toLowerCase().includes(searchQuery.toLowerCase());

			const matchesRisk = riskFilter === null || item.risk_level === riskFilter;

			return matchesSearch && matchesRisk;
		});
	}

	async function cleanSelectedItems() {
		const selected = scanner.selectedItems;
		if (selected.length === 0) return;

		const highRiskItems = selected.filter(item => item.risk_level >= 4);
		const totalSize = selected.reduce((sum, item) => sum + item.size, 0);

		const confirmed = await confirmation.show({
			title: 'Clean Selected Items',
			message: `Are you sure you want to clean ${selected.length} selected items (${formatBytes(totalSize)})? ${highRiskItems.length > 0 ? `Warning: ${highRiskItems.length} items are high risk and may affect system stability. ` : ''}This action will move items to trash where they can be recovered.`,
			confirmText: 'Clean Selected',
			cancelText: 'Cancel',
			type: highRiskItems.length > 0 ? 'warning' : 'info'
		});

		if (!confirmed) return;

		try {
			const result = await invoke<{ cleaned: number; failed: number; total_size: number }>('clean_items', {
				item_ids: selected.map(item => item.id),
				item_paths: selected.map(item => item.path),
				use_trash: true,
				retention_days: 3
			});

			scanner.clearSelection();

			// Show success/error feedback
			if (result.failed > 0) {
				notificationStore.warning(
					'Cleanup Partial',
					`Cleaned ${result.cleaned} items (${formatBytes(result.total_size)}), but ${result.failed} failed.`
				);
			} else {
				notificationStore.success(
					'Cleanup Complete',
					`Successfully cleaned ${result.cleaned} items, freed ${formatBytes(result.total_size)}.`
				);
			}

			// Reload scan results
			if (scanner.results) {
				// Trigger a re-scan or update results
				// For now, just clear results so user can re-scan
				scanner.setResults(null);
			}
		} catch (e) {
			logger.error('Failed to clean items', { component: 'TreeView', action: 'clean_items', operation: 'clean_selected_items' }, e);
			notificationStore.error('Cleanup Failed', `Failed to clean items: ${e instanceof Error ? e.message : String(e)}`);
		}
	}
</script>

<div class="p-6 space-y-4 animate-in">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold mb-1 flex items-center gap-2">
				<span class="text-2xl">🔍</span>
				System Scan Results
			</h1>
			{#if scanner.results}
				<p class="text-sm text-[var(--color-text-secondary)]">
					{scanner.results.total_items} items analyzed · {formatBytes(scanner.results.total_size)} potential space savings
				</p>
				<div class="flex items-center gap-4 mt-2 text-xs text-[var(--color-text-muted)]">
					<span class="flex items-center gap-1">
						<span class="w-2 h-2 rounded-full bg-safe"></span>
						Safe to remove
					</span>
					<span class="flex items-center gap-1">
						<span class="w-2 h-2 rounded-full bg-caution"></span>
						Review first
					</span>
					<span class="flex items-center gap-1">
						<span class="w-2 h-2 rounded-full bg-warning"></span>
						Use caution
					</span>
				</div>
			{/if}
		</div>

		<div class="flex items-center gap-3">
			{#if scanner.results}
				<button
					class="btn btn-ghost hover:bg-[var(--color-bg-secondary)] transition-colors"
					onclick={() => showPreview = !showPreview}
					title={showPreview ? 'Hide preview pane' : 'Show preview pane'}
				>
					<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						{#if showPreview}
							<path d="M15 3h4a2 2 0 012 2v14a2 2 0 01-2 2h-4M10 17l5-5-5-5M4 12h8" />
						{:else}
							<path d="M9 3h6a2 2 0 012 2v14a2 2 0 01-2 2H9M3 17l6-6-6-6M3 12h8" />
						{/if}
					</svg>
					{showPreview ? 'Hide' : 'Show'} Preview
				</button>
			{/if}

			{#if scanner.selectedItems.length > 0}
				<div class="flex items-center gap-3">
					<div class="text-sm text-[var(--color-text-secondary)]">
						{scanner.selectedItems.length} items selected · {formatBytes(scanner.selectedSize)}
					</div>
					<button class="btn btn-primary hover:bg-primary-700 transition-colors" onclick={cleanSelectedItems}>
						🧹 Clean Selected
					</button>
				</div>
			{/if}
		</div>
	</div>

	<!-- Smart Filters -->
	<div class="card p-4 mb-4">
		<h3 class="font-medium mb-3 flex items-center gap-2">
			<span class="text-lg">🎯</span>
			Refine Your Results
		</h3>
		<div class="flex flex-wrap items-center gap-3">
			<div class="flex-1 min-w-[200px]">
				<label for="search-input" class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1">
					Search files & folders
				</label>
				<input
					id="search-input"
					type="text"
					placeholder="Type to filter results..."
					class="input w-full"
					bind:value={searchQuery}
				/>
			</div>

			<div class="min-w-[150px]">
				<label for="risk-filter" class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1">
					Safety Level
				</label>
				<select
					id="risk-filter"
					class="input w-full"
					bind:value={riskFilter}
				>
					<option value={null}>All safety levels</option>
					<option value={0}>🟢 Safe only</option>
					<option value={1}>🟡 Low risk</option>
					<option value={2}>🟠 Review needed</option>
					<option value={3}>🔴 High risk</option>
				</select>
			</div>

			<div class="flex gap-2">
				<button
					class="btn btn-secondary hover:bg-secondary-100 transition-colors"
					onclick={() => scanner.selectAll()}
					title="Select all visible items for cleanup"
				>
					Select All
				</button>
				<button
					class="btn btn-ghost hover:bg-red-50 hover:text-red-600 transition-colors"
					onclick={() => scanner.clearSelection()}
					title="Clear all selections"
				>
					Clear All
				</button>
			</div>
		</div>

		{#if scanner.selectedItems.length > 0}
			<div class="mt-3 p-3 bg-primary-50 dark:bg-primary-900/20 rounded-lg border border-primary-200 dark:border-primary-800">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<span class="text-sm font-medium text-primary-700 dark:text-primary-300">
							Ready to clean: {scanner.selectedItems.length} items
						</span>
						<span class="text-sm text-primary-600 dark:text-primary-400">
							({formatBytes(scanner.selectedSize)} space)
						</span>
					</div>
					<div class="flex gap-2">
						<button class="btn btn-outline btn-sm" onclick={() => scanner.clearSelection()}>
							Cancel
						</button>
						<button class="btn btn-primary btn-sm" onclick={cleanSelectedItems}>
							🧹 Start Cleanup
						</button>
					</div>
				</div>
			</div>
		{/if}
	</div>

	<!-- Tree View with Preview Pane -->
	<div class="flex gap-4">
		{#if scanner.results}
			<!-- Main Tree View -->
			<div class="flex-1 card divide-y divide-[var(--color-border)]">
				{#each filterItems(scanner.results.items) as item (item.id)}
				{@const riskInfo = getRiskLevelInfo(item.risk_level)}
				{@const isExpanded = expandedIds.has(item.id)}
				{@const isSelected = scanner.selectedIds.has(item.id)}
				{@const hasChildren = item.children && item.children.length > 0}

				<div class="p-4 hover:bg-[var(--color-bg-secondary)]/30 transition-colors cursor-pointer" role="button" tabindex="0" onclick={() => selectItemForPreview(item)} onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); selectItemForPreview(item); } }}>
					<div class="flex items-start gap-3">
						<!-- Expand/Collapse -->
						{#if hasChildren}
							<button
								class="p-1 rounded hover:bg-[var(--color-bg-secondary)] transition-colors mt-1"
								aria-label="{isExpanded ? 'Collapse' : 'Expand'} {item.name} folder"
								onclick={() => toggleExpand(item.id)}
							>
								<svg
									class="w-4 h-4 transition-transform {isExpanded ? 'rotate-90' : ''}"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
								>
									<path d="M9 18l6-6-6-6" />
								</svg>
							</button>
						{:else}
							<div class="w-6"></div>
						{/if}

						<!-- Checkbox with better styling -->
						<button
							class="w-5 h-5 rounded border-2 flex items-center justify-center transition-all mt-1 {isSelected
								? 'bg-primary-600 border-primary-600 shadow-sm'
								: 'border-gray-300 dark:border-gray-600 hover:border-primary-400 hover:shadow-sm'}"
							onclick={() => scanner.toggleSelection(item.id)}
							title={isSelected ? 'Remove from cleanup selection' : 'Add to cleanup selection'}
						>
							{#if isSelected}
								<svg class="w-3 h-3 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
									<path d="M5 13l4 4L19 7" />
								</svg>
							{/if}
						</button>

						<!-- Enhanced Icon with better colors -->
						<div class="w-10 h-10 rounded-lg flex items-center justify-center mt-1 {item.type === 'cache'
							? 'bg-blue-100 dark:bg-blue-900/30'
							: item.type === 'package'
							? 'bg-green-100 dark:bg-green-900/30'
							: item.type === 'directory'
							? 'bg-purple-100 dark:bg-purple-900/30'
							: 'bg-gray-100 dark:bg-gray-900/30'}">
							{#if item.type === 'cache'}
								<svg class="w-5 h-5 text-blue-600" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<path d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" />
								</svg>
							{:else if item.type === 'package'}
								<svg class="w-5 h-5 text-green-600" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<path d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
								</svg>
							{:else if item.type === 'directory'}
								<svg class="w-5 h-5 text-purple-600" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
								</svg>
							{:else}
								<svg class="w-5 h-5 text-gray-600" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z" />
									<path d="M14 2v6h6" />
								</svg>
							{/if}
						</div>

						<!-- Enhanced Info Section -->
						<div class="flex-1 min-w-0">
							<div class="flex items-start justify-between mb-1">
								<div class="flex items-center gap-2 min-w-0 flex-1">
									<span class="font-medium truncate text-base">{item.name}</span>
									<span class="badge {riskInfo.bgClass} shrink-0 text-xs px-2 py-1">
										{riskInfo.icon} {riskInfo.label}
									</span>
								</div>
								<div class="text-sm font-bold text-[var(--color-text)] shrink-0 ml-3">
									{formatBytes(item.size)}
								</div>
							</div>

							{#if item.path}
								<div class="text-xs text-[var(--color-text-muted)] truncate mb-1 font-mono">
									📁 {item.path}
								</div>
							{/if}

							<div class="text-sm text-[var(--color-text-secondary)] leading-relaxed">
								{item.description}
								{#if item.risk_level === 0}
									<span class="text-safe text-xs ml-2">✓ Safe to remove</span>
								{:else if item.risk_level === 1}
									<span class="text-caution text-xs ml-2">⚠ Review before removing</span>
								{:else if item.risk_level >= 2}
									<span class="text-warning text-xs ml-2">🚨 Use caution</span>
								{/if}
							</div>

							{#if item.type === 'cache'}
								<div class="text-xs text-blue-600 mt-1">💾 Application cache files - regenerated automatically</div>
							{:else if item.type === 'package'}
								<div class="text-xs text-green-600 mt-1">📦 Package manager files - may affect package operations</div>
							{:else if item.type === 'directory'}
								<div class="text-xs text-purple-600 mt-1">📂 Directory with multiple files</div>
							{/if}
						</div>
					</div>

					<!-- Children -->
					{#if hasChildren && isExpanded}
						<div class="ml-14 mt-2 pl-4 border-l-2 border-[var(--color-border)] space-y-2">
							{#each item.children as child (child.id)}
								{@const childRiskInfo = getRiskLevelInfo(child.risk_level)}
								{@const childSelected = scanner.selectedIds.has(child.id)}

								<div class="flex items-center gap-3 py-1">
									<button
										class="w-4 h-4 rounded border-2 flex items-center justify-center transition-colors {childSelected
											? 'bg-primary-600 border-primary-600'
											: 'border-gray-300 dark:border-gray-600'}"
										onclick={() => scanner.toggleSelection(child.id)}
									>
										{#if childSelected}
											<svg class="w-2.5 h-2.5 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
												<path d="M5 13l4 4L19 7" />
											</svg>
										{/if}
									</button>

									<div class="flex-1 min-w-0">
										<div class="flex items-center gap-2">
											<span class="text-sm truncate">{child.name}</span>
											<span class="badge {childRiskInfo.bgClass} text-[10px]">{childRiskInfo.label}</span>
										</div>
									</div>

									<div class="text-xs font-mono text-[var(--color-text-muted)]">
										{formatBytes(child.size)}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/each}
		</div>
		{:else}
			<!-- No results state -->
			<div class="flex-1 card p-12 text-center">
				<svg class="w-16 h-16 mx-auto mb-4 text-[var(--color-text-muted)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<circle cx="11" cy="11" r="8" />
					<path d="m21 21-4.35-4.35" />
				</svg>
				<h3 class="text-lg font-medium mb-2">No scan results yet</h3>
				<p class="text-[var(--color-text-secondary)] mb-4">
					Click "Scan System" to analyze your system for cleanup opportunities
				</p>
			</div>
		{/if}

			<!-- Preview Pane -->
			{#if showPreview && selectedItem}
				<aside class="w-80 card p-4 h-fit">
					<div class="flex items-center justify-between mb-3">
						<h3 class="font-semibold">Preview</h3>
						<button
							class="p-1 rounded hover:bg-[var(--color-bg-secondary)] transition-colors"
							onclick={closePreview}
							title="Close preview"
						>
							<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M18 6L6 18M6 6l12 12" />
							</svg>
						</button>
					</div>

					<!-- Item Preview Content -->
					<div class="space-y-4">
						<!-- Icon and Basic Info -->
						<div class="flex items-center gap-3 p-3 bg-[var(--color-bg-secondary)] rounded-lg">
							<div class="w-12 h-12 rounded-lg flex items-center justify-center {selectedItem.type === 'cache'
								? 'bg-blue-100 dark:bg-blue-900/30'
								: selectedItem.type === 'package'
								? 'bg-green-100 dark:bg-green-900/30'
								: selectedItem.type === 'directory'
								? 'bg-purple-100 dark:bg-purple-900/30'
								: 'bg-gray-100 dark:bg-gray-900/30'}">
								{#if selectedItem.type === 'cache'}
									<svg class="w-6 h-6 text-blue-600" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" />
									</svg>
								{:else if selectedItem.type === 'package'}
									<svg class="w-6 h-6 text-green-600" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
									</svg>
								{:else if selectedItem.type === 'directory'}
									<svg class="w-6 h-6 text-purple-600" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
									</svg>
								{:else}
									<svg class="w-6 h-6 text-gray-600" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z" />
										<path d="M14 2v6h6" />
									</svg>
								{/if}
							</div>
							<div class="min-w-0 flex-1">
								<h4 class="font-medium truncate">{selectedItem.name}</h4>
								<p class="text-sm text-[var(--color-text-secondary)]">{selectedItem.category}</p>
							</div>
						</div>

						<!-- File Details -->
						<div class="space-y-2">
							<div class="flex justify-between">
								<span class="text-sm text-[var(--color-text-secondary)]">Size:</span>
								<span class="text-sm font-medium">{formatBytes(selectedItem.size)}</span>
							</div>
							<div class="flex justify-between">
								<span class="text-sm text-[var(--color-text-secondary)]">Type:</span>
								<span class="text-sm font-medium">{selectedItem.type}</span>
							</div>
							<div class="flex justify-between">
								<span class="text-sm text-[var(--color-text-secondary)]">Risk Level:</span>
								<span class="text-sm font-medium">{getRiskLevelInfo(selectedItem.risk_level).label}</span>
							</div>
						</div>

						<!-- Path -->
						<div>
							<h5 class="text-sm font-medium mb-1">Location</h5>
							<div class="text-xs font-mono bg-[var(--color-bg-secondary)] p-2 rounded break-all">
								{selectedItem.path}
							</div>
						</div>

						<!-- Description -->
						{#if selectedItem.description}
							<div>
								<h5 class="text-sm font-medium mb-1">Details</h5>
								<p class="text-sm text-[var(--color-text-secondary)]">{selectedItem.description}</p>
							</div>
						{/if}

						<!-- Image Preview (if applicable) -->
						{#if selectedItem.type === 'file' && isImageFile(selectedItem.name)}
							<div>
								<h5 class="text-sm font-medium mb-2">Preview</h5>
								<div class="border rounded-lg overflow-hidden bg-[var(--color-bg-secondary)]">
									<!-- Note: In a real implementation, you'd load and display the actual image -->
									<div class="aspect-video flex items-center justify-center text-[var(--color-text-muted)]">
										<div class="text-center">
											<svg class="w-8 h-8 mx-auto mb-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
												<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
												<circle cx="9" cy="9" r="2" />
												<path d="m21 15-3.086-3.086a2 2 0 00-2.828 0L6 21" />
											</svg>
											<p class="text-xs">Image preview not available</p>
											<p class="text-xs">File: {selectedItem.name}</p>
										</div>
									</div>
								</div>
							</div>
						{/if}
					</div>
				</aside>
			{/if}
		</div>
	</div>
