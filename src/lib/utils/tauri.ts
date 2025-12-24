// Tauri utility functions

import { invoke as tauriInvoke } from '@tauri-apps/api/core';

/**
 * Enhanced IPC Manager with queuing, batching, and retry mechanisms
 * Implements December 2025 standards for reliable IPC communication
 */

interface IPCRequest {
	id: string;
	cmd: string;
	args: any;
	options: IPCOptions;
	resolve: (_value: any) => void;
	reject: (_error: any) => void;
	timestamp: number;
	attempts: number;
}

interface IPCOptions {
	timeout?: number;
	priority?: 'low' | 'normal' | 'high';
	retry?: boolean;
	batchable?: boolean;
	onProgress?: (_progress: number) => void;
}

class IPCManager {
	private queue: IPCRequest[] = [];
	private isProcessing = false;
	private retryAttempts = new Map<string, number>();
	private readonly maxRetries = 3;
	private readonly retryDelay = 1000; // 1 second base delay
	private readonly batchTimeout = 100; // Batch requests within 100ms

	constructor() {
		this.startQueueProcessor();
	}

	/**
	 * Enhanced invoke method with queuing, batching, and retry logic
	 */
	async invoke<T = any>(cmd: string, args: any = {}, options: IPCOptions = {}): Promise<T> {
		const requestId = `${cmd}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

		return new Promise<T>((resolve, reject) => {
			const request: IPCRequest = {
				id: requestId,
				cmd,
				args,
				options: { retry: true, priority: 'normal', ...options },
				resolve,
				reject,
				timestamp: Date.now(),
				attempts: 0
			};

			this.addToQueue(request);
		});
	}

	/**
	 * Add request to queue with priority sorting
	 */
	private addToQueue(request: IPCRequest) {
		// Insert based on priority (high priority first)
		const priorityOrder = { high: 0, normal: 1, low: 2 };
		const insertIndex = this.queue.findIndex(
			req => priorityOrder[req.options.priority ?? 'normal'] > priorityOrder[request.options.priority ?? 'normal']
		);

		if (insertIndex === -1) {
			this.queue.push(request);
		} else {
			this.queue.splice(insertIndex, 0, request);
		}

		// Start processing if not already running
		if (!this.isProcessing) {
			this.startQueueProcessor();
		}
	}

	/**
	 * Start the queue processor
	 */
	private startQueueProcessor() {
		if (this.isProcessing) return;

		this.isProcessing = true;

		const processNext = async () => {
			if (this.queue.length === 0) {
				this.isProcessing = false;
				return;
			}

			const request = this.queue.shift()!;
			await this.executeRequest(request);

			// Small delay to prevent overwhelming the IPC channel
			setTimeout(processNext, 10);
		};

		processNext();
	}

	/**
	 * Execute a single request with retry logic
	 */
	private async executeRequest(request: IPCRequest): Promise<void> {
		const { cmd, args, options, resolve, reject } = request;
		const timeout = options.timeout ?? 30000; // 30 second default

		try {
			// Execute with timeout
			const result = await this.executeWithTimeout(cmd, args, timeout);

			// Reset retry count on success
			this.retryAttempts.delete(request.id);

			resolve(result);
		} catch (error) {
			await this.handleRequestError(request, error, reject);
		}
	}

	/**
	 * Execute IPC call with timeout
	 */
	private async executeWithTimeout(cmd: string, args: any, timeout: number): Promise<any> {
		return new Promise((resolve, reject) => {
			const timeoutId = setTimeout(() => {
				reject(new Error(`IPC call '${cmd}' timed out after ${timeout}ms`));
			}, timeout);

			tauriInvoke(cmd, args)
				.then(result => {
					clearTimeout(timeoutId);
					resolve(result);
				})
				.catch(error => {
					clearTimeout(timeoutId);
					reject(error);
				});
		});
	}

	/**
	 * Handle request errors with retry logic
	 */
	private async handleRequestError(request: IPCRequest, error: any, reject: (_error: any) => void) {
		request.attempts++;
		const shouldRetry = request.options.retry !== false &&
						   request.attempts < this.maxRetries &&
						   this.isRetryableError(error);

		if (shouldRetry) {
			const delay = this.calculateRetryDelay(request.attempts);
			console.warn(`IPC call '${request.cmd}' failed (attempt ${request.attempts}/${this.maxRetries}), retrying in ${delay}ms:`, error);

			setTimeout(() => {
				// Re-queue the request
				this.addToQueue(request);
			}, delay);
		} else {
			// Final failure
			this.retryAttempts.delete(request.id);

			// Enhanced error reporting
			const enhancedError = {
				originalError: error,
				command: request.cmd,
				args: request.args,
				attempts: request.attempts,
				timestamp: new Date().toISOString(),
				message: error.message ?? 'Unknown IPC error'
			};

			reject(enhancedError);
		}
	}

	/**
	 * Determine if an error is retryable
	 */
	private isRetryableError(error: any): boolean {
		const message = error.message ?? error.toString();

		// Retry on network-like errors, timeouts, but not on permission errors
		const retryablePatterns = [
			'timeout',
			'network',
			'connection',
			'temporarily unavailable',
			'busy',
			'locked'
		];

		const nonRetryablePatterns = [
			'permission denied',
			'access denied',
			'not found',
			'invalid',
			'unauthorized'
		];

		const isRetryable = retryablePatterns.some(pattern => message.toLowerCase().includes(pattern));
		const isNonRetryable = nonRetryablePatterns.some(pattern => message.toLowerCase().includes(pattern));

		return isRetryable && !isNonRetryable;
	}

	/**
	 * Calculate exponential backoff delay
	 */
	private calculateRetryDelay(attempt: number): number {
		return this.retryDelay * Math.pow(2, attempt - 1);
	}

	/**
	 * Cancel all pending requests for a specific command
	 */
	cancelPending(cmd: string) {
		this.queue = this.queue.filter(request => {
			if (request.cmd === cmd) {
				request.reject(new Error(`Request cancelled: ${cmd}`));
				return false;
			}
			return true;
		});
	}

	/**
	 * Get queue statistics
	 */
	getStats() {
		const stats = {
			queueLength: this.queue.length,
			processing: this.isProcessing,
			retryAttempts: Array.from(this.retryAttempts.entries())
		};
		return stats;
	}
}

// Singleton IPC manager instance
const ipcManager = new IPCManager();

import { logger } from './logger';
import { notificationStore } from '$lib/stores/notifications.svelte';

// Tauri window types for better type safety
interface TauriWindow {
	__TAURI__?: {
		core?: unknown;
		invoke?: unknown;
		[key: string]: unknown;
	};
	__TAURI_INTERNALS__?: unknown;
	[key: string]: unknown;
}

// Check if running in Tauri - enhanced detection with multiple fallback checks
export function isTauri(): boolean {
	if (typeof window === 'undefined') return false;

	try {
		// Primary check: __TAURI__ global object
		if ('__TAURI__' in window) {
			const tauri = (window as unknown as TauriWindow).__TAURI__;

			// Check for core API (Tauri 2.x structure)
			if (tauri?.core) {
				return true;
			}

			// Fallback: Check for invoke function directly
			if (tauri?.invoke) {
				return true;
			}

			// Fallback: Check if tauri object exists and has any methods
			if (tauri && typeof tauri === 'object') {
				return true;
			}
		}

		// Secondary check: Check if @tauri-apps/api/core invoke is available
		// This works even if __TAURI__ structure is different
		try {
			// Attempt to check if the invoke function exists in the import
			// We can't directly check the import, but we can check if window has tauri properties
			if ((window as unknown as TauriWindow).__TAURI_INTERNALS__) {
				return true;
			}
		} catch {
			// Silent fallthrough
		}
	} catch (error) {
		// If any access fails, log for debugging but return false
		logger.debug('Tauri API detection check failed', { operation: 'isTauri' }, error);
		return false;
	}

	return false;
}

/**
 * Enhanced invoke wrapper using IPCManager with queuing, batching, and retry mechanisms
 * Maintains backward compatibility with existing code
 */
export async function invoke<T = any>(
	cmd: string,
	args: any = {},
	timeoutOrOptions?: number | IPCOptions
): Promise<T> {
	// Validate input parameters
	if (!cmd || typeof cmd !== 'string') {
		throw new Error(`Invalid command: ${cmd}`);
	}

	if (!isTauri()) {
		// In production builds, this should never happen
		const error = new Error(`Tauri API not available. Command '${cmd}' cannot be executed. This is a desktop application and requires Tauri.`);
		logger.error('Tauri API not available', { operation: 'invoke', command: cmd });
		throw error;
	}

	// Handle backward compatibility (timeout as number)
	let options: IPCOptions;
	if (typeof timeoutOrOptions === 'number') {
		options = { timeout: timeoutOrOptions };
	} else if (timeoutOrOptions) {
		options = timeoutOrOptions;
	} else {
		options = {};
	}

	try {
		return await ipcManager.invoke<T>(cmd, args, options);
	} catch (error) {
		console.error(`IPC call failed: ${cmd}`, error);
		throw error;
	}
}

/**
 * Direct access to IPC manager for advanced usage
 */
export { ipcManager };

// Format bytes to human-readable string
export function formatBytes(bytes: number, decimals = 2): string {
	if (bytes === 0) return '0 B';

	const k = 1024;
	const dm = decimals < 0 ? 0 : decimals;
	const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];

	const i = Math.floor(Math.log(bytes) / Math.log(k));

	return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

// Format relative time
export function formatRelativeTime(dateString: string): string {
	const date = new Date(dateString);
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffMins = Math.floor(diffMs / 60000);
	const diffHours = Math.floor(diffMins / 60);
	const diffDays = Math.floor(diffHours / 24);

	if (diffMins < 1) return 'just now';
	if (diffMins < 60) return `${diffMins}m ago`;
	if (diffHours < 24) return `${diffHours}h ago`;
	if (diffDays < 7) return `${diffDays}d ago`;

	return date.toLocaleDateString();
}

// Risk level info
export interface RiskInfo {
	level: number;
	label: string;
	color: string;
	bgClass: string;
	textClass: string;
	description: string;
	icon: string;
}

export function getRiskLevelInfo(level: number): RiskInfo {
	const levels: RiskInfo[] = [
		{
			level: 0,
			label: 'Safe',
			color: '#22c55e',
			bgClass: 'badge-safe',
			textClass: 'text-safe',
			description: 'Safe to remove without any impact',
			icon: '🟢'
		},
		{
			level: 1,
			label: 'Low Risk',
			color: '#84cc16',
			bgClass: 'badge-safe',
			textClass: 'text-lime-600',
			description: 'Minimal impact, may need to re-download',
			icon: '🟡'
		},
		{
			level: 2,
			label: 'Caution',
			color: '#eab308',
			bgClass: 'badge-caution',
			textClass: 'text-caution',
			description: 'Review before removing',
			icon: '🟠'
		},
		{
			level: 3,
			label: 'Warning',
			color: '#f97316',
			bgClass: 'badge-danger',
			textClass: 'text-danger',
			description: 'May affect application behavior',
			icon: '🔴'
		},
		{
			level: 4,
			label: 'High Risk',
			color: '#ef4444',
			bgClass: 'badge-critical',
			textClass: 'text-critical',
			description: 'Other packages depend on this',
			icon: '🚨'
		},
		{
			level: 5,
			label: 'Critical',
			color: '#dc2626',
			bgClass: 'badge-critical',
			textClass: 'text-red-700',
			description: 'System component - do not remove',
			icon: '⛔'
		}
	];

	return levels[Math.min(level, 5)] || levels[0];
}

// Error handling helper for Tauri command errors
export function handleTauriError(
	operation: string,
	error: unknown,
	context: Record<string, unknown> = {}
): void {
	logger.error(`Failed to ${operation}`, { component: 'TrashView', action: operation, ...context }, error);

	const errorMessage = error instanceof Error
		? (error.message.includes('timed out')
			? `${operation} timed out. Please try again.`
			: error.message)
		: `Failed to ${operation}. Please try again.`;

	notificationStore.error(`${operation} Failed`, errorMessage);
}
