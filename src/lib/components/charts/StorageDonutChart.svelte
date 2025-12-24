<script lang="ts">
	import { onMount } from 'svelte';
	import { Doughnut } from 'svelte5-chartjs';
	import { Chart, ArcElement, Tooltip, Legend } from 'chart.js';
	import { getDefaultChartOptions, getStorageColors, formatBytesForChart, transparentize } from '$lib/utils/charts';

	// Register Chart.js components
	Chart.register(ArcElement, Tooltip, Legend);

	interface Props {
		totalDiskSpace: number;
		usedDiskSpace: number;
		cacheSize: number;
		orphanPackagesSize?: number;
		cleanableSpace: number;
		onSegmentClick?: (_segment: string) => void;
	}

	let {
		totalDiskSpace,
		usedDiskSpace,
		cacheSize,
		orphanPackagesSize = 0,
		cleanableSpace,
		onSegmentClick
	}: Props = $props();

	let isDark = $derived(
		typeof document !== 'undefined' && document.documentElement.classList.contains('dark')
	);

	// Calculate storage breakdown
	let freeSpace = $derived(totalDiskSpace - usedDiskSpace);
	let userFiles = $derived(Math.max(0, usedDiskSpace - cacheSize - (orphanPackagesSize || 0) - cleanableSpace));

	let chartData = $derived({
		labels: ['Caches', 'Packages', 'Logs', 'User Files', 'Free Space'],
		datasets: [
			{
				data: [
					cacheSize,
					orphanPackagesSize || 0,
					cleanableSpace,
					userFiles,
					freeSpace
				],
				backgroundColor: (() => {
					const colors = getStorageColors(isDark);
					return [
						colors.caches,
						colors.packages,
						colors.logs,
						colors.userFiles,
						colors.free
					];
				})(),
				borderColor: (() => {
					const colors = getStorageColors(isDark);
					return [
						transparentize(colors.caches, 1),
						transparentize(colors.packages, 1),
						transparentize(colors.logs, 1),
						transparentize(colors.userFiles, 1),
						transparentize(colors.free, 1)
					];
				})(),
				borderWidth: 2,
				hoverOffset: 8
			}
		]
	});

	let chartOptions = $derived(() => {
		const defaultOptions = getDefaultChartOptions(isDark);
		return {
			...defaultOptions,
			plugins: {
				...(defaultOptions.plugins || {}),
				legend: {
				position: 'right' as const,
				labels: {
					...(defaultOptions.plugins?.legend?.labels || {}),
					padding: 15,
					usePointStyle: true,
					font: {
						size: 12
					},
					generateLabels: (chart: any) => {
						const data = chart.data;
						if (data.labels.length && data.datasets.length) {
							return data.labels.map((label: string, i: number) => {
								const dataset = data.datasets[0];
								const value = dataset.data[i];
								const total = dataset.data.reduce((a: number, b: number) => a + b, 0);
								const percentage = total > 0 ? ((value / total) * 100).toFixed(1) : '0';
								return {
									text: `${label}: ${formatBytesForChart(value)} (${percentage}%)`,
									fillStyle: dataset.backgroundColor[i],
									strokeStyle: dataset.borderColor[i],
									lineWidth: dataset.borderWidth,
									hidden: false,
									index: i
								};
							});
						}
						return [];
					}
				},
				onClick: (e: any, legendItem: any) => {
					if (onSegmentClick && legendItem) {
						const labels = ['caches', 'packages', 'logs', 'userFiles', 'free'];
						onSegmentClick(labels[legendItem.index]);
					}
				}
			},
			tooltip: {
				...(defaultOptions.plugins?.tooltip || {}),
				callbacks: {
					label: (context: any) => {
						const label = context.label || '';
						const value = context.parsed || 0;
						const total = context.dataset.data.reduce((a: number, b: number) => a + b, 0);
						const percentage = total > 0 ? ((value / total) * 100).toFixed(1) : '0';
						return `${label}: ${formatBytesForChart(value)} (${percentage}%)`;
					}
				}
			}
		},
		cutout: '60%',
		animation: {
			animateRotate: true,
			duration: 1000,
			easing: 'easeInOutQuart' as const
		},
		onClick: (event: any, elements: any[]) => {
			if (elements.length > 0 && onSegmentClick) {
				const index = elements[0].index;
				const labels = ['caches', 'packages', 'logs', 'userFiles', 'free'];
				onSegmentClick(labels[index]);
			}
		}
		};
	});

	// Watch for theme changes
	onMount(() => {
		const observer = new MutationObserver(() => {
			// Trigger reactivity by accessing isDark
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

<div class="w-full h-full min-h-[320px]">
	<Doughnut data={chartData} options={chartOptions} />
</div>
