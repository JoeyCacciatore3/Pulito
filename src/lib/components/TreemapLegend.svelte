<script lang="ts">
	import { getThemeColors } from '$lib/utils/charts';
	import { getFileTypeColor, getAllFileTypeCategories, getFileTypeInfo } from '$lib/utils/file-types';
	import type { FileTypeCategory } from '$lib/utils/file-types';

	interface Props {
		colorMode: 'size' | 'risk' | 'usage' | 'filetype';
		isDark?: boolean;
		totalSize?: number;
		minSize?: number;
		maxSize?: number;
		collapsible?: boolean;
	}

	let {
		colorMode = 'size',
		isDark = false,
		totalSize = 0,
		minSize = 0,
		maxSize = 0,
		collapsible = false
	}: Props = $props();

	let collapsed = $state(false);

	// Get theme colors
	let themeColors = $derived(getThemeColors(isDark));

	// Size ranges for size mode legend
	let sizeRanges = $derived(() => {
		if (!totalSize || totalSize === 0) {
			return [];
		}

		const ranges = [
			{ label: '< 1 MB', max: 1024 * 1024, color: 'rgba(59, 130, 246, 0.3)' },
			{ label: '1-10 MB', min: 1024 * 1024, max: 10 * 1024 * 1024, color: 'rgba(59, 130, 246, 0.5)' },
			{ label: '10-100 MB', min: 10 * 1024 * 1024, max: 100 * 1024 * 1024, color: 'rgba(59, 130, 246, 0.7)' },
			{ label: '100 MB - 1 GB', min: 100 * 1024 * 1024, max: 1024 * 1024 * 1024, color: 'rgba(59, 130, 246, 0.85)' },
			{ label: '1+ GB', min: 1024 * 1024 * 1024, color: 'rgba(59, 130, 246, 1.0)' }
		];

		return ranges;
	});

	// Risk level colors
	let riskColors = $derived(() => [
		{ label: 'Safe', color: themeColors.secondary, description: 'Low risk items' },
		{ label: 'Caution', color: themeColors.warning, description: 'Medium risk items' },
		{ label: 'Warning', color: themeColors.danger, description: 'High risk items' }
	]);

	// Usage pattern colors
	let usageColors = $derived(() => [
		{ label: 'Frequent', color: themeColors.secondary, description: 'Regularly accessed' },
		{ label: 'Occasional', color: themeColors.primary, description: 'Sometimes accessed' },
		{ label: 'Rare', color: themeColors.warning, description: 'Rarely accessed' },
		{ label: 'Never', color: themeColors.danger, description: 'Never accessed' }
	]);

	// File type colors
	let fileTypeColors = $derived(() => {
		const categories = getAllFileTypeCategories();
		return categories
			.filter((cat) => cat !== 'other') // Exclude 'other' from legend
			.map((category) => {
				const info = getFileTypeInfo(category);
				return {
					label: info.name,
					color: getFileTypeColor(category, isDark),
					description: info.description,
					category
				};
			});
	});
</script>

<div class="treemap-legend border-t border-[var(--color-border)] bg-[var(--color-bg-secondary)] p-3">
	{#if collapsible}
		<button
			class="flex items-center justify-between w-full mb-2 text-sm font-medium text-[var(--color-text)] hover:text-[var(--color-text-primary)]"
			onclick={() => (collapsed = !collapsed)}
			aria-expanded={!collapsed}
			aria-label="Toggle legend"
		>
			<span>Color Legend</span>
			<span class="text-xs">{collapsed ? '▼' : '▲'}</span>
		</button>
	{/if}

	{#if !collapsed}
		<div class="space-y-2">
			{#if colorMode === 'size'}
				<!-- Size-based color legend -->
				<div class="flex flex-wrap items-center gap-3">
					<span class="text-xs font-medium text-[var(--color-text-secondary)]">Size:</span>
					{#each sizeRanges() as range}
						<div class="flex items-center gap-2">
							<div
								class="w-4 h-4 rounded border border-[var(--color-border)]"
								style="background-color: {range.color};"
								aria-label="{range.label}"
							></div>
							<span class="text-xs text-[var(--color-text-secondary)]">{range.label}</span>
						</div>
					{/each}
				</div>
			{:else if colorMode === 'risk'}
				<!-- Risk level legend -->
				<div class="flex flex-wrap items-center gap-3">
					<span class="text-xs font-medium text-[var(--color-text-secondary)]">Risk Level:</span>
					{#each riskColors() as risk}
						<div class="flex items-center gap-2">
							<div
								class="w-4 h-4 rounded border border-[var(--color-border)]"
								style="background-color: {risk.color};"
								aria-label="{risk.label}"
							></div>
							<span class="text-xs text-[var(--color-text-secondary)]">{risk.label}</span>
						</div>
					{/each}
				</div>
			{:else if colorMode === 'usage'}
				<!-- Usage pattern legend -->
				<div class="flex flex-wrap items-center gap-3">
					<span class="text-xs font-medium text-[var(--color-text-secondary)]">Usage Pattern:</span>
					{#each usageColors() as usage}
						<div class="flex items-center gap-2">
							<div
								class="w-4 h-4 rounded border border-[var(--color-border)]"
								style="background-color: {usage.color};"
								aria-label="{usage.label}"
							></div>
							<span class="text-xs text-[var(--color-text-secondary)]">{usage.label}</span>
						</div>
					{/each}
				</div>
			{:else if colorMode === 'filetype'}
				<!-- File type legend -->
				<div class="space-y-2">
					<span class="text-xs font-medium text-[var(--color-text-secondary)]">File Types:</span>
					<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
						{#each fileTypeColors() as fileType}
							<div class="flex items-center gap-2">
								<div
									class="w-3 h-3 rounded border border-[var(--color-border)] flex-shrink-0"
									style="background-color: {fileType.color};"
									aria-label="{fileType.label}"
								></div>
								<span class="text-xs text-[var(--color-text-secondary)] truncate" title={fileType.description}>
									{fileType.label}
								</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.treemap-legend {
		user-select: none;
	}
</style>
