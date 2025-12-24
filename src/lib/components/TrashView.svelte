<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke, formatBytes, formatRelativeTime, handleTauriError } from '$lib/utils/tauri';
	import { confirmation } from '$lib/stores/confirmation.svelte';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import type { TrashData } from '$lib/generated/types';

	let trashData = $state<TrashData | null>(null);
	let loading = $state(true);

	onMount(async () => {
		await loadTrash();
	});

	async function loadTrash() {
		loading = true;
		try {
			trashData = await invoke<TrashData>('get_trash_items', undefined, 15000); // 15s timeout
		} catch (e) {
			handleTauriError('load trash', e, { operation: 'get_trash_items' });
		} finally {
			loading = false;
		}
	}

	async function restoreItem(id: string) {
		const confirmed = await confirmation.show({
			title: 'Restore Item',
			message: 'Are you sure you want to restore this item to its original location?',
			confirmText: 'Restore',
			cancelText: 'Cancel',
			type: 'info'
		});

		if (!confirmed) return;

		try {
			await invoke('restore_from_trash', { id });
			notificationStore.success('Item Restored', 'Item has been restored to its original location');
			await loadTrash();
		} catch (e) {
			handleTauriError('restore item', e, { operation: 'restore_from_trash', itemId: id });
		}
	}

	async function deleteItem(id: string) {
		const confirmed = await confirmation.show({
			title: 'Delete Permanently',
			message: 'This action cannot be undone. The item will be permanently deleted.',
			confirmText: 'Delete Forever',
			cancelText: 'Cancel',
			type: 'danger'
		});

		if (!confirmed) return;

		try {
			await invoke('delete_from_trash', { id });
			notificationStore.success('Item Deleted', 'Item has been permanently deleted');
			await loadTrash();
		} catch (e) {
			handleTauriError('delete item', e, { operation: 'delete_from_trash', itemId: id });
		}
	}

	async function emptyTrash() {
		const confirmed = await confirmation.show({
			title: 'Empty Trash',
			message: `Permanently delete all ${trashData?.total_items || 0} items in trash? This action cannot be undone.`,
			confirmText: 'Empty Trash',
			cancelText: 'Cancel',
			type: 'danger'
		});

		if (!confirmed) return;

		try {
			await invoke('empty_trash');
			notificationStore.success('Trash Emptied', 'All items have been permanently deleted');
			await loadTrash();
		} catch (e) {
			handleTauriError('empty trash', e, { operation: 'empty_trash' });
		}
	}

	function getTimeUntilExpiry(expiresAt: string): string {
		const expires = new Date(expiresAt);
		const now = new Date();
		const diff = expires.getTime() - now.getTime();

		if (diff <= 0) return 'Expired';

		const hours = Math.floor(diff / (1000 * 60 * 60));
		const days = Math.floor(hours / 24);

		if (days > 0) return `${days}d ${hours % 24}h remaining`;
		return `${hours}h remaining`;
	}
</script>

<div class="p-6 space-y-6 animate-in">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold mb-2">Trash</h1>
			<p class="text-[var(--color-text-secondary)]">
				Items moved to trash before permanent deletion
			</p>
		</div>

		{#if trashData && trashData.total_items > 0}
			<button class="btn btn-danger" onclick={emptyTrash}>
				<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
				</svg>
				Empty Trash
			</button>
		{/if}
	</div>

	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="spinner"></div>
		</div>
		{:else if !trashData || trashData.total_items === 0}
		<div class="card p-12 text-center">
			<svg class="w-16 h-16 mx-auto mb-4 text-[var(--color-text-muted)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<path d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
			</svg>
			<h3 class="text-lg font-medium mb-2">Trash is empty</h3>
			<p class="text-[var(--color-text-secondary)]">
				Items you delete will appear here before permanent removal
			</p>
		</div>
	{:else}
		<!-- Summary -->
		<div class="card p-4 flex items-center justify-between">
			<div>
				<span class="text-[var(--color-text-secondary)]">Total items:</span>
				<span class="font-semibold ml-2">{trashData.total_items}</span>
			</div>
			<div>
				<span class="text-[var(--color-text-secondary)]">Total size:</span>
				<span class="font-semibold ml-2">{formatBytes(trashData.total_size)}</span>
			</div>
		</div>

		<!-- Items -->
		<div class="space-y-2">
			{#each trashData.items as item (item.id)}
				<div class="card p-4 flex items-center gap-4">
					<svg class="w-8 h-8 text-[var(--color-text-muted)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
						<path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
					</svg>

					<div class="flex-1 min-w-0">
						<div class="font-medium truncate">{item.original_path.split('/').pop()}</div>
						<div class="text-sm text-[var(--color-text-muted)] truncate">{item.original_path}</div>
						<div class="text-xs text-[var(--color-text-muted)] mt-1">
							Deleted {formatRelativeTime(item.deleted_at)} · {getTimeUntilExpiry(item.expires_at)}
						</div>
					</div>

					<div class="text-sm font-mono text-[var(--color-text-secondary)]">
						{formatBytes(item.size)}
					</div>

					<div class="flex items-center gap-2">
						<button
							class="btn btn-secondary btn-sm"
							onclick={() => restoreItem(item.id)}
							title="Restore"
						>
							<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M3 10h10a5 5 0 015 5v2M3 10l4 4M3 10l4-4" />
							</svg>
						</button>
						<button
							class="btn btn-ghost btn-sm text-critical hover:bg-critical/10"
							onclick={() => deleteItem(item.id)}
							title="Delete permanently"
						>
							<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M6 18L18 6M6 6l12 12" />
							</svg>
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
