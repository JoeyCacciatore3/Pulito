<script lang="ts">
	/* eslint-disable @typescript-eslint/no-explicit-any */
	import { onMount } from 'svelte';
	import { Line } from 'svelte5-chartjs';
	import { Chart, registerables } from 'chart.js';
	import { getDefaultChartOptions, getCPUCoreColors, formatPercentForChart, formatTimeForChart, formatDateForChart, transparentize } from '$lib/utils/charts';

	Chart.register(...registerables);

	interface DataPoint {
		timestamp: number;
		usage: number;
		coreUsages?: number[];
	};

	interface Props {
		data: DataPoint[];
		timeRange?: '1h' | '6h' | '24h' | 'all';
		height?: string;
	};
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
		} else {
		};
	// Build datasets\tlet datasets = $derived(() => {\t\tconst colors = getCPUCoreColors(hasCoreData ? coreCount : 1, isDark);\t\tconst datasets: any[] = [];\t\t\t// Main CPU usage dataset\t\tdatasets.push({\t\t\tlabel: 'CPU Usage',\t\t\tdata: filteredData.map(d => d.usage),\t\t\tborderColor: colors[0],\t\t\tbackgroundColor: transparentize(colors[0], 0.1),\t\t\tfill: true,\t\t\ttension: 0.4,\t\t\tpointRadius: 0,\t\t\tborderWidth: 1.5,\t\t\tpointHoverRadius: 3\t\t});\t\t\t// Core usage datasets (if available)\t\tif (hasCoreData) {\t\t\tfilteredData[0].coreUsages!.forEach((_, coreIndex) => {\t\t\t\tdatasets.push({\t\t\t\t\tlabel: `Core ${coreIndex + 1}`,\t\t\t\t\tdata: filteredData.map(d => d.coreUsages![coreIndex]),\t\t\t\t\tborderColor: colors[coreIndex + 1] || colors[0],\t\t\t\t\tbackgroundColor: transparentize(colors[coreIndex + 1] || colors[0], 0.1),\t\t\t\t\tfill: true,\t\t\t\t\ttension: 0.4,\t\t\t\t\tpointRadius: 0,\t\t\t\t\tborderWidth: 1.5,\t\t\t\t\tpointHoverRadius: 3\t\t\t\t});\t\t\t});\t\t}\t\treturn datasets;\t});

	let chartData = $derived(() => {
		return {
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
