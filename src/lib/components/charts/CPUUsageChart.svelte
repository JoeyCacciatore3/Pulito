<script lang="ts">
	import { onMount } from 'svelte';
	import { Line } from 'svelte5-chartjs';
	import { Chart, registerables } from 'chart.js';
	import { getThemeColors, transparentize } from '$lib/utils/charts';

	Chart.register(...registerables);

	interface Props {
		data: number[];
		color?: string;
		height?: number;
		width?: string;
	}

	let {
		data = [],
		color,
		height = 40,
		width = '100%'
	}: Props = $props();

	let isDark = $derived(
		typeof document !== 'undefined' && document.documentElement.classList.contains('dark')
	);

	// Use provided color or theme default
	let chartColor = $derived(color || getThemeColors(isDark).primary);

	let chartData = $derived({
		labels: data.map(() => ''),
		datasets: [
			{
				data: data,
				borderColor: chartColor,
				backgroundColor: transparentize(chartColor, 0.1),
				fill: true,
				tension: 0.4,
				pointRadius: 0,
				borderWidth: 2
			}
		]
	});

	let chartOptions = $derived({
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			legend: {
				display: false
			},
			tooltip: {
				enabled: false
			}
		},
		scales: {
			x: {
				display: false
			},
			y: {
				display: false
			}
		},
		elements: {
			point: {
				radius: 0
			}
		},
		animation: {
			duration: 0 // Disable animation for cpu-usages (they update frequently)
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

<div class="w-full" style="height: {height}px; width: {width}">
	<Line data={chartData} options={chartOptions} />
</div>
