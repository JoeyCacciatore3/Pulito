<script lang="ts">
	import { navigation, type View } from '$lib/stores/navigation.svelte';
	import { scanner } from '$lib/stores/scanner.svelte';
	import { formatBytes } from '$lib/utils/tauri';
	import { confirmation } from '$lib/stores/confirmation.svelte';
	import { invoke } from '$lib/utils/tauri';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';

	async function cleanSelectedFromSidebar() {
		const selected = scanner.selectedItems;
		if (selected.length === 0) return;

		const totalSize = scanner.selectedSize;
		const highRiskItems = selected.filter(item => item.risk_level >= 4);

		const confirmed = await confirmation.show({
			title: 'Clean Selected Items',
			message: `Are you sure you want to clean ${selected.length} selected items (${formatBytes(totalSize)})? ${highRiskItems.length > 0 ? `Warning: ${highRiskItems.length} items are high risk. ` : ''}This action will move items to trash where they can be recovered.`,
			confirmText: 'Clean Selected',
			cancelText: 'Cancel',
			type: highRiskItems.length > 0 ? 'warning' : 'info'
		});

		if (!confirmed) return;

		try {
			const result = await invoke<{ cleaned: number; failed: number; total_size: number }>('clean_items', {
				item_ids: selected.map(item => item.id),
				item_paths: selected.map(item => item.path),
				use_trash: true,
				retention_days: 3
			});

			scanner.clearSelection();

			if (result.failed > 0) {
				notificationStore.warning(
					'Cleanup Partial',
					`Cleaned ${result.cleaned} items (${formatBytes(result.total_size)}), but ${result.failed} failed.`
				);
			} else {
				notificationStore.success(
					'Cleanup Complete',
					`Successfully cleaned ${result.cleaned} items, freed ${formatBytes(result.total_size)}.`
				);
			}
		} catch (e) {
			logger.error('Failed to clean items', { component: 'Sidebar', action: 'clean_selected', operation: 'clean_selected_items' }, e);
			notificationStore.error('Cleanup Failed', `Failed to clean items: ${e instanceof Error ? e.message : String(e)}`);
		}
	}

	const navItems: { id: View; label: string; icon: string }[] = [
		{ id: 'dashboard', label: 'Dashboard', icon: 'home' },
		{ id: 'cleanup', label: 'Smart Cleanup', icon: 'sparkles' },
		{ id: 'treeview', label: 'File Explorer', icon: 'folder' },
		{ id: 'filesystem-health', label: 'Filesystem Health', icon: 'health' },
		{ id: 'trash', label: 'Trash', icon: 'trash' },
		{ id: 'settings', label: 'Settings', icon: 'settings' }
	];

		function getIcon(name: string) {
		const icons: Record<string, string> = {
			home: 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6',
			search: 'M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z',
			trash: 'M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16',
			activity: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z',
			'alert-triangle': 'M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z M12 9v4m0 4h.01',
			settings: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z',
			sparkles: 'M5 3a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2V5a2 2 0 00-2-2H5zM5 11a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2v-2a2 2 0 00-2-2H5zM11 5a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V5zM11 13a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z',
			folder: 'M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z M8 5a2 2 0 012-2h4a2 2 0 012 2v2H8V5z',
			health: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z'
		};
		return icons[name] ?? icons.home;
	}
</script>

<aside class="w-64 border-r border-[var(--color-border)] bg-[var(--color-bg-secondary)] flex flex-col">
	<!-- Logo -->
	<div class="px-4 py-4 border-b border-[var(--color-border)]">
		<div class="flex items-center gap-3">
			<div class="w-10 h-10 rounded-lg bg-primary-600 flex items-center justify-center">
				<svg class="w-6 h-6 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M3 6h18M3 12h18M3 18h18" />
					<circle cx="17" cy="6" r="2" fill="currentColor" />
					<circle cx="8" cy="12" r="2" fill="currentColor" />
					<circle cx="14" cy="18" r="2" fill="currentColor" />
				</svg>
			</div>
			<div>
				<div class="font-semibold text-sm">Pulito</div>
				<div class="text-xs text-[var(--color-text-muted)]">v1.0.0</div>
			</div>
		</div>
	</div>

	<!-- Navigation -->
	<nav class="flex-1 px-3 py-4 space-y-1">
		{#each navItems as item}
			<button
				class="w-full sidebar-item {navigation.view === item.id ? 'sidebar-item-active' : 'sidebar-item-inactive'}"
				onclick={() => navigation.set(item.id)}
			>
				<svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d={getIcon(item.icon)} />
				</svg>
				{item.label}
			</button>
		{/each}
	</nav>

	<!-- Selection Summary -->
	{#if scanner.selectedItems.length > 0}
		<div class="px-4 py-3 border-t border-[var(--color-border)] bg-[var(--color-bg)]">
			<div class="text-sm font-medium mb-1">Selected for cleanup</div>
			<div class="text-xs text-[var(--color-text-secondary)]">
				{scanner.selectedItems.length} items · {formatBytes(scanner.selectedSize)}
			</div>
			<button class="btn btn-primary btn-sm w-full mt-2" onclick={cleanSelectedFromSidebar}>
				Clean Selected
			</button>
		</div>
	{/if}

	<!-- Footer -->
	<div class="px-4 py-3 border-t border-[var(--color-border)] text-xs text-[var(--color-text-muted)]">
		Made with ❤️ for Linux
	</div>
</aside>
