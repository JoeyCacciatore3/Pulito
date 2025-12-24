/**
 * Confirmation Dialog Store using Svelte 5 runes
 *
 * Provides glassmorphism confirmation dialogs for all cleanup operations.
 * Follows Nielsen Norman Group UX guidelines for confirmation dialogs.
 *
 * Usage pattern:
 * ```typescript
 * const confirmed = await confirmation.show({
 *     title: 'Operation Title',
 *     message: 'Detailed description with impact',
 *     confirmText: 'Action Button', // Specific action
 *     cancelText: 'Cancel',
 *     type: 'info' | 'warning' | 'danger'
 * });
 *
 * if (!confirmed) return; // Always check result
 * ```
 */

export interface ConfirmationOptions {
	title: string;
	message: string;
	confirmText?: string;
	cancelText?: string;
	type?: 'danger' | 'warning' | 'info' | 'success';
	size?: 'sm' | 'md' | 'lg';
}

export interface ConfirmationState extends ConfirmationOptions {
	isOpen: boolean;
	resolve?: (_value: boolean) => void;
}

// State using Svelte 5 runes
let confirmationState = $state<ConfirmationState>({
	isOpen: false,
	title: '',
	message: '',
	confirmText: 'Confirm',
	cancelText: 'Cancel',
	type: 'danger',
	size: 'md'
});

export const confirmation = {
	get state() {
		return confirmationState;
	},

	show: (options: ConfirmationOptions): Promise<boolean> => {
		return new Promise((resolve) => {
			confirmationState = {
				...options,
				isOpen: true,
				resolve
			};
		});
	},

	confirm: () => {
		if (confirmationState.resolve) {
			confirmationState.resolve(true);
		}
		confirmationState = {
			...confirmationState,
			isOpen: false,
			resolve: undefined
		};
	},

	cancel: () => {
		if (confirmationState.resolve) {
			confirmationState.resolve(false);
		}
		confirmationState = {
			...confirmationState,
			isOpen: false,
			resolve: undefined
		};
	},

	close: () => {
		confirmationState = {
			...confirmationState,
			isOpen: false,
			resolve: undefined
		};
	}
};
