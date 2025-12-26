<script lang="ts">
	import { formatBytes } from '$lib/utils/tauri';
	import { calculateStatistics, type TreemapStatistics } from '$lib/utils/treemap-utils';
	import type { TreeNode as GeneratedTreeNode } from '$lib/generated/types';

	interface Props {
		nodes: GeneratedTreeNode[];
		selectedNodes?: Set<string>;
		currentPath?: string;
		zoomLevel?: number;
		collapsible?: boolean;
	}

	let {
		nodes = [],
		selectedNodes = new Set(),
		currentPath,
		zoomLevel = 1,
		collapsible = false
	}: Props = $props();

	let collapsed = $state(false);

	// Calculate statistics
	let stats = $derived.by(() => {
		if (nodes.length === 0) {
			return null;
		}
		return calculateStatistics(nodes);
	});

	// Calculate selected items summary
	let selectedSummary = $derived.by(() => {
		if (selectedNodes.size === 0 || !stats) {
			return null;
		}

		const selectedItems = nodes.filter((node) => {
			if (selectedNodes.has(node.id)) {
				return true;
			}
			// Check children recursively
			if (node.children) {
				return node.children.some((child) => selectedNodes.has(child.id));
			}
			return false;
		});

		let totalSize = 0;
		let fileCount = 0;
		let folderCount = 0;

		function processNode(node: GeneratedTreeNode): number {
			if (node.isDirectory) {
				folderCount++;
				if (node.children && node.children.length > 0) {
					return node.children.reduce((sum, child) => sum + processNode(child), 0);
				}
				return 0;
			} else {
				fileCount++;
				totalSize += node.size;
				return node.size;
			}
		}

		selectedItems.forEach((item) => {
			if (selectedNodes.has(item.id)) {
				processNode(item);
			}
		});

		return {
			count: selectedItems.length,
			totalSize,
			fileCount,
			folderCount
		};
	});
</script>

<div class="treemap-stats-panel bg-[var(--color-bg-secondary)] border border-[var(--color-border)] rounded-lg p-4 shadow-sm">
	{#if collapsible}
		<button
			class="flex items-center justify-between w-full mb-3 text-sm font-semibold text-[var(--color-text-primary)] hover:text-[var(--color-text)]"
			onclick={() => (collapsed = !collapsed)}
			aria-expanded={!collapsed}
			aria-label="Toggle statistics panel"
		>
			<span>Statistics</span>
			<span class="text-xs">{collapsed ? '▼' : '▲'}</span>
		</button>
	{/if}

	{#if !collapsed}
		{#if stats}
			<div class="space-y-4">
				<!-- Overall Statistics -->
				<div class="space-y-2">
					<h3 class="text-xs font-semibold text-[var(--color-text-secondary)] uppercase tracking-wide">
						Overall
					</h3>
					<div class="grid grid-cols-2 gap-3">
						<div>
							<div class="text-xs text-[var(--color-text-secondary)]">Total Size</div>
							<div class="text-sm font-semibold text-[var(--color-text-primary)]">
								{formatBytes(stats.totalSize)}
							</div>
						</div>
						<div>
							<div class="text-xs text-[var(--color-text-secondary)]">Files</div>
							<div class="text-sm font-semibold text-[var(--color-text-primary)]">
								{stats.fileCount.toLocaleString()}
							</div>
						</div>
						<div>
							<div class="text-xs text-[var(--color-text-secondary)]">Folders</div>
							<div class="text-sm font-semibold text-[var(--color-text-primary)]">
								{stats.folderCount.toLocaleString()}
							</div>
						</div>
						{#if zoomLevel > 1}
							<div>
								<div class="text-xs text-[var(--color-text-secondary)]">Zoom</div>
								<div class="text-sm font-semibold text-[var(--color-text-primary)]">
									{zoomLevel.toFixed(1)}x
								</div>
							</div>
						{/if}
					</div>
				</div>

				<!-- Selected Items Summary -->
				{#if selectedSummary && selectedSummary.count > 0}
					<div class="space-y-2 border-t border-[var(--color-border)] pt-3">
						<h3 class="text-xs font-semibold text-[var(--color-text-secondary)] uppercase tracking-wide">
							Selected ({selectedSummary.count})
						</h3>
						<div class="grid grid-cols-2 gap-3">
							<div>
								<div class="text-xs text-[var(--color-text-secondary)]">Size</div>
								<div class="text-sm font-semibold text-[var(--color-text-primary)]">
									{formatBytes(selectedSummary.totalSize)}
								</div>
							</div>
							<div>
								<div class="text-xs text-[var(--color-text-secondary)]">Files</div>
								<div class="text-sm font-semibold text-[var(--color-text-primary)]">
									{selectedSummary.fileCount.toLocaleString()}
								</div>
							</div>
						</div>
					</div>
				{/if}

				<!-- Current Path (if zoomed) -->
				{#if currentPath && currentPath !== '/'}
					<div class="space-y-2 border-t border-[var(--color-border)] pt-3">
						<h3 class="text-xs font-semibold text-[var(--color-text-secondary)] uppercase tracking-wide">
							Current View
						</h3>
						<div class="text-xs text-[var(--color-text-secondary)] truncate" title={currentPath}>
							{currentPath}
						</div>
					</div>
				{/if}

				<!-- Top 5 Largest Files -->
				{#if stats.largestFiles.length > 0}
					<div class="space-y-2 border-t border-[var(--color-border)] pt-3">
						<h3 class="text-xs font-semibold text-[var(--color-text-secondary)] uppercase tracking-wide">
							Largest Files
						</h3>
						<div class="space-y-1">
							{#each stats.largestFiles.slice(0, 5) as file}
								<div class="flex items-center justify-between text-xs">
									<span class="text-[var(--color-text-secondary)] truncate flex-1 mr-2" title={file.path}>
										{file.name}
									</span>
									<span class="text-[var(--color-text-primary)] font-medium flex-shrink-0">
										{formatBytes(file.size)}
									</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Top 5 Largest Directories -->
				{#if stats.largestDirectories.length > 0}
					<div class="space-y-2 border-t border-[var(--color-border)] pt-3">
						<h3 class="text-xs font-semibold text-[var(--color-text-secondary)] uppercase tracking-wide">
							Largest Directories
						</h3>
						<div class="space-y-1">
							{#each stats.largestDirectories.slice(0, 5) as dir}
								<div class="flex items-center justify-between text-xs">
									<span class="text-[var(--color-text-secondary)] truncate flex-1 mr-2" title={dir.path}>
										{dir.name}
									</span>
									<span class="text-[var(--color-text-primary)] font-medium flex-shrink-0">
										{formatBytes(dir.size)}
									</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{:else}
			<div class="text-xs text-[var(--color-text-secondary)] text-center py-4">
				No data available
			</div>
		{/if}
	{/if}
</div>

<style>
	.treemap-stats-panel {
		user-select: none;
		max-height: calc(100vh - 200px);
		overflow-y: auto;
	}
</style>
