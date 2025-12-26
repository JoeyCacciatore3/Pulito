<script lang="ts">
	interface TreeNodeData {
		id: string;
		name: string;
		path: string;
		size: number;
		isDirectory: boolean;
		lastModified: number;
		lastAccessed: number;
		children?: TreeNodeData[];
		expanded: boolean;
		selected: boolean;
		riskLevel: 'safe' | 'caution' | 'warning';
		usagePattern?: 'frequent' | 'occasional' | 'rare' | 'never';
		parent?: TreeNodeData;
	}

	interface Props {
		node: TreeNodeData;
		toggleNodeExpansion: (_node: TreeNodeData) => void;
		toggleNodeSelection: (_node: TreeNodeData, _selected: boolean) => void;
		getRiskColor: (_risk: string) => string;
		getUsageColor: (_pattern: string) => string;
	}

	let { node, toggleNodeExpansion, toggleNodeSelection, getRiskColor, getUsageColor }: Props = $props();

	import TreeNode from './TreeNode.svelte';
	import { formatBytes } from '$lib/utils/tauri';
	import SizeBar from './ui/SizeBar.svelte';
</script>

<div class="tree-node">
	<div class="flex items-center gap-2 py-1 hover:bg-gray-50 dark:hover:bg-gray-900/50 px-2 rounded">
		<!-- Expansion Toggle -->
		{#if node.isDirectory && node.children && node.children.length > 0}
			<button
				class="w-4 h-4 flex items-center justify-center text-gray-500 hover:text-gray-700"
				onclick={() => toggleNodeExpansion(node)}
			>
				{node.expanded ? '▼' : '▶'}
			</button>
		{:else}
			<div class="w-4"></div>
		{/if}

		<!-- Selection Checkbox -->
		<input
			type="checkbox"
			bind:checked={node.selected}
			onchange={() => toggleNodeSelection(node, node.selected)}
			class="w-4 h-4"
		/>

		<!-- Icon -->
		<div class="w-5 flex-shrink-0">
			{node.isDirectory ? '📁' : '📄'}
		</div>

		<!-- Content -->
		<div class="flex-1 min-w-0">
			<div class="flex items-center gap-2">
				<span class="font-medium truncate">{node.name}</span>
			</div>

			<div class="flex items-center gap-4 text-xs text-muted">
				<span>{formatBytes(node.size)}</span>
				<span>Last accessed: {new Date(node.lastAccessed).toLocaleDateString()}</span>
				{#if node.usagePattern}
					<span class="px-2 py-1 rounded text-xs {getUsageColor(node.usagePattern)}">
						{node.usagePattern}
					</span>
				{/if}
			</div>
		</div>

		<!-- Risk Indicator -->
		<div class="px-2 py-1 text-xs rounded border {getRiskColor(node.riskLevel)}">
			{node.riskLevel}
		</div>
	</div>

	<!-- Children -->
	{#if node.expanded && node.children && node.children.length > 0}
		<div class="ml-6 border-l border-gray-200 pl-2">
			{#each node.children as child}
				<TreeNode
					node={child}
					{toggleNodeExpansion}
					{toggleNodeSelection}
					{getRiskColor}
					{getUsageColor}
				/>
			{/each}
		</div>
	{/if}
</div>

<style>
	.tree-node {
		transition: background-color 0.15s ease;
	}
</style>
