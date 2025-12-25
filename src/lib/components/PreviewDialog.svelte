<script lang="ts">
	import { formatBytes } from '$lib/utils/tauri';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import type { CleanupPreview, PreviewItem } from '$lib/generated/types';

	let {
		preview = $bindable(),
		onConfirm = $bindable()
	} = $props<{
		preview: CleanupPreview | null;
		onConfirm: (_selectedItems: PreviewItem[]) => Promise<void>;
	}>();

	let selectedItems = $state<Set<string>>(new Set());
	let expandedCategories = $state<Set<string>>(new Set(['cache', 'filesystem']));

	function toggleItem(id: string) {
		if (selectedItems.has(id)) {
			selectedItems.delete(id);
		} else {
			selectedItems.add(id);
		}
	}

	function toggleCategory(category: string) {
		if (expandedCategories.has(category)) {
			expandedCategories.delete(category);
		} else {
			expandedCategories.add(category);
		}
	}

	function selectAllInCategory(items: PreviewItem[]) {
		items.forEach(item => selectedItems.add(item.id));
	}

	function deselectAllInCategory(items: PreviewItem[]) {
		items.forEach(item => selectedItems.delete(item.id));
	}

	function getSelectedSize(): number {
		if (!preview) return 0;

		const allItems = [
			...preview.cache_items,
			...preview.log_items,
			...preview.filesystem_items,
			...preview.storage_items,
		];

		return allItems
			.filter(item => selectedItems.has(item.id))
			.reduce((sum, item) => sum + item.size, 0);
	}

	async function handleConfirm() {
		if (!preview) return;

		const allItems = [
			...preview.cache_items,
			...preview.log_items,
			...preview.filesystem_items,
			...preview.storage_items,
		];

		const selected = allItems.filter(item => selectedItems.has(item.id));

		if (selected.length === 0) {
			notificationStore.warning('No Items Selected', 'Please select at least one item to clean');
			return;
		}

		await onConfirm(selected);
	}

	function getRiskColor(risk: number): string {
		if (risk === 0) return 'text-green-600 bg-green-50 border-green-200 dark:text-green-400 dark:bg-green-900/20 dark:border-green-800';
		if (risk === 1) return 'text-yellow-600 bg-yellow-50 border-yellow-200 dark:text-yellow-400 dark:bg-yellow-900/20 dark:border-yellow-800';
		return 'text-red-600 bg-red-50 border-red-200 dark:text-red-400 dark:bg-red-900/20 dark:border-red-800';
	}
</script>

