<script lang="ts">
	import { formatBytes } from '$lib/utils/tauri';
	import { getUsageColor } from '$lib/utils/color-utils';

	interface ProcessInfo {
		pid: number;
		name: string;
		cpu_usage: number;
		memory_usage: number;
		status: string;
		user_id?: number;
	}

	interface Props {
		topProcesses: ProcessInfo[];
	}

	let { topProcesses }: Props = $props();
</script>

<div class="card p-6">
	<h3 class="font-semibold flex items-center gap-2 mb-4">
		<span class="text-xl">⚙️</span>
		Top Processes
	</h3>

	<div class="space-y-2 max-h-64 overflow-y-auto">
		{#each topProcesses.slice(0, 5) as process}
			<div class="flex items-center justify-between py-2 px-3 bg-muted/20 rounded">
				<div class="min-w-0 flex-1">
					<div class="font-medium truncate">{process.name}</div>
					<div class="text-xs text-[var(--color-text-muted)]">PID: {process.pid}</div>
				</div>
				<div class="text-right ml-4">
					<div class="text-sm font-medium {getUsageColor(process.cpu_usage)}">
						{process.cpu_usage.toFixed(1)}%
					</div>
					<div class="text-xs text-[var(--color-text-muted)]">
						{formatBytes(process.memory_usage)}
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
