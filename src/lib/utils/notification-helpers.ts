/**
 * Notification helper utilities
 * Consolidates common notification patterns and reduces code duplication
 */

import { notificationStore } from '$lib/stores/notifications.svelte';
import { formatBytes } from './tauri';

/**
 * Show success notification for cleanup operations
 */
export function notifyCleanupSuccess(operation: string, details: string) {
	notificationStore.success(`${operation} Complete`, details);
}

/**
 * Show error notification for failed operations
 */
export function notifyOperationError(operation: string, error?: string) {
	// Avoid duplication: if operation already contains "Failed", don't add it again
	const operationName = operation.includes('Failed') ? operation : `${operation} Failed`;
	const message = error ? `${operationName}: ${error}` : operationName;
	notificationStore.error(message, 'Please try again');
}

/**
 * Show warning notification for partial operations
 */
export function notifyPartialSuccess(operation: string, successCount: number, failCount: number) {
	notificationStore.warning(
		`Partial ${operation}`,
		`Completed ${successCount} operations, ${failCount} failed`
	);
}

/**
 * Show loading/scan completion notification
 */
export function notifyScanComplete(operation: string, itemCount: number, sizeBytes: number) {
	const size = formatBytes(sizeBytes);
	notificationStore.success(`${operation} Complete`, `Found ${itemCount} items (${size})`);
}

/**
 * Show nothing found notification
 */
export function notifyNothingFound(operation: string) {
	notificationStore.info(`No ${operation} Found`, 'Everything looks clean!');
}
