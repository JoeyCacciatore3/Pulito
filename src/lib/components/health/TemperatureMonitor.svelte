<script lang="ts">
	import { getTempColor } from '$lib/utils/color-utils';

	interface Props {
		temperatures: {
			cpu: number;
			cpu_sensors: number;
			system: number;
			gpu?: number;
		};
	}

	let { temperatures }: Props = $props();

	function formatTemp(temp: number): string {
		return `${temp.toFixed(1)}°C`;
	}

	function getTempStatus(temp: number): { status: string; description: string; zone: string } {
		if (temp < 50) {
			return {
				status: 'Excellent',
				description: 'Idle state. Excellent cooling performance.',
				zone: 'Optimal operating range for all workloads.'
			};
		}
		if (temp < 70) {
			return {
				status: 'Safe',
				description: 'Light to moderate load. Excellent cooling.',
				zone: 'Safe for continuous operation. No concerns.'
			};
		}
		if (temp < 80) {
			return {
				status: 'Normal',
				description: 'Moderate to heavy load. Normal operation.',
				zone: 'Typical for gaming, video editing, and intensive applications.'
			};
		}
		if (temp < 90) {
			return {
				status: 'Warm',
				description: 'Heavy load. Normal for modern CPUs under stress.',
				zone: 'Safe operating range. Laptop CPUs commonly reach this during intensive tasks.'
			};
		}
		if (temp < 95) {
			return {
				status: 'Hot',
				description: 'Very heavy load. Still within safe operating range.',
				zone: 'Normal for laptop CPUs under maximum load. Thermal throttling may begin at 100°C.'
			};
		}
		if (temp < 100) {
			return {
				status: 'Very Hot',
				description: 'Extreme load. Approaching thermal throttling threshold.',
				zone: 'Monitor closely. Performance may be throttled above 100°C.'
			};
		}
		return {
			status: 'Critical',
			description: 'At thermal limit. Thermal throttling active.',
			zone: 'Immediate attention required. CPU is reducing performance to prevent damage.'
		};
	}
</script>

<div class="card p-6">
	<h3 class="font-semibold flex items-center gap-2 mb-4">
		<span class="text-xl">🌡️</span>
		Temperatures
	</h3>

	<div class="space-y-4">
		<!-- CPU Temperature (single primary reading) -->
		<div class="space-y-1">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<span class="text-blue-500">🖥️</span>
					<div class="flex flex-col">
						<span class="text-sm text-[var(--color-text-secondary)]">CPU Temperature</span>
						<span class="text-xs text-[var(--color-text-secondary)] opacity-75">
							{temperatures.cpu_sensors > 0 ? '(lm-sensors)' : '(thermal zone)'}
						</span>
					</div>
				</div>
				<span class="font-medium {getTempColor(temperatures.cpu)}">
					{temperatures.cpu > 0 ? formatTemp(temperatures.cpu) : 'N/A'}
				</span>
			</div>
			{#if temperatures.cpu > 0}
				<div class="text-xs text-[var(--color-text-secondary)] opacity-75 pl-7 space-y-1">
					<div>{getTempStatus(temperatures.cpu).description}</div>
					<div class="text-[10px] opacity-60">{getTempStatus(temperatures.cpu).zone}</div>
				</div>
			{/if}
		</div>

		<div class="space-y-1">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<span class="text-gray-500">🏠</span>
					<span class="text-sm text-[var(--color-text-secondary)]">System</span>
				</div>
				<span class="font-medium {getTempColor(temperatures.system)}">
					{temperatures.system > 0 ? formatTemp(temperatures.system) : 'N/A'}
				</span>
			</div>
			{#if temperatures.system > 0}
				<div class="text-xs text-[var(--color-text-secondary)] opacity-75 pl-7 space-y-1">
					<div>{getTempStatus(temperatures.system).description}</div>
					<div class="text-[10px] opacity-60">{getTempStatus(temperatures.system).zone}</div>
				</div>
			{/if}
		</div>

		{#if temperatures.gpu && temperatures.gpu > 0}
			<div class="space-y-1">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<span class="text-purple-500">🎮</span>
						<span class="text-sm text-[var(--color-text-secondary)]">GPU</span>
					</div>
					<span class="font-medium {getTempColor(temperatures.gpu)}">
						{formatTemp(temperatures.gpu)}
					</span>
				</div>
				<div class="text-xs text-[var(--color-text-secondary)] opacity-75 pl-7 space-y-1">
					<div>{getTempStatus(temperatures.gpu).description}</div>
					<div class="text-[10px] opacity-60">{getTempStatus(temperatures.gpu).zone}</div>
				</div>
			</div>
		{/if}
	</div>

	<!-- Temperature Status -->
	<div class="mt-4 pt-4 border-t border-[var(--color-border)]">
		<div class="flex flex-col items-center gap-2">
			<div class="flex items-center gap-2">
				<div class="w-2 h-2 rounded-full bg-safe animate-pulse"></div>
				<span class="text-sm text-[var(--color-text-secondary)]">Overall Status: Normal</span>
			</div>
			<div class="text-xs text-[var(--color-text-secondary)] opacity-75 text-center max-w-sm space-y-1">
				<div><strong>Modern CPU Temperature Guidelines (2025):</strong></div>
				<div>• <strong>&lt; 70°C:</strong> Excellent - Safe for all workloads</div>
				<div>• <strong>70-90°C:</strong> Normal - Typical for heavy load (gaming, rendering)</div>
				<div>• <strong>90-95°C:</strong> Warm - Still safe, common for laptop CPUs under stress</div>
				<div>• <strong>95-100°C:</strong> Hot - Monitor closely, throttling may begin</div>
				<div>• <strong>≥ 100°C:</strong> Critical - Thermal limit reached, performance throttled</div>
				<div class="mt-2 text-[10px] opacity-60">
					Laptop CPUs (like Intel i5-10500H) are designed to operate up to 100°C.
					Temperatures in the 80-95°C range are normal during intensive tasks.
				</div>
			</div>
		</div>
	</div>
</div>
