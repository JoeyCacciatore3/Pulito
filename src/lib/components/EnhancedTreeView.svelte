<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke, formatBytes } from '$lib/utils/tauri';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import LoadingSpinner from './ui/LoadingSpinner.svelte';
	import TreeNode from './TreeNode.svelte';
	import TreemapView from './TreemapView.svelte';
	import TreemapLegend from './TreemapLegend.svelte';
	import TreemapStatsPanel from './TreemapStatsPanel.svelte';
	import SizeBar from './ui/SizeBar.svelte';
	import type { TreeNode as GeneratedTreeNode } from '$lib/generated/types';

	interface TreeNode {
		id: string;
		name: string;
		path: string;
		size: number;
		isDirectory: boolean;
		lastModified: number;
		lastAccessed: number;
		children?: TreeNode[];
		expanded: boolean;
		selected: boolean;
		riskLevel: 'safe' | 'caution' | 'warning';
		aiInsight?: string;
		usagePattern?: 'frequent' | 'occasional' | 'rare' | 'never';
		parent?: TreeNode;
	}

	interface TreeViewOptions {
		rootPath: string;
		maxDepth: number;
		includeHidden: boolean;
		sizeThreshold: number; // Only show files above this size
		filterPatterns: string[]; // File patterns to include/exclude
	}

	let treeData = $state<TreeNode[]>([]);
	let loading = $state(false);
	let searchQuery = $state('');
	let viewMode = $state<'tree' | 'list' | 'treemap'>('tree');
	let sortBy = $state<'name' | 'size' | 'date' | 'risk'>('size');
	let sortOrder = $state<'asc' | 'desc'>('desc');
	let treemapColorMode = $state<'size' | 'risk' | 'usage' | 'filetype'>('size');
	let isFullscreen = $state(false);
	let selectedNodeIds = $derived(() => {
		const ids = new Set<string>();
		function collectSelected(node: TreeNode) {
			if (node.selected) {
				ids.add(node.id);
			}
			if (node.children) {
				node.children.forEach(collectSelected);
			}
		}
		treeData.forEach(collectSelected);
		return ids;
	});

	let options = $state<TreeViewOptions>({
		rootPath: '~',
		maxDepth: 3,
		includeHidden: false,
		sizeThreshold: 1024 * 1024, // 1MB
		filterPatterns: []
	});

	// Helper function to flatten tree into list
	function getAllNodes(nodes: TreeNode[]): TreeNode[] {
		const result: TreeNode[] = [];

		function traverse(node: TreeNode) {
			result.push(node);
			if (node.children && node.expanded) {
				node.children.forEach(traverse);
			}
		}

		nodes.forEach(traverse);
		return result;
	}

	// Statistics for the current view
	let stats = $state({
		totalFiles: 0,
		totalDirectories: 0,
		totalSize: 0,
		selectedCount: 0,
		selectedSize: 0
	});

	// Extract error message from various error types
	function extractErrorMessage(error: unknown): string {
		if (typeof error === 'string') {
			return error;
		}
		if (error instanceof Error) {
			return error.message;
		}
		if (error && typeof error === 'object') {
			const errObj = error as { message?: string; originalError?: unknown };
			if (errObj.message) return errObj.message;
			if (errObj.originalError) {
				if (typeof errObj.originalError === 'string') {
					return errObj.originalError;
				}
				if (errObj.originalError instanceof Error) {
					return errObj.originalError.message;
				}
			}
			try {
				return JSON.stringify(error);
			} catch {
				return String(error);
			}
		}
		return 'Unknown error occurred';
	}

	async function loadTreeData() {
		try {
			loading = true;
			console.log('Loading tree data with options:', options);

			const data = await invoke<TreeNode[]>('scan_filesystem_tree', {
				rootPath: options.rootPath,
				maxDepth: options.maxDepth,
				includeHidden: options.includeHidden,
				sizeThreshold: options.sizeThreshold,
				filterPatterns: options.filterPatterns
			});

			// Enhance data with AI insights
			const enhancedData = await enhanceWithAIInsights(data);

			treeData = enhancedData;
			updateStats();

			console.log('Tree data loaded:', treeData.length, 'items');
		} catch (error) {
			console.error('Failed to load tree data:', error);
			logger.error('Failed to load tree data', { component: 'EnhancedTreeView', options }, error);
			const errorMessage = extractErrorMessage(error);
			notificationStore.error('Tree Load Failed', `Could not load file tree structure: ${errorMessage}`);
		} finally {
			loading = false;
		}
	}

	async function enhanceWithAIInsights(nodes: TreeNode[]): Promise<TreeNode[]> {
		// Simulate AI analysis (in real implementation, this would call an AI service)
		return nodes.map(node => {
			// Convert Unix timestamp (seconds) to milliseconds for Date calculations
			const lastAccessedMs = node.lastAccessed * 1000;
			const daysSinceAccess = (Date.now() - lastAccessedMs) / (1000 * 60 * 60 * 24);

			let insight = '';
			let usagePattern: TreeNode['usagePattern'] = 'occasional';

			if (daysSinceAccess > 365) {
				insight = 'Not accessed in over a year - potential cleanup candidate';
				usagePattern = 'never';
			} else if (daysSinceAccess > 90) {
				insight = 'Not accessed in 3+ months - consider archiving';
				usagePattern = 'rare';
			} else if (daysSinceAccess > 7) {
				insight = 'Accessed within last 3 months';
				usagePattern = 'occasional';
			} else {
				insight = 'Recently accessed - likely important';
				usagePattern = 'frequent';
			}

			return {
				...node,
				aiInsight: insight,
				usagePattern
			};
		});
	}

	function updateStats() {
		let totalFiles = 0;
		let totalDirectories = 0;
		let totalSize = 0;
		let selectedCount = 0;
		let selectedSize = 0;

		function traverse(node: TreeNode) {
			if (node.isDirectory) {
				totalDirectories++;
			} else {
				totalFiles++;
				totalSize += node.size;
			}

			if (node.selected) {
				selectedCount++;
				selectedSize += node.size;
			}

			if (node.children) {
				node.children.forEach(traverse);
			}
		}

		treeData.forEach(traverse);

		stats = { totalFiles, totalDirectories, totalSize, selectedCount, selectedSize };
	}

	function toggleNodeExpansion(node: TreeNode) {
		node.expanded = !node.expanded;
		// Force reactivity update
		treeData = [...treeData];
	}

	function toggleNodeSelection(node: TreeNode, selected: boolean) {
		node.selected = selected;

		// If it's a directory, optionally select/deselect children
		if (node.isDirectory && node.children) {
			node.children.forEach(child => toggleNodeSelection(child, selected));
		}

		updateStats();
		// Force reactivity update
		treeData = [...treeData];
	}

	function getFilteredNodes(): TreeNode[] {
		if (!searchQuery) return treeData;

		function filterNode(node: TreeNode): TreeNode | null {
			const matchesSearch = node.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
								node.path.toLowerCase().includes(searchQuery.toLowerCase());

			if (node.children) {
				const filteredChildren = node.children
					.map(filterNode)
					.filter(child => child !== null) as TreeNode[];

				if (filteredChildren.length > 0 || matchesSearch) {
					return {
						...node,
						children: filteredChildren,
						expanded: true // Expand filtered nodes
					};
				}
			}

			return matchesSearch ? node : null;
		}

		return treeData
			.map(filterNode)
			.filter(node => node !== null) as TreeNode[];
	}

	function getSortedNodes(nodes: TreeNode[]): TreeNode[] {
		const sorted = [...nodes].sort((a, b) => {
			let comparison = 0;

			switch (sortBy) {
				case 'name':
					comparison = a.name.localeCompare(b.name);
					break;
				case 'size':
					comparison = a.size - b.size;
					break;
				case 'date':
					comparison = a.lastModified - b.lastModified;
					break;
				case 'risk': {
					const riskOrder = { safe: 0, caution: 1, warning: 2 };
					comparison = riskOrder[a.riskLevel] - riskOrder[b.riskLevel];
					break;
				}
			}

			return sortOrder === 'asc' ? comparison : -comparison;
		});

		// Sort children recursively
		return sorted.map(node => ({
			...node,
			children: node.children ? getSortedNodes(node.children) : undefined
		}));
	}

	function getRiskColor(risk: string): string {
		switch (risk) {
			case 'safe': return 'text-green-600 border-green-200';
			case 'caution': return 'text-yellow-600 border-yellow-200';
			case 'warning': return 'text-red-600 border-red-200';
			default: return 'text-gray-600 border-gray-200';
		}
	}

	function getUsageColor(pattern: string): string {
		switch (pattern) {
			case 'frequent': return 'bg-green-100 dark:bg-green-900/30';
			case 'occasional': return 'bg-blue-100 dark:bg-blue-900/30';
			case 'rare': return 'bg-yellow-100 dark:bg-yellow-900/30';
			case 'never': return 'bg-red-100 dark:bg-red-900/30';
			default: return 'bg-gray-100 dark:bg-gray-900/30';
		}
	}

	onMount(() => {
		loadTreeData();
	});

	// Use $derived for reactive computed values
	let filteredNodes = $derived(getFilteredNodes());
	let sortedNodes = $derived(getSortedNodes(filteredNodes));
