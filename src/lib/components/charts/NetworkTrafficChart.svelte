<script lang="ts">
	/* eslint-disable @typescript-eslint/no-explicit-any */
	import { onMount } from 'svelte';
	import { Line } from 'svelte5-chartjs';
	import { Chart, registerables } from 'chart.js';
	import { getDefaultChartOptions, getThemeColors, formatBytesForChart, formatTimeForChart, formatDateForChart, transparentize } from '$lib/utils/charts';

	Chart.register(...registerables);

	interface DataPoint {
		timestamp: number;
		upload: number;
		download: number;
	}

	interface Props {
		data: DataPoint[];
		timeRange?: '1h' | '6h' | '24h' | 'all';
		height?: string;
	}

	let { data = [], timeRange = 'all', height = '300px' }: Props = $props();

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
		const colors = getThemeColors(isDark);

		return [
			{
				label: 'Upload',
				data: filteredData.map(d => d.upload),
				borderColor: colors.danger,
				backgroundColor: transparentize(colors.danger, 0.1),
				fill: true,
				tension: 0.4,
				pointRadius: 0,
				borderWidth: 2,
				yAxisID: 'y'
			},
			{
				label: 'Download',
				data: filteredData.map(d => d.download),
				borderColor: colors.secondary,
				backgroundColor: transparentize(colors.secondary, 0.1),
				fill: true,
				tension: 0.4,
				pointRadius: 0,
				borderWidth: 2,
				yAxisID: 'y'
			}
		];
	})());

	let chartData = $derived({
		labels,
		datasets
	});

	// Calculate max value for Y axis
	let maxValue = $derived(filteredData.length > 0
		? Math.max(...filteredData.map(d => Math.max(d.upload, d.download))) * 1.1
		: 1000);

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
						return `${context.dataset.label}: ${formatBytesForChart(context.parsed.y)}`;
					}
				}
			}
		},
		scales: {
			...(defaultOptions.scales ?? {}),
			y: {
				...(defaultOptions.scales?.y ?? {}),
				beginAtZero: true,
				max: maxValue,
				ticks: {
					...(defaultOptions.scales?.y?.ticks ?? {}),
					callback: (value: number) => formatBytesForChart(value)
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
