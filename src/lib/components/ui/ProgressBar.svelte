<script lang="ts">
	interface Props {
		percentage: number;
		height?: string;
		color?: string;
		showLabel?: boolean;
		label?: string;
		animated?: boolean;
	}

	let {
		percentage,
		height = 'h-3',
		color = 'bg-primary-500',
		showLabel = false,
		label = '',
		animated = true
	}: Props = $props();
</script>

<div class="progress-bar {height}">
	<div
		class="progress-bar-fill {color} {animated ? 'transition-all duration-300' : ''}"
		style="width: {Math.min(100, Math.max(0, percentage))}%"
	></div>
</div>

{#if showLabel}
	<div class="text-xs text-[var(--color-text-muted)] mt-1">
		{label || `${percentage.toFixed(1)}%`}
	</div>
{/if}

<style>
	.progress-bar {
		background: var(--color-border);
		border-radius: 3px;
		overflow: hidden;
	}

	.progress-bar-fill {
		height: 100%;
		border-radius: 3px;
	}
</style>
