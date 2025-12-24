<script lang="ts">
	import { formatBytes } from '$lib/utils/tauri';

	interface NetworkInterfaceInfo {
		name: string;
		received: number;
		transmitted: number;
		packets_received: number;
		packets_transmitted: number;
		errors_received: number;
		errors_transmitted: number;
	}

	interface NetworkConnection {
		local_address: string;
		remote_address: string;
		local_port: number;
		remote_port: number;
		state: string;
		process_name?: string;
		process_pid?: number;
	}

	interface Props {
		networkUp: number;
		networkDown: number;
		networkInterfaces: NetworkInterfaceInfo[];
		activeConnections: NetworkConnection[];
	}

	let { networkUp, networkDown, networkInterfaces, activeConnections }: Props = $props();
</script>

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
			<span class="font-medium">{formatBytes(networkUp)}/s</span>
		</div>

		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				<span class="text-blue-500">⬇️</span>
				<span class="text-sm text-[var(--color-text-secondary)]">Download</span>
			</div>
			<span class="font-medium">{formatBytes(networkDown)}/s</span>
		</div>
	</div>

	<!-- Network Interfaces -->
	{#if networkInterfaces.length > 0}
		<div class="mt-4 pt-4 border-t border-[var(--color-border)]">
			<h4 class="text-sm font-medium mb-2">Interfaces</h4>
			<div class="space-y-1 max-h-32 overflow-y-auto">
				{#each networkInterfaces.slice(0, 3) as iface}
					<div class="flex items-center justify-between text-xs">
						<span class="text-[var(--color-text-secondary)] truncate max-w-24">{iface.name}</span>
						<div class="text-right">
							<div>↑{formatBytes(iface.transmitted)}</div>
							<div>↓{formatBytes(iface.received)}</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Active Connections -->
	{#if activeConnections.length > 0}
		<div class="mt-4 pt-4 border-t border-[var(--color-border)]">
			<h4 class="text-sm font-medium mb-2">
				Active Connections ({activeConnections.length})
			</h4>
			<div class="space-y-1 max-h-40 overflow-y-auto">
				{#each activeConnections.slice(0, 8) as conn}
					<div class="flex items-center justify-between text-xs p-2 bg-muted/20 rounded">
						<div class="min-w-0 flex-1">
							<div class="truncate">
								{conn.local_address}:{conn.local_port} → {conn.remote_address}:{conn.remote_port}
							</div>
							{#if conn.process_name}
								<div class="text-[var(--color-text-muted)] truncate">
									{conn.process_name} (PID: {conn.process_pid})
								</div>
							{/if}
						</div>
						<div class="ml-2">
							<span class="px-1 py-0.5 rounded text-xs
								{conn.state === 'ESTABLISHED' ? 'bg-green-100 text-green-800' :
								 conn.state === 'LISTEN' ? 'bg-blue-100 text-blue-800' :
								 'bg-gray-100 text-gray-800'}">
								{conn.state}
							</span>
						</div>
					</div>
				{/each}
				{#if activeConnections.length > 8}
					<div class="text-center text-xs text-[var(--color-text-muted)] py-1">
						...and {activeConnections.length - 8} more connections
					</div>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Network Activity Indicator -->
	<div class="mt-4 pt-4 border-t border-[var(--color-border)]">
		<div class="flex items-center justify-center gap-2">
			<div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
			<span class="text-sm text-[var(--color-text-secondary)]">Active</span>
		</div>
	</div>
</div>