</script>

<div class="space-y-4">
	<!-- Header with Controls -->
	<div class="flex items-center justify-between">
		<div>
			<h2 class="text-xl font-semibold">Enhanced File Explorer</h2>
			<p class="text-sm text-muted">AI-powered visual file management with granular control</p>
		</div>

		<div class="flex gap-2">
			<button
				class="btn {viewMode === 'tree' ? 'btn-primary' : 'btn-secondary'}"
				onclick={() => viewMode = 'tree'}
			>
				🌳 Tree
			</button>
			<button
				class="btn {viewMode === 'list' ? 'btn-primary' : 'btn-secondary'}"
				onclick={() => viewMode = 'list'}
			>
				📋 List
			</button>
			<button
				class="btn {viewMode === 'treemap' ? 'btn-primary' : 'btn-secondary'}"
				onclick={() => viewMode = 'treemap'}
			>
				📊 Map
			</button>
		</div>
	</div>

	<!-- Search and Filters -->
	<div class="flex gap-4 items-center">
		<div class="flex-1">
			<input
				type="text"
				placeholder="Search files and folders..."
				bind:value={searchQuery}
				class="input w-full"
			/>
		</div>

		<div class="flex gap-2 items-center">
			<label for="sort-select" class="text-sm">Sort by:</label>
			<select id="sort-select" bind:value={sortBy} class="select">
				<option value="size">Size</option>
				<option value="name">Name</option>
				<option value="date">Date</option>
				<option value="risk">Risk</option>
			</select>

			<button
				class="btn btn-secondary"
				onclick={() => sortOrder = sortOrder === 'asc' ? 'desc' : 'asc'}
			>
				{sortOrder === 'asc' ? '↑' : '↓'}
			</button>
		</div>
	</div>

	<!-- Statistics Bar -->
	<div class="card p-4 bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-900/30 dark:to-indigo-900/30">
		<div class="grid grid-cols-2 md:grid-cols-5 gap-4 text-sm">
			<div>
				<div class="font-medium">{stats.totalFiles.toLocaleString()}</div>
				<div class="text-muted">Files</div>
			</div>
			<div>
				<div class="font-medium">{stats.totalDirectories.toLocaleString()}</div>
				<div class="text-muted">Folders</div>
			</div>
			<div>
				<div class="font-medium">{formatBytes(stats.totalSize)}</div>
				<div class="text-muted">Total Size</div>
			</div>
			<div>
				<div class="font-medium text-primary">{stats.selectedCount}</div>
				<div class="text-muted">Selected</div>
			</div>
			<div>
				<div class="font-medium text-primary">{formatBytes(stats.selectedSize)}</div>
				<div class="text-muted">Selected Size</div>
			</div>
		</div>
	</div>

	<!-- Tree View Content -->
	<div class="card p-4">
		{#if loading}
			<div class="flex items-center justify-center h-64">
				<div class="flex flex-col items-center gap-4">
					<LoadingSpinner size="lg" />
					<p class="text-muted">Analyzing file system...</p>
				</div>
			</div>
		{:else if sortedNodes.length === 0}
			<div class="text-center py-8">
				<div class="text-4xl mb-4">📁</div>
				<p class="text-muted">No files found matching your criteria</p>
				<button class="btn btn-primary mt-4" onclick={loadTreeData}>
					🔄 Refresh
				</button>
			</div>
		{:else}
			<div class="max-h-96 overflow-y-auto">
				{#if viewMode === 'tree'}
					<!-- Tree View -->
					<div class="space-y-1">
						{#each sortedNodes as node}
							<TreeNode
								{node}
								{toggleNodeExpansion}
								{toggleNodeSelection}
								{getRiskColor}
								{getUsageColor}
							/>
						{/each}
					</div>
				{:else if viewMode === 'list'}
					<!-- List View -->
					<div class="space-y-2">
						{#each getAllNodes(sortedNodes) as node}
							<div class="flex items-center gap-3 p-2 hover:bg-gray-50 dark:hover:bg-gray-900/50 rounded">
								<input
									type="checkbox"
									bind:checked={node.selected}
									onchange={() => toggleNodeSelection(node, node.selected)}
									class="w-4 h-4"
								/>
								<div class="w-6 flex-shrink-0">
									{node.isDirectory ? '📁' : '📄'}
								</div>
								<div class="flex-1 min-w-0">
									<div class="font-medium truncate">{node.name}</div>
									<div class="text-xs text-muted truncate">{node.path}</div>
								</div>
								<div class="text-right text-sm">
									<div class="font-medium">{formatBytes(node.size)}</div>
									<div class="text-xs {getRiskColor(node.riskLevel)}">
										{node.riskLevel}
									</div>
								</div>
							</div>
						{/each}
					</div>
				{:else if viewMode === 'treemap'}
					<!-- Treemap View -->
					<div class="space-y-2 {isFullscreen ? 'fixed inset-0 z-50 bg-[var(--color-bg)] p-4' : ''}">
						{#if isFullscreen}
							<div class="flex items-center justify-between mb-4">
								<h3 class="text-lg font-semibold">Treemap Visualization</h3>
								<button
									class="btn btn-secondary"
									onclick={() => isFullscreen = false}
									aria-label="Exit fullscreen"
								>
									✕ Close
								</button>
							</div>
						{/if}
						<div class="flex gap-4 {isFullscreen ? 'h-[calc(100vh-120px)]' : ''}">
							<!-- Main Treemap Area -->
							<div class="flex-1 flex flex-col {isFullscreen ? 'min-h-0' : 'min-h-[600px]'}">
								<!-- Controls -->
								<div class="flex items-center justify-between mb-2">
									<div class="flex items-center gap-2">
										<label for="color-mode-select" class="text-sm text-muted">Color by:</label>
										<select
											id="color-mode-select"
											bind:value={treemapColorMode}
											class="select text-sm"
										>
											<option value="size">Size</option>
											<option value="risk">Risk Level</option>
											<option value="usage">Usage Pattern</option>
											<option value="filetype">File Type</option>
										</select>
									</div>
									{#if !isFullscreen}
										<button
											class="btn btn-secondary text-sm"
											onclick={() => isFullscreen = true}
											aria-label="Enter fullscreen"
										>
											⛶ Fullscreen
										</button>
									{/if}
								</div>
								<!-- Treemap Container -->
								<div class="flex-1 border border-[var(--color-border)] rounded-lg overflow-hidden bg-[var(--color-bg-secondary)]">
									<TreemapView
										nodes={sortedNodes as unknown as GeneratedTreeNode[]}
										onNodeClick={(node) => toggleNodeSelection(node as unknown as TreeNode, !(node as unknown as TreeNode).selected)}
										colorMode={treemapColorMode}
										minNodeSize={8}
										padding={2}
									/>
								</div>
								<!-- Legend -->
								<TreemapLegend
									colorMode={treemapColorMode}
									isDark={typeof document !== 'undefined' && document.documentElement.classList.contains('dark')}
									totalSize={stats.totalSize}
									collapsible={true}
								/>
							</div>
							<!-- Statistics Panel -->
							<div class="w-64 flex-shrink-0">
								<TreemapStatsPanel
									nodes={sortedNodes as unknown as GeneratedTreeNode[]}
									selectedNodes={selectedNodeIds()}
									collapsible={true}
								/>
							</div>
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>


<style>
	.tree-node {
		transition: background-color 0.15s ease;
	}
</style>
