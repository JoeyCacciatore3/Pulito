<script lang="ts">
	import { notificationStore, type Notification } from '$lib/stores/notifications.svelte';
	import { fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	function getTypeStyles(type: Notification['type']) {
		switch (type) {
			case 'success':
				return {
					bg: 'bg-green-500/20 dark:bg-green-500/30',
					border: 'border-green-500/50',
					text: 'text-green-700 dark:text-green-400',
					icon: '✅'
				};
			case 'error':
				return {
					bg: 'bg-red-500/20 dark:bg-red-500/30',
					border: 'border-red-500/50',
					text: 'text-red-700 dark:text-red-400',
					icon: '❌'
				};
			case 'warning':
				return {
					bg: 'bg-yellow-500/20 dark:bg-yellow-500/30',
					border: 'border-yellow-500/50',
					text: 'text-yellow-700 dark:text-yellow-400',
					icon: '⚠️'
				};
			case 'info':
				return {
					bg: 'bg-blue-500/20 dark:bg-blue-500/30',
					border: 'border-blue-500/50',
					text: 'text-blue-700 dark:text-blue-400',
					icon: 'ℹ️'
				};
		}
	}
</script>

<div class="fixed top-4 right-4 z-50 flex flex-col gap-2 max-w-md">
	{#each notificationStore.all as notification (notification.id)}
		<div
			class="card p-4 {getTypeStyles(notification.type).bg} border {getTypeStyles(notification.type).border} shadow-lg backdrop-blur-sm"
			transition:fly={{ y: -20, duration: 300, easing: quintOut }}
		>
			<div class="flex items-start gap-3">
				<div class="text-xl flex-shrink-0">{getTypeStyles(notification.type).icon}</div>
				<div class="flex-1 min-w-0">
					<div class="font-semibold {getTypeStyles(notification.type).text} mb-1">
						{notification.title}
					</div>
					<div class="text-sm text-[var(--color-text-secondary)]">
						{notification.message}
					</div>
				</div>
				<button
					class="flex-shrink-0 text-[var(--color-text-muted)] hover:text-[var(--color-text)] transition-colors"
					onclick={() => notificationStore.dismiss(notification.id)}
					aria-label="Dismiss notification"
				>
					<svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</div>
		</div>
	{/each}
</div>