{#if preview}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-[var(--color-bg)] rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] flex flex-col">
			<div class="p-6 border-b border-[var(--color-border)]">
				<h2 class="text-2xl font-bold mb-2">Preview Cleanup</h2>
				<p class="text-[var(--color-text-secondary)]">
					Review items before cleaning. {selectedItems.size} of {preview.total_items} items selected
					({formatBytes(getSelectedSize())})
				</p>
			</div>

			<div class="flex-1 overflow-y-auto p-6 space-y-4">
				<!-- Cache Items -->
				{#if preview.cache_items.length > 0}
					<div class="border border-[var(--color-border)] rounded-lg">
						<div
							class="p-4 bg-gray-50 dark:bg-gray-900/50 flex items-center justify-between cursor-pointer"
							onclick={() => toggleCategory('cache')}
						>
							<div class="flex items-center gap-3">
								<span class="text-lg">{expandedCategories.has('cache') ? '▼' : '▶'}</span>
								<h3 class="font-semibold">Cache Files ({preview.cache_items.length})</h3>
							</div>
							<div class="flex gap-2">
								<button
									class="text-xs px-2 py-1 bg-blue-100 hover:bg-blue-200 dark:bg-blue-900/50 dark:hover:bg-blue-800 rounded"
									onclick={(event) => { event.stopPropagation(); selectAllInCategory(preview.cache_items); }}
								>
									Select All
								</button>
								<button
									class="text-xs px-2 py-1 bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 rounded"
									onclick={(event) => { event.stopPropagation(); deselectAllInCategory(preview.cache_items); }}
								>
									Deselect All
								</button>
							</div>
						</div>
						{#if expandedCategories.has('cache')}
							<div class="divide-y max-h-64 overflow-y-auto">
								{#each preview.cache_items as item}
									<div class="p-3 flex items-center gap-3 hover:bg-gray-50 dark:hover:bg-gray-900/50">
										<input
											type="checkbox"
											checked={selectedItems.has(item.id)}
											onchange={() => toggleItem(item.id)}
											class="w-4 h-4"
										/>
										<div class="flex-1 min-w-0">
											<div class="font-medium truncate">{item.name}</div>
											<div class="text-sm text-[var(--color-text-secondary)] truncate">{item.path}</div>
										</div>
										<div class="text-right">
											<div class="font-medium">{formatBytes(item.size)}</div>
											<div class="text-xs text-[var(--color-text-secondary)]">{item.description}</div>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}

				<!-- Log Items -->
				{#if preview.log_items.length > 0}
					<div class="border border-[var(--color-border)] rounded-lg">
						<div
							class="p-4 bg-gray-50 dark:bg-gray-900/50 flex items-center justify-between cursor-pointer"
							onclick={() => toggleCategory('logs')}
						>
							<div class="flex items-center gap-3">
								<span class="text-lg">{expandedCategories.has('logs') ? '▼' : '▶'}</span>
								<h3 class="font-semibold">Log Files ({preview.log_items.length})</h3>
							</div>
							<div class="flex gap-2">
								<button
									class="text-xs px-2 py-1 bg-blue-100 hover:bg-blue-200 dark:bg-blue-900/50 dark:hover:bg-blue-800 rounded"
									onclick={(event) => { event.stopPropagation(); selectAllInCategory(preview.log_items); }}
								>
									Select All
								</button>
								<button
									class="text-xs px-2 py-1 bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 rounded"
									onclick={(event) => { event.stopPropagation(); deselectAllInCategory(preview.log_items); }}
								>
									Deselect All
								</button>
							</div>
						</div>
						{#if expandedCategories.has('logs')}
							<div class="divide-y max-h-64 overflow-y-auto">
								{#each preview.log_items as item}
									<div class="p-3 flex items-center gap-3 hover:bg-gray-50 dark:hover:bg-gray-900/50">
										<input
											type="checkbox"
											checked={selectedItems.has(item.id)}
											onchange={() => toggleItem(item.id)}
											class="w-4 h-4"
										/>
										<div class="flex-1 min-w-0">
											<div class="font-medium truncate">{item.name}</div>
											<div class="text-sm text-[var(--color-text-secondary)] truncate">{item.path}</div>
										</div>
										<div class="text-right">
											<div class="font-medium">{formatBytes(item.size)}</div>
											<div class="text-xs text-[var(--color-text-secondary)]">{item.description}</div>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}

				<!-- Filesystem Items -->
				{#if preview.filesystem_items.length > 0}
					<div class="border border-[var(--color-border)] rounded-lg">
						<div
							class="p-4 bg-gray-50 dark:bg-gray-900/50 flex items-center justify-between cursor-pointer"
							onclick={() => toggleCategory('filesystem')}
						>
							<div class="flex items-center gap-3">
								<span class="text-lg">{expandedCategories.has('filesystem') ? '▼' : '▶'}</span>
								<h3 class="font-semibold">Filesystem Health ({preview.filesystem_items.length})</h3>
							</div>
							<div class="flex gap-2">
								<button
									class="text-xs px-2 py-1 bg-blue-100 hover:bg-blue-200 dark:bg-blue-900/50 dark:hover:bg-blue-800 rounded"
									onclick={(event) => { event.stopPropagation(); selectAllInCategory(preview.filesystem_items); }}
								>
									Select All
								</button>
								<button
									class="text-xs px-2 py-1 bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 rounded"
									onclick={(event) => { event.stopPropagation(); deselectAllInCategory(preview.filesystem_items); }}
								>
									Deselect All
								</button>
							</div>
						</div>
						{#if expandedCategories.has('filesystem')}
							<div class="divide-y max-h-64 overflow-y-auto">
								{#each preview.filesystem_items as item}
									<div class="p-3 flex items-center gap-3 hover:bg-gray-50 dark:hover:bg-gray-900/50">
										<input
											type="checkbox"
											checked={selectedItems.has(item.id)}
											onchange={() => toggleItem(item.id)}
											class="w-4 h-4"
										/>
										<div class="flex-1 min-w-0">
											<div class="font-medium truncate">{item.name}</div>
											<div class="text-sm text-[var(--color-text-secondary)] truncate">{item.path}</div>
										</div>
										<div class="text-right">
											<div class="font-medium">{formatBytes(item.size)}</div>
											<div class="text-xs px-2 py-1 rounded {getRiskColor(item.risk_level)}">
												Risk: {item.risk_level}
											</div>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}

				<!-- Storage Items -->
				{#if preview.storage_items.length > 0}
					<div class="border border-[var(--color-border)] rounded-lg">
						<div
							class="p-4 bg-gray-50 dark:bg-gray-900/50 flex items-center justify-between cursor-pointer"
							onclick={() => toggleCategory('storage')}
						>
							<div class="flex items-center gap-3">
								<span class="text-lg">{expandedCategories.has('storage') ? '▼' : '▶'}</span>
								<h3 class="font-semibold">Storage Recovery ({preview.storage_items.length})</h3>
							</div>
							<div class="flex gap-2">
								<button
									class="text-xs px-2 py-1 bg-blue-100 hover:bg-blue-200 dark:bg-blue-900/50 dark:hover:bg-blue-800 rounded"
									onclick={(event) => { event.stopPropagation(); selectAllInCategory(preview.storage_items); }}
								>
									Select All
								</button>
								<button
									class="text-xs px-2 py-1 bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 rounded"
									onclick={(event) => { event.stopPropagation(); deselectAllInCategory(preview.storage_items); }}
								>
									Deselect All
								</button>
							</div>
						</div>
						{#if expandedCategories.has('storage')}
							<div class="divide-y max-h-64 overflow-y-auto">
								{#each preview.storage_items as item}
									<div class="p-3 flex items-center gap-3 hover:bg-gray-50 dark:hover:bg-gray-900/50">
										<input
											type="checkbox"
											checked={selectedItems.has(item.id)}
											onchange={() => toggleItem(item.id)}
											class="w-4 h-4"
										/>
										<div class="flex-1 min-w-0">
											<div class="font-medium truncate">{item.name}</div>
											<div class="text-sm text-[var(--color-text-secondary)] truncate">{item.path}</div>
										</div>
										<div class="text-right">
											<div class="font-medium">{formatBytes(item.size)}</div>
											<div class="text-xs text-[var(--color-text-secondary)] capitalize">{item.category.replace('_', ' ')}</div>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</div>

			<div class="p-6 border-t border-[var(--color-border)] flex items-center justify-between">
				<div>
					<div class="font-semibold">{formatBytes(getSelectedSize())} selected</div>
					<div class="text-sm text-[var(--color-text-secondary)]">{selectedItems.size} items</div>
				</div>
				<div class="flex gap-3">
					<button class="btn btn-secondary" onclick={() => preview = null}>
						Cancel
					</button>
					<button
						class="btn btn-primary"
						onclick={handleConfirm}
						disabled={selectedItems.size === 0}
					>
						Clean Selected ({selectedItems.size})
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}
