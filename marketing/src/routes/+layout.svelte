<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';

	let { children } = $props();

	onMount(() => {
		// Initialize theme if needed (simple implementation)
		if (typeof window !== 'undefined') {
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			const storedTheme = localStorage.getItem('theme');
			const theme = storedTheme || (prefersDark ? 'dark' : 'light');
			document.documentElement.classList.toggle('dark', theme === 'dark');
		}
	});
</script>

<div class="min-h-screen bg-[var(--color-bg)] text-[var(--color-text)]">
	{@render children()}
</div>
