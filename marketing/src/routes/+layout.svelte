<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let { children } = $props();

	onMount(() => {
		// Initialize theme if needed (simple implementation)
		if (browser) {
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			const storedTheme = localStorage.getItem('theme');
			const theme = storedTheme || (prefersDark ? 'dark' : 'light');
			document.documentElement.classList.toggle('dark', theme === 'dark');

			// Add reduced motion support
			const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
			if (prefersReducedMotion) {
				document.documentElement.classList.add('reduced-motion');
			}
		}
	});
</script>

<div class="min-h-screen bg-[var(--color-bg)] text-[var(--color-text)]" role="application" aria-label="Pulito - Linux System Cleanup Tool">
	{@render children()}
</div>

<style>
	/* Focus management for better accessibility */
	:focus-visible {
		outline: 2px solid var(--color-border-focus);
		outline-offset: 2px;
	}

	/* High contrast mode support */
	@media (prefers-contrast: high) {
		:root {
			--color-border: #000000;
			--color-border-focus: #0000ff;
		}

		.dark {
			--color-border: #ffffff;
			--color-border-focus: #ffff00;
		}
	}
</style>
