<script lang="ts">
	interface Props {
		cpuUsage: number;
		loadAverage?: {
			one_minute: number;
			five_minutes: number;
			fifteen_minutes: number;
		};
		usedMemory: number;
		totalMemory: number;
		gpuInfo?: {
			usage: number;
		};
		timestamp: number;
		lastUpdate: number;
		updateCount: number;
	}

	let { cpuUsage, loadAverage, usedMemory, totalMemory, gpuInfo, timestamp, lastUpdate, updateCount }: Props = $props();
</script>

<div class="card p-6">
	<h3 class="font-semibold flex items-center gap-2 mb-4">
		<span class="text-xl">📊</span>
		System Status
	</h3>

	<div class="space-y-3">
		<div class="flex items-center justify-between">
			<span class="text-sm text-[var(--color-text-secondary)]">Overall Load</span>
			<div class="flex items-center gap-2">
				<div class="w-16 h-2 bg-gray-200 dark:bg-gray-700 rounded">
					<div
						class="h-full bg-gradient-to-r from-safe to-caution rounded transition-all duration-300"
						style="width: {Math.min(100, cpuUsage)}%"
					></div>
				</div>
				<span class="text-sm font-medium">
					{Math.round(cpuUsage)}%
				</span>
			</div>
		</div>

		{#if loadAverage}
			<div class="flex items-center justify-between">
				<span class="text-sm text-[var(--color-text-secondary)]">Load Average</span>
				<div class="text-sm font-medium">
					{loadAverage.one_minute.toFixed(2)} / {loadAverage.five_minutes.toFixed(2)} / {loadAverage.fifteen_minutes.toFixed(2)}
				</div>
			</div>
		{/if}

		<div class="flex items-center justify-between">
			<span class="text-sm text-[var(--color-text-secondary)]">Memory Pressure</span>
			<div class="flex items-center gap-2">
				<div class="w-16 h-2 bg-gray-200 dark:bg-gray-700 rounded">
					<div
						class="h-full bg-gradient-to-r from-safe to-caution rounded transition-all duration-300"
						style="width: {((usedMemory / totalMemory) * 100)}%"
					></div>
				</div>
				<span class="text-sm font-medium">
					{Math.round((usedMemory / totalMemory) * 100)}%
				</span>
			</div>
		</div>

		{#if gpuInfo}
			<div class="flex items-center justify-between">
				<span class="text-sm text-[var(--color-text-secondary)]">GPU Load</span>
				<div class="flex items-center gap-2">
					<div class="w-16 h-2 bg-gray-200 dark:bg-gray-700 rounded">
						<div
							class="h-full bg-gradient-to-r from-safe to-caution rounded transition-all duration-300"
							style="width: {gpuInfo.usage}%"
						></div>
					</div>
					<span class="text-sm font-medium">
						{Math.round(gpuInfo.usage)}%
					</span>
				</div>
			</div>
		{/if}
	</div>

	<!-- Last Updated -->
	<div class="mt-4 pt-4 border-t border-[var(--color-border)] text-center">
		<div class="text-xs text-[var(--color-text-muted)]">
			Last Update: {new Date(lastUpdate).toLocaleTimeString()} • Total Updates: {updateCount}
		</div>
		<div class="text-xs text-[var(--color-text-muted)] mt-1">
			Data Timestamp: {new Date(timestamp).toLocaleTimeString()}
		</div>
	</div>
</div>
