<script lang="ts">
	interface Props {
		size?: 'sm' | 'md' | 'lg';
		message?: string;
		fullscreen?: boolean;
	}

	let {
		size = 'md',
		message = 'Loading...',
		fullscreen = false
	}: Props = $props();
</script>

{#if fullscreen}
	<div class="flex items-center justify-center h-64">
		<div class="text-center">
			<div class="spinner {size === 'sm' ? 'w-4 h-4' : size === 'lg' ? 'w-12 h-12' : 'w-8 h-8'} mx-auto mb-4"></div>
			<p class="text-lg">{message}</p>
		</div>
	</div>
{:else}
	<div class="flex items-center justify-center h-24">
		<div class="spinner {size === 'sm' ? 'w-4 h-4' : size === 'lg' ? 'w-12 h-12' : 'w-8 h-8'}"></div>
	</div>
{/if}

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
