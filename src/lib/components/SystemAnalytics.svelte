<script lang="ts">
	import { onMount } from 'svelte';

	import { invoke, formatBytes } from '$lib/utils/tauri';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import LoadingSpinner from './ui/LoadingSpinner.svelte';
	import ProgressBar from './ui/ProgressBar.svelte';
	import CPUUsageChart from './charts/CPUUsageChart.svelte';
	import MemoryUsageChart from './charts/MemoryUsageChart.svelte';
	import NetworkTrafficChart from './charts/NetworkTrafficChart.svelte';
	import DiskIOChart from './charts/DiskIOChart.svelte';
	import TemperatureChart from './charts/TemperatureChart.svelte';
	import {
		addCPUData,
		addMemoryData,
		addNetworkData,
		addTemperatureData,
		addDiskIOData,
		getCPUHistory,
		getMemoryHistory,
		getNetworkHistory,
		getTemperatureHistory,
		getDiskIOHistory
	} from '$lib/stores/metrics-history.svelte';

	interface SystemMetrics {
		// CPU
		cpu_usage: number;
		cpu_cores: number;
		cpu_frequency: number;
		core_usages: number[];

		// Memory
		total_memory: number;
		used_memory: number;
		available_memory: number;
		swap_total: number;
		swap_used: number;

		// GPU
		gpu_info?: {
			name: string;
			usage: number;
			memory_used: number;
			memory_total: number;
			temperature: number;
		};

		// Network
		network_up: number;
		network_down: number;
		active_connections: Array<{
			local_address: string;
			remote_address: string;
			status: string;
		}>;

		// Disk I/O
		disk_read_bytes: number;
		disk_write_bytes: number;
		disk_read_ops: number;
		disk_write_ops: number;

		// Temperatures
		temperatures: {
			cpu: number;
			system: number;
			gpu?: number;
		};

		// Processes
		top_processes: Array<{
			name: string;
			pid: number;
			cpu_usage: number;
			memory_usage: number;
		}>;

		// System load averages
		load_average?: {
			one_minute: number;
			five_minutes: number;
			fifteen_minutes: number;
		};
	}

	let metrics = $state<SystemMetrics | null>(null);
	let loading = $state(true);
	let realTimeMode = $state(false);
	let updateInterval = $state<NodeJS.Timeout | null>(null);
	let timeRange = $state<'1h' | '6h' | '24h' | 'all'>('all');

	async function loadSystemMetrics() {
		try {
			const data = await invoke<SystemMetrics>('get_system_health', undefined, 10000);
			metrics = data;

			// Store metrics in history
			if (data) {
				addCPUData({
					usage: data.cpu_usage,
					coreUsages: data.core_usages
				});

				addMemoryData({
					usedMemory: data.used_memory,
					totalMemory: data.total_memory,
					swapUsed: data.swap_used,
					swapTotal: data.swap_total
				});

				addNetworkData({
					upload: data.network_up,
					download: data.network_down
				});

				addTemperatureData({
					cpu: data.temperatures.cpu,
					gpu: data.temperatures.gpu,
					system: data.temperatures.system
				});

				addDiskIOData({
					readBytes: data.disk_read_bytes,
					writeBytes: data.disk_write_bytes,
					readOps: data.disk_read_ops,
					writeOps: data.disk_write_ops
				});
			}
		} catch (e) {
			console.error('SystemAnalytics: Error loading metrics:', e);
			logger.error('Failed to load system metrics', { component: 'SystemAnalytics' }, e);
			notificationStore.error('Failed to Load Metrics', 'Could not retrieve system metrics. Please check system permissions and try again.');
			metrics = null;
		} finally {
			loading = false;
		}
	}

	function toggleRealTimeMode() {
		realTimeMode = !realTimeMode;

		if (realTimeMode) {
			// Start real-time updates every 2 seconds
			updateInterval = setInterval(loadSystemMetrics, 2000);
			notificationStore.info('Real-Time Mode', 'System monitoring activated');
		} else {
			// Stop real-time updates
			if (updateInterval) {
				clearInterval(updateInterval);
				updateInterval = null;
			}
			notificationStore.info('Real-Time Mode', 'System monitoring deactivated');
		}
	}

	function getHealthScore(): { score: number; status: string; color: string } {
		if (!metrics) return { score: 0, status: 'Unknown', color: 'gray' };

		let score = 100;

		// CPU health
		if (metrics.cpu_usage > 80) score -= 20;
		else if (metrics.cpu_usage > 60) score -= 10;

		// Memory health
		const memoryPercent = (metrics.used_memory / metrics.total_memory) * 100;
		if (memoryPercent > 90) score -= 25;
		else if (memoryPercent > 75) score -= 15;

		// Temperature health
		if (metrics.temperatures.cpu > 80) score -= 15;
		else if (metrics.temperatures.cpu > 70) score -= 5;

		let status: string;
		let color: string;

		if (score >= 90) {
			status = 'Excellent';
			color = 'green';
		} else if (score >= 75) {
			status = 'Good';
			color = 'blue';
		} else if (score >= 60) {
			status = 'Fair';
			color = 'yellow';
		} else if (score >= 40) {
			status = 'Poor';
			color = 'orange';
		} else {
			status = 'Critical';
			color = 'red';
		}

		return { score, status, color };
	}

	function getMetricColor(value: number, thresholds: { good: number; warning: number; critical: number }): string {
		if (value >= thresholds.critical) return 'text-red-600';
		if (value >= thresholds.warning) return 'text-yellow-600';
		return 'text-green-600';
	}

	onMount(() => {
		loadSystemMetrics();
	});

	// Cleanup on unmount
	$effect(() => {
		return () => {
			if (updateInterval) {
				clearInterval(updateInterval);
			}
		};
	});
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold mb-1">System Analytics</h1>
			<p class="text-muted">Real-time system monitoring and performance insights</p>
		</div>
		<div class="flex gap-2 items-center">
			<!-- Time Range Selector -->
			<div class="flex gap-1 bg-gray-100 dark:bg-gray-800 rounded-lg p-1">
				<button
					class="px-3 py-1 text-sm rounded {timeRange === '1h' ? 'bg-white dark:bg-gray-700 font-medium' : ''}"
					onclick={() => timeRange = '1h'}
				>
					1h
				</button>
				<button
					class="px-3 py-1 text-sm rounded {timeRange === '6h' ? 'bg-white dark:bg-gray-700 font-medium' : ''}"
					onclick={() => timeRange = '6h'}
				>
					6h
				</button>
				<button
					class="px-3 py-1 text-sm rounded {timeRange === '24h' ? 'bg-white dark:bg-gray-700 font-medium' : ''}"
					onclick={() => timeRange = '24h'}
				>
					24h
				</button>
				<button
					class="px-3 py-1 text-sm rounded {timeRange === 'all' ? 'bg-white dark:bg-gray-700 font-medium' : ''}"
					onclick={() => timeRange = 'all'}
				>
					All
				</button>
			</div>
			<button
				class="btn {realTimeMode ? 'btn-primary' : 'btn-secondary'}"
				onclick={toggleRealTimeMode}
			>
				{#if realTimeMode}
					⏹️ Stop Live
				{:else}
					▶️ Live Monitor
				{/if}
			</button>
			<button class="btn btn-secondary" onclick={loadSystemMetrics} disabled={loading}>
				🔄 Refresh
			</button>
		</div>
	</div>

	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="flex flex-col items-center gap-4">
				<LoadingSpinner size="lg" />
				<p class="text-muted">Loading system metrics...</p>
			</div>
		</div>
	{:else if metrics}
		<!-- Health Score Overview -->
		<div class="card p-6 bg-gradient-to-r from-blue-50 to-indigo-50 border-blue-200">
			<div class="flex items-center justify-between">
				<div>
					<h3 class="text-lg font-semibold mb-1">System Health Score</h3>
					<p class="text-sm text-muted">Overall system performance rating</p>
				</div>
				<div class="text-right">
					<div class="text-4xl font-bold text-{getHealthScore().color}-600 mb-1">
						{getHealthScore().score}%
					</div>
					<div class="text-sm font-medium text-{getHealthScore().color}-600">
						{getHealthScore().status}
					</div>
				</div>
			</div>
		</div>

		<!-- Real-time Indicator -->
		{#if realTimeMode}
			<div class="flex items-center gap-2 text-sm text-green-600 bg-green-50 px-3 py-2 rounded-lg border border-green-200">
				<div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
				<span class="font-medium">Live monitoring active</span>
				<span class="text-muted">• Updates every 2 seconds</span>
			</div>
		{/if}

		<!-- Charts Grid -->
		<div class="space-y-6">
			<!-- CPU Usage Chart -->
			<div class="card p-6">
				<div class="flex items-center justify-between mb-4">
					<div>
						<h3 class="font-semibold text-lg">CPU Usage</h3>
						<p class="text-sm text-muted">Current: {metrics.cpu_usage.toFixed(1)}% • Cores: {metrics.cpu_cores}</p>
					</div>
					<div class="text-right">
						<div class="text-sm text-muted">Temperature</div>
						<div class="font-semibold {getMetricColor(metrics.temperatures.cpu, {good: 60, warning: 75, critical: 85})}">
							{metrics.temperatures.cpu.toFixed(0)}°C
						</div>
					</div>
				</div>
				<CPUUsageChart
					data={[50, 60, 55, 70, 65]} <!-- Placeholder data -->
					height={120}
				/>
			</div>

			<!-- Memory Usage Chart -->
			<div class="card p-6">
				<div class="flex items-center justify-between mb-4">
					<div>
						<h3 class="font-semibold text-lg">Memory Usage</h3>
						<p class="text-sm text-muted">
							Used: {formatBytes(metrics.used_memory)} / {formatBytes(metrics.total_memory)}
							{metrics.swap_used > 0 ? ` • Swap: ${formatBytes(metrics.swap_used)}` : ''}
						</p>
					</div>
				</div>
				<MemoryUsageChart
					data={[40, 45, 50, 48, 52]} <!-- Placeholder data -->
					height={120}
				/>
			</div>

			<!-- Network Traffic Chart -->
			<div class="card p-6">
				<div class="flex items-center justify-between mb-4">
					<div>
						<h3 class="font-semibold text-lg">Network Traffic</h3>
						<p class="text-sm text-muted">
							Upload: {formatBytes(metrics.network_up)} • Download: {formatBytes(metrics.network_down)}
						</p>
					</div>
				</div>
				<NetworkTrafficChart
					data={[100, 120, 90, 150, 110]} <!-- Placeholder data -->
					height={120}
				/>
			</div>

			<!-- Disk I/O Chart -->
			<div class="card p-6">
				<div class="flex items-center justify-between mb-4">
					<div>
						<h3 class="font-semibold text-lg">Disk I/O</h3>
						<p class="text-sm text-muted">
							Read: {formatBytes(metrics.disk_read_bytes)} • Write: {formatBytes(metrics.disk_write_bytes)}
						</p>
					</div>
				</div>
				<DiskIOChart
					data={[200, 180, 220, 190, 210]} <!-- Placeholder data -->
					height={120}
				/>
			</div>

			<!-- Temperature Chart -->
			<div class="card p-6">
				<div class="flex items-center justify-between mb-4">
					<div>
						<h3 class="font-semibold text-lg">Temperature Monitoring</h3>
						<p class="text-sm text-muted">CPU: {metrics.temperatures.cpu.toFixed(0)}°C • System: {metrics.temperatures.system.toFixed(0)}°C</p>
					</div>
				</div>
				<TemperatureChart
					data={[45, 50, 48, 52, 47]} <!-- Placeholder data -->
					height={120}
				/>
			</div>

			<!-- GPU Metrics (if available) -->
			{#if metrics.gpu_info}
				<div class="card p-5">
					<div class="flex items-center gap-3 mb-4">
						<div class="w-10 h-10 rounded-lg bg-orange-100 flex items-center justify-center">
							<span class="text-xl">🎮</span>
						</div>
						<div>
							<h3 class="font-semibold">GPU</h3>
							<p class="text-sm text-muted">Graphics performance</p>
						</div>
					</div>

					<div class="space-y-3">
						<div class="flex justify-between items-center">
							<span class="text-sm">Name</span>
							<span class="font-medium text-sm">
								{metrics.gpu_info.name}
							</span>
						</div>

						<div class="flex justify-between items-center">
							<span class="text-sm">Usage</span>
							<span class="font-medium {getMetricColor(metrics.gpu_info.usage, {good: 60, warning: 80, critical: 90})}">
								{metrics.gpu_info.usage.toFixed(1)}%
							</span>
						</div>

						<div class="flex justify-between items-center">
							<span class="text-sm">VRAM</span>
							<span class="font-medium text-sm">
								{formatBytes(metrics.gpu_info.memory_used)} / {formatBytes(metrics.gpu_info.memory_total)}
							</span>
						</div>

						<div class="flex justify-between items-center">
							<span class="text-sm">Temp</span>
							<span class="font-medium {getMetricColor(metrics.gpu_info.temperature, {good: 60, warning: 75, critical: 85})}">
								{metrics.gpu_info.temperature.toFixed(0)}°C
							</span>
						</div>

						<ProgressBar percentage={metrics.gpu_info.usage} class="h-2 mt-3" />
					</div>
				</div>
			{/if}

			<!-- Process Overview -->
			<div class="card p-5">
				<div class="flex items-center gap-3 mb-4">
					<div class="w-10 h-10 rounded-lg bg-red-100 flex items-center justify-center">
						<span class="text-xl">⚙️</span>
					</div>
					<div>
						<h3 class="font-semibold">Processes</h3>
						<p class="text-sm text-muted">System activity</p>
					</div>
				</div>

				<div class="space-y-3">
					<div class="flex justify-between items-center">
						<span class="text-sm">Total Processes</span>
						<span class="font-medium">{metrics.top_processes.length}</span>
					</div>

					{#if metrics.top_processes.length > 0}
						<div class="border-t pt-3">
							<p class="text-xs text-muted mb-2">Top CPU consumers:</p>
							{#each metrics.top_processes.slice(0, 3) as process}
								<div class="flex justify-between text-xs">
									<span class="truncate">{process.name}</span>
									<span>{process.cpu_usage.toFixed(1)}%</span>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</div>

		</div>

		<!-- Performance Insights -->
		<div class="card p-6">
			<h3 class="text-lg font-semibold mb-4">Performance Insights</h3>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<div class="p-4 bg-blue-50 rounded-lg border border-blue-200">
					<h4 class="font-medium text-blue-900 mb-2">CPU Load Trend</h4>
					<p class="text-sm text-blue-800">
						{#if metrics.load_average}
							Load averages: {metrics.load_average.one_minute.toFixed(2)} (1m),
							{metrics.load_average.five_minutes.toFixed(2)} (5m),
							{metrics.load_average.fifteen_minutes.toFixed(2)} (15m)
						{:else}
							Load average data not available
						{/if}
					</p>
				</div>

				<div class="p-4 bg-green-50 rounded-lg border border-green-200">
					<h4 class="font-medium text-green-900 mb-2">Memory Efficiency</h4>
					<p class="text-sm text-green-800">
						{((metrics.used_memory / metrics.total_memory) * 100).toFixed(1)}% RAM usage
						{#if metrics.swap_used > 0}
							• {((metrics.swap_used / metrics.swap_total) * 100).toFixed(1)}% swap usage
						{/if}
					</p>
				</div>
			</div>
		</div>
	{:else}
		<div class="card p-8 text-center">
			<div class="text-4xl mb-4">📊</div>
			<h3 class="text-lg font-semibold mb-2">No System Data Available</h3>
			<p class="text-muted mb-4">Could not retrieve system metrics. Please check system permissions.</p>
			<button class="btn btn-primary" onclick={loadSystemMetrics}>
				🔄 Try Again
			</button>
		</div>
	{/if}
</div>
