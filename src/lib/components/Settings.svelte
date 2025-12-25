<script lang="ts">
	import { settings, type AppSettings } from '$lib/stores/settings.svelte';
	import { theme } from '$lib/stores/theme.svelte';
	import { invoke } from '$lib/utils/tauri';
	import { onMount } from 'svelte';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import type { ScheduleStatus } from '$lib/generated/types';

	let saving = $state(false);
	let saved = $state(false);
	let loading = $state(true);
	let scheduleStatus = $state<ScheduleStatus | null>(null);

	async function loadScheduleStatus() {
		try {
			scheduleStatus = await invoke<ScheduleStatus>('get_schedule_status', undefined, 5000);
		} catch (e) {
			logger.error('Failed to load schedule status', { component: 'Settings' }, e);
		}
	}

	onMount(() => {
		(async () => {
			try {
				const savedSettings = await invoke<AppSettings>('get_settings');
				settings.load(savedSettings);
				theme.set(savedSettings.theme as 'light' | 'dark' | 'system');
			} catch (e) {
				logger.error('Failed to load settings', { component: 'Settings', action: 'load_settings', operation: 'get_settings' }, e);
				// Continue with defaults if loading fails
				const errorMessage = e instanceof Error
					? (e.message.includes('timed out')
						? 'Failed to load settings. Request timed out. Using default settings.'
						: 'Failed to load settings. Using default settings.')
					: 'Failed to load settings. Using default settings.';
				notificationStore.warning('Load Failed', errorMessage);
			} finally {
				loading = false;
			}

			await loadScheduleStatus();
		})();

		// Refresh status every 30 seconds
		const interval = setInterval(loadScheduleStatus, 30000);
		return () => clearInterval(interval);
	});

	async function saveSettings() {
		saving = true;
		try {
			// Save main settings
			await invoke('save_settings', { settings: settings.value });

			// Save scheduling separately if it exists
			if (settings.value.scheduling) {
				await invoke('save_schedule_settings', { settings: settings.value.scheduling });
				await loadScheduleStatus(); // Refresh status
			}

			saved = true;
			notificationStore.success('Settings Saved', 'Your settings have been saved successfully');
			setTimeout(() => (saved = false), 2000);
		} catch (e) {
			logger.error('Failed to save settings', { component: 'Settings', action: 'save_settings', operation: 'save_settings' }, e);
			const errorMessage = e instanceof Error
				? (e.message.includes('timed out')
					? 'Failed to save settings. Request timed out. Please try again.'
					: e.message)
				: 'Failed to save settings. Please try again.';
			notificationStore.error('Save Failed', errorMessage);
		} finally {
			saving = false;
		}
	}

	function handleThemeChange(newTheme: AppSettings['theme']) {
		settings.update({ theme: newTheme });
		theme.set(newTheme as 'light' | 'dark' | 'system');
	}
</script>

