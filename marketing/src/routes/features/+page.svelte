<script lang="ts">
	import { base } from '$app/paths';
</script>

<svelte:head>
	<title>Features - Pulito Linux System Cleanup</title>
	<meta name="description" content="Discover all the powerful features of Pulito: security protection, smart analysis, trash management, and modern UI for Linux system cleanup." />
	<meta property="og:title" content="Features - Pulito Linux System Cleanup" />
	<meta property="og:description" content="Discover all the powerful features of Pulito for Linux system cleanup and optimization." />
</svelte:head>

<div class="min-h-screen bg-[var(--color-bg)]">
	<!-- Navigation -->
	<header class="sticky top-0 z-40 bg-[var(--color-bg)]/95 backdrop-blur-sm border-b border-[var(--color-border)]">
		<nav class="container mx-auto px-4 py-4 max-w-7xl">
			<div class="flex items-center justify-between">
				<a href="{base}/" class="flex items-center gap-2 text-xl font-bold text-[var(--color-text)]">
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
				<div class="flex items-center gap-4">
					<a href="{base}/" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Home</a>
					<a href="{base}/download" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Download</a>
					<a href="{base}/faq" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">FAQ</a>
				</div>
			</div>
		</nav>
	</header>

	<main class="container mx-auto max-w-6xl px-4 py-12 md:py-20">
		<!-- Header -->
		<div class="text-center mb-12 md:mb-16">
			<h1 class="text-4xl md:text-5xl font-bold mb-4 text-[var(--color-text)]">Powerful Features</h1>
			<p class="text-lg text-[var(--color-text-secondary)] max-w-2xl mx-auto">
				Everything you need to keep your Linux system clean, secure, and optimized
			</p>
		</div>

		<!-- Security & Safety -->
		<section class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<div class="w-12 h-12 rounded-lg bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center">
					<svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
					</svg>
				</div>
				<h2 class="text-3xl font-bold text-[var(--color-text)]">Security & Safety</h2>
			</div>

			<div class="grid md:grid-cols-2 gap-6">
				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Path Traversal Protection</h3>
					<p class="text-[var(--color-text-secondary)]">
						Comprehensive input validation prevents directory traversal attacks and unauthorized file access.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">System File Protection</h3>
					<p class="text-[var(--color-text-secondary)]">
						Blocks deletion of critical system directories like /bin, /etc, /usr, and other essential paths.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">User Home Restriction</h3>
					<p class="text-[var(--color-text-secondary)]">
						Only allows operations within the user's home directory, preventing accidental system modifications.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">6-Tier Risk Assessment</h3>
					<p class="text-[var(--color-text-secondary)]">
						Intelligent risk evaluation categorizes files from Safe to Critical, helping you make informed decisions.
					</p>
				</div>
			</div>
		</section>

		<!-- Smart Analysis -->
		<section class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<div class="w-12 h-12 rounded-lg bg-purple-100 dark:bg-purple-900/30 flex items-center justify-center">
					<svg class="w-6 h-6 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
					</svg>
				</div>
				<h2 class="text-3xl font-bold text-[var(--color-text)]">Smart Analysis</h2>
			</div>

			<div class="grid md:grid-cols-2 gap-6">
				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Multi-Format Scanning</h3>
					<p class="text-[var(--color-text-secondary)]">
						Detects caches, orphaned packages, large files, and system logs across multiple locations.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Dependency Resolution</h3>
					<p class="text-[var(--color-text-secondary)]">
						Advanced package dependency analysis prevents accidental breakage when removing packages.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Real-time Size Calculation</h3>
					<p class="text-[var(--color-text-secondary)]">
						Accurate disk space analysis with live updates as you select items for cleanup.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Cross-Platform Package Support</h3>
					<p class="text-[var(--color-text-secondary)]">
						Supports apt/dpkg, pip, npm, and system caches for comprehensive cleanup.
					</p>
				</div>
			</div>
		</section>

		<!-- Trash Management -->
		<section class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<div class="w-12 h-12 rounded-lg bg-green-100 dark:bg-green-900/30 flex items-center justify-center">
					<svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
					</svg>
				</div>
				<h2 class="text-3xl font-bold text-[var(--color-text)]">Advanced Trash Management</h2>
			</div>

			<div class="grid md:grid-cols-2 gap-6">
				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Recoverable Deletions</h3>
					<p class="text-[var(--color-text-secondary)]">
						Custom trash system with configurable retention periods keeps your deleted files recoverable.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Metadata Tracking</h3>
					<p class="text-[var(--color-text-secondary)]">
						Complete file history and restoration capabilities for every deleted item.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Automatic Cleanup</h3>
					<p class="text-[var(--color-text-secondary)]">
						Configurable expired file removal keeps your trash from growing too large.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Size Management</h3>
					<p class="text-[var(--color-text-secondary)]">
						Trash size limits with intelligent cleanup to manage disk space efficiently.
					</p>
				</div>
			</div>
		</section>

		<!-- User Experience -->
		<section class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<div class="w-12 h-12 rounded-lg bg-orange-100 dark:bg-orange-900/30 flex items-center justify-center">
					<svg class="w-6 h-6 text-orange-600 dark:text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
					</svg>
				</div>
				<h2 class="text-3xl font-bold text-[var(--color-text)]">User Experience</h2>
			</div>

			<div class="grid md:grid-cols-2 gap-6">
				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Modern UI</h3>
					<p class="text-[var(--color-text-secondary)]">
						Svelte 5 with Tailwind CSS for a responsive, accessible, and beautiful interface.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">System Tray Integration</h3>
					<p class="text-[var(--color-text-secondary)]">
						Background monitoring with tray notifications keeps you informed without interruption.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Comprehensive Settings</h3>
					<p class="text-[var(--color-text-secondary)]">
						Granular control over all application behavior to suit your preferences.
					</p>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">Real-time Feedback</h3>
					<p class="text-[var(--color-text-secondary)]">
						Live progress updates and detailed operation logs keep you informed of every action.
					</p>
				</div>
			</div>
		</section>

		<!-- Performance & Benchmarks -->
		<section class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<div class="w-12 h-12 rounded-lg bg-red-100 dark:bg-red-900/30 flex items-center justify-center">
					<svg class="w-6 h-6 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
					</svg>
				</div>
				<h2 class="text-3xl font-bold text-[var(--color-text)]">Performance & Safety</h2>
			</div>

			<div class="grid md:grid-cols-3 gap-6">
				<div class="card p-6 text-center">
					<div class="text-3xl font-bold text-green-600 mb-2">99.9%</div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">Accuracy</h3>
					<p class="text-[var(--color-text-secondary)] text-sm">
						System data reading and analysis accuracy
					</p>
				</div>

				<div class="card p-6 text-center">
					<div class="text-3xl font-bold text-blue-600 mb-2">0</div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">System Breaks</h3>
					<p class="text-[var(--color-text-secondary)] text-sm">
						No reported system destabilization incidents
					</p>
				</div>

				<div class="card p-6 text-center">
					<div class="text-3xl font-bold text-purple-600 mb-2">&lt;2s</div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">Startup Time</h3>
					<p class="text-[var(--color-text-secondary)] text-sm">
						Cold start time for instant responsiveness
					</p>
				</div>
			</div>
		</section>

		<!-- Use Cases -->
		<section class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<div class="w-12 h-12 rounded-lg bg-indigo-100 dark:bg-indigo-900/30 flex items-center justify-center">
					<svg class="w-6 h-6 text-indigo-600 dark:text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
					</svg>
				</div>
				<h2 class="text-3xl font-bold text-[var(--color-text)]">Perfect For</h2>
			</div>

			<div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
				<div class="card p-6">
					<div class="w-12 h-12 bg-blue-100 dark:bg-blue-900/30 rounded-lg flex items-center justify-center mb-4">
						<svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
						</svg>
					</div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">Daily Users</h3>
					<p class="text-[var(--color-text-secondary)] text-sm">
						Quick maintenance checks and cleanup to keep your system running smoothly
					</p>
				</div>

				<div class="card p-6">
					<div class="w-12 h-12 bg-green-100 dark:bg-green-900/30 rounded-lg flex items-center justify-center mb-4">
						<svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
						</svg>
					</div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">System Admins</h3>
					<p class="text-[var(--color-text-secondary)] text-sm">
						Advanced scanning and safety features for professional system management
					</p>
				</div>

				<div class="card p-6">
					<div class="w-12 h-12 bg-purple-100 dark:bg-purple-900/30 rounded-lg flex items-center justify-center mb-4">
						<svg class="w-6 h-6 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
						</svg>
					</div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">Students</h3>
					<p class="text-[var(--color-text-secondary)] text-sm">
						Learn about Linux system internals while safely maintaining your development environment
					</p>
				</div>

				<div class="card p-6">
					<div class="w-12 h-12 bg-orange-100 dark:bg-orange-900/30 rounded-lg flex items-center justify-center mb-4">
						<svg class="w-6 h-6 text-orange-600 dark:text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
						</svg>
					</div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">Power Users</h3>
					<p class="text-[var(--color-text-secondary)] text-sm">
						Granular control and advanced features for users who want complete system optimization
					</p>
				</div>
			</div>
		</section>

		<!-- Technical Specifications -->
		<section class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<div class="w-12 h-12 rounded-lg bg-gray-100 dark:bg-gray-900/30 flex items-center justify-center">
					<svg class="w-6 h-6 text-gray-600 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
					</svg>
				</div>
				<h2 class="text-3xl font-bold text-[var(--color-text)]">Technical Specifications</h2>
			</div>

			<div class="grid md:grid-cols-2 gap-6">
				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-4 text-[var(--color-text)]">System Requirements</h3>
					<ul class="space-y-2 text-[var(--color-text-secondary]">
						<li>• <strong>OS:</strong> Linux (Ubuntu 20.04+, Debian 11+, Fedora 35+)</li>
						<li>• <strong>Architecture:</strong> x86_64, ARM64</li>
						<li>• <strong>Memory:</strong> 2GB RAM minimum, 4GB recommended</li>
						<li>• <strong>Storage:</strong> 100MB free space</li>
						<li>• <strong>GUI:</strong> WebKitGTK 4.0+ or Qt WebEngine</li>
					</ul>
				</div>

				<div class="card p-6">
					<h3 class="text-xl font-semibold mb-4 text-[var(--color-text)]">Package Support</h3>
					<ul class="space-y-2 text-[var(--color-text-secondary]">
						<li>• <strong>Package Managers:</strong> apt, dpkg, rpm, pacman</li>
						<li>• <strong>Languages:</strong> Python (pip), Node.js (npm), Rust (cargo)</li>
						<li>• <strong>System Caches:</strong> Browser caches, thumbnails, temp files</li>
						<li>• <strong>Container Support:</strong> Docker, Podman, Flatpak</li>
					</ul>
				</div>
			</div>
		</section>

		<!-- Integration & Compatibility -->
		<section class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<div class="w-12 h-12 rounded-lg bg-teal-100 dark:bg-teal-900/30 flex items-center justify-center">
					<svg class="w-6 h-6 text-teal-600 dark:text-teal-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
					</svg>
				</div>
				<h2 class="text-3xl font-bold text-[var(--color-text)]">Integration & Compatibility</h2>
			</div>

			<div class="grid md:grid-cols-3 gap-6">
				<div class="card p-6">
					<h3 class="text-lg font-semibold mb-3 text-[var(--color-text)]">Desktop Environments</h3>
					<div class="flex flex-wrap gap-2">
						<span class="px-3 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-300 rounded-full text-sm">GNOME</span>
						<span class="px-3 py-1 bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-300 rounded-full text-sm">KDE</span>
						<span class="px-3 py-1 bg-orange-100 dark:bg-orange-900/30 text-orange-800 dark:text-orange-300 rounded-full text-sm">XFCE</span>
						<span class="px-3 py-1 bg-purple-100 dark:bg-purple-900/30 text-purple-800 dark:text-purple-300 rounded-full text-sm">Cinnamon</span>
						<span class="px-3 py-1 bg-red-100 dark:bg-red-900/30 text-red-800 dark:text-red-300 rounded-full text-sm">MATE</span>
					</div>
				</div>

				<div class="card p-6">
					<h3 class="text-lg font-semibold mb-3 text-[var(--color-text)]">Package Formats</h3>
					<div class="flex flex-wrap gap-2">
						<span class="px-3 py-1 bg-indigo-100 dark:bg-indigo-900/30 text-indigo-800 dark:text-indigo-300 rounded-full text-sm">.deb</span>
						<span class="px-3 py-1 bg-cyan-100 dark:bg-cyan-900/30 text-cyan-800 dark:text-cyan-300 rounded-full text-sm">AppImage</span>
						<span class="px-3 py-1 bg-pink-100 dark:bg-pink-900/30 text-pink-800 dark:text-pink-300 rounded-full text-sm">Snap</span>
						<span class="px-3 py-1 bg-yellow-100 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-300 rounded-full text-sm">Flatpak</span>
					</div>
				</div>

				<div class="card p-6">
					<h3 class="text-lg font-semibold mb-3 text-[var(--color-text)]">System Integration</h3>
					<div class="space-y-2 text-sm text-[var(--color-text-secondary]">
						<div class="flex items-center gap-2">
							<svg class="w-4 h-4 text-green-500" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg>
							System tray support
						</div>
						<div class="flex items-center gap-2">
							<svg class="w-4 h-4 text-green-500" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg>
							Notifications
						</div>
						<div class="flex items-center gap-2">
							<svg class="w-4 h-4 text-green-500" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg>
							Theme integration
						</div>
					</div>
				</div>
			</div>
		</section>

		<!-- CTA -->
		<div class="text-center">
			<div class="mb-6">
				<h2 class="text-2xl font-bold mb-2 text-[var(--color-text)]">Ready to Experience the Difference?</h2>
				<p class="text-[var(--color-text-secondary)]">Join thousands of Linux users who trust Pulito for safe, effective system cleanup.</p>
			</div>
			<div class="flex flex-col sm:flex-row gap-4 justify-center">
				<a href="{base}/download" class="btn btn-primary btn-lg px-8 py-4 text-lg font-semibold">
					🚀 Download Free
				</a>
				<a href="{base}/faq" class="btn btn-secondary btn-lg px-8 py-4 text-lg">
					Learn More
				</a>
			</div>
		</div>
	</main>

	<!-- Footer -->
	<footer class="bg-[var(--color-bg-secondary)] border-t border-[var(--color-border)] py-8 px-4 mt-12">
		<div class="container mx-auto max-w-6xl text-center text-sm text-[var(--color-text-secondary)]">
			<p>&copy; 2025 Pulito. Made with ❤️ for Linux.</p>
		</div>
	</footer>
</div>
