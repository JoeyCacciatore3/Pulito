<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let mobileMenuOpen = $state(false);
	let latestVersion = $state<string>('1.0.0');

	async function getLatestVersion() {
		if (!browser) return '1.0.0';
		try {
			const response = await fetch('https://api.github.com/repos/JoeyCacciatore3/pulito/releases/latest');
			if (!response.ok) throw new Error('Failed to fetch version');
			const data = await response.json();
			return data.tag_name?.replace(/^v/, '') || '1.0.0';
		} catch (error) {
			return '1.0.0'; // Fallback
		}
	}

	onMount(async () => {
		if (browser) {
			latestVersion = await getLatestVersion();
		}
	});

	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
		const menuButton = document.getElementById('mobile-menu-button');
		if (menuButton) {
			menuButton.setAttribute('aria-expanded', String(mobileMenuOpen));
		}
	}

</script>

<svelte:head>
	<title>Pulito - Smart Linux System Cleanup Tool</title>
	<meta name="description" content="Professional system cleanup tool for Linux. Free up disk space, optimize performance, and keep your system running smoothly. 100% free and open source." />
	<meta property="og:title" content="Pulito - Smart Linux System Cleanup Tool" />
	<meta property="og:description" content="Professional system cleanup tool for Linux. Free up disk space, optimize performance, and keep your system running smoothly." />
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Pulito - Smart Linux System Cleanup Tool" />
	<meta name="twitter:description" content="Professional system cleanup tool for Linux. Free up disk space, optimize performance, and keep your system running smoothly." />

	<!-- Structured Data (JSON-LD) -->
	<script type="application/ld+json">
	{
		"@context": "https://schema.org",
		"@type": "SoftwareApplication",
		"name": "Pulito",
		"applicationCategory": "UtilityApplication",
		"operatingSystem": "Linux",
		"description": "Smart Linux system cleanup and optimization tool that safely frees up disk space by identifying and removing unnecessary files, caches, and orphaned packages.",
		"offers": {
			"@type": "Offer",
			"price": "0",
			"priceCurrency": "USD"
		},
		"author": {
			"@type": "Organization",
			"name": "Pulito Team"
		},
		"license": "https://opensource.org/licenses/MIT",
		"codeRepository": "https://github.com/JoeyCacciatore3/pulito",
		"downloadUrl": "https://github.com/JoeyCacciatore3/pulito/releases",
		"softwareVersion": "{latestVersion}",
		"releaseNotes": "Latest stable release"
	}
	</script>
</svelte:head>

<!-- Skip to content link for accessibility -->
<a href="#main-content" class="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 focus:z-50 focus:px-4 focus:py-2 focus:bg-primary-600 focus:text-white focus:rounded-lg">
	Skip to main content
</a>

<!-- Navigation -->
<header class="sticky top-0 z-40 bg-[var(--color-bg)]/95 backdrop-blur-sm border-b border-[var(--color-border)]">
	<nav class="container mx-auto px-4 py-4 max-w-7xl">
		<div class="flex items-center justify-between">
			<a href="./" class="flex items-center gap-2 text-xl font-bold text-[var(--color-text)]">
				<div class="w-8 h-8 rounded-lg bg-primary-600 flex items-center justify-center">
					<svg class="w-5 h-5 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M3 6h18M3 12h18M3 18h18" />
						<circle cx="17" cy="6" r="2" fill="currentColor" />
						<circle cx="8" cy="12" r="2" fill="currentColor" />
						<circle cx="14" cy="18" r="2" fill="currentColor" />
					</svg>
				</div>
				<span>Pulito</span>
			</a>

			<!-- Mobile menu button -->
			<button
				id="mobile-menu-button"
				aria-label="Toggle navigation menu"
				aria-expanded={mobileMenuOpen}
				onclick={toggleMobileMenu}
				class="md:hidden p-2 rounded-lg hover:bg-[var(--color-bg-secondary)]"
			>
				<svg class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M4 6h16M4 12h16M4 18h16" />
				</svg>
			</button>

			<!-- Desktop navigation -->
			<div class="hidden md:flex items-center gap-6">
				<a href="./features" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">Features</a>
				<a href="./download" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">Download</a>
				<a href="./faq" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">FAQ</a>
				<button
					onclick={() => goto('./download')}
					class="btn btn-primary"
					aria-label="Download Pulito for free"
				>
					Download Free
				</button>
			</div>
		</div>

		<!-- Mobile navigation menu -->
		<div id="mobile-menu" class="{mobileMenuOpen ? '' : 'hidden'} md:hidden mt-4 pb-4 border-t border-[var(--color-border)]">
			<div class="flex flex-col gap-3 pt-4">
				<a href="./features" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] py-2">Features</a>
				<a href="./download" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] py-2">Download</a>
				<a href="./faq" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] py-2">FAQ</a>
				<button
					onclick={() => goto('./download')}
					class="btn btn-primary w-full mt-2"
					aria-label="Download Pulito for free"
				>
					Download Free
				</button>
			</div>
		</div>
	</nav>
