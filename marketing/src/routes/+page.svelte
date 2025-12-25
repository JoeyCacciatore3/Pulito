<script lang="ts">
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
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
<header class="sticky top-0 z-40 bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl border-b border-white/20 dark:border-gray-700/50 shadow-lg">
	<nav class="container mx-auto px-4 py-4 max-w-7xl">
		<div class="flex items-center justify-between">
			<a href="{base}/" class="flex items-center gap-3 text-xl font-bold text-[var(--color-text)] group">
				<div class="relative">
					<img
						src="/Pulito/assets/pulitologo.png"
						alt="Pulito Logo"
						class="w-10 h-10 transition-transform duration-300 group-hover:scale-110"
					/>
				</div>
				<span class="bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent font-extrabold">Pulito</span>
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
				<a href="{base}/features" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">Features</a>
				<a href="{base}/download" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">Download</a>
				<a href="{base}/faq" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">FAQ</a>
				<button
					onclick={() => goto(`${base}/download`)}
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
				<a href="{base}/features" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] py-2">Features</a>
				<a href="{base}/download" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] py-2">Download</a>
				<a href="{base}/faq" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] py-2">FAQ</a>
				<button
					onclick={() => goto(`${base}/download`)}
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
	<section class="relative bg-gradient-to-br from-blue-50/50 via-white/80 to-purple-50/50 dark:from-gray-900/90 dark:via-gray-800/90 dark:to-gray-900/90 py-12 md:py-20 px-4 overflow-hidden">
		<!-- Background decorative elements -->
		<div class="absolute inset-0 overflow-hidden">
			<div class="absolute top-10 left-10 w-32 h-32 bg-blue-200/30 dark:bg-blue-800/20 rounded-full blur-3xl"></div>
			<div class="absolute bottom-20 right-20 w-40 h-40 bg-purple-200/30 dark:bg-purple-800/20 rounded-full blur-3xl"></div>
			<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-64 h-64 bg-gradient-to-r from-blue-300/20 to-purple-300/20 dark:from-blue-700/10 dark:to-purple-700/10 rounded-full blur-3xl"></div>
		</div>

		<div class="container mx-auto max-w-6xl relative z-10">
			<div class="grid md:grid-cols-2 gap-8 md:gap-12 items-center">
				<!-- Left: Content -->
				<div class="text-center md:text-left">
					<div class="inline-flex items-center gap-2 bg-white/60 dark:bg-gray-800/60 backdrop-blur-sm border border-white/20 dark:border-gray-700/50 rounded-full px-4 py-2 mb-6 text-sm font-medium">
						<img src="/Pulito/assets/pllogo.png" alt="Pulito" class="w-4 h-4" />
						<span class="text-gray-700 dark:text-gray-300">Smart Linux System Cleanup</span>
					</div>

					<h1 class="text-4xl md:text-5xl lg:text-6xl font-bold mb-4 md:mb-6 leading-tight">
						<span class="bg-gradient-to-r from-blue-600 via-purple-600 to-blue-700 bg-clip-text text-transparent">Clean Your Linux System</span>
						<br />
						<span class="text-gray-900 dark:text-white">Like a Pro</span>
					</h1>
					<p class="text-lg md:text-xl text-gray-600 dark:text-gray-300 mb-6 md:mb-8 leading-relaxed max-w-xl">
						Pulito intelligently identifies and removes unnecessary files,
						caches, and orphaned packages. Safe, fast, and completely free.
					</p>

					<div class="flex flex-col sm:flex-row gap-4 justify-center md:justify-start mb-6 md:mb-8">
						<button
							onclick={() => goto(`${base}/download`)}
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
					<!-- Main glassmorphic card -->
					<div class="relative bg-white/70 dark:bg-gray-800/70 backdrop-blur-xl rounded-3xl shadow-2xl p-6 md:p-8 border border-white/30 dark:border-gray-700/50">
						<!-- Subtle inner glow -->
						<div class="absolute inset-0 bg-gradient-to-br from-blue-500/5 to-purple-500/5 rounded-3xl"></div>

						<div class="relative space-y-6">
							<!-- Logo showcase -->
							<div class="flex justify-center mb-6">
								<div class="relative">
									<img
										src="/Pulito/assets/pulitomatrix.png"
										alt="Pulito Matrix Interface"
										class="w-20 h-20 rounded-xl shadow-lg object-cover"
									/>
								</div>
							</div>

							<!-- Feature cards with glassmorphism -->
							<div class="grid grid-cols-2 gap-4">
								<div class="group relative bg-white/50 dark:bg-gray-700/50 backdrop-blur-sm rounded-xl p-4 border border-white/20 dark:border-gray-600/30 hover:bg-white/60 dark:hover:bg-gray-700/60 transition-all duration-300">
									<div class="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-blue-600/10 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
									<div class="relative">
										<div class="text-xl font-bold text-blue-600 dark:text-blue-400 mb-1">Smart</div>
										<div class="text-xs text-gray-600 dark:text-gray-400 font-medium">Analysis</div>
									</div>
								</div>

								<div class="group relative bg-white/50 dark:bg-gray-700/50 backdrop-blur-sm rounded-xl p-4 border border-white/20 dark:border-gray-600/30 hover:bg-white/60 dark:hover:bg-gray-700/60 transition-all duration-300">
									<div class="absolute inset-0 bg-gradient-to-br from-green-500/10 to-green-600/10 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
									<div class="relative">
										<div class="text-xl font-bold text-green-600 dark:text-green-400 mb-1">Safe</div>
										<div class="text-xs text-gray-600 dark:text-gray-400 font-medium">Cleanup</div>
									</div>
								</div>
							</div>

							<!-- Stats preview -->
							<div class="bg-white/30 dark:bg-gray-700/30 backdrop-blur-sm rounded-xl p-4 border border-white/10 dark:border-gray-600/20">
								<div class="flex items-center justify-between text-sm">
									<span class="text-gray-700 dark:text-gray-300">Files Cleaned</span>
									<span class="font-bold text-green-600 dark:text-green-400">2,847</span>
								</div>
								<div class="flex items-center justify-between text-sm mt-2">
									<span class="text-gray-700 dark:text-gray-300">Space Saved</span>
									<span class="font-bold text-blue-600 dark:text-blue-400">1.2 GB</span>
								</div>
							</div>
						</div>
					</div>

					<!-- Floating decorative element -->
					<div class="absolute -top-4 -right-4 w-20 h-20 bg-gradient-to-br from-blue-400/20 to-purple-400/20 rounded-full blur-2xl"></div>
					<div class="absolute -bottom-4 -left-4 w-16 h-16 bg-gradient-to-br from-green-400/20 to-blue-400/20 rounded-full blur-2xl"></div>
				</div>
			</div>
		</div>
	</section>


	<!-- Features Section -->
	<section id="features" class="py-12 md:py-20 px-4 bg-gradient-to-b from-white/50 to-gray-50/30 dark:from-gray-900/50 dark:to-gray-800/30 relative overflow-hidden">
		<!-- Background pattern -->
		<div class="absolute inset-0 opacity-5">
			<div class="absolute top-20 left-20 w-32 h-32 border border-gray-300 dark:border-gray-600 rounded-full"></div>
			<div class="absolute bottom-20 right-20 w-24 h-24 border border-gray-300 dark:border-gray-600 rounded-full"></div>
			<div class="absolute top-1/2 left-1/3 w-16 h-16 border border-gray-300 dark:border-gray-600 rounded-full"></div>
		</div>

		<div class="container mx-auto max-w-6xl relative z-10">
			<div class="text-center mb-12 md:mb-16">
				<div class="inline-flex items-center gap-2 bg-white/60 dark:bg-gray-800/60 backdrop-blur-sm border border-white/20 dark:border-gray-700/50 rounded-full px-4 py-2 mb-6 text-sm font-medium">
					<img src="/Pulito/assets/pulitocande.png" alt="Pulito Features" class="w-4 h-4 rounded-full" />
					<span class="text-gray-700 dark:text-gray-300">Advanced Capabilities</span>
				</div>

				<h2 class="text-3xl md:text-4xl font-bold mb-4 text-[var(--color-text)]">Powerful Features</h2>
				<p class="text-lg text-[var(--color-text-secondary)] max-w-2xl mx-auto">
					Everything you need to keep your Linux system clean and optimized
				</p>
			</div>

			<div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6 md:gap-8">
				<!-- Feature 1: Security -->
				<div class="group relative bg-white/70 dark:bg-gray-800/70 backdrop-blur-xl rounded-2xl p-6 border border-white/30 dark:border-gray-700/50 hover:bg-white/80 dark:hover:bg-gray-800/80 transition-all duration-300 hover:shadow-xl hover:scale-105">
					<div class="absolute inset-0 bg-gradient-to-br from-blue-500/5 to-blue-600/5 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
					<div class="relative">
						<div class="w-12 h-12 rounded-xl bg-gradient-to-br from-blue-100 to-blue-200 dark:from-blue-900/50 dark:to-blue-800/50 flex items-center justify-center mb-4 shadow-lg">
							<svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
							</svg>
						</div>
						<h3 class="text-xl font-semibold mb-3 text-[var(--color-text)]">Security & Safety</h3>
						<p class="text-[var(--color-text-secondary)] text-sm leading-relaxed">
							Path traversal protection, system file safeguards, and 6-tier risk assessment keep your system safe.
						</p>
					</div>
				</div>

				<!-- Feature 2: Smart Analysis -->
				<div class="group relative bg-white/70 dark:bg-gray-800/70 backdrop-blur-xl rounded-2xl p-6 border border-white/30 dark:border-gray-700/50 hover:bg-white/80 dark:hover:bg-gray-800/80 transition-all duration-300 hover:shadow-xl hover:scale-105">
					<div class="absolute inset-0 bg-gradient-to-br from-purple-500/5 to-purple-600/5 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
					<div class="relative">
						<div class="w-12 h-12 rounded-xl bg-gradient-to-br from-purple-100 to-purple-200 dark:from-purple-900/50 dark:to-purple-800/50 flex items-center justify-center mb-4 shadow-lg">
							<svg class="w-6 h-6 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
							</svg>
						</div>
						<h3 class="text-xl font-semibold mb-3 text-[var(--color-text)]">Smart Analysis</h3>
						<p class="text-[var(--color-text-secondary)] text-sm leading-relaxed">
							Multi-format scanning with dependency resolution and real-time size calculation.
						</p>
					</div>
				</div>

				<!-- Feature 3: Trash Management -->
				<div class="group relative bg-white/70 dark:bg-gray-800/70 backdrop-blur-xl rounded-2xl p-6 border border-white/30 dark:border-gray-700/50 hover:bg-white/80 dark:hover:bg-gray-800/80 transition-all duration-300 hover:shadow-xl hover:scale-105">
					<div class="absolute inset-0 bg-gradient-to-br from-green-500/5 to-green-600/5 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
					<div class="relative">
						<div class="w-12 h-12 rounded-xl bg-gradient-to-br from-green-100 to-green-200 dark:from-green-900/50 dark:to-green-800/50 flex items-center justify-center mb-4 shadow-lg">
							<svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
							</svg>
						</div>
						<h3 class="text-xl font-semibold mb-3 text-[var(--color-text)]">Trash Management</h3>
						<p class="text-[var(--color-text-secondary)] text-sm leading-relaxed">
							Recoverable deletions with configurable retention periods and automatic cleanup.
						</p>
					</div>
				</div>

				<!-- Feature 4: Modern UI -->
				<div class="group relative bg-white/70 dark:bg-gray-800/70 backdrop-blur-xl rounded-2xl p-6 border border-white/30 dark:border-gray-700/50 hover:bg-white/80 dark:hover:bg-gray-800/80 transition-all duration-300 hover:shadow-xl hover:scale-105">
					<div class="absolute inset-0 bg-gradient-to-br from-orange-500/5 to-orange-600/5 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
					<div class="relative">
						<div class="w-12 h-12 rounded-xl bg-gradient-to-br from-orange-100 to-orange-200 dark:from-orange-900/50 dark:to-orange-800/50 flex items-center justify-center mb-4 shadow-lg">
							<svg class="w-6 h-6 text-orange-600 dark:text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
							</svg>
						</div>
						<h3 class="text-xl font-semibold mb-3 text-[var(--color-text)]">Modern UI</h3>
						<p class="text-[var(--color-text-secondary)] text-sm leading-relaxed">
							Beautiful, responsive interface with system tray integration and real-time feedback.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- CTA Section -->
	<section class="py-12 md:py-20 px-4 bg-gradient-to-r from-blue-600/90 via-purple-600/90 to-blue-700/90 relative overflow-hidden">
		<!-- Glassmorphic background elements -->
		<div class="absolute inset-0">
			<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-white/10 rounded-full blur-3xl"></div>
			<div class="absolute top-10 right-10 w-32 h-32 bg-blue-300/20 rounded-full blur-2xl"></div>
			<div class="absolute bottom-10 left-10 w-40 h-40 bg-purple-300/20 rounded-full blur-2xl"></div>
		</div>

		<div class="container mx-auto max-w-4xl text-center relative z-10">
			<div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-sm border border-white/30 rounded-full px-4 py-2 mb-6 text-sm font-medium text-white">
				<img src="/Pulito/assets/pllogo.png" alt="Pulito" class="w-4 h-4 brightness-0 invert" />
				<span>Free & Open Source</span>
			</div>

			<h2 class="text-3xl md:text-4xl font-bold text-white mb-4">Ready to Clean Your System?</h2>
			<p class="text-xl text-blue-100 mb-8 max-w-2xl mx-auto leading-relaxed">
				Get started with Pulito today and keep your Linux system running smoothly.
			</p>

			<div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
				<button
					onclick={() => goto(`${base}/download`)}
					class="group relative bg-white/90 backdrop-blur-sm text-blue-600 hover:bg-white btn-lg px-8 py-4 text-lg font-semibold shadow-xl hover:shadow-2xl transition-all duration-300 rounded-xl border border-white/20"
					aria-label="Download Pulito now"
				>
					<div class="absolute inset-0 bg-gradient-to-r from-blue-50 to-purple-50 opacity-0 group-hover:opacity-100 transition-opacity duration-300 rounded-xl"></div>
					<span class="relative flex items-center gap-2">
						🚀 Download Free
						<img src="/Pulito/assets/pulitologo.png" alt="" class="w-5 h-5 opacity-70" />
					</span>
				</button>

				<div class="text-blue-100 text-sm">
					No credit card required • Works on all major Linux distributions
				</div>
			</div>
		</div>
	</section>

	<!-- Footer -->
	<footer class="bg-gradient-to-t from-gray-50/80 to-white/60 dark:from-gray-900/80 dark:to-gray-800/60 backdrop-blur-xl border-t border-white/20 dark:border-gray-700/50 py-12 px-4 relative overflow-hidden">
		<!-- Subtle background elements -->
		<div class="absolute inset-0 opacity-30">
			<div class="absolute top-20 right-20 w-24 h-24 bg-blue-200/20 dark:bg-blue-800/10 rounded-full blur-2xl"></div>
			<div class="absolute bottom-20 left-20 w-32 h-32 bg-purple-200/20 dark:bg-purple-800/10 rounded-full blur-2xl"></div>
		</div>

		<div class="container mx-auto max-w-6xl relative z-10">
			<div class="grid md:grid-cols-4 gap-8 mb-8">
				<div class="md:col-span-2">
					<div class="flex items-center gap-3 mb-4">
						<img src="/Pulito/assets/pulitomatrix.png" alt="Pulito Logo" class="w-10 h-10 rounded-lg object-cover" />
						<h3 class="font-semibold text-lg text-[var(--color-text)]">Pulito</h3>
					</div>
					<p class="text-sm text-[var(--color-text-secondary)] mb-4 max-w-sm">
						Smart Linux system cleanup and optimization tool. Keep your system clean, fast, and secure.
					</p>
					<div class="flex gap-3">
						<a href="https://github.com/JoeyCacciatore3/pulito" class="w-8 h-8 bg-white/50 dark:bg-gray-700/50 backdrop-blur-sm border border-white/20 dark:border-gray-600/30 rounded-lg flex items-center justify-center hover:bg-white/70 dark:hover:bg-gray-700/70 transition-colors" target="_blank" rel="noopener noreferrer" aria-label="GitHub">
							<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
						</a>
						<a href="https://x.com/joeycacciatorex" class="w-8 h-8 bg-white/50 dark:bg-gray-700/50 backdrop-blur-sm border border-white/20 dark:border-gray-600/30 rounded-lg flex items-center justify-center hover:bg-white/70 dark:hover:bg-gray-700/70 transition-colors" target="_blank" rel="noopener noreferrer" aria-label="X (Twitter)">
							<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
						</a>
					</div>
				</div>

				<div>
					<h4 class="font-semibold mb-4 text-[var(--color-text)]">Product</h4>
					<ul class="space-y-2 text-sm">
						<li><a href="{base}/features" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">Features</a></li>
						<li><a href="{base}/download" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">Download</a></li>
						<li><a href="{base}/faq" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">FAQ</a></li>
					</ul>
				</div>

				<div>
					<h4 class="font-semibold mb-4 text-[var(--color-text)]">Resources</h4>
					<ul class="space-y-2 text-sm">
						<li><a href="https://github.com/JoeyCacciatore3/pulito/blob/main/CONTRIBUTING.md" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors" target="_blank" rel="noopener noreferrer">Contributing</a></li>
						<li><a href="https://github.com/JoeyCacciatore3/pulito/blob/main/LICENSE" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors" target="_blank" rel="noopener noreferrer">License</a></li>
						<li><a href="{base}/privacy" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">Privacy</a></li>
						<li><a href="{base}/terms" class="text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors">Terms</a></li>
					</ul>
				</div>
			</div>

			<div class="border-t border-white/20 dark:border-gray-700/50 pt-8 text-center text-sm text-[var(--color-text-secondary)]">
				<div class="flex items-center justify-center gap-2 mb-2">
					<span>&copy; 2025 Pulito. Made with</span>
					<span class="text-red-500">❤️</span>
					<span>for Linux. Open source under MIT License.</span>
				</div>
				<div class="flex items-center justify-center gap-4 text-xs opacity-75">
					<span>v1.0.0</span>
					<span>•</span>
					<span>100% Free</span>
					<span>•</span>
					<span>Privacy Focused</span>
				</div>
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
