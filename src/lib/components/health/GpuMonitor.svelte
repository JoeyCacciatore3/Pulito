<script lang="ts">
	import { formatBytes } from '$lib/utils/tauri';
	import { getUsageColor, getUsageBgColor, getTempColor } from '$lib/utils/color-utils';
	import ProgressBar from '$lib/components/ui/ProgressBar.svelte';

	interface GpuInfo {
		name: string;
		usage: number;
		memory_used: number;
		memory_total: number;
		temperature?: number;
	}

	interface Props {
		gpuInfo?: GpuInfo;
	}

	let { gpuInfo }: Props = $props();
</script>

{#if gpuInfo}
	<div class="card p-6">
		<div class="flex items-center justify-between mb-4">
			<h3 class="font-semibold flex items-center gap-2">
				<span class="text-xl">🎮</span>
				GPU
			</h3>
			<div class="text-right">
				<div class="text-lg font-bold {getUsageColor(gpuInfo.usage)}">
					{gpuInfo.usage.toFixed(1)}%
				</div>
			</div>
		</div>

		<!-- GPU Name -->
		<div class="text-sm text-[var(--color-text-secondary)] mb-3 truncate">
			{gpuInfo.name}
		</div>

		<!-- GPU Usage Bar -->
		<ProgressBar
			percentage={gpuInfo.usage}
			color={getUsageBgColor(gpuInfo.usage)}
		/>

		<!-- GPU Memory -->
		<div class="space-y-2 text-sm">
			<div class="flex justify-between">
				<span class="text-[var(--color-text-secondary)]">VRAM</span>
				<span class="font-medium">
					{formatBytes(gpuInfo.memory_used)} / {formatBytes(gpuInfo.memory_total)}
				</span>
			</div>
			<div class="flex justify-between pt-2 border-t border-[var(--color-border)]">
				<span class="text-[var(--color-text-secondary)]">Usage</span>
				<span class="font-medium">
					{Math.round((gpuInfo.memory_used / gpuInfo.memory_total) * 100)}%
				</span>
			</div>
		</div>

		<!-- GPU Temperature -->
		{#if gpuInfo.temperature && gpuInfo.temperature > 0}
			<div class="mt-4 pt-4 border-t border-[var(--color-border)]">
				<div class="flex items-center justify-between text-sm">
					<span class="text-[var(--color-text-secondary)]">Temperature</span>
					<span class="font-medium {getTempColor(gpuInfo.temperature)}">
						{gpuInfo.temperature.toFixed(1)}°C
					</span>
				</div>
			</div>
		{/if}
	</div>
{:else}
	<!-- Network Monitor (takes GPU slot if no GPU) -->
	<div class="card p-6">
		<div class="flex items-center justify-between mb-4">
			<h3 class="font-semibold flex items-center gap-2">
				<span class="text-xl">🌐</span>
				Network
			</h3>
		</div>

		<!-- Network Stats -->
		<div class="space-y-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<span class="text-green-500">⬆️</span>
					<span class="text-sm text-[var(--color-text-secondary)]">Upload</span>
				</div>
				<span class="font-medium">0 B/s</span>
			</div>

			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<span class="text-blue-500">⬇️</span>
					<span class="text-sm text-[var(--color-text-secondary)]">Download</span>
				</div>
				<span class="font-medium">0 B/s</span>
			</div>
		</div>

		<!-- Network Activity Indicator -->
		<div class="mt-4 pt-4 border-t border-[var(--color-border)]">
			<div class="flex items-center justify-center gap-2">
				<div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
				<span class="text-sm text-[var(--color-text-secondary)]">Active</span>
			</div>
		</div>
	</div>
{/if}
