<script lang="ts">
	let openFaq = $state<number | null>(null);

	function toggleFaq(index: number) {
		openFaq = openFaq === index ? null : index;
	}
</script>

<svelte:head>
	<title>FAQ - Pulito Linux System Cleanup</title>
	<meta name="description" content="Frequently asked questions about Pulito: installation, system requirements, safety, features, and troubleshooting." />
	<meta property="og:title" content="FAQ - Pulito Linux System Cleanup" />
	<meta property="og:description" content="Find answers to common questions about Pulito Linux system cleanup tool." />
</svelte:head>

<div class="min-h-screen bg-[var(--color-bg)]">
	<!-- Navigation -->
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
					<a href="/download" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Download</a>
				</div>
			</div>
		</nav>
	</header>

	<main class="container mx-auto max-w-4xl px-4 py-12 md:py-20">
		<!-- Header -->
		<div class="text-center mb-12 md:mb-16">
			<h1 class="text-4xl md:text-5xl font-bold mb-4 text-[var(--color-text)]">Frequently Asked Questions</h1>
			<p class="text-lg text-[var(--color-text-secondary)]">
				Find answers to common questions about Pulito
			</p>
		</div>

		<!-- FAQ Items -->
		<div class="space-y-4 mb-12">
			<!-- FAQ 1 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(0)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 0}
					aria-controls="faq-0"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">What is Pulito?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 0 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-0"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 0 ? '' : 'hidden'}"
				>
					<p>
						Pulito is a smart, secure, and intelligent Linux system cleanup tool that helps you free up disk space
						by safely identifying and removing unnecessary files, caches, and orphaned packages. It features advanced
						security protections, intelligent risk assessment, and a recoverable trash system.
					</p>
				</div>
			</div>

			<!-- FAQ 2 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(1)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 1}
					aria-controls="faq-1"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">Is Pulito safe to use?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 1 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-1"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 1 ? '' : 'hidden'}"
				>
					<p class="mb-2">
						Yes! Pulito includes multiple layers of security:
					</p>
					<ul class="list-disc list-inside space-y-1 ml-2">
						<li>Path traversal protection prevents unauthorized file access</li>
						<li>System file protection blocks deletion of critical directories</li>
						<li>User home restriction limits operations to safe locations</li>
						<li>6-tier risk assessment helps you make informed decisions</li>
						<li>Recoverable trash system allows you to restore deleted files</li>
					</ul>
				</div>
			</div>

			<!-- FAQ 3 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(2)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 2}
					aria-controls="faq-2"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">What are the system requirements?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 2 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-2"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 2 ? '' : 'hidden'}"
				>
					<ul class="list-disc list-inside space-y-1">
						<li>Ubuntu 22.04+ or compatible Linux distribution</li>
						<li>WebKitGTK 4.1 (usually pre-installed on modern distributions)</li>
						<li>GTK 3.0+ for desktop integration</li>
						<li>100MB free disk space</li>
						<li>Modern CPU and sufficient RAM for your system</li>
					</ul>
				</div>
			</div>

			<!-- FAQ 4 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(3)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 3}
					aria-controls="faq-3"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">How do I install Pulito?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 3 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-3"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 3 ? '' : 'hidden'}"
				>
					<p class="mb-2">
						Installation is easy! You can choose from several methods:
					</p>
					<ul class="list-disc list-inside space-y-1 ml-2">
						<li><strong>.deb package:</strong> Download and run <code class="bg-[var(--color-bg-secondary)] px-1 rounded">sudo dpkg -i pulito_*.deb</code></li>
						<li><strong>AppImage:</strong> Download, make executable with <code class="bg-[var(--color-bg-secondary)] px-1 rounded">chmod +x</code>, and run</li>
						<li><strong>One-liner:</strong> Use our quick install script: <code class="bg-[var(--color-bg-secondary)] px-1 rounded">curl -fsSL https://raw.githubusercontent.com/JoeyCacciatore3/pulito/main/static/install.sh | bash</code></li>
					</ul>
					<p class="mt-3">
						Visit our <a href="/download" class="text-primary-600 hover:underline">Download page</a> for detailed instructions.
					</p>
				</div>
			</div>

			<!-- FAQ 5 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(4)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 4}
					aria-controls="faq-4"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">Can I recover deleted files?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 4 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-4"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 4 ? '' : 'hidden'}"
				>
					<p>
						Yes! Pulito includes a custom trash system that stores deleted files for a configurable retention period.
						You can browse the trash, restore individual files or entire directories, and manage the trash size limits.
						Files in the trash are automatically cleaned up after the retention period expires (configurable in settings).
					</p>
				</div>
			</div>

			<!-- FAQ 6 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(5)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 5}
					aria-controls="faq-5"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">Is Pulito free?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 5 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-5"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 5 ? '' : 'hidden'}"
				>
					<p>
						Yes, Pulito is 100% free and open source! It's released under the MIT License, which means you can
						use, modify, and distribute it freely. The source code is available on GitHub for transparency and community contributions.
					</p>
				</div>
			</div>

			<!-- FAQ 7 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(6)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 6}
					aria-controls="faq-6"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">What package managers does Pulito support?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 6 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-6"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 6 ? '' : 'hidden'}"
				>
					<p>
						Pulito supports multiple package managers and formats:
					</p>
					<ul class="list-disc list-inside space-y-1 ml-2 mt-2">
						<li>apt/dpkg (Debian, Ubuntu, and derivatives)</li>
						<li>pip (Python packages)</li>
						<li>npm (Node.js packages)</li>
						<li>System caches (browser, application, and system caches)</li>
					</ul>
				</div>
			</div>

			<!-- FAQ 8 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(7)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 7}
					aria-controls="faq-7"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">I'm getting a GTK3 warning. Is this normal?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 7 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-7"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 7 ? '' : 'hidden'}"
				>
					<p>
						Yes, this is normal and expected. Tauri (the framework Pulito uses) uses GTK3 bindings, which display
						a deprecation warning on newer systems. This warning does not affect functionality, and the application
						will continue to work perfectly. This is a known issue with Tauri and GTK3, and will be resolved when
						Tauri migrates to newer GTK versions.
					</p>
				</div>
			</div>

			<!-- FAQ 9 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(8)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 8}
					aria-controls="faq-8"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">How can I contribute to Pulito?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 8 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-8"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 8 ? '' : 'hidden'}"
				>
					<p class="mb-2">
						We welcome contributions! You can help in several ways:
					</p>
					<ul class="list-disc list-inside space-y-1 ml-2">
						<li>Report bugs and issues on GitHub</li>
						<li>Suggest new features</li>
						<li>Submit pull requests with improvements</li>
						<li>Improve documentation</li>
						<li>Help other users in discussions</li>
					</ul>
					<p class="mt-3">
						Please read our <a href="https://github.com/JoeyCacciatore3/pulito/blob/main/CONTRIBUTING.md" class="text-primary-600 hover:underline">Contributing Guide</a> for details.
					</p>
				</div>
			</div>

			<!-- FAQ 9 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(9)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 9}
					aria-controls="faq-9"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">Does Pulito collect any personal data?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 9 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-9"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 9 ? '' : 'hidden'}"
				>
					<p class="mb-2">
						<strong>No, Pulito does not collect or transmit any personal data.</strong> The application is designed with privacy as a core principle:
					</p>
					<ul class="list-disc list-inside space-y-1 ml-2">
						<li>All system analysis happens locally on your device</li>
						<li>No telemetry or usage data is sent to external servers</li>
						<li>No internet connection is required for core functionality</li>
						<li>All data processing remains within your user space</li>
						<li>Settings and preferences are stored locally only</li>
					</ul>
					<p class="mt-2">
						You can verify this by running the application offline or monitoring network activity.
					</p>
				</div>
			</div>

			<!-- FAQ 10 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(10)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 10}
					aria-controls="faq-10"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">How does Pulito compare to BleachBit or Stacer?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 10 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-10"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 10 ? '' : 'hidden'}"
				>
					<p class="mb-3">
						Pulito offers several advantages over other Linux cleanup tools:
					</p>
					<div class="grid md:grid-cols-2 gap-4">
						<div>
							<h4 class="font-semibold text-[var(--color-text)] mb-2">vs BleachBit:</h4>
							<ul class="list-disc list-inside space-y-1 text-sm">
								<li>Advanced security protections</li>
								<li>Recoverable trash system</li>
								<li>Modern UI with better UX</li>
								<li>Dependency-aware cleaning</li>
							</ul>
						</div>
						<div>
							<h4 class="font-semibold text-[var(--color-text)] mb-2">vs Stacer:</h4>
							<ul class="list-disc list-inside space-y-1 text-sm">
								<li>Actively maintained</li>
								<li>Open source and transparent</li>
								<li>Comprehensive safety features</li>
								<li>Cross-distribution support</li>
							</ul>
						</div>
					</div>
					<p class="mt-3">
						While other tools focus primarily on deletion, Pulito emphasizes safety, recoverability, and intelligent analysis.
					</p>
				</div>
			</div>

			<!-- FAQ 11 -->
			<div class="card overflow-hidden">
				<button
					onclick={() => toggleFaq(11)}
					class="w-full px-6 py-4 text-left flex items-center justify-between hover:bg-[var(--color-bg-secondary)] transition-colors"
					aria-expanded={openFaq === 11}
					aria-controls="faq-11"
				>
					<h2 class="text-lg font-semibold text-[var(--color-text)]">How often should I run Pulito?</h2>
					<svg
						class="w-5 h-5 text-[var(--color-text-secondary)] transform transition-transform {openFaq === 11 ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				<div
					id="faq-11"
					class="px-6 pb-4 text-[var(--color-text-secondary)] {openFaq === 11 ? '' : 'hidden'}"
				>
					<p class="mb-2">
						The frequency depends on your usage patterns:
					</p>
					<ul class="list-disc list-inside space-y-1 ml-2">
						<li><strong>Weekly:</strong> For active development machines or systems with frequent package installations</li>
						<li><strong>Monthly:</strong> For general desktop use with moderate activity</li>
						<li><strong>Quarterly:</strong> For stable systems with minimal changes</li>
						<li><strong>As needed:</strong> When you notice performance issues or low disk space</li>
					</ul>
					<p class="mt-2">
						Pulito includes monitoring features to help you determine the optimal cleanup schedule for your system.
					</p>
				</div>
			</div>
		</div>

		<!-- Troubleshooting Section -->
		<div class="card p-6 md:p-8 mb-12">
			<h2 class="text-2xl font-bold mb-4 text-[var(--color-text)]">Troubleshooting</h2>

			<div class="space-y-4">
				<div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">Application won't start</h3>
					<ul class="list-disc list-inside space-y-1 text-[var(--color-text-secondary)] ml-2">
						<li>Check that WebKitGTK 4.1 is installed: <code class="bg-[var(--color-bg-secondary)] px-1 rounded">apt list --installed | grep webkit2gtk</code></li>
						<li>Verify system dependencies are met (see System Requirements)</li>
						<li>Check application logs for error messages</li>
					</ul>
				</div>

				<div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">Package installation fails</h3>
					<ul class="list-disc list-inside space-y-1 text-[var(--color-text-secondary)] ml-2">
						<li>Ensure you have sufficient disk space</li>
						<li>Install missing dependencies: <code class="bg-[var(--color-bg-secondary)] px-1 rounded">sudo apt-get install -f</code></li>
						<li>Check file permissions and ownership</li>
					</ul>
				</div>

				<div>
					<h3 class="text-lg font-semibold mb-2 text-[var(--color-text)]">Scan doesn't find files</h3>
					<ul class="list-disc list-inside space-y-1 text-[var(--color-text-secondary)] ml-2">
						<li>Check scan settings - ensure relevant scan types are enabled</li>
						<li>Verify you have permissions to access scanned directories</li>
						<li>Some package manager queries may require elevated privileges</li>
					</ul>
				</div>
			</div>
		</div>

		<!-- Still have questions -->
		<div class="text-center">
			<p class="text-lg text-[var(--color-text-secondary)] mb-4">Still have questions?</p>
			<a
				href="https://github.com/JoeyCacciatore3/pulito/issues"
				class="btn btn-primary"
				aria-label="Open GitHub issues page"
			>
				Ask on GitHub
			</a>
		</div>
	</main>

	<!-- Footer -->
	<footer class="bg-[var(--color-bg-secondary)] border-t border-[var(--color-border)] py-8 px-4 mt-12">
		<div class="container mx-auto max-w-6xl text-center text-sm text-[var(--color-text-secondary)]">
			<p>&copy; 2025 Pulito. Made with ❤️ for Linux.</p>
		</div>
	</footer>
</div>
