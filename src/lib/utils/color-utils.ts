/**
 * Color utility functions for status indicators and UI elements
 * Consolidates duplicate color logic from various components
 */

/**
 * Get usage color based on percentage (0-100)
 * Used for CPU, memory, disk, and other resource usage indicators
 */
export function getUsageColor(usage: number): string {
	if (usage < 50) return 'text-safe';
	if (usage < 80) return 'text-caution';
	return 'text-critical';
}

/**
 * Get usage background color class for progress bars and indicators
 */
export function getUsageBgColor(usage: number): string {
	if (usage < 50) return 'bg-safe';
	if (usage < 80) return 'bg-caution';
	return 'bg-critical';
}

/**
 * Get temperature color based on temperature in Celsius
 * Modern CPUs (especially laptops) safely operate up to 90-95°C under load
 * Thermal throttling typically occurs at 100°C
 * - Safe: < 70°C (idle to light load, excellent cooling)
 * - Normal: 70-90°C (moderate to heavy load, normal operation for modern CPUs)
 * - Warm: 90-95°C (very heavy load, still safe but monitor)
 * - Critical: ≥ 100°C (at thermal limit, throttling occurs)
 */
export function getTempColor(temp: number): string {
	if (temp < 70) return 'text-safe';
	if (temp < 90) return 'text-caution'; // Yellow - normal for heavy load
	if (temp < 100) return 'text-danger'; // Orange - warm but still safe
	return 'text-critical'; // Red - at thermal limit
}

/**
 * Get status color for disk health and system status
 * Returns hex color codes for direct style usage
 */
export function getStatusColor(status: string): string {
	switch (status.toLowerCase()) {
		case 'green':
		case 'safe':
			return '#22c55e';
		case 'yellow':
		case 'caution':
			return '#eab308';
		case 'red':
		case 'critical':
			return '#ef4444';
		default:
			return '#6b7280';
	}
}

/**
 * Get status icon emoji for disk health and system status
 */
export function getStatusIcon(status: string): string {
	switch (status.toLowerCase()) {
		case 'green':
		case 'safe':
			return '🟢';
		case 'yellow':
		case 'caution':
			return '🟡';
		case 'red':
		case 'critical':
			return '🔴';
		default:
			return '⚪';
	}
}

/**
 * Get risk level color classes for UI elements
 */
export function getRiskLevelClasses(level: number): {
	textClass: string;
	bgClass: string;
	borderClass?: string;
} {
	switch (level) {
		case 0:
			return {
				textClass: 'text-safe',
				bgClass: 'badge-safe',
			};
		case 1:
			return {
				textClass: 'text-lime-600',
				bgClass: 'badge-safe',
			};
		case 2:
			return {
				textClass: 'text-caution',
				bgClass: 'badge-caution',
			};
		case 3:
			return {
				textClass: 'text-danger',
				bgClass: 'badge-danger',
			};
		case 4:
		case 5:
		default:
			return {
				textClass: 'text-critical',
				bgClass: 'badge-critical',
			};
	}
}
