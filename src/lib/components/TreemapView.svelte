<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		convertToD3Hierarchy,
		calculateTreemapLayout,
		filterVisibleNodes,
		getNodeColor,
		getTreemapNodes,
		findNodeById,
		calculateSizePercentage,
		calculateTotalSize
	} from '$lib/utils/treemap-utils';
	import { formatBytes } from '$lib/utils/tauri';
	import { getFileExtension, getFileTypeFromPath } from '$lib/utils/file-types';
	import type { TreeNode as GeneratedTreeNode } from '$lib/generated/types';
	import type { TreeNode as InternalTreeNode } from '$lib/utils/treemap-utils';

	// Type guard to ensure proper riskLevel and usagePattern
	function ensureTreeNode(node: GeneratedTreeNode): InternalTreeNode {
		const validUsagePatterns = ['frequent', 'occasional', 'rare', 'never'] as const;
		const usagePattern = node.usagePattern && validUsagePatterns.includes(node.usagePattern as any)
			? (node.usagePattern as 'frequent' | 'occasional' | 'rare' | 'never')
			: undefined;

		return {
			...node,
			riskLevel: (node.riskLevel === 'safe' || node.riskLevel === 'caution' || node.riskLevel === 'warning'
				? node.riskLevel
				: 'safe') as 'safe' | 'caution' | 'warning',
			usagePattern,
			children: node.children ? node.children.map(ensureTreeNode) : undefined
		};
	}

	interface Props {
		nodes: Array<{
			id: string;
			name: string;
			path: string;
			size: number;
			isDirectory: boolean;
			lastModified: number;
			lastAccessed: number;
			children?: any[];
			expanded: boolean;
			selected: boolean;
			riskLevel: string;
			usagePattern?: string;
		}>;
		onNodeClick?: (node: any) => void;
		onNodeHover?: (node: any | null) => void;
		colorMode?: 'size' | 'risk' | 'usage' | 'filetype';
		minNodeSize?: number;
		padding?: number;
		aggregationThreshold?: number;
		maxItemsPerLevel?: number;
		maxDepth?: number;
	}

	let {
		nodes = $bindable([] as GeneratedTreeNode[]),
		onNodeClick,
		onNodeHover,
		colorMode = $bindable('size'),
		minNodeSize = 8, // Increased from 4px to 8px
		padding = 2
	}: Props = $props();

	let containerRef = $state<HTMLDivElement | null>(null);
	let svgRef = $state<SVGSVGElement | null>(null);
	let hoveredNode = $state<GeneratedTreeNode | null>(null);
	let tooltipPosition = $state({ x: 0, y: 0 });
	let zoomStack = $state<GeneratedTreeNode[]>([]); // Store as GeneratedTreeNode for compatibility
	let isDark = $state(false);
	let containerSize = $state({ width: 800, height: 600 });
	let isPanning = $state(false);
	let panStart = $state({ x: 0, y: 0 });
	let panOffset = $state({ x: 0, y: 0 });
	let zoomLevel = $state(1);
	let totalSize = $derived(() => {
		if (!displayNodes() || displayNodes().length === 0) {
			return 0;
		}
		const hierarchy = convertToD3Hierarchy(displayNodes());
		if (!hierarchy) {
			return 0;
		}
		return calculateTotalSize(hierarchy);
	});

	// Get current root node (either zoomed node or original root)
	let currentRoot = $derived((): GeneratedTreeNode | null => {
		if (zoomStack.length > 0) {
			return zoomStack[zoomStack.length - 1];
		}
		return null;
	});

	// Update zoom level
	$effect(() => {
		zoomLevel = zoomStack.length + 1;
	});

	// Get nodes to display (either zoomed subtree or all nodes)
	// Convert GeneratedTreeNode to internal TreeNode format
	let displayNodes = $derived(() => {
		const convertNodes = (nodeList: GeneratedTreeNode[]): InternalTreeNode[] => {
			return nodeList.map(ensureTreeNode);
		};

		if (currentRoot()) {
			// Show only children of zoomed node
			return convertNodes(currentRoot()?.children || []);
		}
		return convertNodes(nodes);
	});

	// Calculate treemap layout
	let layoutNodes = $derived(() => {
		if (!displayNodes() || displayNodes().length === 0) {
			return [];
		}

		// Convert to D3 hierarchy
		const hierarchy = convertToD3Hierarchy(displayNodes());
		if (!hierarchy) {
			return [];
		}

		// Calculate layout
		const layout = calculateTreemapLayout(
			hierarchy,
			containerSize.width,
			containerSize.height,
			padding
		);

		if (!layout) {
			return [];
		}

		// Get all nodes for rendering
		const allNodes = getTreemapNodes(layout);

		// Filter visible nodes
		return filterVisibleNodes(allNodes, minNodeSize);
	});

	// Handle container resize
	function handleResize() {
		if (containerRef) {
			const rect = containerRef.getBoundingClientRect();
			containerSize = {
				width: rect.width - 20, // Account for padding
				height: rect.height - 20
			};
		}
	}

	// Handle node hover
	function handleNodeMouseEnter(
		event: MouseEvent,
		node: { data: GeneratedTreeNode; x0?: number; y0?: number; x1?: number; y1?: number }
	) {
		hoveredNode = node.data;
		// Position tooltip relative to mouse, but keep it within viewport
		if (containerRef) {
			const rect = containerRef.getBoundingClientRect();
			tooltipPosition = {
				x: event.clientX - rect.left,
				y: event.clientY - rect.top
			};
		} else {
			tooltipPosition = { x: event.clientX, y: event.clientY };
		}
		if (onNodeHover) {
			onNodeHover(node.data);
		}
	}

	function handleNodeMouseLeave() {
		hoveredNode = null;
		if (onNodeHover) {
			onNodeHover(null);
		}
	}

	// Handle node click
	function handleNodeClick(node: { data: GeneratedTreeNode }) {
		if (onNodeClick) {
			onNodeClick(node.data);
		}
	}

	// Handle double-click to zoom
	function handleNodeDoubleClick(node: { data: GeneratedTreeNode }) {
		if (node.data.isDirectory && node.data.children && node.data.children.length > 0) {
			zoomStack = [...zoomStack, node.data];
			panOffset = { x: 0, y: 0 }; // Reset pan when zooming
		}
	}

	// Handle pan start
	function handlePanStart(event: MouseEvent) {
		isPanning = true;
		panStart = { x: event.clientX - panOffset.x, y: event.clientY - panOffset.y };
	}

	// Handle pan move
	function handlePanMove(event: MouseEvent) {
		if (isPanning) {
			panOffset = {
				x: event.clientX - panStart.x,
				y: event.clientY - panStart.y
			};
		}
	}

	// Handle pan end
	function handlePanEnd() {
		isPanning = false;
	}

	// Handle mouse wheel zoom
	function handleWheel(event: WheelEvent) {
		event.preventDefault();
		const delta = event.deltaY > 0 ? 0.9 : 1.1;
		// Limit zoom between 0.5x and 5x
		const newZoom = Math.max(0.5, Math.min(5, zoomLevel * delta));
		zoomLevel = newZoom;
	}

	// Format date for tooltip
	function formatDate(timestamp: number): string {
		const date = new Date(timestamp * 1000);
		return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
	}

	// Zoom out
	function zoomOut() {
		if (zoomStack.length > 0) {
			zoomStack = zoomStack.slice(0, -1);
		}
	}

	// Reset zoom
	function resetZoom() {
		zoomStack = [];
	}

	// Check theme
	function checkTheme() {
		if (typeof document !== 'undefined') {
			isDark = document.documentElement.classList.contains('dark');
		}
	}

	// Get node color
	function getColor(node: GeneratedTreeNode): string {
		const nodeWithRisk = ensureTreeNode(node);
		return getNodeColor(nodeWithRisk, colorMode as 'size' | 'risk' | 'usage' | 'filetype', isDark);
	}

	// Get node border color for selected state
	function getBorderColor(node: GeneratedTreeNode): string {
		if (node.selected) {
			return isDark ? 'rgb(59, 130, 246)' : 'rgb(37, 99, 235)'; // Blue border for selected
		}
		return isDark ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)';
	}

	onMount(() => {
		checkTheme();
		handleResize();

		// Watch for theme changes
		if (typeof document !== 'undefined') {
			const observer = new MutationObserver(() => {
				checkTheme();
			});

			observer.observe(document.documentElement, {
				attributes: true,
				attributeFilter: ['class']
			});

			// Watch for resize
			const resizeObserver = new ResizeObserver(() => {
				handleResize();
			});

			if (containerRef) {
				resizeObserver.observe(containerRef);
			}

			// Debounced window resize handler
			let resizeTimeout: ReturnType<typeof setTimeout>;
			const handleWindowResize = () => {
				clearTimeout(resizeTimeout);
				resizeTimeout = setTimeout(handleResize, 150);
			};

			window.addEventListener('resize', handleWindowResize);

			return () => {
				observer.disconnect();
				if (containerRef) {
					resizeObserver.unobserve(containerRef);
				}
				resizeObserver.disconnect();
				window.removeEventListener('resize', handleWindowResize);
				clearTimeout(resizeTimeout);
			};
		}

		return undefined;
	});
