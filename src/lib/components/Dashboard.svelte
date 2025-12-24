<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke, formatBytes } from '$lib/utils/tauri';
	import { navigation } from '$lib/stores/navigation.svelte';
	import { confirmation } from '$lib/stores/confirmation.svelte';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import StorageDonutChart from './charts/StorageDonutChart.svelte';
	import PerformanceGauge from './charts/PerformanceGauge.svelte';
	import SparklineChart from './charts/SparklineChart.svelte';
	import { getCPUHistory, getMemoryHistory } from '$lib/stores/metrics-history.svelte';
	import { invoke as tauriInvoke } from '@tauri-apps/api/core';

	import type { SystemStats } from '$lib/generated/types';

	interface CleanupCategory {
		id: string;
		title: string;
		description: string;
		icon: string;
		color: string;
		estimatedSavings: number | null; // null means "Scan to see savings"
		itemCount: number;
		riskLevel: 'safe' | 'caution' | 'warning';
		action: string;
		route: string;
		hasScanned: boolean; // Whether this category has been scanned
	}

	let stats = $state<SystemStats | null>(null);
	let loading = $state(true);
	let categories = $state<CleanupCategory[]>([]);
	let healthScore = $state(85); // Default health score
	let cpuHistory = $derived(getCPUHistory('1h').map(d => d.usage).slice(-30));
	let memoryHistory = $derived(getMemoryHistory('1h').map(d => (d.usedMemory / d.totalMemory) * 100).slice(-30));

	// Generate cleanup categories based on system stats
	function generateCategories(): CleanupCategory[] {
		if (!stats) return [];

		const cats: CleanupCategory[] = [];

		// Smart Cleanup - unified AI-powered cleanup
		cats.push({
			id: 'smart_cleanup',
			title: 'Smart Cleanup',
			description: 'AI-powered system optimization with intelligent recommendations',
			icon: '🤖',
			color: 'bg-gradient-to-r from-blue-100 to-purple-100 border-blue-200 text-blue-800',
			estimatedSavings: (stats.cache_size + (stats.orphan_packages_size || 0) + (stats.cleanable_space || 0)) || null,
			itemCount: stats.orphan_packages + (stats.cleanable_space > 0 ? 1 : 0) + (stats.cache_size > 0 ? 1 : 0),
			riskLevel: 'safe',
			action: 'Smart Clean',
			route: 'cleanup',
			hasScanned: true
		});

		// System Analytics - real-time monitoring
		cats.push({
			id: 'analytics',
			title: 'System Analytics',
			description: 'Real-time performance monitoring and health insights',
			icon: '📊',
			color: 'bg-gradient-to-r from-green-100 to-blue-100 border-green-200 text-green-800',
			estimatedSavings: null,
			itemCount: 0,
			riskLevel: 'safe',
			action: 'View Analytics',
			route: 'analytics',
			hasScanned: false
		});

		return cats;
	}

	function getRiskIcon(risk: string): string {
		switch (risk) {
			case 'safe': return '🟢';
			case 'caution': return '🟡';
			case 'warning': return '🟠';
			default: return '⚪';
		}
	}

	// Validate system stats data for reasonableness
	function validateStats(data: SystemStats | null): boolean {
		if (!data) return false;

		// Check for negative values
		if (data.total_disk_space < 0 || data.used_disk_space < 0 || data.cache_size < 0) {
			logger.warn('Invalid stats: negative values detected', { component: 'Dashboard', action: 'validate_stats' });
			notificationStore.warning('Invalid Data', 'Received invalid system statistics with negative values. Please try refreshing.');
			return false;
		}

		// Check that used space doesn't exceed total
		if (data.used_disk_space > data.total_disk_space) {
			logger.warn('Invalid stats: used space exceeds total space', { component: 'Dashboard', action: 'validate_stats' });
			notificationStore.warning('Invalid Data', 'Received invalid system statistics. Used space exceeds total space.');
			return false;
		}

		// Check for impossibly large values (more than 1 petabyte)
		const maxReasonable = 1024 * 1024 * 1024 * 1024 * 1024; // 1 PB
		if (data.total_disk_space > maxReasonable) {
			logger.warn('Invalid stats: total disk space seems unreasonably large', { component: 'Dashboard', action: 'validate_stats' });
			notificationStore.warning('Invalid Data', 'Received invalid system statistics with unreasonably large values.');
			return false;
		}

		return true;
	}

	async function loadStats() {
		try {
			const data = await invoke<SystemStats>('get_system_stats', undefined, 30000); // 30s timeout
			if (validateStats(data)) {
				stats = data;
				categories = generateCategories();
				logger.debug('System stats loaded successfully', { component: 'Dashboard', action: 'load_stats' });

				// Calculate health score
				try {
					const healthData = await tauriInvoke<any>('get_system_health', undefined);
					if (healthData) {
						let score = 100;
						if (healthData.cpu_usage > 80) score -= 20;
						else if (healthData.cpu_usage > 60) score -= 10;
						const memoryPercent = (healthData.used_memory / healthData.total_memory) * 100;
						if (memoryPercent > 90) score -= 25;
						else if (memoryPercent > 75) score -= 15;
						if (healthData.temperatures?.cpu > 80) score -= 15;
						else if (healthData.temperatures?.cpu > 70) score -= 5;
						healthScore = Math.max(0, Math.min(100, score));
					}
				} catch (_e) {
					// Ignore health score errors
				}
			} else {
				logger.error('Received invalid system stats, keeping previous data', { component: 'Dashboard', action: 'load_stats' });
				notificationStore.error('Load Failed', 'Failed to load system statistics. Invalid data received from server.');
			}
		} catch (e) {
			logger.error('Failed to get system stats', { component: 'Dashboard', action: 'load_stats', operation: 'get_system_stats' }, e);
			const errorMessage = e instanceof Error
				? (e.message.includes('timed out')
					? 'Request timed out. Please try again.'
					: e.message)
				: 'Failed to load system statistics. Please try again.';
			notificationStore.error('Load Failed', errorMessage);
		}
	}

	onMount(() => {
		// Load initial stats
		loadStats().then(() => {
			loading = false;
		});

		// Refresh dashboard stats every 30 seconds with exponential backoff on errors
		let consecutiveErrors = 0;
		const baseInterval = 30000; // 30 seconds
		let currentInterval = baseInterval;

		const refreshStats = async () => {
			try {
				const data = await invoke<SystemStats>('get_system_stats', undefined, 15000); // 15s timeout for refresh
				if (validateStats(data)) {
					stats = data;
					categories = generateCategories();
					consecutiveErrors = 0;
					currentInterval = baseInterval; // Reset to base interval on success
				} else {
					logger.warn('Received invalid stats during refresh, keeping previous data', { component: 'Dashboard', action: 'refresh_stats' });
					consecutiveErrors++;
					// Only show notification after multiple consecutive failures to avoid spam
					if (consecutiveErrors >= 3) {
						notificationStore.warning('Refresh Failed', 'Failed to refresh dashboard statistics. Showing last known data.');
					}
				}
			} catch (e) {
				consecutiveErrors++;
				logger.error('Failed to refresh dashboard stats', { component: 'Dashboard', action: 'refresh_stats', consecutiveErrors }, e);

				// Only show notification after multiple consecutive failures to avoid spam
				if (consecutiveErrors >= 3) {
					const errorMessage = e instanceof Error && e.message.includes('timed out')
						? 'Dashboard refresh timed out. Please try refreshing manually.'
						: 'Failed to refresh dashboard statistics. Showing last known data.';
					notificationStore.warning('Refresh Failed', errorMessage);
				}

				// Exponential backoff: double the interval after each error, max 5 minutes
				if (consecutiveErrors > 0) {
					currentInterval = Math.min(baseInterval * Math.pow(2, consecutiveErrors - 1), 300000);
					logger.debug('Refresh interval increased due to errors', { component: 'Dashboard', action: 'refresh_stats', interval: `${currentInterval}ms` });
				}
			}
		};

		// Initial refresh after base interval
		let refreshTimeout: ReturnType<typeof setTimeout>;
		const scheduleRefresh = () => {
			refreshTimeout = setTimeout(async () => {
				await refreshStats();
				scheduleRefresh(); // Schedule next refresh
			}, currentInterval);
		};

		scheduleRefresh();

		return () => {
			clearTimeout(refreshTimeout);
			logger.debug('Dashboard refresh cleared', { component: 'Dashboard', action: 'cleanup' });
		};
	});

	function getDiskUsagePercent(): number {
		if (!stats) return 0;
		return (stats.used_disk_space / stats.total_disk_space) * 100;
	}

	function _getDiskUsageColor(): string {
		const percent = getDiskUsagePercent();
		if (percent < 60) return 'bg-safe';
		if (percent < 80) return 'bg-caution';
		return 'bg-critical';
	}

	function getGreeting(): string {
		const hour = new Date().getHours();
		if (hour < 12) return 'Good morning';
		if (hour < 18) return 'Good afternoon';
		return 'Good evening';
	}

	async function handleCategoryAction(category: CleanupCategory) {
		// Navigate to the appropriate feature
		navigation.set(category.route as any);
	}

	async function startQuickClean() {
		const confirmed = await confirmation.show({
			title: 'Quick System Clean',
			message: 'Run AI-powered smart cleanup for immediate system optimization.',
			confirmText: 'Start Smart Clean',
			cancelText: 'Cancel',
			type: 'info'
		});

		if (confirmed) {
			navigation.set('cleanup');
		}
	}
