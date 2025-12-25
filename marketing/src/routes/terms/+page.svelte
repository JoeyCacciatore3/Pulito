<script lang="ts">
	import { base } from '$app/paths';
</script>

<svelte:head>
	<title>Terms of Service - Pulito</title>
	<meta name="description" content="Terms of service for Pulito." />
</svelte:head>

<div class="min-h-screen bg-[var(--color-bg)]">
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
				<a href="{base}/" class="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)]">Back to Home</a>
			</div>
		</nav>
	</header>

	<main class="container mx-auto max-w-4xl px-4 py-12 md:py-20">
		<h1 class="text-4xl md:text-5xl font-bold mb-8 text-[var(--color-text)]">Terms of Service</h1>

		<div class="prose prose-lg dark:prose-invert max-w-none">
			<p class="text-[var(--color-text-secondary)] mb-6">
				<strong>Last updated:</strong> December 20, 2025
			</p>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">License</h2>
				<p class="text-[var(--color-text-secondary)] mb-4">
					Pulito is provided under the MIT License. By using this software, you agree to the terms
					of the MIT License.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">Disclaimer of Warranty</h2>
				<p class="text-[var(--color-text-secondary)] mb-4">
					THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
					TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">User Responsibility</h2>
				<p class="text-[var(--color-text-secondary)] mb-4">
					Users are responsible for:
				</p>
				<ul class="list-disc list-inside space-y-2 text-[var(--color-text-secondary)] ml-4">
					<li>Backing up important data before using the cleanup features</li>
					<li>Reviewing items before deletion</li>
					<li>Understanding the risks associated with file deletion</li>
					<li>Using the application in accordance with applicable laws</li>
				</ul>
			</section>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">Limitation of Liability</h2>
				<p class="text-[var(--color-text-secondary)] mb-4">
					IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
					WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
					OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="text-2xl font-semibold mb-4 text-[var(--color-text)]">Contact</h2>
				<p class="text-[var(--color-text-secondary)]">
					If you have questions about these terms, please open an issue on our
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