</script>

<div class="treemap-container relative w-full h-full min-h-[400px]" bind:this={containerRef}>
	{#if layoutNodes().length === 0}
		<div class="flex items-center justify-center h-full">
			<div class="text-center">
				<div class="text-4xl mb-4">📊</div>
				<p class="text-muted">No data to display</p>
			</div>
		</div>
	{:else}
		<!-- Breadcrumb navigation and zoom indicator -->
		<div class="mb-2 flex items-center justify-between text-sm">
			{#if zoomStack.length > 0}
				<div class="flex items-center gap-2">
					<button
						class="px-2 py-1 text-xs bg-gray-100 dark:bg-gray-800 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
						onclick={resetZoom}
						aria-label="Reset zoom to root"
					>
						🏠 Root
					</button>
					{#each zoomStack as node, index}
						<span class="text-muted">/</span>
						<button
							class="px-2 py-1 text-xs bg-gray-100 dark:bg-gray-800 rounded hover:bg-gray-200 dark:hover:bg-gray-700 truncate max-w-[200px] transition-colors"
							onclick={() => {
								zoomStack = zoomStack.slice(0, index + 1);
								panOffset = { x: 0, y: 0 };
							}}
						>
							{node.name}
						</button>
					{/each}
				</div>
			{:else}
				<div></div>
			{/if}
			{#if zoomLevel > 1}
				<div class="text-xs text-[var(--color-text-secondary)]">
					Zoom: {zoomLevel.toFixed(1)}x
				</div>
			{/if}
		</div>

		<!-- SVG Container -->
		<svg
			bind:this={svgRef}
			class="w-full h-full cursor-move"
			viewBox="0 0 {containerSize.width} {containerSize.height}"
			preserveAspectRatio="none"
			role="img"
			aria-label="Treemap visualization of file system structure"
			onmousedown={handlePanStart}
			onmousemove={handlePanMove}
			onmouseup={handlePanEnd}
			onmouseleave={handlePanEnd}
			onwheel={handleWheel}
		>
			<g transform="translate({panOffset.x}, {panOffset.y}) scale({zoomLevel})">
				{#each layoutNodes() as node}
					{@const x = node.x0 ?? 0}
					{@const y = node.y0 ?? 0}
					{@const width = Math.max(0, (node.x1 ?? 0) - x)}
					{@const height = Math.max(0, (node.y1 ?? 0) - y)}
					{@const isHovered = hoveredNode?.id === node.data.id}
					{@const nodeData = node.data as GeneratedTreeNode}
					{@const sizePercent = totalSize() > 0 ? calculateSizePercentage(nodeData.size, totalSize()) : 0}
					{@const fontSize = Math.min(Math.max(10, Math.min(width, height) / 8), 14)}
					{#if width > 0 && height > 0}
					<rect
						x={x}
						y={y}
						width={width}
						height={height}
						fill={getColor(nodeData)}
						stroke={getBorderColor(nodeData)}
						stroke-width={isHovered || nodeData.selected ? 3 : nodeData.isDirectory ? 1.5 : 0.5}
						opacity={isHovered ? 0.9 : nodeData.selected ? 0.85 : 0.7}
						class="cursor-pointer transition-all duration-200"
						onmouseenter={(e) => handleNodeMouseEnter(e, node)}
						onmouseleave={handleNodeMouseLeave}
						onclick={(e) => {
							e.stopPropagation();
							handleNodeClick(node);
						}}
						ondblclick={(e) => {
							e.stopPropagation();
							handleNodeDoubleClick(node);
						}}
						onkeydown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') {
								e.preventDefault();
								handleNodeClick(node);
							}
						}}
						role="button"
						tabindex="0"
						aria-label="{nodeData.name} - {formatBytes(nodeData.size)} - {nodeData.isDirectory ? 'Directory' : 'File'}"
						aria-selected={nodeData.selected}
						data-node-id={nodeData.id}
					/>
					<!-- Label for larger rectangles (increased threshold from 60px to 100px) -->
					{#if width > 100 && height > 30}
						<text
							x={x + width / 2}
							y={y + height / 2 - (width > 120 ? 8 : 0)}
							text-anchor="middle"
							dominant-baseline="middle"
							class="pointer-events-none select-none"
							style="font-size: {fontSize}px; fill: {isDark ? 'rgba(255, 255, 255, 0.9)' : 'rgba(0, 0, 0, 0.9)'}; font-weight: 500; text-shadow: 1px 1px 2px {isDark ? 'rgba(0, 0, 0, 0.8)' : 'rgba(255, 255, 255, 0.8)'}, -1px -1px 2px {isDark ? 'rgba(0, 0, 0, 0.8)' : 'rgba(255, 255, 255, 0.8)'};"
						>
							{nodeData.name.length > Math.floor(width / fontSize * 1.5)
								? nodeData.name.substring(0, Math.floor(width / fontSize * 1.5) - 3) + '...'
								: nodeData.name}
						</text>
						<!-- Percentage label for rectangles >120px -->
						{#if width > 120 && height > 40}
							<text
								x={x + width / 2}
								y={y + height / 2 + 12}
								text-anchor="middle"
								dominant-baseline="middle"
								class="pointer-events-none select-none"
								style="font-size: {Math.max(9, fontSize - 2)}px; fill: {isDark ? 'rgba(255, 255, 255, 0.7)' : 'rgba(0, 0, 0, 0.7)'}; font-weight: 400; text-shadow: 1px 1px 2px {isDark ? 'rgba(0, 0, 0, 0.8)' : 'rgba(255, 255, 255, 0.8)'};"
							>
								{sizePercent.toFixed(1)}%
							</text>
						{/if}
					{/if}
					{/if}
				{/each}
			</g>
		</svg>

		<!-- Enhanced Tooltip -->
		{#if hoveredNode}
			{@const hoveredSizePercent = totalSize() > 0 ? calculateSizePercentage(hoveredNode.size, totalSize()) : 0}
			{@const fileExtension = hoveredNode.isDirectory ? '' : getFileExtension(hoveredNode.path)}
			{@const fileType = hoveredNode.isDirectory ? 'Directory' : fileExtension ? fileExtension.toUpperCase() : 'File'}
			{@const childrenCount = hoveredNode.isDirectory && hoveredNode.children ? hoveredNode.children.length : 0}
			<div
				class="absolute z-50 px-3 py-2 bg-gray-900 dark:bg-gray-800 text-white text-xs rounded shadow-lg pointer-events-none max-w-[400px] border border-gray-700"
				style="left: {Math.min(tooltipPosition.x + 10, containerSize.width - 250)}px; top: {Math.min(tooltipPosition.y + 10, containerSize.height - 150)}px;"
			>
				<div class="font-semibold mb-1 text-sm">{hoveredNode.name}</div>
				<div class="text-gray-300 text-[10px] mb-2 truncate max-w-[350px] border-b border-gray-700 pb-2">
					{hoveredNode.path}
				</div>
				<div class="space-y-1">
					<div class="flex items-center justify-between">
						<span class="text-gray-400">Size:</span>
						<span class="font-medium">{formatBytes(hoveredNode.size)} ({hoveredSizePercent.toFixed(2)}%)</span>
					</div>
					<div class="flex items-center justify-between">
						<span class="text-gray-400">Type:</span>
						<span>{fileType}</span>
					</div>
					{#if hoveredNode.isDirectory}
						<div class="flex items-center justify-between">
							<span class="text-gray-400">Items:</span>
							<span>{childrenCount} {childrenCount === 1 ? 'item' : 'items'}</span>
						</div>
					{/if}
					<div class="flex items-center justify-between">
						<span class="text-gray-400">Modified:</span>
						<span class="text-[10px]">{formatDate(hoveredNode.lastModified)}</span>
					</div>
					<div class="flex items-center justify-between">
						<span class="text-gray-400">Risk:</span>
						<span class="capitalize">{hoveredNode.riskLevel}</span>
					</div>
					{#if hoveredNode.usagePattern}
						<div class="flex items-center justify-between">
							<span class="text-gray-400">Usage:</span>
							<span class="capitalize">{hoveredNode.usagePattern}</span>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	{/if}
</div>

<style>
	.treemap-container {
		user-select: none;
	}
</style>
