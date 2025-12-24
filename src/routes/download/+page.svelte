<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { logger } from '$lib/utils/logger';

	let detectedOS = $state<string>('linux');
	let showInstallScript = $state(false);

	function copyInstallScript() {
		const command = 'curl -fsSL https://raw.githubusercontent.com/JoeyCacciatore3/pulito/main/static/install.sh | bash';
		navigator.clipboard.writeText(command).then(() => {
			showInstallScript = true;
			setTimeout(() => showInstallScript = false, 2000);
		}).catch(err => {
			logger.error('Failed to copy install script to clipboard', { component: 'DownloadPage', action: 'copy_install_script', operation: 'clipboard_write' }, err);
		});
	}

	onMount(() => {
		if (browser) {
			// Detect operating system
			const userAgent = navigator.userAgent.toLowerCase();
			if (userAgent.includes('linux')) {
				detectedOS = 'linux';
			} else {
				detectedOS = 'other';
			}
		}
	});

	const packageFormats = [
		{
			name: 'Debian Package (.deb)',
			description: 'Recommended for Ubuntu, Debian, and derivatives',
			icon: '📦',
			installCommand: 'sudo dpkg -i pulito_*.deb',
			downloadUrl: '#'
		},
		{
			name: 'AppImage',
			description: 'Universal Linux package, no installation needed',
			icon: '📱',
			installCommand: 'chmod +x pulito_*.AppImage && ./pulito_*.AppImage',
			downloadUrl: '#'
		},
		{
			name: 'Snap',
			description: 'Available on Snap Store (coming soon)',
			icon: '🎯',
			installCommand: 'sudo snap install pulito',
			downloadUrl: '#'
		},
		{
			name: 'Flatpak',
			description: 'Available on Flathub (coming soon)',
			icon: '📦',
			installCommand: 'flatpak install flathub com.pulito.app',
			downloadUrl: '#'
		}
	];
</script>

<svelte:head>
	<title>Download Pulito - Linux System Cleanup Tool</title>
	<meta name="description" content="Download Pulito for Linux. Available as .deb, AppImage, Snap, and Flatpak packages. One-click installation script available." />
	<meta property="og:title" content="Download Pulito - Linux System Cleanup Tool" />
	<meta property="og:description" content="Download Pulito for Linux. Available in multiple package formats for easy installation." />
</svelte:head>

