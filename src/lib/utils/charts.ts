/**
 * Chart utilities and configuration presets for Pulito
 * Provides theme-aware colors, chart configurations, and data transformation helpers
 */

import type { ChartConfiguration } from 'chart.js';

/**
 * Get theme-aware colors based on current theme
 */
export function getThemeColors(isDark: boolean = false) {
	if (isDark) {
		return {
			background: 'rgba(30, 30, 30, 0.8)',
			border: 'rgba(200, 200, 200, 0.3)',
			text: 'rgba(255, 255, 255, 0.9)',
			textSecondary: 'rgba(255, 255, 255, 0.6)',
			grid: 'rgba(255, 255, 255, 0.05)',
			// Chart colors (dark mode optimized)
			primary: 'rgb(59, 130, 246)',      // Blue
			secondary: 'rgb(16, 185, 129)',    // Green
			warning: 'rgb(245, 158, 11)',     // Yellow
			danger: 'rgb(239, 68, 68)',       // Red
			purple: 'rgb(139, 92, 246)',      // Purple
			cyan: 'rgb(6, 182, 212)',         // Cyan
			orange: 'rgb(251, 146, 60)',      // Orange
		};
	} else {
		return {
			background: 'rgba(255, 255, 255, 0.8)',
			border: 'rgba(0, 0, 0, 0.1)',
			text: 'rgba(0, 0, 0, 0.9)',
			textSecondary: 'rgba(0, 0, 0, 0.6)',
			grid: 'rgba(0, 0, 0, 0.05)',
			// Chart colors (light mode optimized)
			primary: 'rgb(59, 130, 246)',      // Blue
			secondary: 'rgb(16, 185, 129)',    // Green
			warning: 'rgb(245, 158, 11)',     // Yellow
			danger: 'rgb(239, 68, 68)',       // Red
			purple: 'rgb(139, 92, 246)',      // Purple
			cyan: 'rgb(6, 182, 212)',         // Cyan
			orange: 'rgb(251, 146, 60)',      // Orange
		};
	}
}

/**
 * Get color with transparency
 */
export function transparentize(color: string, alpha: number): string {
	// Handle rgb() format
	if (color.startsWith('rgb(')) {
		const rgb = color.match(/\d+/g);
		if (rgb && rgb.length >= 3) {
			return `rgba(${rgb[0]}, ${rgb[1]}, ${rgb[2]}, ${alpha})`;
		}
	}
	// Handle hex format
	if (color.startsWith('#')) {
		const hex = color.replace('#', '');
		const r = parseInt(hex.substring(0, 2), 16);
		const g = parseInt(hex.substring(2, 4), 16);
		const b = parseInt(hex.substring(4, 6), 16);
		return `rgba(${r}, ${g}, ${b}, ${alpha})`;
	}
	return color;
}

/**
 * Default chart options with theme support
 */
export function getDefaultChartOptions(isDark: boolean = false): Partial<ChartConfiguration['options']> {
	const colors = getThemeColors(isDark);

	return {
		responsive: true,
		maintainAspectRatio: false,
		animation: {
			duration: 750,
			easing: 'easeInOutQuart' as const,
		},
		plugins: {
			legend: {
				labels: {
					color: colors.text,
					font: {
						size: 12,
					},
					usePointStyle: true,
					padding: 15,
				},
			},
			tooltip: {
				backgroundColor: isDark ? 'rgba(0, 0, 0, 0.9)' : 'rgba(0, 0, 0, 0.8)',
				titleColor: colors.text,
				bodyColor: colors.text,
				borderColor: colors.border,
				borderWidth: 1,
				padding: 12,
				titleFont: {
					size: 14,
					weight: 'bold',
				},
				bodyFont: {
					size: 12,
				},
				displayColors: true,
			},
		},
		scales: {
			x: {
				grid: {
					color: colors.grid,
					display: false,
				},
				ticks: {
					color: colors.textSecondary,
					font: {
						size: 11,
					},
				},
			},
			y: {
				grid: {
					color: colors.grid,
				},
				ticks: {
					color: colors.textSecondary,
					font: {
						size: 11,
					},
				},
			},
		},
	};
}

/**
 * Storage breakdown color scheme
 */
export function getStorageColors(isDark: boolean = false) {
	const colors = getThemeColors(isDark);
	return {
		caches: transparentize(colors.primary, 0.8),
		packages: transparentize(colors.secondary, 0.8),
		logs: transparentize(colors.warning, 0.8),
		userFiles: transparentize(colors.purple, 0.8),
		free: transparentize(colors.textSecondary, 0.3),
	};
}

