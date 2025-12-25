<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '$lib/utils/tauri';
	import { notificationStore } from '$lib/stores/notifications.svelte';
	import { logger } from '$lib/utils/logger';
	import { confirmation } from '$lib/stores/confirmation.svelte';
	import type { StartupProgram, StartupProgramsList } from '$lib/generated/types';
	import LoadingSpinner from './ui/LoadingSpinner.svelte';

	let programs = $state<StartupProgram[]>([]);
	let loading = $state(true);
	let toggling = $state<Set<string>>(new Set());

	async function loadPrograms() {
		loading = true;
		try {
			const result = await invoke<StartupProgramsList>('get_startup_programs', undefined, 10000);
			programs = result.programs;
		} catch (e) {
			logger.error('Failed to load startup programs', { component: 'StartupManager' }, e);
			notificationStore.error('Load Failed', 'Could not load startup programs');
		} finally {
			loading = false;
		}
	}

	async function toggleProgram(program: StartupProgram) {
		const newState = !program.enabled;
		const action = newState ? 'enable' : 'disable';

		const confirmed = await confirmation.show({
			title: `${action === 'enable' ? 'Enable' : 'Disable'} Startup Program`,
			message: `Are you sure you want to ${action} "${program.name}"? This will ${action} it from starting automatically.`,
			confirmText: action === 'enable' ? 'Enable' : 'Disable',
			cancelText: 'Cancel',
			type: 'info'
		});

		if (!confirmed) return;

		toggling.add(program.id);
		try {
			await invoke('toggle_startup_program', { id: program.id, enabled: newState });
			program.enabled = newState;
			notificationStore.success(
				'Startup Updated',
				`"${program.name}" has been ${action}d`
			);
		} catch (e) {
			logger.error('Failed to toggle startup program', { component: 'StartupManager', programId: program.id }, e);
			notificationStore.error('Toggle Failed', `Could not ${action} startup program`);
		} finally {
			toggling.delete(program.id);
		}
	}

	function getImpactColor(impact: string): string {
		switch (impact) {
			case 'low': return 'text-green-600 bg-green-50 dark:text-green-400 dark:bg-green-900/20';
			case 'medium': return 'text-yellow-600 bg-yellow-50 dark:text-yellow-400 dark:bg-yellow-900/20';
			case 'high': return 'text-red-600 bg-red-50 dark:text-red-400 dark:bg-red-900/20';
			default: return 'text-gray-600 bg-gray-50 dark:text-gray-400 dark:bg-gray-900/20';
		}
	}

	function getLocationLabel(location: string): string {
		switch (location) {
			case 'xdg_autostart': return 'XDG Autostart';
			case 'systemd_user': return 'Systemd User';
			case 'systemd_system': return 'Systemd System';
			default: return location;
		}
	}

	onMount(() => {
		loadPrograms();
	});
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold mb-1">Startup Manager</h1>
			<p class="text-[var(--color-text-secondary)]">Manage programs that launch at system startup</p>
		</div>
		<button class="btn btn-secondary" onclick={loadPrograms} disabled={loading}>
			{#if loading}
				<LoadingSpinner size="sm" />
			{:else}
				🔄 Refresh
			{/if}
		</button>
	</div>

	{#if loading}
		<div class="flex items-center justify-center h-64">
			<LoadingSpinner size="lg" />
		</div>
	{:else if programs.length === 0}
		<div class="card p-8 text-center">
			<div class="text-4xl mb-4">📋</div>
			<h3 class="text-lg font-semibold mb-2">No Startup Programs Found</h3>
			<p class="text-[var(--color-text-secondary)]">No autostart entries detected in standard locations.</p>
		</div>
	{:else}
		<div class="card">
			<div class="p-4 border-b border-[var(--color-border)]">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold">Startup Programs ({programs.length})</h3>
						<p class="text-sm text-[var(--color-text-secondary)]">
							{programs.filter(p => p.enabled).length} enabled, {programs.filter(p => !p.enabled).length} disabled
						</p>
					</div>
				</div>
			</div>

			<div class="divide-y">
				{#each programs as program}
					<div class="p-4 hover:bg-gray-50 dark:hover:bg-gray-900/50 transition-colors">
						<div class="flex items-center justify-between">
							<div class="flex-1 min-w-0">
								<div class="flex items-center gap-3 mb-2">
									<h4 class="font-semibold truncate">{program.name}</h4>
									<span class="px-2 py-1 text-xs rounded {getImpactColor(program.impact)}">
										{program.impact} impact
									</span>
								</div>
								{#if program.description}
									<p class="text-sm text-[var(--color-text-secondary)] mb-2">{program.description}</p>
								{/if}
								<div class="flex items-center gap-4 text-xs text-[var(--color-text-secondary)]">
									<span>📍 {getLocationLabel(program.location)}</span>
									{#if program.exec_command}
										<span class="truncate max-w-md">⚙️ {program.exec_command}</span>
									{/if}
								</div>
							</div>
							<div class="ml-4">
								<button
									class="relative w-12 h-6 rounded-full transition-colors {program.enabled
										? 'bg-primary-600'
										: 'bg-gray-300 dark:bg-gray-600'}"
									aria-label="{program.enabled ? 'Disable' : 'Enable'} {program.name}"
									onclick={() => toggleProgram(program)}
									disabled={toggling.has(program.id)}
								>
									<span
										class="absolute top-1 w-4 h-4 bg-white rounded-full transition-transform {program.enabled
											? 'translate-x-7'
											: 'translate-x-1'}"
									></span>
								</button>
								{#if toggling.has(program.id)}
									<div class="mt-2 text-xs text-center text-[var(--color-text-secondary)]">Updating...</div>
								{/if}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>
