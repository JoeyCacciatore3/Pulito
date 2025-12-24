<script lang="ts">
	import { getUsageColor, getUsageBgColor } from '$lib/utils/color-utils';
	import ProgressBar from '$lib/components/ui/ProgressBar.svelte';

	interface Props {
		cpuUsage: number;
		cpuFrequency: number;
		cpuCores: number;
		coreUsages: number[];
		temperature?: number;
	}

	let { cpuUsage, cpuFrequency, cpuCores, coreUsages, temperature }: Props = $props();

	function formatFrequency(freq: number): string {
		if (freq >= 1000) {
			return `${(freq / 1000).toFixed(1)} GHz`;
		}
		return `${freq.toFixed(0)} MHz`;
	}
</script>

<div class="card p-6">
	<div class="flex items-center justify-between mb-4">
		<h3 class="font-semibold flex items-center gap-2">
			<span class="text-xl">🖥️</span>
			CPU
		</h3>
		<div class="text-right">
			<div class="text-2xl font-bold {getUsageColor(cpuUsage)}">
				{cpuUsage.toFixed(1)}%
			</div>
			<div class="text-xs text-[var(--color-text-muted)]">
				{formatFrequency(cpuFrequency)}
			</div>
		</div>
	</div>

	<!-- CPU Usage Bar -->
	<ProgressBar
		percentage={cpuUsage}
		color={getUsageBgColor(cpuUsage)}
	/>

	<!-- CPU Cores -->
	<div class="space-y-2">
		<div class="text-sm text-[var(--color-text-secondary)]">
			Cores ({cpuCores})
		</div>
		<div class="grid grid-cols-4 gap-1">
			{#each coreUsages as usage, i}
				<div class="text-xs text-center">
					<div class="bg-gray-200 dark:bg-gray-700 rounded h-6 flex items-end justify-center relative overflow-hidden">
						<div
							class="absolute bottom-0 left-0 right-0 bg-gradient-to-t {getUsageBgColor(usage)} transition-all duration-300"
							style="height: {usage}%"
						></div>
						<span class="relative text-[10px] font-medium text-gray-700 dark:text-gray-300">
							{i + 1}
						</span>
					</div>
					<div class="text-[9px] text-[var(--color-text-muted)] mt-1">
						{usage.toFixed(0)}%
					</div>
				</div>
			{/each}
		</div>
	</div>

	<!-- CPU Temperature -->
	{#if temperature && temperature > 0}
		<div class="mt-4 pt-4 border-t border-[var(--color-border)]">
			<div class="flex items-center justify-between text-sm">
				<span class="text-[var(--color-text-secondary)]">Temperature</span>
				<span class="font-medium {getUsageColor(temperature > 80 ? 100 : temperature > 60 ? 60 : 20)}">
					{temperature.toFixed(1)}°C
				</span>
			</div>
		</div>
	{/if}
</div>
