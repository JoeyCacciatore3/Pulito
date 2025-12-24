<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke, formatBytes } from '$lib/utils/tauri';
	import { confirmation } from '$lib/stores/confirmation.svelte';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import type { CacheAnalytics } from '$lib/generated/types';

	let analytics = $state<CacheAnalytics | null>(null);
	let loading = $state(false);

	async function loadCacheAnalytics() {
		loading = true;
		try {
			// 30 second timeout for cache analytics
			analytics = await invoke<CacheAnalytics>('get_cache_analytics', undefined, 30000);
		} catch (e) {
			logger.error('Failed to load cache analytics', { component: 'CacheOptimization', action: 'load_analytics' }, e);
			const errorMessage = e instanceof Error && e.message.includes('timed out')
				? 'Cache analytics timed out. Please try again.'
				: 'Could not load cache analytics';
			notificationStore.error('Analytics Error', errorMessage);
		} finally {
			loading = false;
		}
	}

	async function cleanCacheForSource(source: string) {
		const contributor = analytics?.cache_breakdown.find(c => c.source === source);
		if (!contributor) return;

		const confirmed = await confirmation.show({
			title: 'Clean Cache',
			message: `Clean ${formatBytes(contributor.size)} of ${source} cache? This will remove cached data but applications will regenerate it as needed.`,
			confirmText: 'Clean Cache',
			cancelText: 'Cancel',
			type: 'info'
		});

		if (!confirmed) return;

		try {
			// Use the existing clean_cache function based on source
			let result: { cleaned: number; failed: number; total_size: number };
			switch (source) {
				case 'browser':
				case 'Chrome temporary files':
				case 'Firefox cache':
					result = await invoke('clear_cache');
					break;
				case 'development':
				case 'Python packages cache':
				case 'NPM cache':
				case 'Rust Cargo cache':
					result = await invoke('clean_packages');
					break;
				default:
					notificationStore.warning('Not Supported', `Cache cleaning for ${source} is not yet supported`);
					return;
			}

			notificationStore.success('Cache Cleaned', `Removed ${formatBytes(result.total_size)} from ${source} cache`);
			await loadCacheAnalytics(); // Refresh data
		} catch (e) {
			logger.error('Failed to clean cache', { component: 'CacheOptimization', action: 'clean_cache', operation: 'clear_cache' }, e);
			notificationStore.error('Clean Failed', 'Could not clean cache');
		}
	}

	async function cleanAllCaches() {
		if (!analytics) return;

		const confirmed = await confirmation.show({
			title: 'Clean All Caches',
			message: `Clean ${formatBytes(analytics.total_cache_size)} from all cache types? This will remove cached data but applications will regenerate it as needed.`,
			confirmText: 'Clean All Caches',
			cancelText: 'Cancel',
			type: 'warning'
		});

		if (!confirmed) return;

		try {
			// Clean all supported caches
			let totalCleaned = 0;
			let totalSize = 0;

			// Browser caches
			try {
				const browserResult = await invoke<{ cleaned: number; failed: number; total_size: number }>('clear_cache');
				totalCleaned += browserResult.cleaned;
				totalSize += browserResult.total_size;
			} catch (e) {
				logger.warn('Failed to clean browser cache', { component: 'CacheOptimization', action: 'clean_browser_cache' }, e);
			}

			// Package caches
			try {
				const packageResult = await invoke<{ cleaned: number; failed: number; total_size: number }>('clean_packages');
				totalCleaned += packageResult.cleaned;
				totalSize += packageResult.total_size;
			} catch (e) {
				logger.warn('Failed to clean package cache', { component: 'CacheOptimization', action: 'clean_package_cache' }, e);
			}

			// Logs
			try {
				const logResult = await invoke<{ cleaned: number; failed: number; total_size: number }>('clear_logs');
				totalCleaned += logResult.cleaned;
				totalSize += logResult.total_size;
			} catch (e) {
				logger.warn('Failed to clean logs', { component: 'CacheOptimization', action: 'clean_logs' }, e);
			}

			notificationStore.success('All Caches Cleaned', `Removed ${formatBytes(totalSize)} from ${totalCleaned} cache items`);
			await loadCacheAnalytics(); // Refresh data
		} catch (e) {
			logger.error('Failed to clean caches', { component: 'CacheOptimization', action: 'clean_all_caches', operation: 'clean_caches' }, e);
			notificationStore.error('Clean Failed', 'Could not clean all caches');
		}
	}

	function formatGrowthRate(rate: number): string {
		if (rate === 0) return 'No growth';
		if (rate < 0) return `${rate.toFixed(1)} MB/day (shrinking)`;
		return `+${rate.toFixed(1)} MB/day`;
	}

	function getLastActivityText(timestamp: number): string {
		const now = Date.now() / 1000;
		const diff = now - timestamp;
		const days = Math.floor(diff / (24 * 3600));

		if (days === 0) return 'Today';
		if (days === 1) return 'Yesterday';
		if (days < 7) return `${days} days ago`;
		if (days < 30) return `${Math.floor(days / 7)} weeks ago`;
		return `${Math.floor(days / 30)} months ago`;
	}

	function getCacheIcon(source: string): string {
		if (source.toLowerCase().includes('browser') || source.toLowerCase().includes('chrome') || source.toLowerCase().includes('firefox')) {
			return '🌐';
		}
		if (source.toLowerCase().includes('python') || source.toLowerCase().includes('npm') || source.toLowerCase().includes('cargo') || source.toLowerCase().includes('rust')) {
			return '📦';
		}
		if (source.toLowerCase().includes('system') || source.toLowerCase().includes('thumbnail')) {
			return '🖥️';
		}
		return '🗂️';
	}

	// Auto-load analytics on mount
	onMount(async () => {
		await loadCacheAnalytics();
	});