<div class="min-h-screen bg-[var(--color-bg)]">
	<!-- Navigation (simplified, same as landing) -->
	<header class="sticky top-0 z-40 bg-[var(--color-bg)]/95 backdrop-blur-sm border-b border-[var(--color-border)]">
		<nav class="container mx-auto px-4 py-4 max-w-7xl">
			<div class="flex items-center justify-between">
				<a href="/" class="flex items-center gap-2 text-xl font-bold text-[var(--color-text)]">
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
					<a href="/" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Home</a>
					<a href="/features" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Features</a>
					<a href="/faq" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">FAQ</a>
				</div>
			</div>
		</nav>
	</header>

	<main class="container mx-auto max-w-4xl px-4 py-12 md:py-20">
		<!-- Header -->
		<div class="text-center mb-12">
			<h1 class="text-4xl md:text-5xl font-bold mb-4 text-[var(--color-text)]">Download Pulito</h1>
			<p class="text-lg text-[var(--color-text-secondary)]">
				Choose the installation method that works best for your Linux distribution
			</p>
		</div>

		<!-- Quick Install (One-liner) -->
		{#if detectedOS === 'linux'}
			<div class="card p-6 md:p-8 mb-8 bg-gradient-to-r from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20">
				<h2 class="text-2xl font-bold mb-4 text-[var(--color-text)]">Quick Install</h2>
				<p class="text-[var(--color-text-secondary)] mb-4">
					Run this command to automatically detect your distribution and install the appropriate package:
				</p>
				<div class="bg-gray-900 dark:bg-gray-800 rounded-lg p-4 mb-4 overflow-x-auto">
					<code class="text-green-400 text-sm md:text-base">
						curl -fsSL https://raw.githubusercontent.com/JoeyCacciatore3/pulito/main/static/install.sh | bash
					</code>
				</div>
				<button
					onclick={copyInstallScript}
					class="btn btn-primary"
					aria-label="Copy installation script to clipboard"
				>
					{showInstallScript ? '✓ Copied!' : '📋 Copy Command'}
				</button>
			</div>
		{/if}

		<!-- Package Formats -->
		<div class="space-y-6 mb-12">
			<h2 class="text-2xl font-bold text-[var(--color-text)]">Package Formats</h2>

			{#each packageFormats as format}
				<div class="card p-6">
					<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
						<div class="flex-1">
							<div class="flex items-center gap-3 mb-2">
								<span class="text-2xl">{format.icon}</span>
								<h3 class="text-xl font-semibold text-[var(--color-text)]">{format.name}</h3>
							</div>
							<p class="text-[var(--color-text-secondary)] mb-3">{format.description}</p>
							<div class="bg-[var(--color-bg-secondary)] rounded-lg p-3 font-mono text-sm text-[var(--color-text)] overflow-x-auto">
								{format.installCommand}
							</div>
						</div>
						<div class="md:ml-4">
							<button
								onclick={() => window.open(format.downloadUrl, '_blank')}
								class="btn btn-primary whitespace-nowrap"
								aria-label={`Download ${format.name}`}
							>
								Download
							</button>
						</div>
					</div>
				</div>
			{/each}
		</div>

		<!-- Installation Instructions -->
		<div class="card p-6 md:p-8 mb-8">
			<h2 class="text-2xl font-bold mb-4 text-[var(--color-text)]">Installation Instructions</h2>

			<div class="space-y-6">
				<div>
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">For Debian/Ubuntu (.deb)</h3>
					<ol class="list-decimal list-inside space-y-2 text-[var(--color-text-secondary)]">
						<li>Download the .deb package from the releases page</li>
						<li>Open terminal in the download directory</li>
						<li>Run: <code class="bg-[var(--color-bg-secondary)] px-2 py-1 rounded">sudo dpkg -i pulito_*.deb</code></li>
						<li>If dependencies are missing, run: <code class="bg-[var(--color-bg-secondary)] px-2 py-1 rounded">sudo apt-get install -f</code></li>
					</ol>
				</div>

				<div>
					<h3 class="text-xl font-semibold mb-2 text-[var(--color-text)]">For AppImage</h3>
					<ol class="list-decimal list-inside space-y-2 text-[var(--color-text-secondary)]">
						<li>Download the AppImage file</li>
						<li>Make it executable: <code class="bg-[var(--color-bg-secondary)] px-2 py-1 rounded">chmod +x pulito_*.AppImage</code></li>
						<li>Run it: <code class="bg-[var(--color-bg-secondary)] px-2 py-1 rounded">./pulito_*.AppImage</code></li>
						<li>Optionally move it to a permanent location like ~/Applications</li>
					</ol>
				</div>
			</div>
		</div>

		<!-- Latest Release Info -->
		<div class="card p-6 md:p-8 mb-8 bg-gradient-to-r from-green-50 to-blue-50 dark:from-green-900/20 dark:to-blue-900/20">
			<div class="flex items-start gap-4">
				<div class="w-12 h-12 bg-green-500 rounded-full flex items-center justify-center flex-shrink-0">
					<svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
					</svg>
				</div>
				<div class="flex-1">
					<h2 class="text-xl font-bold mb-2 text-[var(--color-text)]">Latest Release: v1.0.0</h2>
					<p class="text-[var(--color-text-secondary)] mb-3">
						Complete system optimization suite with advanced safety features and modern UI.
					</p>
					<div class="flex flex-wrap gap-2">
						<span class="px-3 py-1 bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-300 rounded-full text-sm font-medium">
							Stable Release
						</span>
						<span class="px-3 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-300 rounded-full text-sm font-medium">
							Security Audited
						</span>
						<span class="px-3 py-1 bg-purple-100 dark:bg-purple-900/30 text-purple-800 dark:text-purple-300 rounded-full text-sm font-medium">
							Open Source
						</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Security & Verification -->
		<div class="card p-6 md:p-8 mb-8">
			<h2 class="text-2xl font-bold mb-4 text-[var(--color-text)]">Security & Verification</h2>
			<div class="space-y-4">
				<div class="flex items-start gap-3">
					<svg class="w-6 h-6 text-green-500 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
					<div>
						<h3 class="font-semibold text-[var(--color-text)]">SHA256 Checksums</h3>
						<p class="text-sm text-[var(--color-text-secondary)]">All downloads include SHA256 checksums for verification. Verify with: <code class="bg-[var(--color-bg-secondary)] px-1 rounded">sha256sum -c checksums.txt</code></p>
					</div>
				</div>
				<div class="flex items-start gap-3">
					<svg class="w-6 h-6 text-blue-500 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
					</svg>
					<div>
						<h3 class="font-semibold text-[var(--color-text)]">GPG Signatures</h3>
						<p class="text-sm text-[var(--color-text-secondary)]">Release artifacts are signed with GPG. Import our public key and verify: <code class="bg-[var(--color-bg-secondary)] px-1 rounded">gpg --verify file.sig file</code></p>
					</div>
				</div>
				<div class="flex items-start gap-3">
					<svg class="w-6 h-6 text-purple-500 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
					</svg>
					<div>
						<h3 class="font-semibold text-[var(--color-text)]">Automatic Updates</h3>
						<p class="text-sm text-[var(--color-text-secondary)]">Built-in updater keeps your installation secure with the latest patches and features.</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Alternative Installation Methods -->
		<div class="card p-6 md:p-8 mb-8">
			<h2 class="text-2xl font-bold mb-4 text-[var(--color-text)]">Alternative Installation Methods</h2>
			<div class="grid md:grid-cols-2 gap-6">
				<div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">From Source (Developers)</h3>
					<div class="bg-[var(--color-bg-secondary)] rounded-lg p-4 font-mono text-sm">
						<div class="space-y-1">
							<div>git clone https://github.com/JoeyCacciatore3/pulito.git</div>
							<div>cd pulito</div>
							<div>npm install</div>
							<div>npm run tauri build</div>
						</div>
					</div>
				</div>
				<div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">Docker (Testing)</h3>
					<div class="bg-[var(--color-bg-secondary)] rounded-lg p-4 font-mono text-sm">
						<div class="space-y-1">
							<div>docker run --rm -it \</div>
							<div>  -v /tmp/.X11-unix:/tmp/.X11-unix \</div>
							<div>  pulito:latest</div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Troubleshooting -->
		<div class="card p-6 md:p-8 mb-8">
			<h2 class="text-2xl font-bold mb-4 text-[var(--color-text)]">Troubleshooting</h2>
			<div class="space-y-4">
				<details class="border border-[var(--color-border)] rounded-lg">
					<summary class="px-4 py-3 font-semibold text-[var(--color-text)] cursor-pointer hover:bg-[var(--color-bg-secondary)]">
						App won't start after installation
					</summary>
					<div class="px-4 pb-4 pt-2 text-[var(--color-text-secondary)]">
						<p class="mb-2">Try these solutions:</p>
						<ul class="list-disc list-inside space-y-1 ml-4">
							<li>Ensure WebKitGTK is installed: <code class="bg-[var(--color-bg-secondary)] px-1 rounded">sudo apt install webkit2gtk-4.1</code></li>
							<li>Check for missing dependencies: <code class="bg-[var(--color-bg-secondary)] px-1 rounded">ldd /usr/bin/pulito</code></li>
							<li>Run from terminal to see error messages</li>
						</ul>
					</div>
				</details>

				<details class="border border-[var(--color-border)] rounded-lg">
					<summary class="px-4 py-3 font-semibold text-[var(--color-text)] cursor-pointer hover:bg-[var(--color-bg-secondary)]">
						Permission denied errors
					</summary>
					<div class="px-4 pb-4 pt-2 text-[var(--color-text-secondary)]">
						<p class="mb-2">Pulito only cleans files you own. For system-wide cleaning:</p>
						<ul class="list-disc list-inside space-y-1 ml-4">
							<li>Run with sudo for system cache cleaning</li>
							<li>Check file ownership: <code class="bg-[var(--color-bg-secondary)] px-1 rounded">ls -la /path/to/file</code></li>
							<li>Ensure you're in the appropriate user group</li>
						</ul>
					</div>
				</details>

				<details class="border border-[var(--color-border)] rounded-lg">
					<summary class="px-4 py-3 font-semibold text-[var(--color-text)] cursor-pointer hover:bg-[var(--color-bg-secondary)]">
						App appears frozen or unresponsive
					</summary>
					<div class="px-4 pb-4 pt-2 text-[var(--color-text-secondary)]">
						<p class="mb-2">Large scans can take time. If truly frozen:</p>
						<ul class="list-disc list-inside space-y-1 ml-4">
							<li>Force quit and restart the application</li>
							<li>Check system resources with <code class="bg-[var(--color-bg-secondary)] px-1 rounded">top</code> or <code class="bg-[var(--color-bg-secondary)] px-1 rounded">htop</code></li>
							<li>Clear application cache: <code class="bg-[var(--color-bg-secondary)] px-1 rounded">rm -rf ~/.config/pulito/cache</code></li>
						</ul>
					</div>
				</details>
			</div>
		</div>

		<!-- System Requirements -->
		<div class="card p-6 md:p-8">
			<h2 class="text-2xl font-bold mb-4 text-[var(--color-text)]">System Requirements</h2>
			<div class="grid md:grid-cols-2 gap-6">
				<div>
					<h3 class="text-lg font-semibold mb-3 text-[var(--color-text)]">Minimum Requirements</h3>
					<ul class="space-y-2 text-[var(--color-text-secondary)]">
						<li>• Ubuntu 20.04+ or compatible distribution</li>
						<li>• WebKitGTK 4.0+ for GUI</li>
						<li>• 100MB free disk space</li>
						<li>• 2GB RAM (recommended 4GB)</li>
					</ul>
				</div>
				<div>
					<h3 class="text-lg font-semibold mb-3 text-[var(--color-text)]">Supported Distributions</h3>
					<ul class="space-y-2 text-[var(--color-text-secondary)]">
						<li>• Ubuntu 20.04+</li>
						<li>• Debian 11+</li>
						<li>• Fedora 35+</li>
						<li>• Arch Linux</li>
						<li>• Manjaro</li>
						<li>• Linux Mint 21+</li>
					</ul>
				</div>
			</div>
		</div>
	</main>

	<!-- Footer (simplified) -->
	<footer class="bg-[var(--color-bg-secondary)] border-t border-[var(--color-border)] py-8 px-4 mt-12">
		<div class="container mx-auto max-w-6xl text-center text-sm text-[var(--color-text-secondary)]">
			<p>&copy; 2025 Pulito. Made with ❤️ for Linux.</p>
		</div>
	</footer>
</div>
