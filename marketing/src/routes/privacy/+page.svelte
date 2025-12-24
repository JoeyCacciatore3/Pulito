<svelte:head>
	<title>Privacy Policy - Pulito</title>
	<meta name="description" content="Privacy policy for Pulito - your privacy is important to us." />
</svelte:head>

<div class="min-h-screen bg-[var(--color-bg)]">
	<header class="sticky top-0 z-40 bg-[var(--color-bg)]/95 backdrop-blur-sm border-b border-[var(--color-border)]">
		<nav class="container mx-auto px-4 py-4 max-w-7xl">
			<div class="flex items-center justify-between">
				<a href="../" class="flex items-center gap-2 text-xl font-bold text-[var(--color-text)]">
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
				<a href="../" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Back to Home</a>
			</div>
		</nav>
	</header>

	<main class="container mx-auto max-w-4xl px-4 py-12 md:py-20">
		<h1 class="text-4xl md:text-5xl font-bold mb-8 text-[var(--color-text)]">Privacy Policy</h1>

		<div class="prose prose-lg dark:prose-invert max-w-none">
			<p class="text-[var(--color-text-secondary)] mb-6">
				<strong>Last updated:</strong> December 20, 2025
			</p>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">Your Privacy Matters</h2>
				<p class="text-[var(--color-text-secondary)] mb-4">
					Pulito is committed to protecting your privacy. This application runs entirely on your local machine
					and does not collect, transmit, or store any personal information or data.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">Local-Only Operation</h2>
				<p class="text-[var(--color-text-secondary)] mb-4">
					All operations performed by Pulito occur locally on your device:
				</p>
				<ul class="list-disc list-inside space-y-2 text-[var(--color-text-secondary)] ml-4">
					<li>File scanning and analysis</li>
					<li>Database storage (SQLite on your local machine)</li>
					<li>Settings and preferences</li>
					<li>Trash management and recovery</li>
				</ul>
			</section>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">No Data Collection</h2>
				<p class="text-[var(--color-text-secondary)] mb-4">
					Pulito does not:
				</p>
				<ul class="list-disc list-inside space-y-2 text-[var(--color-text-secondary)] ml-4">
					<li>Collect or transmit any personal information</li>
					<li>Connect to external servers or analytics services</li>
					<li>Share data with third parties</li>
					<li>Track your usage or behavior</li>
				</ul>
			</section>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">Open Source</h2>
				<p class="text-[var(--color-text-secondary)] mb-4">
					As an open-source project, all code is publicly available for review. You can inspect the source code
					to verify that no data collection occurs.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">Contact</h2>
				<p class="text-[var(--color-text-secondary)]">
					If you have questions about this privacy policy, please open an issue on our
					<a href="https://github.com/JoeyCacciatore3/pulito" class="text-primary-600 hover:underline">GitHub repository</a>.
				</p>
			</section>
		</div>
	</main>

	<footer class="bg-[var(--color-bg-secondary)] border-t border-[var(--color-border)] py-8 px-4 mt-12">
		<div class="container mx-auto max-w-6xl text-center text-sm text-[var(--color-text-secondary)]">
			<p>&copy; 2025 Pulito. Made with ❤️ for Linux.</p>
		</div>
	</footer>
</div>
