<script lang="ts">
	import { getThemeColors } from '$lib/utils/charts';

	interface Props {
		size: number;
		totalSize: number;
		color?: string;
		showLabel?: boolean;
		showPercentage?: boolean;
		height?: string;
		animated?: boolean;
		riskLevel?: 'safe' | 'caution' | 'warning';
		usagePattern?: 'frequent' | 'occasional' | 'rare' | 'never';
	}

	let {
		size = 0,
		totalSize = 1,
		color,
		showLabel = false,
		showPercentage = true,
		height = 'h-4',
		animated = true,
		riskLevel,
		usagePattern
	}: Props = $props();

	let isDark = $derived(
		typeof document !== 'undefined' && document.documentElement.classList.contains('dark')
	);

	// Calculate percentage
	let percentage = $derived(() => {
		if (!totalSize || totalSize === 0) return 0;
		return Math.min(100, Math.max(0, (size / totalSize) * 100));
	});

	// Determine color based on risk/usage or use provided color
	let barColor = $derived(() => {
		if (color) return color;

		const themeColors = getThemeColors(isDark);

		if (riskLevel) {
			switch (riskLevel) {
				case 'safe':
					return themeColors.secondary; // Green
				case 'caution':
					return themeColors.warning; // Yellow
				case 'warning':
					return themeColors.danger; // Red
				default:
					return themeColors.primary;
			}
		}

		if (usagePattern) {
			switch (usagePattern) {
				case 'frequent':
					return themeColors.secondary; // Green
				case 'occasional':
					return themeColors.primary; // Blue
				case 'rare':
					return themeColors.warning; // Yellow
				case 'never':
					return themeColors.danger; // Red
				default:
					return themeColors.primary;
			}
		}

		return themeColors.primary;
	});
</script>

<div class="size-bar-container flex items-center gap-2 w-full" role="progressbar" aria-valuenow={percentage()} aria-valuemin="0" aria-valuemax="100" aria-label="Size: {percentage().toFixed(1)}%">
	<div class="size-bar flex-1 {height} relative overflow-hidden rounded" style="background-color: var(--color-border);">
		<div
			class="size-bar-fill {animated ? 'transition-all duration-300 ease-out' : ''}"
			style="width: {percentage()}%; background-color: {barColor()};"
			aria-hidden="true"
		></div>
	</div>
	{#if showLabel || showPercentage}
		<div class="size-bar-label text-xs text-[var(--color-text-secondary)] min-w-[60px] text-right">
			{#if showPercentage}
				{percentage().toFixed(1)}%
			{/if}
		</div>
	{/if}
</div>

<style>
	.size-bar-container {
		min-width: 0; /* Allow flex shrinking */
	}

	.size-bar {
		min-width: 0;
	}

	.size-bar-fill {
		height: 100%;
		border-radius: inherit;
	}
</style>
