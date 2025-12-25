<script lang="ts">
	import { onMount } from 'svelte';
	import { navigation } from '$lib/stores/navigation.svelte';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	// Direct imports for all components - using eager loading for stability
	// Previous lazy loading approach caused white screen issues
	import Dashboard from '$lib/components/Dashboard.svelte';
	import SmartCleanup from '$lib/components/SmartCleanup.svelte';
	import EnhancedTreeView from '$lib/components/EnhancedTreeView.svelte';
	import FilesystemHealth from '$lib/components/FilesystemHealth.svelte';
	import StartupManager from '$lib/components/StartupManager.svelte';
	import TrashView from '$lib/components/TrashView.svelte';
	import Settings from '$lib/components/Settings.svelte';
	import { invoke, isTauri } from '$lib/utils/tauri';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';

	onMount(async () => {
		logger.debug('App page mounting, checking Tauri API availability...', { component: 'AppPage', action: 'mount' });

		// Check Tauri API availability at runtime with detailed logging
		const tauriAvailable = isTauri();
		logger.debug('Tauri API availability check', { component: 'AppPage', action: 'check_tauri_api', available: tauriAvailable });

		if (!tauriAvailable) {
			logger.error('Tauri API not available', { component: 'AppPage', action: 'check_tauri_api' });
			notificationStore.error(
				'Tauri API Unavailable',
				'This application requires Tauri runtime. Please run with "npm run tauri:dev" (not "npm run dev"). The app is currently running in browser mode which does not support Tauri features.'
			);
			return;
		}

		logger.info('Tauri API is available, initializing application...', { component: 'AppPage', action: 'check_tauri_api' });

		try {
			logger.debug('Calling initialize_app command...', { component: 'AppPage', action: 'initialize_app' });
			await invoke('initialize_app');
			logger.info('App initialized successfully', { component: 'AppPage', action: 'initialize_app' });
		} catch (error) {
			// Log error but don't prevent UI from rendering
			logger.error('App initialization failed', { component: 'AppPage', action: 'initialize_app' }, error);
			const errorMessage = error instanceof Error
				? (error.message.includes('timed out')
					? 'Application initialization timed out. Some features may not work correctly.'
					: error.message.includes('Tauri API not available')
						? 'Tauri API is not available. Please run with "npm run tauri:dev" to launch the desktop application.'
						: `Application initialization failed: ${error.message}`)
				: 'Application initialization failed. Some features may not work correctly.';
			notificationStore.error('Initialization Failed', errorMessage);
		}
	});
</script>

<div class="flex h-screen overflow-hidden">
	<Sidebar />

	<div class="flex flex-col flex-1 overflow-hidden">
		<Header />

		<main class="flex-1 overflow-y-auto scrollbar-thin p-6">
			{#if navigation.view === 'dashboard'}
				<Dashboard />
			{:else if navigation.view === 'cleanup'}
				<SmartCleanup />
			{:else if navigation.view === 'treeview'}
				<EnhancedTreeView />
			{:else if navigation.view === 'filesystem-health'}
				<FilesystemHealth />
			{:else if navigation.view === 'startup'}
				<StartupManager />
			{:else if navigation.view === 'trash'}
				<TrashView />
			{:else if navigation.view === 'settings'}
				<Settings />
			{/if}
		</main>
	</div>
</div>