/**
 * CPU core colors (for multi-core visualization)
 */
export function getCPUCoreColors(coreCount: number, isDark: boolean = false): string[] {
	const colors = getThemeColors(isDark);
	const baseHue = 200; // Blue base
	const colorsArray: string[] = [];

	for (let i = 0; i < coreCount; i++) {
		const hue = (baseHue + (i * 360 / coreCount)) % 360;
		const saturation = isDark ? 70 : 60;
		const lightness = isDark ? 50 : 45;
		colorsArray.push(`hsl(${hue}, ${saturation}%, ${lightness}%)`);
	}

	return colorsArray;
}

/**
 * Temperature threshold colors
 */
export function getTemperatureColor(temp: number, thresholds: { good: number; warning: number; critical: number }): string {
	if (temp < thresholds.good) return 'rgb(16, 185, 129)'; // Green
	if (temp < thresholds.warning) return 'rgb(245, 158, 11)'; // Yellow
	if (temp < thresholds.critical) return 'rgb(251, 146, 60)'; // Orange
	return 'rgb(239, 68, 68)'; // Red
}

/**
 * Format bytes for chart tooltips
 */
export function formatBytesForChart(bytes: number): string {
	if (bytes === 0) return '0 B';
	const k = 1024;
	const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
}

/**
 * Format percentage for chart tooltips
 */
export function formatPercentForChart(value: number, decimals: number = 1): string {
	return `${value.toFixed(decimals)}%`;
}

/**
 * Format time for chart labels
 */
export function formatTimeForChart(timestamp: number): string {
	const date = new Date(timestamp);
	const hours = date.getHours().toString().padStart(2, '0');
	const minutes = date.getMinutes().toString().padStart(2, '0');
	return `${hours}:${minutes}`;
}

/**
 * Format date for chart labels (for longer time ranges)
 */
export function formatDateForChart(timestamp: number): string {
	const date = new Date(timestamp);
	const month = (date.getMonth() + 1).toString().padStart(2, '0');
	const day = date.getDate().toString().padStart(2, '0');
	const hours = date.getHours().toString().padStart(2, '0');
	const minutes = date.getMinutes().toString().padStart(2, '0');
	return `${month}/${day} ${hours}:${minutes}`;
}

/**
 * Generate gradient fill for area charts
 */
export function createGradient(
	ctx: CanvasRenderingContext2D,
	color: string,
	direction: 'vertical' | 'horizontal' = 'vertical'
): CanvasGradient {
	const gradient = direction === 'vertical'
		? ctx.createLinearGradient(0, 0, 0, ctx.canvas.height)
		: ctx.createLinearGradient(0, 0, ctx.canvas.width, 0);

	gradient.addColorStop(0, transparentize(color, 0.3));
	gradient.addColorStop(1, transparentize(color, 0.05));

	return gradient;
}

/**
 * Limit data points for performance (keep most recent N points)
 */
export function limitDataPoints<T extends { timestamp: number }>(
	data: T[],
	maxPoints: number = 500
): T[] {
	if (data.length <= maxPoints) return data;

	// Keep most recent points
	return data.slice(-maxPoints);
}

/**
 * Filter data by time range
 */
export function filterByTimeRange<T extends { timestamp: number }>(
	data: T[],
	range: '1h' | '6h' | '24h' | 'all'
): T[] {
	if (range === 'all') return data;

	const now = Date.now();
	const ranges = {
		'1h': 60 * 60 * 1000,
		'6h': 6 * 60 * 60 * 1000,
		'24h': 24 * 60 * 60 * 1000,
	};

	const cutoff = now - ranges[range];
	return data.filter(item => item.timestamp >= cutoff);
}

/**
 * Interpolate missing data points (for smooth charts)
 */
export function interpolateData<T extends { timestamp: number; value: number }>(
	data: T[],
	intervalMs: number = 2000
): T[] {
	if (data.length < 2) return data;

	const interpolated: T[] = [data[0]];

	for (let i = 1; i < data.length; i++) {
		const prev = data[i - 1];
		const curr = data[i];
		const timeDiff = curr.timestamp - prev.timestamp;

		if (timeDiff > intervalMs * 2) {
			// Missing data, interpolate
			const steps = Math.floor(timeDiff / intervalMs);
			for (let j = 1; j < steps; j++) {
				const ratio = j / steps;
				interpolated.push({
					...prev,
					timestamp: prev.timestamp + (timeDiff * ratio),
					value: prev.value + (curr.value - prev.value) * ratio,
				} as T);
			}
		}

		interpolated.push(curr);
	}

	return interpolated;
}
