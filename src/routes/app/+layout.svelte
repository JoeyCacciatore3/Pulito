<script lang="ts">
	import { onMount } from 'svelte';
	import ConfirmationDialog from '$lib/components/ConfirmationDialog.svelte';
	import NotificationToast from '$lib/components/NotificationToast.svelte';
	import { confirmation } from '$lib/stores/confirmation.svelte';

	let { children } = $props();

	onMount(() => {
		// Initialize any app-specific setup here
	});
</script>

<div class="min-h-screen bg-[var(--color-bg)] text-[var(--color-text)]">
	{@render children()}
</div>

<!-- Global confirmation dialog -->
<ConfirmationDialog
	bind:isOpen={confirmation.state.isOpen}
	title={confirmation.state.title}
	message={confirmation.state.message}
	confirmText={confirmation.state.confirmText}
	cancelText={confirmation.state.cancelText}
	type={confirmation.state.type}
	size={confirmation.state.size}
	on:confirm={confirmation.confirm}
	on:cancel={confirmation.cancel}
/>

<!-- Global notification toasts -->
<NotificationToast />