</script>

<div class="cache-optimization">
	<div class="flex items-center justify-between mb-6">
		<div>
			<h2 class="text-2xl font-semibold mb-2 flex items-center gap-2">
				<span class="text-2xl">🗂️</span>
				Cache Optimization
			</h2>
			<p class="text-muted">Analyze and optimize application caches</p>
		</div>
		<button
			class="btn btn-primary"
			onclick={loadCacheAnalytics}
			disabled={loading}
		>
			{#if loading}
				<div class="spinner w-4 h-4 mr-2"></div>
			{/if}
			{loading ? 'Analyzing...' : 'Refresh Analysis'}
		</button>
	</div>

	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="text-center">
				<div class="spinner w-12 h-12 mx-auto mb-4"></div>
				<p class="text-lg">Analyzing cache usage...</p>
				<p class="text-muted">This may take a moment</p>
			</div>
		</div>
	{:else if analytics}
		<!-- Summary Card -->
		<div class="card p-6 mb-6">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-4">
					<div class="w-16 h-16 rounded-full bg-blue-100 flex items-center justify-center">
						<span class="text-3xl">📊</span>
					</div>
					<div>
						<h3 class="text-xl font-semibold">Cache Analysis Complete</h3>
						<p class="text-muted">Found {analytics.cache_breakdown.length} cache sources</p>
					</div>
				</div>
				<div class="text-right">
					<div class="text-2xl font-bold text-blue-600">{formatBytes(analytics.total_cache_size)}</div>
					<div class="text-sm text-muted">total cache size</div>
				</div>
			</div>

			{#if analytics.total_cache_size > 0}
				<div class="mt-6">
					<button
						class="btn btn-primary w-full py-3 text-lg"
						onclick={cleanAllCaches}
					>
						🧹 Clean All Caches ({formatBytes(analytics.total_cache_size)})
					</button>
				</div>
			{/if}
		</div>

		<!-- Cache Breakdown -->
		<div class="space-y-4">
			{#each analytics.cache_breakdown as contributor}
				<div class="card p-5">
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-4">
							<div class="w-12 h-12 rounded-lg bg-gray-100 flex items-center justify-center">
								<span class="text-2xl">{getCacheIcon(contributor.source)}</span>
							</div>
							<div class="min-w-0 flex-1">
								<h3 class="font-semibold">{contributor.source}</h3>
								<div class="text-sm text-muted">
									Last activity: {getLastActivityText(contributor.last_activity)}
								</div>
								<div class="text-sm text-muted">
									Growth: {formatGrowthRate(contributor.growth_rate)}
								</div>
							</div>
						</div>

						<div class="text-right">
							<div class="text-lg font-bold">{formatBytes(contributor.size)}</div>
							<div class="text-sm text-muted">
								{#if contributor.recommended_limit}
									Limit: {formatBytes(contributor.recommended_limit)}
								{:else}
									No limit set
								{/if}
							</div>
						</div>
					</div>

					<!-- Growth indicator -->
					<div class="mt-4">
						<div class="flex items-center justify-between text-sm mb-1">
							<span>Growth trend</span>
							<span class="text-muted">{formatGrowthRate(contributor.growth_rate)}</span>
						</div>
						<div class="w-full bg-gray-200 rounded-full h-2">
							<div
								class="bg-blue-600 h-2 rounded-full transition-all duration-300"
								class:bg-red-600={contributor.growth_rate > 10}
								class:bg-yellow-600={contributor.growth_rate > 5}
								style="width: {Math.min(100, Math.max(10, (contributor.growth_rate / 10) * 100))}%"
							></div>
						</div>
					</div>

					<!-- Action buttons -->
					<div class="mt-4 flex gap-2">
						<button
							class="btn btn-secondary flex-1"
							onclick={() => cleanCacheForSource(contributor.source)}
							disabled={contributor.size === 0}
						>
							Clean {contributor.source}
						</button>
						{#if contributor.recommended_limit}
							<div class="text-xs text-muted self-center px-2">
								Recommended: {formatBytes(contributor.recommended_limit)}
							</div>
						{/if}
					</div>
				</div>
			{/each}
		</div>

		<!-- Growth Trend Chart (Simplified) -->
		{#if analytics.growth_trend.length > 0}
			<div class="card p-6 mt-6">
				<h3 class="text-lg font-semibold mb-4">Cache Growth Trend (Last 7 Days)</h3>
				<div class="flex items-end gap-2 h-32">
					{#each analytics.growth_trend.slice().reverse() as point}
						<div class="flex-1 flex flex-col items-center">
							<div
								class="w-full bg-blue-500 rounded-t transition-all duration-300"
								style="height: {Math.max(4, (point.total_size / Math.max(...analytics.growth_trend.map(p => p.total_size))) * 100)}%"
							></div>
							<div class="text-xs text-muted mt-2">
								{new Date(point.timestamp * 1000).toLocaleDateString('en-US', { weekday: 'short' })}
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	{:else}
		<div class="card p-12 text-center">
			<div class="w-16 h-16 mx-auto mb-4 rounded-full bg-muted flex items-center justify-center">
				<span class="text-3xl">🗂️</span>
			</div>
			<h3 class="text-lg font-medium mb-2">Cache Optimization</h3>
			<p class="text-muted mb-4">Analyze your application cache usage and growth patterns</p>
			<button
				class="btn btn-primary"
				onclick={loadCacheAnalytics}
			>
				Start Cache Analysis
			</button>
		</div>
	{/if}
</div>

<style>
	.spinner {
		border: 2px solid #f3f3f3;
		border-top: 2px solid #3498db;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}
</style>