</script>

<div class="p-6 space-y-6 animate-in">
	<div>
		<h1 class="text-2xl font-bold mb-2">{getGreeting()}! 👋</h1>
		<p class="text-[var(--color-text-secondary)]">
			Choose a cleanup category to optimize your Linux system
		</p>
	</div>

	<!-- Quick Actions Bar -->
	{#if stats}
		<div class="card p-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-4">
					<div class="w-12 h-12 rounded-full bg-green-100 flex items-center justify-center">
						<span class="text-2xl">⚡</span>
					</div>
					<div>
						<h3 class="font-semibold">Ready to Clean</h3>
						<p class="text-sm text-muted">Safe cleanup options available</p>
					</div>
				</div>
				<div class="flex gap-2">
					<button class="btn btn-secondary" onclick={startQuickClean}>
						Smart Clean
					</button>
					<button class="btn btn-primary" onclick={() => navigation.set('analytics')}>
						System Analytics
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- Cleanup Categories Grid -->
	{#if categories.length > 0}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
			{#each categories as category}
				<div class="card p-5 hover:shadow-lg transition-shadow cursor-pointer" role="button" tabindex="0" onclick={() => handleCategoryAction(category)} onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handleCategoryAction(category); } }}>
					<div class="flex items-center gap-3 mb-3">
						<div class="w-12 h-12 rounded-lg {category.color} flex items-center justify-center">
							<span class="text-2xl">{category.icon}</span>
						</div>
						<div>
							<h3 class="font-semibold">{category.title}</h3>
							{#if category.itemCount > 0}
								<p class="text-sm text-muted">{category.itemCount} items found</p>
							{/if}
						</div>
					</div>
					<p class="text-sm text-muted mb-4">{category.description}</p>

					<div class="flex items-center justify-between mb-3">
						{#if category.estimatedSavings !== null && category.estimatedSavings > 0}
							<span class="text-sm font-medium text-green-600">
								{formatBytes(category.estimatedSavings)} savings
							</span>
						{:else}
							<span class="text-sm font-medium text-gray-500 italic">
								Scan to see savings
							</span>
						{/if}
						<span class="text-xs px-2 py-1 rounded-full bg-gray-100 text-gray-600">
							{getRiskIcon(category.riskLevel)} {category.riskLevel}
						</span>
					</div>

					<button class="btn btn-primary w-full" onclick={() => handleCategoryAction(category)}>
						{category.action}
					</button>
				</div>
			{/each}
		</div>
	{/if}

	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="spinner"></div>
		</div>
	{:else if stats}
		<!-- Charts Grid -->
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
			<!-- Storage Breakdown -->
			<div class="lg:col-span-2 card p-6">
				<h2 class="font-semibold mb-4">Storage Breakdown</h2>
				<StorageDonutChart
					data={[60, 30, 10]}
					height={200}
				/>
			</div>
			<div class="card p-6 flex flex-col items-center justify-center">
				<h2 class="font-semibold mb-4">System Health</h2>
				<PerformanceGauge value={healthScore} size={180} />
			</div>

		<!-- Quick Stats with Sparklines -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
			<div class="card p-4">
				<div class="flex items-center justify-between mb-2">
					<span class="text-sm text-muted">CPU Usage</span>
					<span class="font-semibold">
						{cpuHistory.length > 0 ? cpuHistory[cpuHistory.length - 1].toFixed(1) : '0'}%
					</span>
				</div>
				<SparklineChart data={cpuHistory.length > 0 ? cpuHistory : [0]} color="rgb(59, 130, 246)" height={40} />
			</div>
			<div class="card p-4">
				<div class="flex items-center justify-between mb-2">
					<span class="text-sm text-muted">Memory Usage</span>
					<span class="font-semibold">
						{memoryHistory.length > 0 ? memoryHistory[memoryHistory.length - 1].toFixed(1) : '0'}%
					</span>
				</div>
				<SparklineChart data={memoryHistory.length > 0 ? memoryHistory : [0]} color="rgb(16, 185, 129)" height={40} />
			</div>
		</div>
		</div>
	{/if}
</div>
