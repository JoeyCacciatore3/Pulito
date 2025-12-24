/**
 * Performance monitoring utilities for Pulito
 * Tracks app performance, memory usage, and responsiveness
 */

import { logger } from './logger';

/**
 * Performance Memory API interface (non-standard, Chrome-specific)
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Performance/memory
 */
interface PerformanceMemory {
	usedJSHeapSize: number;
	totalJSHeapSize: number;
	jsHeapSizeLimit: number;
}

/**
 * Extended Performance interface with memory property
 */
interface PerformanceWithMemory extends Performance {
	memory?: PerformanceMemory;
}

/**
 * Type guard for PerformanceEventTiming (First Input Delay)
 */
function isPerformanceEventTiming(entry: PerformanceEntry): entry is PerformanceEventTiming {
	return entry.entryType === 'first-input' && 'processingStart' in entry;
}

/**
 * LayoutShift interface for Cumulative Layout Shift entries
 * @see https://developer.mozilla.org/en-US/docs/Web/API/LayoutShift
 */
interface LayoutShift extends PerformanceEntry {
	value: number;
	hadRecentInput: boolean;
	sources: ReadonlyArray<LayoutShiftAttribution>;
}

interface LayoutShiftAttribution {
	node?: Node;
	previousRect: DOMRectReadOnly;
	currentRect: DOMRectReadOnly;
}

/**
 * Type guard for LayoutShift (Cumulative Layout Shift)
 */
function isLayoutShift(entry: PerformanceEntry): entry is LayoutShift {
	return entry.entryType === 'layout-shift' && 'value' in entry && 'hadRecentInput' in entry;
}

interface PerformanceMetrics {
	timestamp: number;
	renderTime: number;
	memoryUsage?: number;
	fps?: number;
	networkRequests: number;
	largestContentfulPaint?: number;
	firstInputDelay?: number;
	cumulativeLayoutShift?: number;
}

class PerformanceMonitor {
	private metrics: PerformanceMetrics[] = [];
	private observers: PerformanceObserver[] = [];
	private isMonitoring = false;
	private maxMetricsHistory = 100;

	/**
	 * Start performance monitoring. Includes SSR compatibility checks
	 * to prevent errors during server-side rendering where window is undefined.
	 */
	startMonitoring() {
		// SSR compatibility: Only run in browser environment
		if (typeof window === 'undefined') return;

		if (this.isMonitoring) return;
		this.isMonitoring = true;

		// Monitor Core Web Vitals
		this.setupCoreWebVitals();

		// Monitor memory usage
		this.setupMemoryMonitoring();

		// Monitor FPS
		this.setupFPSMonitoring();

		// Monitor network requests
		this.setupNetworkMonitoring();

		logger.debug('Performance monitoring started');
	}

	stopMonitoring() {
		if (!this.isMonitoring) return;

		this.observers.forEach(observer => observer.disconnect());
		this.observers = [];
		this.isMonitoring = false;

		logger.debug('Performance monitoring stopped');
	}

