<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { invoke, formatBytes } from '$lib/utils/tauri';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte';
	import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
	import CpuMonitor from '$lib/components/health/CpuMonitor.svelte';
	import MemoryMonitor from '$lib/components/health/MemoryMonitor.svelte';
	import GpuMonitor from '$lib/components/health/GpuMonitor.svelte';
	import NetworkMonitor from '$lib/components/health/NetworkMonitor.svelte';
	import DiskMonitor from '$lib/components/health/DiskMonitor.svelte';
	import TemperatureMonitor from '$lib/components/health/TemperatureMonitor.svelte';
	import ProcessMonitor from '$lib/components/health/ProcessMonitor.svelte';
	import BatteryMonitor from '$lib/components/health/BatteryMonitor.svelte';
	import PerformanceSummary from '$lib/components/health/PerformanceSummary.svelte';
	import type {
		SystemHealthData,
	} from '$lib/generated/types';

	let healthData = $state<SystemHealthData | null>(null);
	let loading = $state(true);
	let updateInterval: ReturnType<typeof setInterval>;
	let isMonitoring = $state(false);
	let lastUpdate = $state(0);
	let updateCount = $state(0);
	let consecutiveErrors = $state(0);

	// Update frequency in milliseconds (5 seconds for system load optimization)
	const UPDATE_INTERVAL = 5000;

	async function fetchHealthData() {
		try {
		updateCount++;
		const startTime = Date.now();
		// 30-second timeout matches backend timeout
		const data = await invoke<SystemHealthData>('get_system_health', undefined, 30000);
		const fetchTime = Date.now() - startTime;

			healthData = data;
			lastUpdate = Date.now();
			consecutiveErrors = 0; // Reset error count on success

		// Only log in development to reduce overhead
		if (import.meta.env.DEV) {
			logger.debug(`System Health Update #${updateCount} (${fetchTime}ms): CPU ${data.cpu_usage.toFixed(1)}%, Memory ${(data.used_memory / 1024 / 1024 / 1024).toFixed(1)}GB, Temp ${data.temperatures.cpu.toFixed(1)}°C`);
		}
		} catch (e) {
			consecutiveErrors++;
			logger.error(`System Health Update #${updateCount} Failed`, { component: 'SystemHealthMonitor', action: 'update_health', updateCount, consecutiveErrors }, e);

			// Show notification after multiple consecutive failures (to avoid spam)
			if (consecutiveErrors === 5) {
				const errorMessage = e instanceof Error && e.message.includes('timed out')
					? 'System health monitoring is experiencing timeouts. The system may be under heavy load.'
					: 'System health monitoring is experiencing errors. Some data may not be up to date.';
				notificationStore.warning('Monitoring Issues', errorMessage);
			}
			// Don't stop monitoring on errors - keep trying
		} finally {
			loading = false;
		}
	}

	function startMonitoring() {
		isMonitoring = true;
		fetchHealthData();
		updateInterval = setInterval(fetchHealthData, UPDATE_INTERVAL);
	}

	function stopMonitoring() {
		isMonitoring = false;
		if (updateInterval) {
			clearInterval(updateInterval);
		}
	}

	onMount(() => {
		startMonitoring();
	});

	onDestroy(() => {
		stopMonitoring();
	});

	function _formatFrequency(freq: number): string {
		if (freq >= 1000) {
			return `${(freq / 1000).toFixed(1)} GHz`;
		}
		return `${freq.toFixed(0)} MHz`;
	}

	function _formatTemp(temp: number): string {
		return `${temp.toFixed(1)}°C`;
	}

</script>

