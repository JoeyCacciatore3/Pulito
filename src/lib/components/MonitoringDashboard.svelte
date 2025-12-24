<script lang="ts">
	import { monitoringService, type SystemAlert } from '$lib/services/monitoring.service';
	import { performanceMonitor } from '$lib/utils/performance';
	import { formatBytes } from '$lib/utils/tauri';
	import { onMount } from 'svelte';

	let alerts: SystemAlert[] = $state([]);
	let healthStatus: any = $state({});
	let performanceMetrics: any = $state({});

	onMount(() => {
		updateMonitoringData();

		// Update every 30 seconds
		const interval = setInterval(updateMonitoringData, 30000);

		return () => clearInterval(interval);
	});

	function updateMonitoringData() {
		alerts = monitoringService.getAlerts();
		healthStatus = monitoringService.getHealthStatus();
		performanceMetrics = performanceMonitor.getLatestMetrics() || {};
	}

	function acknowledgeAlert(alertId: string) {
		monitoringService.acknowledgeAlert(alertId);
		alerts = monitoringService.getAlerts(); // Update local state
	}

	function resolveAlert(alertId: string) {
		monitoringService.resolveAlert(alertId);
		alerts = monitoringService.getAlerts(); // Update local state
	}

	function clearResolvedAlerts() {
		monitoringService.clearResolvedAlerts();
		alerts = monitoringService.getAlerts(); // Update local state
	}

	function getAlertIcon(type: string) {
		switch (type) {
			case 'critical': return '🚨';
			case 'warning': return '⚠️';
			default: return 'ℹ️';
		}
	}

	function getAlertColor(type: string) {
		switch (type) {
			case 'critical': return 'text-red-600 bg-red-50 border-red-200';
			case 'warning': return 'text-orange-600 bg-orange-50 border-orange-200';
			default: return 'text-blue-600 bg-blue-50 border-blue-200';
		}
	}

	function formatTimestamp(timestamp: number) {
		return new Date(timestamp).toLocaleString();
	}
</script>

<div class="monitoring-dashboard p-6 space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold mb-1 flex items-center gap-2">
				<span class="text-2xl">📊</span>
				System Monitoring
			</h1>
			<p class="text-[var(--color-text-secondary)]">
				Real-time alerts and performance monitoring
			</p>
		</div>

		<div class="flex items-center gap-3">
			<div class="flex items-center gap-2">
				<div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
				<span class="text-sm text-[var(--color-text-secondary)]">
					{healthStatus.isMonitoring ? 'Active' : 'Inactive'}
				</span>
			</div>
			<div class="text-sm text-[var(--color-text-secondary)]">
				{alerts.filter(a => !a.resolvedAt).length} active alerts
			</div>
		</div>
	</div>

	<!-- System Status Overview -->
	<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
		<div class="card p-4">
			<h3 class="font-semibold mb-2">System Health</h3>
			<div class="space-y-2 text-sm">
				<div class="flex justify-between">
					<span>Status</span>
					<span class="font-medium text-green-600">Healthy</span>
				</div>
				<div class="flex justify-between">
					<span>Last Check</span>
					<span class="font-medium">
						{healthStatus.lastHealthCheck ? formatTimestamp(healthStatus.lastHealthCheck.timestamp) : 'Never'}
					</span>
				</div>
			</div>
		</div>

		<div class="card p-4">
			<h3 class="font-semibold mb-2">Performance</h3>
			<div class="space-y-2 text-sm">
				<div class="flex justify-between">
					<span>FPS</span>
					<span class="font-medium">{performanceMetrics.fps || 'N/A'}</span>
				</div>
				<div class="flex justify-between">
					<span>Memory</span>
					<span class="font-medium">
						{performanceMetrics.memoryUsage ? formatBytes(performanceMetrics.memoryUsage) : 'N/A'}
					</span>
				</div>
			</div>
		</div>

		<div class="card p-4">
			<h3 class="font-semibold mb-2">Alerts Summary</h3>
			<div class="space-y-2 text-sm">
				<div class="flex justify-between">
					<span>Active</span>
					<span class="font-medium text-orange-600">{alerts.filter(a => !a.resolvedAt).length}</span>
				</div>
				<div class="flex justify-between">
					<span>Total</span>
					<span class="font-medium">{alerts.length}</span>
				</div>
			</div>
		</div>
	</div>

	<!-- Active Alerts -->
	<div class="card p-6">
		<div class="flex items-center justify-between mb-4">
			<h2 class="text-xl font-semibold">Active Alerts</h2>
			{#if alerts.filter(a => a.resolvedAt).length > 0}
				<button
					class="btn btn-outline btn-sm"
					onclick={clearResolvedAlerts}
				>
					Clear Resolved
				</button>
			{/if}
		</div>

		{#if alerts.filter(a => !a.resolvedAt).length === 0}
			<div class="text-center py-8 text-[var(--color-text-secondary)]">
				<span class="text-4xl">✅</span>
				<p class="mt-2">No active alerts</p>
				<p class="text-sm">System is running normally</p>
			</div>
		{:else}
			<div class="space-y-3">
				{#each alerts.filter(a => !a.resolvedAt) as alert}
					<div class="alert-item border rounded-lg p-4 {getAlertColor(alert.type)}">
						<div class="flex items-start justify-between">
							<div class="flex items-start gap-3">
								<span class="text-xl">{getAlertIcon(alert.type)}</span>
								<div class="flex-1">
									<h3 class="font-semibold">{alert.title}</h3>
									<p class="text-sm mt-1">{alert.message}</p>
									<div class="flex items-center gap-4 mt-2 text-xs opacity-75">
										<span>Source: {alert.source}</span>
										<span>{formatTimestamp(alert.timestamp)}</span>
									</div>
								</div>
							</div>
							<div class="flex gap-2">
								{#if !alert.acknowledged}
									<button
										class="btn btn-outline btn-xs"
										onclick={() => acknowledgeAlert(alert.id)}
									>
										Acknowledge
									</button>
								{/if}
								{#if alert.autoResolve !== false}
									<button
										class="btn btn-primary btn-xs"
										onclick={() => resolveAlert(alert.id)}
									>
										Resolve
									</button>
								{/if}
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Resolved Alerts History -->
	{#if alerts.filter(a => a.resolvedAt).length > 0}
		<div class="card p-6">
			<h2 class="text-xl font-semibold mb-4">Alert History</h2>
			<div class="space-y-2 max-h-64 overflow-y-auto">
				{#each alerts.filter(a => a.resolvedAt).slice(-10) as alert}
					<div class="alert-history-item border rounded p-3 bg-gray-50 dark:bg-gray-800">
						<div class="flex items-center justify-between text-sm">
							<div class="flex items-center gap-2">
								<span>{getAlertIcon(alert.type)}</span>
								<span class="font-medium">{alert.title}</span>
							</div>
							<div class="text-xs text-[var(--color-text-secondary)]">
								Resolved {formatTimestamp(alert.resolvedAt!)}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style>
	.monitoring-dashboard {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	.alert-item {
		transition: all 0.2s ease;
	}

	.alert-item:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}
</style>