	/**
	 * Setup Core Web Vitals monitoring (LCP, FID, CLS).
	 * Includes SSR compatibility checks to prevent window access errors.
	 */
	private setupCoreWebVitals() {
		// SSR compatibility: Only run in browser environment
		if (typeof window === 'undefined') return;

		// Largest Contentful Paint
		if ('PerformanceObserver' in window) {
			try {
				const lcpObserver = new PerformanceObserver((list) => {
					const entries = list.getEntries();
					const lastEntry = entries[entries.length - 1];
					this.recordMetric({
						largestContentfulPaint: lastEntry.startTime
					});
				});
				lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] });
				this.observers.push(lcpObserver);
			} catch (_e) {
				logger.debug('LCP monitoring not supported');
			}

			// First Input Delay
			try {
				const fidObserver = new PerformanceObserver((list) => {
					const entries = list.getEntries();
					entries.forEach((entry) => {
						if (isPerformanceEventTiming(entry)) {
							this.recordMetric({
								firstInputDelay: entry.processingStart - entry.startTime
							});
						}
					});
				});
				fidObserver.observe({ entryTypes: ['first-input'] });
				this.observers.push(fidObserver);
			} catch (_e) {
				logger.debug('FID monitoring not supported');
			}

			// Cumulative Layout Shift
			try {
				const clsObserver = new PerformanceObserver((list) => {
					let clsValue = 0;
					const entries = list.getEntries();
					entries.forEach((entry) => {
						if (isLayoutShift(entry) && !entry.hadRecentInput) {
							clsValue += entry.value;
						}
					});
					this.recordMetric({
						cumulativeLayoutShift: clsValue
					});
				});
				clsObserver.observe({ entryTypes: ['layout-shift'] });
				this.observers.push(clsObserver);
			} catch (_e) {
				logger.debug('CLS monitoring not supported');
			}
		}
	}

	/**
	 * Setup memory usage monitoring using Performance.memory API.
	 * Includes SSR compatibility checks to prevent window access errors.
	 */
	private setupMemoryMonitoring() {
		// SSR compatibility: Only run in browser environment
		if (typeof window === 'undefined') return;

		// Monitor memory usage every 30 seconds
		setInterval(() => {
			const perfWithMemory = performance as PerformanceWithMemory;
			if (perfWithMemory.memory) {
				this.recordMetric({
					memoryUsage: perfWithMemory.memory.usedJSHeapSize
				});
			}
		}, 30000);
	}

	/**
	 * Setup FPS monitoring using requestAnimationFrame.
	 * Includes SSR compatibility checks to prevent window access errors.
	 */
	private setupFPSMonitoring() {
		// SSR compatibility: Only run in browser environment
		if (typeof window === 'undefined') return;

		let frameCount = 0;
		let lastTime = performance.now();

		const measureFPS = () => {
			frameCount++;
			const currentTime = performance.now();

			if (currentTime - lastTime >= 1000) {
				const fps = Math.round((frameCount * 1000) / (currentTime - lastTime));
				this.recordMetric({ fps });
				frameCount = 0;
				lastTime = currentTime;
			}

			if (this.isMonitoring) {
				requestAnimationFrame(measureFPS);
			}
		};

		requestAnimationFrame(measureFPS);
	}

	/**
	 * Setup network request monitoring to track fetch calls.
	 * Includes SSR compatibility checks to prevent window access errors.
	 */
	private setupNetworkMonitoring() {
		// SSR compatibility: Only run in browser environment
		if (typeof window === 'undefined') return;

		// Monitor fetch requests
		const originalFetch = window.fetch;
		let requestCount = 0;

		window.fetch = function(...args) {
			requestCount++;
			return originalFetch.apply(this, args);
		};

		// Record network requests every 10 seconds
		setInterval(() => {
			this.recordMetric({ networkRequests: requestCount });
			requestCount = 0; // Reset counter
		}, 10000);
	}

	private recordMetric(updates: Partial<PerformanceMetrics>) {
		const metric: PerformanceMetrics = {
			timestamp: Date.now(),
			renderTime: 0,
			networkRequests: 0,
			...updates
		};

		this.metrics.push(metric);

		// Keep only recent metrics
		if (this.metrics.length > this.maxMetricsHistory) {
			this.metrics = this.metrics.slice(-this.maxMetricsHistory);
		}
	}

	getMetrics(): PerformanceMetrics[] {
		return [...this.metrics];
	}

	getLatestMetrics(): PerformanceMetrics | null {
		return this.metrics[this.metrics.length - 1] || null;
	}

	getAverageMetric(key: keyof PerformanceMetrics): number | null {
		const values = this.metrics
			.map(m => m[key])
			.filter(v => typeof v === 'number') as number[];

		if (values.length === 0) return null;

		return values.reduce((sum, val) => sum + val, 0) / values.length;
	}

	clearMetrics() {
		this.metrics = [];
	}

	// Performance measurement utilities
	startTiming(label: string): () => number {
		const startTime = performance.now();
		return () => {
			const endTime = performance.now();
			const duration = endTime - startTime;
			logger.debug(`Performance measurement: ${label} took ${duration.toFixed(2)}ms`);
			return duration;
		};
	}

	measureFunction<T>(label: string, fn: () => T): T {
		const endTiming = this.startTiming(label);
		try {
			return fn();
		} finally {
			endTiming();
		}
	}

	// Memory usage helpers
	getMemoryUsage(): { used: number; total: number; limit: number } | null {
		const perfWithMemory = performance as PerformanceWithMemory;
		if (perfWithMemory.memory) {
			return {
				used: perfWithMemory.memory.usedJSHeapSize,
				total: perfWithMemory.memory.totalJSHeapSize,
				limit: perfWithMemory.memory.jsHeapSizeLimit
			};
		}
		return null;
	}

	// Performance warnings
	checkPerformanceHealth(): string[] {
		const warnings: string[] = [];
		const latest = this.getLatestMetrics();

		if (latest) {
			// Check FPS
			if (latest.fps && latest.fps < 30) {
				warnings.push(`Low FPS detected: ${latest.fps}`);
			}

			// Check memory usage
			const memory = this.getMemoryUsage();
			if (memory && memory.used > memory.limit * 0.8) {
				warnings.push(`High memory usage: ${(memory.used / memory.limit * 100).toFixed(1)}%`);
			}

			// Check Core Web Vitals
			if (latest.largestContentfulPaint && latest.largestContentfulPaint > 2500) {
				warnings.push(`Slow LCP: ${latest.largestContentfulPaint.toFixed(0)}ms`);
			}

			if (latest.firstInputDelay && latest.firstInputDelay > 100) {
				warnings.push(`High FID: ${latest.firstInputDelay.toFixed(0)}ms`);
			}

			if (latest.cumulativeLayoutShift && latest.cumulativeLayoutShift > 0.1) {
				warnings.push(`Layout shift detected: ${latest.cumulativeLayoutShift.toFixed(3)}`);
			}
		}

		return warnings;
	}
}

// Singleton instance
export const performanceMonitor = new PerformanceMonitor();

// Auto-start monitoring in development (SSR-safe)
// Only starts in browser environment to prevent window access errors during SSR
if (import.meta.env.DEV && typeof window !== 'undefined') {
	performanceMonitor.startMonitoring();
}

// Export for convenience
export { PerformanceMonitor };
export type { PerformanceMetrics };
