<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { initTheme } from '$lib/stores/theme.svelte';
	import { performanceMonitor } from '$lib/utils/performance';
	import { monitoringService } from '$lib/services/monitoring.service';
	import { logger } from '$lib/utils/logger';
	import { isTauri } from '$lib/utils/tauri';

	let { children } = $props();

	onMount(() => {
		initTheme();

		// Start performance monitoring
		performanceMonitor.startMonitoring();

		// Start comprehensive monitoring service (only in Tauri environment)
		if (isTauri()) {
			monitoringService.startMonitoring();
		}

		// Log performance warnings in development
		if (import.meta.env.DEV) {
			setInterval(() => {
				const warnings = performanceMonitor.checkPerformanceHealth();
				if (warnings.length > 0) {
					logger.warn('Performance Issues Detected', { warnings });
				}
			}, 60000); // Check every minute
		}
	});
</script>

<div class="min-h-screen bg-[var(--color-bg)] text-[var(--color-text)]">
	{@render children()}
</div>
