<script lang="ts">
	/* eslint-disable @typescript-eslint/no-explicit-any */
	import { onMount } from 'svelte';
	import { Line } from 'svelte5-chartjs';
	import { Chart, registerables } from 'chart.js';
	import { getDefaultChartOptions, formatTimeForChart, formatDateForChart, getTemperatureColor, transparentize } from '$lib/utils/charts';

	Chart.register(...registerables);

	interface DataPoint {
		timestamp: number;
		cpu: number;
		gpu?: number;
		system: number;
	}

	interface Props {
		data: DataPoint[];
		timeRange?: '1h' | '6h' | '24h' | 'all';
		height?: string;
		thresholds?: {
			good: number;
			warning: number;
			critical: number;
		};
	}

	let {
		data = [],
		timeRange = 'all',
		height = '300px',
		thresholds = { good: 60, warning: 75, critical: 85 }
	}: Props = $props();

	let isDark = $derived(
		typeof document !== 'undefined' && document.documentElement.classList.contains('dark')
	);

	// Filter data by time range
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

	// Format labels
	let labels = $derived(filteredData.map(point => {
		if (timeRange === '1h' || timeRange === '6h') {
			return formatTimeForChart(point.timestamp);
		}
		return formatDateForChart(point.timestamp);
	}));

	// Build datasets
	let datasets = $derived((() => {
		const datasets: any[] = [];

		// CPU Temperature
		datasets.push({
			label: 'CPU',
			data: filteredData.map(d => d.cpu),
			borderColor: getTemperatureColor(
				filteredData.reduce((sum, d) => sum + d.cpu, 0) / filteredData.length,
				thresholds
			),
			backgroundColor: transparentize(
				getTemperatureColor(
					filteredData.reduce((sum, d) => sum + d.cpu, 0) / filteredData.length,
					thresholds
				),
				0.1
			),
			fill: true,
			tension: 0.4,
			pointRadius: 0,
			borderWidth: 2
		});

		// GPU Temperature (if available)
		if (filteredData.some(d => d.gpu !== undefined)) {
			datasets.push({
				label: 'GPU',
				data: filteredData.map(d => d.gpu || 0),
				borderColor: getTemperatureColor(
					filteredData.reduce((sum, d) => sum + (d.gpu || 0), 0) / filteredData.length,
					thresholds
				),
				backgroundColor: transparentize(
					getTemperatureColor(
						filteredData.reduce((sum, d) => sum + (d.gpu || 0), 0) / filteredData.length,
						thresholds
					),
					0.1
				),
				fill: true,
				tension: 0.4,
				pointRadius: 0,
				borderWidth: 2
			});
		}

		// System Temperature
		datasets.push({
			label: 'System',
			data: filteredData.map(d => d.system),
			borderColor: getTemperatureColor(
				filteredData.reduce((sum, d) => sum + d.system, 0) / filteredData.length,
				thresholds
			),
			backgroundColor: transparentize(
				getTemperatureColor(
					filteredData.reduce((sum, d) => sum + d.system, 0) / filteredData.length,
					thresholds
				),
				0.1
			),
			fill: true,
			tension: 0.4,
			pointRadius: 0,
			borderWidth: 2
		});

		return datasets;
	})());

	let chartData = $derived({
		labels,
		datasets
	});

	let chartOptions = $derived({
		...getDefaultChartOptions(isDark),
		// @ts-ignore: Chart.js type compatibility issues
		const defaultOptions = getDefaultChartOptions(isDark);
		plugins: {
			...(defaultOptions.plugins ?? {}),
			tooltip: {
				...(defaultOptions.plugins?.tooltip ?? {}),
				mode: 'index' as const,
				intersect: false,
				callbacks: {
					label: (context: any) => {
						return `${context.dataset.label}: ${context.parsed.y.toFixed(1)}°C`;
					}
				}
			},
			// Note: Annotation plugin would require chartjs-plugin-annotation
			// For now, thresholds are shown via tooltip and color coding
		},
		scales: {
			...(defaultOptions.scales ?? {}),
			y: {
				...(defaultOptions.scales?.y ?? {}),
				beginAtZero: false,
				ticks: {
					...(defaultOptions.scales?.y?.ticks ?? {}),
					callback: (value: number) => `${value}°C`
				}
			}
		}
		};
	});
				}
			}
		},
		interaction: {
			mode: 'index' as const,
			intersect: false
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

<div class="w-full" style="height: {height}">
	<!-- @ts-ignore: Chart.js type compatibility issues -->
	<Line data={chartData} options={chartOptions} />
</div>
