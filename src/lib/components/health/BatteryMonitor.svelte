<script lang="ts">
	import ProgressBar from '$lib/components/ui/ProgressBar.svelte';

	interface BatteryInfo {
		percentage: number;
		is_charging: boolean;
		time_to_full?: number;
		time_to_empty?: number;
		power_consumption?: number;
	}

	interface Props {
		batteryInfo?: BatteryInfo;
	}

	let { batteryInfo }: Props = $props();
</script>

{#if batteryInfo}
	<div class="card p-6">
		<div class="flex items-center justify-between mb-4">
			<h3 class="font-semibold flex items-center gap-2">
				<span class="text-xl">{batteryInfo.is_charging ? '🔋' : '🪫'}</span>
				Battery
			</h3>
			<div class="text-right">
				<div class="text-2xl font-bold">
					{batteryInfo.percentage.toFixed(0)}%
				</div>
				<div class="text-sm text-[var(--color-text-secondary)]">
					{batteryInfo.is_charging ? 'Charging' : 'Discharging'}
				</div>
			</div>
		</div>

		<ProgressBar
			percentage={batteryInfo.percentage}
			color="bg-green-500"
			height="h-4"
		/>

		{#if batteryInfo.time_to_full}
			<div class="text-sm text-[var(--color-text-secondary)] mt-2">
				Time to full: {Math.round(batteryInfo.time_to_full / 60)}m
			</div>
		{:else if batteryInfo.time_to_empty}
			<div class="text-sm text-[var(--color-text-secondary)] mt-2">
				Time remaining: {Math.round(batteryInfo.time_to_empty / 60)}m
			</div>
		{/if}
	</div>
{/if}