<div class="p-6 space-y-6 animate-in max-w-3xl">
	<div>
		<h1 class="text-2xl font-bold mb-2 flex items-center gap-2">
			<span class="text-2xl">⚙️</span>
			Settings
		</h1>
		<p class="text-[var(--color-text-secondary)]">Customize how Pulito behaves to match your preferences and workflow</p>
	</div>

	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="spinner"></div>
		</div>
	{:else}

	<!-- Appearance -->
	<section class="card p-5 space-y-4">
		<div>
			<h2 class="font-semibold text-lg mb-1">🎨 Appearance</h2>
			<p class="text-sm text-[var(--color-text-secondary)]">Customize the look and feel of the application</p>
		</div>

		<div class="flex items-center justify-between p-3 bg-[var(--color-bg-secondary)] rounded-lg">
			<div class="flex-1">
				<span class="font-medium">Interface Theme</span>
				<p class="text-xs text-[var(--color-text-muted)] mt-1">
					Choose how Pulito looks. System theme follows your OS preference.
				</p>
			</div>
			<select
				class="input w-40"
				value={settings.value.theme}
				onchange={(e) => handleThemeChange(e.currentTarget.value as AppSettings['theme'])}
				aria-label="Choose application theme: System, Light, or Dark"
			>
				<option value="system">🖥️ System</option>
				<option value="light">☀️ Light</option>
				<option value="dark">🌙 Dark</option>
			</select>
		</div>
	</section>

	<!-- Trash Settings -->
	<section class="card p-5 space-y-4">
		<div>
			<h2 class="font-semibold text-lg mb-1">🗂️ Trash Management</h2>
			<p class="text-sm text-[var(--color-text-secondary)]">
				Control how deleted files are handled. Items are safely moved to trash before permanent deletion.
			</p>
		</div>

		<div class="space-y-4">
			<div class="p-4 border border-[var(--color-border)] rounded-lg bg-gradient-to-r from-blue-50/50 to-transparent dark:from-blue-900/10">
				<div class="flex items-center justify-between mb-2">
					<span class="font-medium">🕒 Auto-cleanup Period</span>
					<span class="text-xs text-safe bg-safe/10 px-2 py-1 rounded">Safety Feature</span>
				</div>
				<p class="text-xs text-[var(--color-text-muted)] mb-3">
					Files in trash older than this period will be permanently deleted to save space.
				</p>
				<div class="flex justify-end">
					<select
						class="input w-40"
						value={settings.value.trash.retention_days}
						onchange={(e) => settings.updateTrash({ retention_days: parseInt(e.currentTarget.value) })}
						aria-label="Choose how long to keep deleted files in trash"
					>
						<option value={1}>1 day - Quick cleanup</option>
						<option value={3}>3 days - Balanced</option>
						<option value={7}>1 week - Conservative</option>
						<option value={14}>2 weeks - Very safe</option>
						<option value={30}>1 month - Maximum safety</option>
					</select>
				</div>
			</div>

			<div class="p-4 border border-[var(--color-border)] rounded-lg bg-gradient-to-r from-orange-50/50 to-transparent dark:from-orange-900/10">
				<div class="flex items-center justify-between mb-2">
					<span class="font-medium">📏 Size Limit</span>
					<span class="text-xs text-caution bg-caution/10 px-2 py-1 rounded">Space Management</span>
				</div>
				<p class="text-xs text-[var(--color-text-muted)] mb-3">
					When trash exceeds this size, oldest files are permanently deleted to prevent disk overflow.
				</p>
				<div class="flex justify-end">
					<select
						class="input w-40"
						value={settings.value.trash.max_size_mb}
						onchange={(e) => settings.updateTrash({ max_size_mb: parseInt(e.currentTarget.value) })}
						aria-label="Set maximum trash size limit"
					>
						<option value={500}>500 MB - Minimal</option>
						<option value={1000}>1 GB - Standard</option>
						<option value={2000}>2 GB - Generous</option>
						<option value={5000}>5 GB - Maximum</option>
					</select>
				</div>
			</div>
		</div>
	</section>

	<!-- Monitoring -->
	<section class="card p-5 space-y-4">
		<div>
			<h2 class="font-semibold text-lg mb-1">👀 Background Monitoring</h2>
			<p class="text-sm text-[var(--color-text-secondary)]">
				Let Pulito watch your system and notify you when cleanup opportunities arise.
			</p>
		</div>

		<div class="space-y-4">
			<div class="p-4 border border-[var(--color-border)] rounded-lg bg-gradient-to-r from-green-50/50 to-transparent dark:from-green-900/10">
				<div class="flex items-center justify-between mb-2">
					<div class="flex-1">
						<span class="font-medium flex items-center gap-2">
							<span class="text-lg">🔍</span>
							Enable Smart Monitoring
						</span>
						<p class="text-xs text-[var(--color-text-muted)] mt-1">
							Automatically scan for cleanup opportunities in the background
						</p>
					</div>
					<button
						class="relative w-12 h-6 rounded-full transition-colors {settings.value.monitoring.enabled
							? 'bg-primary-600'
							: 'bg-gray-300 dark:bg-gray-600'}"
						aria-label="Toggle background monitoring {settings.value.monitoring.enabled ? 'off' : 'on'}"
						onclick={() => settings.updateMonitoring({ enabled: !settings.value.monitoring.enabled })}
					>
						<span
							class="absolute top-1 w-4 h-4 bg-white rounded-full transition-transform {settings.value.monitoring.enabled
								? 'translate-x-7'
								: 'translate-x-1'}"
						></span>
					</button>
				</div>
				{#if settings.value.monitoring.enabled}
					<div class="mt-3 p-3 bg-blue-50 dark:bg-blue-900/20 rounded border border-blue-200 dark:border-blue-800">
						<div class="flex items-center gap-2 text-blue-700 dark:text-blue-300 text-xs">
							<span>ℹ️</span>
							<span>Monitoring active - you'll be notified when cleanup opportunities are found</span>
						</div>
					</div>
				{:else}
					<div class="mt-3 p-3 bg-gray-50 dark:bg-gray-900/20 rounded border border-gray-200 dark:border-gray-800">
						<div class="flex items-center gap-2 text-gray-600 dark:text-gray-400 text-xs">
							<span>⚠️</span>
							<span>Monitoring disabled - manual scans only</span>
						</div>
					</div>
				{/if}
			</div>

			<div class="p-4 border border-[var(--color-border)] rounded-lg bg-gradient-to-r from-purple-50/50 to-transparent dark:from-purple-900/10">
				<div class="flex items-center justify-between mb-2">
					<span class="font-medium">⏰ Check Frequency</span>
					<span class="text-xs text-primary-600 bg-primary-100 dark:bg-primary-900/30 px-2 py-1 rounded">
						{settings.value.monitoring.enabled ? 'Active' : 'Disabled'}
					</span>
				</div>
				<p class="text-xs text-[var(--color-text-muted)] mb-3">
					How often to automatically check for cleanup opportunities.
				</p>
				<div class="flex justify-end">
					<select
						class="input w-48"
						value={settings.value.monitoring.interval_hours}
						onchange={(e) => settings.updateMonitoring({ interval_hours: parseInt(e.currentTarget.value) })}
						disabled={!settings.value.monitoring.enabled}
						aria-label="Choose how often to run automatic cleanup checks"
					>
						<option value={6}>Every 6 hours - Frequent checks</option>
						<option value={12}>Every 12 hours - Moderate</option>
						<option value={24}>Every 24 hours - Daily</option>
						<option value={48}>Every 2 days - Relaxed</option>
					</select>
				</div>
			</div>
		</div>
	</section>

	<!-- Automatic Scheduling -->
	<section class="card p-5 space-y-4">
		<div>
			<h2 class="font-semibold text-lg mb-1">⏰ Automatic Cleanup Schedule</h2>
			<p class="text-sm text-[var(--color-text-secondary)]">
				Set up automatic cleanup to run on a schedule without manual intervention.
			</p>
		</div>

		<div class="space-y-4">
			<div class="p-4 border border-[var(--color-border)] rounded-lg bg-gradient-to-r from-blue-50/50 to-transparent dark:from-blue-900/10">
				<div class="flex items-center justify-between mb-2">
					<div class="flex-1">
						<span class="font-medium flex items-center gap-2">
							<span class="text-lg">🔄</span>
							Enable Scheduled Cleanup
						</span>
						<p class="text-xs text-[var(--color-text-muted)] mt-1">
							Automatically run safe cleanup operations on schedule
						</p>
					</div>
					<button
						class="relative w-12 h-6 rounded-full transition-colors {(settings.value.scheduling?.enabled ?? false)
							? 'bg-primary-600'
							: 'bg-gray-300 dark:bg-gray-600'}"
						aria-label="Toggle scheduled cleanup {(settings.value.scheduling?.enabled ?? false) ? 'off' : 'on'}"
						onclick={() => {
							const current = settings.value.scheduling || { enabled: false, frequency: 'daily' };
							settings.updateScheduling({ enabled: !current.enabled });
						}}
					>
						<span
							class="absolute top-1 w-4 h-4 bg-white rounded-full transition-transform {(settings.value.scheduling?.enabled ?? false)
								? 'translate-x-7'
								: 'translate-x-1'}"
						></span>
					</button>
				</div>
			</div>

			{#if settings.value.scheduling?.enabled}
				<div class="p-4 border border-[var(--color-border)] rounded-lg">
					<div class="mb-3">
						<label class="block text-sm font-medium mb-2" for="scheduling-frequency">Frequency</label>
						<select
							id="scheduling-frequency"
							class="input w-full"
							value={settings.value.scheduling?.frequency || 'daily'}
							onchange={(e) => {
								settings.updateScheduling({ frequency: e.currentTarget.value as 'daily' | 'weekly' | 'on_startup' });
							}}
						>
							<option value="daily">Daily</option>
							<option value="weekly">Weekly</option>
							<option value="on_startup">On App Startup</option>
						</select>
					</div>

					{#if settings.value.scheduling?.frequency === 'daily' || settings.value.scheduling?.frequency === 'weekly'}
						<div class="mb-3">
							<label class="block text-sm font-medium mb-2" for="scheduling-time">Time</label>
							<input
								id="scheduling-time"
								type="time"
								class="input w-full"
								value={settings.value.scheduling?.time || '02:00'}
								onchange={(e) => {
									settings.updateScheduling({ time: e.currentTarget.value });
								}}
							/>
						</div>
					{/if}

					{#if settings.value.scheduling?.frequency === 'weekly'}
						<div class="mb-3">
							<label class="block text-sm font-medium mb-2" for="scheduling-day">Day of Week</label>
							<select
								id="scheduling-day"
								class="input w-full"
								value={settings.value.scheduling?.day_of_week?.toString() || '0'}
								onchange={(e) => {
									settings.updateScheduling({ day_of_week: parseInt(e.currentTarget.value) });
								}}
							>
								<option value="0">Sunday</option>
								<option value="1">Monday</option>
								<option value="2">Tuesday</option>
								<option value="3">Wednesday</option>
								<option value="4">Thursday</option>
								<option value="5">Friday</option>
								<option value="6">Saturday</option>
							</select>
						</div>
					{/if}

					{#if scheduleStatus}
						<div class="mt-3 p-3 bg-blue-50 dark:bg-blue-900/20 rounded border border-blue-200 dark:border-blue-800">
							<div class="text-sm">
								<div class="font-medium text-blue-900 dark:text-blue-100 mb-1">Schedule Status</div>
								{#if scheduleStatus.next_run}
									<div class="text-blue-700 dark:text-blue-300">
										Next run: {new Date(scheduleStatus.next_run * 1000).toLocaleString()}
									</div>
								{/if}
								{#if scheduleStatus.last_run}
									<div class="text-blue-700 dark:text-blue-300">
										Last run: {new Date(scheduleStatus.last_run * 1000).toLocaleString()}
									</div>
								{/if}
							</div>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</section>

	<!-- Notifications -->
	<section class="card p-5 space-y-4">
		<div>
			<h2 class="font-semibold text-lg mb-1">🔔 Notification Preferences</h2>
			<p class="text-sm text-[var(--color-text-secondary)]">
				Choose how and when you want to be notified about cleanup opportunities and system status.
			</p>
		</div>

		<div class="space-y-4">
			<div class="p-4 border border-[var(--color-border)] rounded-lg">
				<div class="flex items-center justify-between mb-2">
					<div class="flex-1">
						<span class="font-medium flex items-center gap-2">
							<span class="text-lg">🖥️</span>
							System Notifications
						</span>
						<p class="text-xs text-[var(--color-text-muted)] mt-1">
							Desktop popups for important cleanup alerts and system warnings
						</p>
					</div>
					<button
						class="relative w-12 h-6 rounded-full transition-colors {settings.value.notifications.system
							? 'bg-primary-600'
							: 'bg-gray-300 dark:bg-gray-600'}"
						aria-label="Toggle system notifications {settings.value.notifications.system ? 'off' : 'on'}"
						onclick={() => settings.updateNotifications({ system: !settings.value.notifications.system })}
					>
						<span
							class="absolute top-1 w-4 h-4 bg-white rounded-full transition-transform {settings.value.notifications.system
								? 'translate-x-7'
								: 'translate-x-1'}"
						></span>
					</button>
				</div>
			</div>

			<div class="p-4 border border-[var(--color-border)] rounded-lg">
				<div class="flex items-center justify-between mb-2">
					<div class="flex-1">
						<span class="font-medium flex items-center gap-2">
							<span class="text-lg">📌</span>
							Tray Icon Indicators
						</span>
						<p class="text-xs text-[var(--color-text-muted)] mt-1">
							Show a badge on the system tray icon when cleanup is recommended
						</p>
					</div>
					<button
						class="relative w-12 h-6 rounded-full transition-colors {settings.value.notifications.tray
							? 'bg-primary-600'
							: 'bg-gray-300 dark:bg-gray-600'}"
						aria-label="Toggle tray icon badge {settings.value.notifications.tray ? 'off' : 'on'}"
						onclick={() => settings.updateNotifications({ tray: !settings.value.notifications.tray })}
					>
						<span
							class="absolute top-1 w-4 h-4 bg-white rounded-full transition-transform {settings.value.notifications.tray
								? 'translate-x-7'
								: 'translate-x-1'}"
						></span>
					</button>
				</div>
			</div>

			<div class="p-4 border border-[var(--color-border)] rounded-lg">
				<div class="flex items-center justify-between mb-2">
					<div class="flex-1">
						<span class="font-medium flex items-center gap-2">
							<span class="text-lg">💬</span>
							In-App Messages
						</span>
						<p class="text-xs text-[var(--color-text-muted)] mt-1">
							Show notification banners and messages within the application interface
						</p>
					</div>
					<button
						class="relative w-12 h-6 rounded-full transition-colors {settings.value.notifications.in_app
							? 'bg-primary-600'
							: 'bg-gray-300 dark:bg-gray-600'}"
						aria-label="Toggle in-app notifications {settings.value.notifications.in_app ? 'off' : 'on'}"
						onclick={() => settings.updateNotifications({ in_app: !settings.value.notifications.in_app })}
					>
						<span
							class="absolute top-1 w-4 h-4 bg-white rounded-full transition-transform {settings.value.notifications.in_app
								? 'translate-x-7'
								: 'translate-x-1'}"
						></span>
					</button>
				</div>
			</div>
		</div>
	</section>

	<!-- Scanning -->
	<section class="card p-5 space-y-4">
		<div>
			<h2 class="font-semibold text-lg mb-1">🔍 Scan Configuration</h2>
			<p class="text-sm text-[var(--color-text-secondary)]">
				Customize what files and folders are included in system scans.
			</p>
		</div>

		<div class="space-y-4">
			<div class="p-4 border border-[var(--color-border)] rounded-lg bg-gradient-to-r from-indigo-50/50 to-transparent dark:from-indigo-900/10">
				<div class="flex items-center justify-between mb-2">
					<div class="flex-1">
						<span class="font-medium flex items-center gap-2">
							<span class="text-lg">👁️</span>
							Include Hidden Files
						</span>
						<p class="text-xs text-[var(--color-text-muted)] mt-1">
							Scan configuration files and hidden directories (files starting with .)
						</p>
					</div>
					<button
						class="relative w-12 h-6 rounded-full transition-colors {settings.value.scan.include_hidden
							? 'bg-primary-600'
							: 'bg-gray-300 dark:bg-gray-600'}"
						aria-label="Toggle include hidden files {settings.value.scan.include_hidden ? 'off' : 'on'}"
						onclick={() => settings.updateScan({ include_hidden: !settings.value.scan.include_hidden })}
					>
						<span
							class="absolute top-1 w-4 h-4 bg-white rounded-full transition-transform {settings.value.scan.include_hidden
								? 'translate-x-7'
								: 'translate-x-1'}"
						></span>
					</button>
				</div>
				{#if settings.value.scan.include_hidden}
					<div class="mt-3 p-2 bg-blue-50 dark:bg-blue-900/20 rounded border border-blue-200 dark:border-blue-800">
						<div class="flex items-center gap-2 text-blue-700 dark:text-blue-300 text-xs">
							<span>ℹ️</span>
							<span>Hidden files included - will find more cleanup opportunities but scans may be slower</span>
						</div>
					</div>
				{/if}
			</div>

			<div class="p-4 border border-[var(--color-border)] rounded-lg bg-gradient-to-r from-teal-50/50 to-transparent dark:from-teal-900/10">
				<div class="flex items-center justify-between mb-2">
					<span class="font-medium">📊 Large File Detection</span>
					<span class="text-xs text-caution bg-caution/10 px-2 py-1 rounded">Performance Setting</span>
				</div>
				<p class="text-xs text-[var(--color-text-muted)] mb-3">
					Files larger than this threshold will be highlighted as potential cleanup targets.
				</p>
				<div class="flex justify-end">
					<select
						class="input w-48"
						value={settings.value.scan.large_file_threshold_mb}
						onchange={(e) => settings.updateScan({ large_file_threshold_mb: parseInt(e.currentTarget.value) })}
						aria-label="Set minimum file size to be considered large"
					>
						<option value={50}>50 MB - Very sensitive</option>
						<option value={100}>100 MB - Standard</option>
						<option value={250}>250 MB - Moderate</option>
						<option value={500}>500 MB - Conservative</option>
						<option value={1000}>1 GB - Large files only</option>
					</select>
				</div>
			</div>
		</div>
	</section>

	<!-- Save Settings -->
	<div class="card p-5 bg-gradient-to-r from-primary-50 to-primary-100/50 dark:from-primary-900/20 dark:to-primary-800/10 border border-primary-200 dark:border-primary-800">
		<div class="flex items-center justify-between">
			<div class="flex-1">
				<h3 class="font-semibold mb-1">💾 Apply Changes</h3>
				<p class="text-sm text-[var(--color-text-secondary)]">
					Save your configuration changes to take effect. Settings are stored securely on your system.
				</p>
			</div>
			<div class="flex items-center gap-4">
				{#if saved}
					<div class="flex items-center gap-2 text-safe">
						<svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M5 13l4 4L19 7" />
						</svg>
						<span class="text-sm font-medium">Settings saved successfully!</span>
					</div>
				{:else if saving}
					<div class="flex items-center gap-2 text-primary-600">
						<div class="spinner w-4 h-4 border-2"></div>
						<span class="text-sm">Saving settings...</span>
					</div>
				{:else}
					<div class="text-xs text-[var(--color-text-muted)]">
						Unsaved changes will be lost
					</div>
				{/if}
				<button
					class="btn btn-primary hover:bg-primary-700 transition-colors disabled:opacity-50"
					onclick={saveSettings}
					disabled={saving}
					aria-label="Save all settings changes"
				>
					{#if saving}
						<div class="spinner w-4 h-4 border-2 border-white"></div>
					{:else}
						<svg class="w-4 h-4 mr-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M5 13l4 4L19 7" />
						</svg>
					{/if}
					Save Settings
				</button>
			</div>
		</div>
	</div>
{/if}
</div>
