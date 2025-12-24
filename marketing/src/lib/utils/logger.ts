/**
 * Structured logging utility for Pulito
 * Provides consistent logging with levels and production filtering
 *
 * Log format: [ISO_TIMESTAMP] [LEVEL] [CONTEXT] message {...args}
 *
 * Usage:
 *   logger.debug('Component initialized', { component: 'Dashboard' });
 *   logger.info('User action', { action: 'scan_started' });
 *   logger.warn('Potential issue', { path: '/some/path' });
 *   logger.error('Operation failed', { error: e, operation: 'cleanup' });
 */

const LogLevelValues = {
	DEBUG: 0,
	INFO: 1,
	WARN: 2,
	ERROR: 3
} as const;

type LogLevel = typeof LogLevelValues[keyof typeof LogLevelValues];

interface LogContext {
	component?: string;
	action?: string;
	operation?: string;
	[key: string]: unknown;
}

class Logger {
	private minLevel: LogLevel;
	private isDevelopment: boolean;

	constructor() {
		this.isDevelopment = import.meta.env.DEV || import.meta.env.MODE === 'development';

		// Allow log level override via environment variable
		const envLevel = import.meta.env.VITE_LOG_LEVEL?.toUpperCase();
		if (envLevel === 'DEBUG') {
			this.minLevel = LogLevelValues.DEBUG;
		} else if (envLevel === 'INFO') {
			this.minLevel = LogLevelValues.INFO;
		} else if (envLevel === 'WARN') {
			this.minLevel = LogLevelValues.WARN;
		} else if (envLevel === 'ERROR') {
			this.minLevel = LogLevelValues.ERROR;
		} else {
			// In production, only show WARN and ERROR
			this.minLevel = this.isDevelopment ? LogLevelValues.DEBUG : LogLevelValues.WARN;
		}
	}

	private shouldLog(level: LogLevel): boolean {
		return level >= this.minLevel;
	}

	private formatContext(context?: LogContext): string {
		if (!context || Object.keys(context).length === 0) {
			return '';
		}

		const parts: string[] = [];
		if (context.component) parts.push(`component=${context.component}`);
		if (context.action) parts.push(`action=${context.action}`);
		if (context.operation) parts.push(`operation=${context.operation}`);

		// Add any additional context fields
		Object.entries(context).forEach(([key, value]) => {
			if (!['component', 'action', 'operation'].includes(key)) {
				// Format value for logging (handle objects, errors, etc.)
				const formattedValue = value instanceof Error
					? value.message
					: typeof value === 'object'
						? JSON.stringify(value)
						: String(value);
				parts.push(`${key}=${formattedValue}`);
			}
		});

		return parts.length > 0 ? `[${parts.join(' ')}]` : '';
	}

	private formatMessage(level: string, message: string, context?: LogContext, ...args: unknown[]): void {
		const timestamp = new Date().toISOString();
		const contextStr = this.formatContext(context);
		const prefix = `[${timestamp}] [${level}]${contextStr ? ` ${contextStr}` : ''}`;

		// Use appropriate console method based on level
		const consoleMethod = level === 'ERROR' ? console.error
			: level === 'WARN' ? console.warn
			: console.log;

		if (args.length > 0) {
			consoleMethod(prefix, message, ...args);
		} else {
			consoleMethod(prefix, message);
		}
	}

	/**
	 * Log debug information (development only)
	 * @param message - Log message
	 * @param context - Optional context object with component/action/operation fields
	 * @param args - Additional arguments to log (e.g., Error objects)
	 */
	debug(message: string, context?: LogContext, ...args: unknown[]): void {
		if (this.shouldLog(LogLevelValues.DEBUG)) {
			this.formatMessage('DEBUG', message, context, ...args);
		}
	}

	/**
	 * Log informational messages
	 * @param message - Log message
	 * @param context - Optional context object with component/action/operation fields
	 * @param args - Additional arguments to log
	 */
	info(message: string, context?: LogContext, ...args: unknown[]): void {
		if (this.shouldLog(LogLevelValues.INFO)) {
			this.formatMessage('INFO', message, context, ...args);
		}
	}

	/**
	 * Log warning messages
	 * @param message - Log message
	 * @param context - Optional context object with component/action/operation fields
	 * @param args - Additional arguments to log
	 */
	warn(message: string, context?: LogContext, ...args: unknown[]): void {
		if (this.shouldLog(LogLevelValues.WARN)) {
			this.formatMessage('WARN', message, context, ...args);
		}
	}

	/**
	 * Log error messages
	 * @param message - Log message
	 * @param context - Optional context object with component/action/operation fields
	 * @param args - Additional arguments (often an Error object)
	 */
	error(message: string, context?: LogContext, ...args: unknown[]): void {
		if (this.shouldLog(LogLevelValues.ERROR)) {
			this.formatMessage('ERROR', message, context, ...args);
		}
	}
}

// Export singleton instance
export const logger = new Logger();
