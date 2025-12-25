// Settings store using Svelte 5 runes

// Import types from generated types
import type {
	AppSettings,
	TrashSettings,
	MonitoringSettings,
	NotificationSettings,
	ScanSettings,
} from '$lib/generated/types';

// Re-export for backward compatibility
export type {
	AppSettings,
	TrashSettings,
	MonitoringSettings,
	NotificationSettings,
	ScanSettings,
};

const defaultSettings: AppSettings = {
	trash: {
		retention_days: 3,
		max_size_mb: 1000
	},
	monitoring: {
		enabled: true,
		interval_hours: 24
	},
	notifications: {
		system: true,
		tray: true,
		in_app: true
	},
	scan: {
		include_hidden: false,
		large_file_threshold_mb: 100
	},
	theme: 'system'
};

let currentSettings = $state<AppSettings>({ ...defaultSettings });

export const settings = {
	get value() {
		return currentSettings;
	},

	update(partial: Partial<AppSettings>) {
		currentSettings = { ...currentSettings, ...partial };
	},

	updateTrash(partial: Partial<AppSettings['trash']>) {
		currentSettings = {
			...currentSettings,
			trash: { ...currentSettings.trash, ...partial }
		};
	},

	updateMonitoring(partial: Partial<AppSettings['monitoring']>) {
		currentSettings = {
			...currentSettings,
			monitoring: { ...currentSettings.monitoring, ...partial }
		};
	},

	updateNotifications(partial: Partial<AppSettings['notifications']>) {
		currentSettings = {
			...currentSettings,
			notifications: { ...currentSettings.notifications, ...partial }
		};
	},

	updateScan(partial: Partial<AppSettings['scan']>) {
		currentSettings = {
			...currentSettings,
			scan: { ...currentSettings.scan, ...partial }
		};
	},

	updateScheduling(partial: Partial<AppSettings['scheduling']>) {
		// Load current scheduling or create new with all required fields
		const current = currentSettings.scheduling ?? {
			enabled: false,
			frequency: 'daily' as const,
			time: undefined,
			day_of_week: undefined,
			last_run: undefined,
			next_run: undefined,
		};

		currentSettings = {
			...currentSettings,
			scheduling: { ...current, ...partial }
		};
	},

	reset() {
		currentSettings = { ...defaultSettings };
	},

	load(saved: AppSettings) {
		currentSettings = { ...defaultSettings, ...saved };
	}
};
