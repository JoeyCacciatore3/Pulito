<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';

	let { isOpen = $bindable(false), title = '', message = '', confirmText = 'Confirm', cancelText = 'Cancel', type = 'danger', size = 'md' } = $props<{
		isOpen: boolean;
		title?: string;
		message?: string;
		confirmText?: string;
		cancelText?: string;
		type?: 'danger' | 'warning' | 'info' | 'success';
		size?: 'sm' | 'md' | 'lg';
	}>();

	const dispatch = createEventDispatcher<{
		confirm: void;
		cancel: void;
	}>();

	let dialog = $state<HTMLDialogElement | null>(null);
	let mounted = $state(false);

	onMount(() => {
		mounted = true;
	});

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			dispatch('cancel');
		} else if (event.key === 'Enter') {
			dispatch('confirm');
		}
	}

	function handleBackdropClick(event: MouseEvent) {
		// Close dialog when clicking on backdrop (outside the dialog)
		if (event.target === event.currentTarget) {
			dispatch('cancel');
		}
	}

	$effect(() => {
		if (mounted && dialog && isOpen) {
			dialog.showModal();
			// Focus management for accessibility
			setTimeout(() => {
				const focusableElement = dialog?.querySelector('[data-autofocus]') as HTMLElement;
				if (focusableElement) {
					focusableElement.focus();
				}
			}, 100);
		} else if (dialog && !isOpen) {
			dialog.close();
		}
	});

	function getTypeStyles() {
		switch (type) {
			case 'danger':
				return {
					icon: '⚠️',
					borderColor: 'border-red-300 dark:border-red-700',
					buttonPrimary: 'bg-red-600 hover:bg-red-700 focus:ring-red-500',
					buttonSecondary: 'text-red-600 hover:bg-red-50 dark:hover:bg-red-950/50'
				};
			case 'warning':
				return {
					icon: '🟡',
					borderColor: 'border-yellow-300 dark:border-yellow-700',
					buttonPrimary: 'bg-yellow-600 hover:bg-yellow-700 focus:ring-yellow-500',
					buttonSecondary: 'text-yellow-600 hover:bg-yellow-50 dark:hover:bg-yellow-950/50'
				};
			case 'success':
				return {
					icon: '✅',
					borderColor: 'border-green-300 dark:border-green-700',
					buttonPrimary: 'bg-green-600 hover:bg-green-700 focus:ring-green-500',
					buttonSecondary: 'text-green-600 hover:bg-green-50 dark:hover:bg-green-950/50'
				};
			case 'info':
			default:
				return {
					icon: 'ℹ️',
					borderColor: 'border-blue-300 dark:border-blue-700',
					buttonPrimary: 'bg-blue-600 hover:bg-blue-700 focus:ring-blue-500',
					buttonSecondary: 'text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-950/50'
				};
		}
	}

	function getSizeStyles() {
		switch (size) {
			case 'sm':
				return 'max-w-sm';
			case 'lg':
				return 'max-w-2xl';
			case 'md':
			default:
				return 'max-w-md';
		}
	}
</script>

{#if mounted && isOpen}
	<!-- Glassmorphism backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/20 backdrop-blur-sm transition-opacity duration-200"
		onclick={handleBackdropClick}
		role="presentation"
	>
		<!-- Glassmorphism dialog -->
		<dialog
			bind:this={dialog}
			class="relative {getSizeStyles()} w-full bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl border border-white/20 dark:border-white/10 shadow-2xl rounded-2xl overflow-hidden"
			style="box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25), 0 0 0 1px rgba(255, 255, 255, 0.05);"
			onkeydown={handleKeydown}
			onclick={(e) => e.stopPropagation()}
			role="alertdialog"
			aria-labelledby="dialog-title"
			aria-describedby="dialog-message"
		>
			<!-- Header with glassmorphism accent -->
			<div class="relative px-6 py-4 bg-gradient-to-r from-white/40 to-white/20 dark:from-white/10 dark:to-white/5 border-b border-white/20 dark:border-white/10">
				<div class="flex items-center gap-3">
					<div class="flex-shrink-0 w-10 h-10 rounded-full bg-gradient-to-br from-white/30 to-white/10 dark:from-white/20 dark:to-white/5 flex items-center justify-center shadow-lg">
						<span class="text-xl">{getTypeStyles().icon}</span>
					</div>
					<div class="flex-1 min-w-0">
						<h2 id="dialog-title" class="text-lg font-semibold text-gray-900 dark:text-white truncate">
							{title}
						</h2>
					</div>
				</div>
			</div>

			<!-- Content -->
			<div class="px-6 py-4">
				<p id="dialog-message" class="text-gray-700 dark:text-gray-300 leading-relaxed">
					{message}
				</p>
			</div>

			<!-- Actions with glassmorphism buttons -->
			<div class="px-6 py-4 bg-gradient-to-r from-white/20 to-white/10 dark:from-white/5 dark:to-white/0 border-t border-white/20 dark:border-white/10 flex gap-3 justify-end">
				<button
					class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white/40 hover:bg-white/60 dark:bg-white/10 dark:hover:bg-white/20 backdrop-blur-sm border border-white/30 dark:border-white/20 rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-white/50 shadow-sm hover:shadow-md"
					onclick={() => dispatch('cancel')}
					data-autofocus
				>
					{cancelText}
				</button>
				<button
					class="px-4 py-2 text-sm font-medium text-white {getTypeStyles().buttonPrimary} backdrop-blur-sm border border-white/20 rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-transparent shadow-sm hover:shadow-md"
					onclick={() => dispatch('confirm')}
				>
					{confirmText}
				</button>
			</div>

			<!-- Glassmorphism highlight effect -->
			<div class="absolute inset-0 rounded-2xl bg-gradient-to-br from-white/5 via-transparent to-white/5 pointer-events-none"></div>
		</dialog>
	</div>
{/if}

<style lang="css">
	/* Custom glassmorphism animations */
	@keyframes glass-appear {
		from {
			opacity: 0;
			transform: scale(0.95) translateY(-10px);
		}
		to {
			opacity: 1;
			transform: scale(1) translateY(0);
		}
	}

	/* Ensure proper focus management */
	button:focus-visible {
		outline: 2px solid currentColor;
		outline-offset: 2px;
	}

	/* Improved backdrop blur support */
	@supports (backdrop-filter: blur(10px)) {
		.backdrop-blur-xl {
			backdrop-filter: blur(20px);
		}
	}
</style>
