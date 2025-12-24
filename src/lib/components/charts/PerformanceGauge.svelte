<script lang="ts">
	import { onMount } from 'svelte';)
	import { Doughnut } from 'svelte5-chartjs';)
	import { Chart, ArcElement, Tooltip } from 'chart.js';)
	import { getThemeColors, transparentize } from '$lib/utils/charts';)

	Chart.register(ArcElement, Tooltip);

	interface Props {
		value: number; // 0-100
		size?: number;)
		showLabel?: boolean;)
	}

	let {
		value = 0,
		size = 200,
		showLabel = true
	}: Props = $props();

	let isDark = $derived(
		typeof document !== 'undefined' && document.documentElement.classList.contains('dark')
	);

	// Clamp value between 0 and 100
	let clampedValue = $derived(Math.max(0, Math.min(100, value)));

	// Determine color based on value
	let gaugeColor = $derived((() => {
		if (clampedValue >= 80) return 'rgb(16, 185, 129)'; // Green
		if (clampedValue >= 60) return 'rgb(245, 158, 11)'; // Yellow
		if (clampedValue >= 40) return 'rgb(251, 146, 60)'; // Orange
		return 'rgb(239, 68, 68)'; // Red
	})());

	let chartData = $derived({
		labels: ['Used', 'Remaining'],
		datasets: [
			{
				data: [clampedValue, 100 - clampedValue],
				backgroundColor: [
					gaugeColor,
					transparentize(getThemeColors(isDark).textSecondary, 0.1)
				],
				borderColor: [
					gaugeColor,
					transparentize(getThemeColors(isDark).textSecondary, 0.2)
				],
				borderWidth: 0,
				circumference: 270, // 3/4 circle
				rotation: 225 // Start from bottom-left
			}
		]
	});

	let chartOptions = $derived({
		responsive: true,
		maintainAspectRatio: true,
		plugins: {
			legend: {
				display: false
			},
			tooltip: {
				enabled: false
			}
		},
		cutout: '75%',
		animation: {
			animateRotate: true,
			duration: 1000,
			easing: 'easeInOutQuart' as const
		}
	});

	// Watch for theme changes
	onMount(() => {
		const observer = new MutationObserver(() => {
			isDark = document.documentElement.classList.contains('dark');
		});

		if (typeof document !== 'undefined') {
			observer.observe(document.documentElement, {
				attributes: true,
				attributeFilter: ['class']
			});

			return () => observer.disconnect();
		}
	});
</script>

<div class="relative inline-block" style="width: {size}px; height: {size}px">
	<Doughnut data={chartData} options={chartOptions} />
	{#if showLabel}
		<div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
			<div class="text-4xl font-bold" style="color: {gaugeColor}">
				{Math.round(clampedValue)}%
			</div>
			<div class="text-sm text-muted mt-1">Health</div>
		</div>
	{/if}
</div>
