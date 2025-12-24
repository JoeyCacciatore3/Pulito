<script lang="ts">
	/* eslint-disable @typescript-eslint/no-explicit-any */
	import { onMount } from 'svelte';
	import { Line } from 'svelte5-chartjs';
	import { Chart, registerables } from 'chart.js';
	import { getDefaultChartOptions, getThemeColors, formatBytesForChart, formatTimeForChart, formatDateForChart, transparentize } from '$lib/utils/charts';

	Chart.register(...registerables);

	interface DataPoint {
		timestamp: number;
		usedMemory: number;
		totalMemory: number;
		swapUsed?: number;
		swapTotal?: number;
		cache?: number;
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

	// Build datasets for stacked area chart
	let datasets = $derived((() => {
		const colors = getThemeColors(isDark);
		const datasets: any[] = [];

		// RAM Used (bottom layer)
		datasets.push({
			label: 'RAM Used',
			data: filteredData.map(d => d.usedMemory - (d.cache || 0)),
			backgroundColor: transparentize(colors.primary, 0.3),
			borderColor: colors.primary,
			fill: true,
			tension: 0.4,
			pointRadius: 0,
			borderWidth: 2
		});

		// Cache (middle layer)
		if (filteredData.some(d => d.cache && d.cache > 0)) {
			datasets.push({
				label: 'Cache',
				data: filteredData.map(d => d.cache || 0),
				backgroundColor: transparentize(colors.secondary, 0.3),
				borderColor: colors.secondary,
				fill: true,
				tension: 0.4,
				pointRadius: 0,
				borderWidth: 2
			});
		}

		// Swap Used (top layer)
		if (filteredData.some(d => d.swapUsed && d.swapUsed > 0)) {
			datasets.push({
				label: 'Swap Used',
				data: filteredData.map(d => d.swapUsed || 0),
				backgroundColor: transparentize(colors.warning, 0.3),
				borderColor: colors.warning,
				fill: true,
				tension: 0.4,
				pointRadius: 0,
				borderWidth: 2
			});
		}

		return datasets;
	})());

	let chartData = $derived({
		labels,
		datasets
	});

	// Calculate max value for Y axis
	let maxValue = $derived(filteredData.length > 0
		? Math.max(...filteredData.map(d => d.totalMemory + (d.swapTotal || 0))) * 1.1
		: 100);

		const defaultOptions = getDefaultChartOptions(isDark);	return {
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
			x: {
				...(defaultOptions.scales?.x ?? {}),
				stacked: true
			},
			y: {
				...(defaultOptions.scales?.y ?? {}),
				stacked: true,
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
		};
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
