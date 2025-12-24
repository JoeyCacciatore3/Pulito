/**
 * Notification Store for User Feedback
 *
 * Provides toast notifications for cleanup operation results.
 * Uses Svelte 5 runes for reactive state management.
 *
 * Notification types:
 * - success: Operation completed successfully
 * - error: Operation failed with details
 * - warning: Operation completed with issues
 * - info: General information
 *
 * Usage:
 * ```typescript
 * notificationStore.success('Cache Cleaned', 'Freed 2.1GB of space');
 * notificationStore.error('Cleanup Failed', 'Permission denied');
 * ```
 */

export type NotificationType = 'success' | 'error' | 'warning' | 'info';

export interface Notification {
	id: string;
	type: NotificationType;
	title: string;
	message: string;
	duration?: number; // Auto-dismiss after milliseconds (default: 5000)
	timestamp: number;
}

let notifications = $state<Notification[]>([]);

function generateId(): string {
	return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

export const notificationStore = {
	get all() {
		return notifications;
	},

	show(notification: Omit<Notification, 'id' | 'timestamp'>) {
		const id = generateId();
		const newNotification: Notification = {
			id,
			timestamp: Date.now(),
			duration: notification.duration ?? 5000,
			...notification
		};

		notifications = [...notifications, newNotification];

		// Auto-dismiss after duration
		const duration = newNotification.duration ?? 5000;
		if (duration > 0) {
			setTimeout(() => {
				this.dismiss(id);
			}, duration);
		}

		return id;
	},

	success(title: string, message: string, duration = 5000) {
		return this.show({ type: 'success', title, message, duration });
	},

	error(title: string, message: string, duration = 8000) {
		return this.show({ type: 'error', title, message, duration }); // Errors stay longer
	},

	warning(title: string, message: string, duration = 5000) {
		return this.show({ type: 'warning', title, message, duration });
	},

	info(title: string, message: string, duration = 5000) {
		return this.show({ type: 'info', title, message, duration });
	},

	dismiss(id: string) {
		notifications = notifications.filter(n => n.id !== id);
	},

	clear() {
		notifications = [];
	}
};