</header>

<main id="main-content">
	<!-- Hero Section -->
	<section class="bg-gradient-to-br from-blue-50 via-white to-purple-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900 py-12 md:py-20 px-4">
		<div class="container mx-auto max-w-6xl">
			<div class="grid md:grid-cols-2 gap-8 md:gap-12 items-center">
				<!-- Left: Content -->
				<div class="text-center md:text-left">
					<h1 class="text-4xl md:text-5xl lg:text-6xl font-bold mb-4 md:mb-6">
						<span class="bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">Clean Your Linux System</span>
						<br />
						<span class="text-gray-900 dark:text-white">Like a Pro</span>
					</h1>
					<p class="text-lg md:text-xl text-gray-600 dark:text-gray-300 mb-6 md:mb-8 leading-relaxed">
						Pulito intelligently identifies and removes unnecessary files,
						caches, and orphaned packages. Safe, fast, and completely free.
					</p>

					<div class="flex flex-col sm:flex-row gap-4 justify-center md:justify-start mb-6 md:mb-8">
						<button
							onclick={() => goto('./download')}
							class="btn btn-primary btn-lg px-8 py-4 text-base md:text-lg font-semibold shadow-lg hover:shadow-xl transition-shadow"
							aria-label="Try Pulito for free"
						>
							🚀 Try It Free
						</button>
						<button
							onclick={() => document.getElementById('features')?.scrollIntoView({ behavior: 'smooth' })}
							class="btn btn-secondary btn-lg px-8 py-4 text-base md:text-lg"
							aria-label="Learn more about Pulito features"
						>
							Learn More
						</button>
					</div>

					<!-- Trust Indicators -->
					<div class="flex flex-col sm:flex-row items-center gap-4 sm:gap-6 justify-center md:justify-start text-sm text-gray-500 dark:text-gray-400">
						<div class="flex items-center gap-2">
							<svg class="w-5 h-5 text-green-500 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
								<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
							</svg>
							<span>100% Free & Open Source</span>
						</div>
						<div class="flex items-center gap-2">
							<svg class="w-5 h-5 text-blue-500 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
								<path fill-rule="evenodd" d="M2.166 4.999A11.954 11.954 0 0010 1.944 11.954 11.954 0 0017.834 5c.11.65.166 1.32.166 2.001 0 5.225-3.34 9.67-8 11.317C5.34 16.67 2 12.225 2 7c0-.682.057-1.35.166-2.001zm11.541 3.708a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
							</svg>
							<span>Secure & Private</span>
						</div>
					</div>
				</div>

				<!-- Right: Visual/Preview -->
				<div class="relative mt-8 md:mt-0">
					<div class="relative bg-white dark:bg-gray-800 rounded-2xl shadow-2xl p-4 md:p-6 border border-gray-200 dark:border-gray-700">
						<div class="space-y-4">
							<div class="grid grid-cols-2 gap-4">
								<div class="p-4 bg-[var(--color-bg-secondary)] rounded-lg">
									<div class="text-2xl font-bold text-primary-600">Smart</div>
									<div class="text-sm text-[var(--color-text-secondary)]">Analysis</div>
								</div>
								<div class="p-4 bg-[var(--color-bg-secondary)] rounded-lg">
									<div class="text-2xl font-bold text-green-600">Safe</div>
									<div class="text-sm text-[var(--color-text-secondary)]">Cleanup</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>


	<!-- Features Section -->
	<section id="features" class="py-12 md:py-20 px-4 bg-[var(--color-bg)]">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-12 md:mb-16">
				<h2 class="text-3xl md:text-4xl font-bold mb-4 text-[var(--color-text)]">Powerful Features</h2>
				<p class="text-lg text-[var(--color-text-secondary)] max-w-2xl mx-auto">
					Everything you need to keep your Linux system clean and optimized
				</p>
			</div>

			<div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6 md:gap-8">
				<!-- Feature 1: Security -->
				<div class="card p-6">
					<div class="w-12 h-12 rounded-lg bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center mb-4">
						<svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
						</svg>
					</div>
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)] relative z-10">Security & Safety</h3>
					<p class="text-[var(--color-text-secondary)] relative z-10">
						Path traversal protection, system file safeguards, and 6-tier risk assessment keep your system safe.
					</p>
				</div>

				<!-- Feature 2: Smart Analysis -->
				<div class="card p-6">
					<div class="w-12 h-12 rounded-lg bg-purple-100 dark:bg-purple-900/30 flex items-center justify-center mb-4">
						<svg class="w-6 h-6 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
						</svg>
					</div>
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Smart Analysis</h3>
					<p class="text-[var(--color-text-secondary)]">
						Multi-format scanning with dependency resolution and real-time size calculation.
					</p>
				</div>

				<!-- Feature 3: Trash Management -->
				<div class="card p-6">
					<div class="w-12 h-12 rounded-lg bg-green-100 dark:bg-green-900/30 flex items-center justify-center mb-4">
						<svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
						</svg>
					</div>
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Trash Management</h3>
					<p class="text-[var(--color-text-secondary)]">
						Recoverable deletions with configurable retention periods and automatic cleanup.
					</p>
				</div>

				<!-- Feature 4: Modern UI -->
				<div class="card p-6">
					<div class="w-12 h-12 rounded-lg bg-orange-100 dark:bg-orange-900/30 flex items-center justify-center mb-4">
						<svg class="w-6 h-6 text-orange-600 dark:text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
						</svg>
					</div>
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Modern UI</h3>
					<p class="text-[var(--color-text-secondary)]">
						Beautiful, responsive interface with system tray integration and real-time feedback.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- CTA Section -->
	<section class="py-12 md:py-20 px-4 bg-gradient-to-r from-blue-600 to-purple-600 relative overflow-hidden">
		<div class="container mx-auto max-w-4xl text-center relative z-10">
			<h2 class="text-3xl md:text-4xl font-bold text-white mb-4">Ready to Clean Your System?</h2>
			<p class="text-xl text-blue-100 mb-8">
				Get started with Pulito today and keep your Linux system running smoothly.
			</p>
			<button
				onclick={() => goto('./download')}
				class="btn bg-white text-blue-600 hover:bg-gray-100 btn-lg px-8 py-4 text-lg font-semibold shadow-lg"
				aria-label="Download Pulito now"
			>
				Download Free
			</button>
		</div>
	</section>

	<!-- Footer -->
	<footer class="bg-[var(--color-bg-secondary)] border-t border-[var(--color-border)] py-12 px-4">
		<div class="container mx-auto max-w-6xl">
			<div class="grid md:grid-cols-4 gap-8 mb-8">
				<div>
					<h3 class="font-semibold text-lg mb-4 text-[var(--color-text)]">Pulito</h3>
					<p class="text-sm text-[var(--color-text-secondary)]">
						Smart Linux system cleanup and optimization tool.
					</p>
				</div>
				<div>
					<h4 class="font-semibold mb-4 text-[var(--color-text)]">Product</h4>
					<ul class="space-y-2 text-sm">
						<li><a href="./features" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Features</a></li>
						<li><a href="./download" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Download</a></li>
						<li><a href="./faq" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">FAQ</a></li>
					</ul>
				</div>
				<div>
					<h4 class="font-semibold mb-4 text-[var(--color-text)]">Resources</h4>
					<ul class="space-y-2 text-sm">
						<li><a href="https://github.com/JoeyCacciatore3/pulito" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)]" target="_blank" rel="noopener noreferrer">GitHub</a></li>
						<li><a href="https://x.com/joeycacciatorex" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)]" target="_blank" rel="noopener noreferrer">X (Twitter)</a></li>
						<li><a href="https://github.com/JoeyCacciatore3/pulito/blob/main/CONTRIBUTING.md" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)]" target="_blank" rel="noopener noreferrer">Contributing</a></li>
						<li><a href="https://github.com/JoeyCacciatore3/pulito/blob/main/LICENSE" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)]" target="_blank" rel="noopener noreferrer">License</a></li>
					</ul>
				</div>
				<div>
					<h4 class="font-semibold mb-4 text-[var(--color-text)]">Legal</h4>
					<ul class="space-y-2 text-sm">
						<li><a href="./privacy" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Privacy</a></li>
						<li><a href="./terms" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Terms</a></li>
					</ul>
				</div>
			</div>
			<div class="border-t border-[var(--color-border)] pt-8 text-center text-sm text-[var(--color-text-secondary)]">
				<p>&copy; 2025 Pulito. Made with ❤️ for Linux. Open source under MIT License.</p>
			</div>
		</div>
	</footer>
</main>


<style>
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}

	.focus\:not-sr-only:focus {
		position: static;
		width: auto;
		height: auto;
		padding: inherit;
		margin: inherit;
		overflow: visible;
		clip: auto;
		white-space: normal;
	}

</style>
