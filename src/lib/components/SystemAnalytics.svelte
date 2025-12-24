<script lang="ts">
	import CPUUsageChart from './charts/CPUUsageChart.svelte';
	import MemoryUsageChart from './charts/MemoryUsageChart.svelte';
	import NetworkTrafficChart from './charts/NetworkTrafficChart.svelte';
	import DiskIOChart from './charts/DiskIOChart.svelte';
	import TemperatureChart from './charts/TemperatureChart.svelte';
	import ProgressBar from './ui/ProgressBar.svelte';
	import { formatBytes } from '$lib/utils/tauri';
	import type { SystemHealthData } from '$lib/generated/types';

	let metrics = $state<SystemHealthData | null>(null);

	function getHealthScore(): { score: number; status: string; color: string } {
		return { score: 85, status: 'Good', color: 'green' };
	}

	function getMetricColor(value: number, thresholds: { good: number; warning: number; critical: number }): string {
		if (value >= thresholds.critical) return 'text-red-600';
		if (value >= thresholds.warning) return 'text-yellow-600';
		return 'text-green-600';
	}


</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold mb-1">System Analytics</h1>
			<p class="text-muted">Real-time system monitoring and performance insights</p>
		</div>
	</div>

	{#if !metrics}
		<div class="card p-8 text-center">
			<div class="text-red-500 mb-4">
				<svg class="w-16 h-16 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
				</svg>
			</div>
			<h2 class="text-xl font-semibold mb-2">System Monitoring Unavailable</h2>
			<p class="text-muted mb-6">Real-time system analytics requires Tauri backend integration.</p>
			<p class="text-sm text-muted">This feature is only available when running as a Tauri application.</p>
		</div>
	{:else}
		{#if metrics}
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
					data={[50, 60, 55, 70, 65]}
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
					data={[40, 45, 50, 48, 52]}
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
					data={[100, 120, 90, 150, 110]}
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
					data={[200, 180, 220, 190, 210]}
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
					data={[45, 50, 48, 52, 47]}
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
							<span class="font-medium {getMetricColor(metrics.gpu_info?.temperature ?? 0, {good: 60, warning: 75, critical: 85})}">
								{metrics.gpu_info?.temperature?.toFixed(0) ?? 'N/A'}°C
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
	{/if}
	{/if}
</div>
