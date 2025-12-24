import { confirmation } from '../stores/confirmation.svelte';
import { logger } from './logger';

interface ConfirmationOptions {
	title: string;
	message: string;
	confirmText?: string;
	cancelText?: string;
	type?: 'danger' | 'warning' | 'info' | 'success';
	size?: 'sm' | 'md' | 'lg';
}

const DEFAULT_CONFIRM_TEXTS = {
	danger: 'Delete',
	warning: 'Continue',
	info: 'OK',
	success: 'OK'
};

// Generic confirmation dialog helper using the frontend confirmation store
export async function confirmAction(options: ConfirmationOptions): Promise<boolean> {
	const {
		title,
		message,
		confirmText,
		cancelText = 'Cancel',
		type = 'info'
	} = options;

	// Use default confirm text if not provided
	const finalConfirmText = confirmText ?? DEFAULT_CONFIRM_TEXTS[type];

	try {
		return await confirmation.show({
			title,
			message,
			confirmText: finalConfirmText,
			cancelText,
			type
		});
	} catch (error) {
		logger.error('Confirmation dialog failed', { operation: 'show_confirmation' }, error);
		return false;
	}
}
