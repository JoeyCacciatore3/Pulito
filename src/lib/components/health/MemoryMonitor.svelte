<script lang="ts">
	import { formatBytes } from '$lib/utils/tauri';
	import ProgressBar from '$lib/components/ui/ProgressBar.svelte';

	interface Props {
		totalMemory: number;
		usedMemory: number;
		availableMemory: number;
	}

	let { totalMemory, usedMemory, availableMemory }: Props = $props();
</script>

<div class="card p-6">
	<div class="flex items-center justify-between mb-4">
		<h3 class="font-semibold flex items-center gap-2">
			<span class="text-xl">🧠</span>
			Memory (RAM)
		</h3>
		<div class="text-right">
			<div class="text-lg font-bold">
				{formatBytes(usedMemory)}
			</div>
			<div class="text-xs text-[var(--color-text-muted)]">
				of {formatBytes(totalMemory)}
			</div>
		</div>
	</div>

	<!-- Memory Usage Bar -->
	<ProgressBar
		percentage={(usedMemory / totalMemory) * 100}
		color="bg-primary-500"
	/>

	<!-- Memory Details -->
	<div class="space-y-2 text-sm">
		<div class="flex justify-between">
			<span class="text-[var(--color-text-secondary)]">Used</span>
			<span class="font-medium">{formatBytes(usedMemory)}</span>
		</div>
		<div class="flex justify-between">
			<span class="text-[var(--color-text-secondary)]">Available</span>
			<span class="font-medium text-safe">{formatBytes(availableMemory)}</span>
		</div>
		<div class="flex justify-between pt-2 border-t border-[var(--color-border)]">
			<span class="text-[var(--color-text-secondary)]">Usage</span>
			<span class="font-medium">
				{Math.round((usedMemory / totalMemory) * 100)}%
			</span>
		</div>
	</div>
</div>
