/**
 * Performance monitoring utilities for file explorer
 */

export interface PerformanceMetrics {
	renderTime: number;
	memoryUsage?: number;
	nodeCount: number;
	visibleNodes: number;
}

export class PerformanceMonitor {
	private static instance: PerformanceMonitor;
	private metrics: PerformanceMetrics[] = [];
	private observers: ((metrics: PerformanceMetrics) => void)[] = [];

	static getInstance(): PerformanceMonitor {
		if (!PerformanceMonitor.instance) {
			PerformanceMonitor.instance = new PerformanceMonitor();
		}
		return PerformanceMonitor.instance;
	}

	startMeasurement(): () => PerformanceMetrics {
		const startTime = performance.now();
		const startMemory = (performance as any).memory?.usedJSHeapSize;

		return () => {
			const endTime = performance.now();
			const endMemory = (performance as any).memory?.usedJSHeapSize;

			const metrics: PerformanceMetrics = {
				renderTime: endTime - startTime,
				memoryUsage: endMemory && startMemory ? endMemory - startMemory : undefined,
				nodeCount: 0,
				visibleNodes: 0
			};

			this.metrics.push(metrics);
			this.notifyObservers(metrics);

			return metrics;
		};
	}

	recordNodeCount(nodeCount: number, visibleNodes: number) {
		if (this.metrics.length > 0) {
			const latest = this.metrics[this.metrics.length - 1];
			latest.nodeCount = nodeCount;
			latest.visibleNodes = visibleNodes;
		}
	}

	addObserver(callback: (metrics: PerformanceMetrics) => void) {
		this.observers.push(callback);
	}

	removeObserver(callback: (metrics: PerformanceMetrics) => void) {
		this.observers = this.observers.filter(obs => obs !== callback);
	}

	private notifyObservers(metrics: PerformanceMetrics) {
		this.observers.forEach(observer => observer(metrics));
	}

	getAverageMetrics(): PerformanceMetrics | null {
		if (this.metrics.length === 0) return null;

		const avg = this.metrics.reduce((acc, curr) => ({
			renderTime: acc.renderTime + curr.renderTime,
			memoryUsage: curr.memoryUsage ? (acc.memoryUsage || 0) + curr.memoryUsage : acc.memoryUsage,
			nodeCount: acc.nodeCount + curr.nodeCount,
			visibleNodes: acc.visibleNodes + curr.visibleNodes
		}), {
			renderTime: 0,
			memoryUsage: 0,
			nodeCount: 0,
			visibleNodes: 0
		});

		return {
			renderTime: avg.renderTime / this.metrics.length,
			memoryUsage: avg.memoryUsage ? avg.memoryUsage / this.metrics.length : undefined,
			nodeCount: Math.round(avg.nodeCount / this.metrics.length),
			visibleNodes: Math.round(avg.visibleNodes / this.metrics.length)
		};
	}

	clearMetrics() {
		this.metrics = [];
	}
}

// Global performance monitor instance
export const performanceMonitor = PerformanceMonitor.getInstance();

// Performance measurement utilities
export function measurePerformance<T>(fn: () => T): { result: T; metrics: PerformanceMetrics } {
	const endMeasurement = performanceMonitor.startMeasurement();
	const result = fn();
	const metrics = endMeasurement();
	return { result, metrics };
}

export function debounce<T extends (...args: any[]) => any>(
	func: T,
	wait: number
): (...args: Parameters<T>) => void {
	let timeout: NodeJS.Timeout;
	return (...args: Parameters<T>) => {
		clearTimeout(timeout);
		timeout = setTimeout(() => func(...args), wait);
	};
}

// Memory usage monitoring (if available)
export function getMemoryUsage(): { used: number; total: number; limit: number } | null {
	const memInfo = (performance as any).memory;
	if (!memInfo) return null;

	return {
		used: memInfo.usedJSHeapSize,
		total: memInfo.totalJSHeapSize,
		limit: memInfo.jsHeapSizeLimit
	};
}

// Performance warning system
export function logPerformanceWarning(component: string, metrics: PerformanceMetrics) {
	if (metrics.renderTime > 100) {
		console.warn(`${component}: Slow render detected (${metrics.renderTime.toFixed(2)}ms)`);
	}

	if (metrics.memoryUsage && metrics.memoryUsage > 10 * 1024 * 1024) { // 10MB
		console.warn(`${component}: High memory usage detected (${(metrics.memoryUsage / 1024 / 1024).toFixed(2)}MB)`);
	}

	if (metrics.nodeCount > 1000) {
		console.warn(`${component}: Large dataset detected (${metrics.nodeCount} nodes)`);
	}
}
