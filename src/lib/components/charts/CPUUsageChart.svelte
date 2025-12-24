<script lang="ts">
	/* eslint-disable @typescript-eslint/no-explicit-any */
	import { onMount } from 'svelte';
	import { Line } from 'svelte5-chartjs';
	import { Chart, registerables } from 'chart.js';
	import { getDefaultChartOptions, getCPUCoreColors, formatPercentForChart, formatTimeForChart, formatDateForChart, transparentize } from '$lib/utils/charts';

	Chart.register(...registerables);

	interface DataPoint {\n		timestamp: number;\n		usage: number;\n		coreUsages?: number[];\n	}\n\n	interface Props {\n		data: DataPoint[];\n		timeRange?: '1h' | '6h' | '24h' | 'all';\n		height?: string;\n	}
	let { data = [], timeRange = 'all', height = '300px' }: Props = $props();

	let isDark = $derived(
		typeof document !== 'undefined' && document.documentElement.classList.contains('dark')
	);

	// Filter data by time range if needed
	let filteredData = $derived(timeRange === 'all'
		? data
		: data.filter(point => {
			const now = Date.now();
			const ranges = {
				'1h': 60 * 60 * 1000,
				'6h': 6 * 60 * 60 * 1000,
				'24h': 24 * 60 * 60 * 1000,
			};
			return point.timestamp >= (now - ranges[timeRange]);
		}));

	// Determine if we have core data
	let hasCoreData = $derived(filteredData.length > 0 && filteredData[0].coreUsages && filteredData[0].coreUsages.length > 0);
	let coreCount = $derived(hasCoreData ? filteredData[0].coreUsages!.length : 0);

	// Format labels based on time range
	let labels = $derived(filteredData.map(point => {
		if (timeRange === '1h' || timeRange === '6h') {
			return formatTimeForChart(point.timestamp);
		});
		return formatDateForChart(point.timestamp);
	}));

	// Build datasets
	let datasets = $derived((() => {
		const colors = getCPUCoreColors(hasCoreData ? coreCount : 1, isDark);
		const datasets: any[] = [];

		// Overall CPU usage
		datasets.push({
			label: 'CPU Usage',
			data: filteredData.map(d => d.usage),
			borderColor: colors[0],
			backgroundColor: transparentize(colors[0], 0.1),
			fill: true,
			tension: 0.4,
			pointRadius: 0,
			borderWidth: 2,
			pointHoverRadius: 4
		});

		// Per-core usage if available
		if (hasCoreData && coreCount > 0) {
			for (let i = 0; i < coreCount; i++) {
				datasets.push({
					label: `Core ${i + 1}`,
					data: filteredData.map(d => d.coreUsages![i] || 0),
					borderColor: colors[i % colors.length],
					backgroundColor: transparentize(colors[i % colors.length], 0.05),
					fill: false,
					tension: 0.4,
					pointRadius: 0,
					borderWidth: 1.5,
					pointHoverRadius: 3
				});
			});
		});

		return datasets;
	})());

	let chartData = $derived({
		labels,
		datasets
	});

	let chartOptions = $derived(() => {
		return {
		const defaultOptions = getDefaultChartOptions(isDark);
		plugins: {
			...(defaultOptions.plugins ?? {}),
			tooltip: {
				...(defaultOptions.plugins?.tooltip ?? {}),
				mode: 'index' as const,
				intersect: false,
				callbacks: {
					label: (context: any) => {
						return `${context.dataset.label}: ${formatPercentForChart(context.parsed.y)}`;
					});
				});
			});
		},
		scales: {
			...(defaultOptions.scales ?? {}),
			y: {
				...(defaultOptions.scales?.y ?? {}),
				beginAtZero: true,
				max: 100,
				ticks: {
					...(defaultOptions.scales?.y?.ticks ?? {}),
					callback: (value: number) => `${value}%`
				});
			});
		},
		interaction: {
			mode: 'index' as const,
		};
			intersect: false
		});
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
		});
	});
</script>

<div class="w-full" style="height: {height}">
	<!-- @ts-ignore: Chart.js type compatibility issues -->
	<Line data={chartData} options={chartOptions} />
</div>