<div class="p-6 space-y-6 animate-in">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold mb-1 flex items-center gap-2">
				<span class="text-2xl">🔥</span>
				System Health Monitor
			</h1>
			<p class="text-[var(--color-text-secondary)]">
				Real-time hardware monitoring and performance metrics
			</p>
		</div>

		<div class="flex items-center gap-3">
			<div class="flex items-center gap-2">
				<div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
				<span class="text-sm text-[var(--color-text-secondary)]">
					{isMonitoring ? `Live • Updates: ${updateCount}` : 'Paused'}
				</span>
			</div>
			<button
				class="btn btn-secondary btn-sm"
				onclick={isMonitoring ? stopMonitoring : startMonitoring}
			>
				{isMonitoring ? '⏸️ Pause' : '▶️ Resume'}
			</button>
		</div>
	</div>

	{#if loading}
		<LoadingSpinner message="Loading system health data..." fullscreen={true} />
	{:else if healthData}
		<!-- Real-time Metrics Grid -->
		<div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
			<!-- CPU Monitor -->
			<CpuMonitor
				cpuUsage={healthData.cpu_usage}
				cpuFrequency={healthData.cpu_frequency}
				cpuCores={healthData.cpu_cores}
				coreUsages={healthData.core_usages}
				temperature={healthData.temperatures.cpu}
			/>

			<!-- Memory Monitor -->
			<MemoryMonitor
				totalMemory={healthData.total_memory}
				usedMemory={healthData.used_memory}
				availableMemory={healthData.available_memory}
			/>

			<!-- GPU Monitor -->
			<GpuMonitor gpuInfo={healthData.gpu_info} />

			<!-- Network Monitor -->
			<NetworkMonitor
				networkUp={healthData.network_up}
				networkDown={healthData.network_down}
				networkInterfaces={healthData.network_interfaces}
				activeConnections={healthData.active_connections}
			/>

			<!-- Disk I/O Monitor -->
			<DiskMonitor
				diskReadBytes={healthData.disk_read_bytes}
				diskWriteBytes={healthData.disk_write_bytes}
				diskReadOps={healthData.disk_read_ops}
				diskWriteOps={healthData.disk_write_ops}
			/>

			<!-- Temperature Monitor -->
			<TemperatureMonitor temperatures={healthData.temperatures} />

			<!-- Process Monitor -->
			<ProcessMonitor topProcesses={healthData.top_processes} />

			<!-- Battery Monitor -->
			<BatteryMonitor batteryInfo={healthData.battery_info} />

			<!-- Swap Memory Monitor -->
			{#if healthData.swap_total > 0}
				<div class="card p-6">
					<div class="flex items-center justify-between mb-4">
						<h3 class="font-semibold flex items-center gap-2">
							<span class="text-xl">💾</span>
							Swap Memory
						</h3>
						<div class="text-right">
							<div class="text-lg font-bold">
								{formatBytes(healthData.swap_used)}
							</div>
							<div class="text-xs text-[var(--color-text-muted)]">
								of {formatBytes(healthData.swap_total)}
							</div>
						</div>
					</div>

					<ProgressBar
						percentage={(healthData.swap_used / healthData.swap_total) * 100}
						color="bg-orange-500"
					/>
				</div>
			{/if}

			<!-- Performance Summary -->
			<PerformanceSummary
				cpuUsage={healthData.cpu_usage}
				loadAverage={healthData.load_average}
				usedMemory={healthData.used_memory}
				totalMemory={healthData.total_memory}
				gpuInfo={healthData.gpu_info}
				timestamp={healthData.timestamp}
				lastUpdate={lastUpdate}
				updateCount={updateCount}
			/>
			<h3 class="font-semibold mb-4 flex items-center gap-2">
				<span class="text-xl">📈</span>
				Performance History
			</h3>
			<div class="text-center py-8 text-[var(--color-text-secondary)]">
				<span class="text-4xl">📊</span>
				<p class="mt-2">Historical performance charts coming soon</p>
				<p class="text-sm mt-1">Track CPU, memory, and GPU usage over time</p>
			</div>
		</div>
	{:else}
		<div class="card p-12 text-center">
			<span class="text-4xl">⚠️</span>
			<h3 class="text-lg font-medium mt-4 mb-2">Unable to Load System Health Data</h3>
			<p class="text-[var(--color-text-secondary)]">
				Please check system permissions or try refreshing the page.
			</p>
		</div>
	{/if}
</div>
