<script lang="ts">
	import { onMount } from 'svelte';
	import { Bar } from 'svelte5-chartjs';
	import { Chart, registerables } from 'chart.js';
	import { getDefaultChartOptions, getThemeColors, formatBytesForChart, formatTimeForChart, formatDateForChart, transparentize } from '$lib/utils/charts';

	Chart.register(...registerables);

	interface DataPoint {
		timestamp: number;
		readBytes: number;
		writeBytes: number;
		readOps?: number;
		writeOps?: number;
	}

	interface Props {
		data: DataPoint[];
		timeRange?: '1h' | '6h' | '24h' | 'all';
		height?: string;
		showIOPS?: boolean;
	}

	let {
		data = [],
		timeRange = 'all',
		height = '300px',
		showIOPS = false
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
		const colors = getThemeColors(isDark);

		const datasets: any[] = [
			{
				label: 'Read',
				data: filteredData.map(d => d.readBytes),
				backgroundColor: transparentize(colors.primary, 0.7),
				borderColor: colors.primary,
				borderWidth: 1,
				yAxisID: 'y'
			},
			{
				label: 'Write',
				data: filteredData.map(d => d.writeBytes),
				backgroundColor: transparentize(colors.secondary, 0.7),
				borderColor: colors.secondary,
				borderWidth: 1,
				yAxisID: 'y'
			}
		];

		// Add IOPS if available and requested
		if (showIOPS && filteredData.some(d => d.readOps !== undefined)) {
			datasets.push(
				{
					label: 'Read Ops',
					data: filteredData.map(d => d.readOps || 0),
					type: 'line' as const,
					borderColor: colors.warning,
					backgroundColor: 'transparent',
					borderWidth: 2,
					fill: false,
					tension: 0.4,
					pointRadius: 0,
					yAxisID: 'y1'
				},
				{
					label: 'Write Ops',
					data: filteredData.map(d => d.writeOps || 0),
					type: 'line' as const,
					borderColor: colors.danger,
					backgroundColor: 'transparent',
					borderWidth: 2,
					fill: false,
					tension: 0.4,
					pointRadius: 0,
					yAxisID: 'y1'
				}
			);
		}

		return datasets;
	})());

	let chartData = $derived({
		labels,
		datasets
	});

	// Calculate max values for Y axes
	let maxBytes = $derived(filteredData.length > 0
		? Math.max(...filteredData.map(d => Math.max(d.readBytes, d.writeBytes))) * 1.1
		: 1000);

	let maxIOPS = $derived(showIOPS && filteredData.length > 0 && filteredData.some(d => d.readOps !== undefined)
		? Math.max(...filteredData.map(d => Math.max(d.readOps || 0, d.writeOps || 0))) * 1.1
		: 1000);

	let chartOptions = $derived({
		...getDefaultChartOptions(isDark),
		plugins: {
			...(getDefaultChartOptions(isDark).plugins ?? {}),
			tooltip: {
				...(getDefaultChartOptions(isDark).plugins?.tooltip ?? {}),
				mode: 'index' as const,
				intersect: false,
				callbacks: {
					label: (context: any) => {
						if (context.dataset.label?.includes('Ops')) {
							return `${context.dataset.label}: ${context.parsed.y.toLocaleString()}`;
						}
						return `${context.dataset.label}: ${formatBytesForChart(context.parsed.y)}`;
					}
				}
			}
		},
		scales: {
			...(getDefaultChartOptions(isDark).scales ?? {}),
			x: {
				...(getDefaultChartOptions(isDark).scales?.x ?? {}),
				stacked: false
			},
			y: {
				...(getDefaultChartOptions(isDark).scales?.y ?? {}),
				type: 'linear' as const,
				beginAtZero: true,
				max: maxBytes,
				position: 'left' as const,
				ticks: {
					...(getDefaultChartOptions(isDark).scales?.y?.ticks ?? {}),
					callback: (value: number) => formatBytesForChart(value)
				}
			},
			...(showIOPS ? {
				y1: {
					type: 'linear' as const,
					beginAtZero: true,
					max: maxIOPS,
					position: 'right' as const,
					grid: {
						drawOnChartArea: false
					},
					ticks: {
						color: getThemeColors(isDark).textSecondary,
						callback: (value: number) => value.toLocaleString()
					}
				}
			} : {})
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
	<Bar data={chartData} options={chartOptions} />
</div>
