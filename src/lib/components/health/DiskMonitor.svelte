<script lang="ts">
	import { formatBytes } from '$lib/utils/tauri';

	interface Props {
		diskReadBytes: number;
		diskWriteBytes: number;
		diskReadOps: number;
		diskWriteOps: number;
	}

	let { diskReadBytes, diskWriteBytes, diskReadOps, diskWriteOps }: Props = $props();
</script>

{#if diskReadBytes > 0 || diskWriteBytes > 0}
	<div class="card p-6">
		<div class="flex items-center justify-between mb-4">
			<h3 class="font-semibold flex items-center gap-2">
				<span class="text-xl">💽</span>
				Disk I/O
			</h3>
		</div>

		<div class="space-y-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<span class="text-blue-500">📖</span>
					<span class="text-sm text-[var(--color-text-secondary)]">Read</span>
				</div>
				<span class="font-medium">{formatBytes(diskReadBytes)}/s</span>
			</div>

			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<span class="text-orange-500">✏️</span>
					<span class="text-sm text-[var(--color-text-secondary)]">Write</span>
				</div>
				<span class="font-medium">{formatBytes(diskWriteBytes)}/s</span>
			</div>

			{#if diskReadOps > 0 || diskWriteOps > 0}
				<div class="pt-2 border-t border-[var(--color-border)]">
					<div class="flex items-center justify-between text-sm">
						<span class="text-[var(--color-text-secondary)]">Operations</span>
						<span class="font-medium">
							{diskReadOps + diskWriteOps}/s
						</span>
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}
