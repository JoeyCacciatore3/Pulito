/**
 * Comprehensive monitoring and alerting service for Pulito
 * Monitors system health, performance, and provides intelligent alerts
 */

import { invoke } from '$lib/utils/tauri';
import { notificationStore } from '$lib/stores/notifications.svelte';
import { performanceMonitor } from '$lib/utils/performance';
import { logger } from '$lib/utils/logger';
import type { SystemHealthData } from '$lib/generated/types';

export interface SystemAlert {
	id: string;
	type: 'critical' | 'warning' | 'info';
	title: string;
	message: string;
	timestamp: number;
	source: string;
	acknowledged: boolean;
	autoResolve?: boolean;
	resolvedAt?: number;
}

export interface MonitoringConfig {
	enabled: boolean;
	alertsEnabled: boolean;
	performanceMonitoring: boolean;
	systemHealthChecks: boolean;
	diskSpaceThreshold: number; // percentage
	memoryThreshold: number; // percentage
	cpuThreshold: number; // percentage
	temperatureThreshold: number; // celsius
	checkInterval: number; // seconds
}

/**
 * System monitoring service for real-time health tracking.
 * NOTE: Uses regular TypeScript arrays instead of Svelte $state runes
 * because this service runs in Node.js context where Svelte runes are unavailable.
 */
class MonitoringService {
	private config: MonitoringConfig;
	/** System alerts queue - regular array, not $state rune (SSR compatibility) */
	private alerts: SystemAlert[] = [];
	private checkInterval: ReturnType<typeof setInterval> | null = null;
	private isMonitoring = false;
	private lastHealthCheck: any = null;

	constructor() {
		this.config = {
			enabled: true,
			alertsEnabled: true,
			performanceMonitoring: true,
			systemHealthChecks: true,
			diskSpaceThreshold: 85, // Alert when disk usage > 85%
			memoryThreshold: 90, // Alert when memory usage > 90%
			cpuThreshold: 95, // Alert when CPU usage > 95%
			temperatureThreshold: 100, // Alert when temperature > 100°C (critical threshold for modern CPUs)
			checkInterval: 60 // Check every 60 seconds
		};
	}

	startMonitoring() {
		if (this.isMonitoring) return;

		this.isMonitoring = true;
		logger.info('Starting comprehensive monitoring service', { service: 'MonitoringService' });

		// Start periodic health checks
		this.checkInterval = setInterval(() => {
			this.performHealthCheck();
		}, this.config.checkInterval * 1000);

		// Start performance monitoring
		if (this.config.performanceMonitoring) {
			performanceMonitor.startMonitoring();
		}

		// Perform initial health check
		this.performHealthCheck();
	}

	stopMonitoring() {
		if (!this.isMonitoring) return;

		this.isMonitoring = false;

		if (this.checkInterval) {
			clearInterval(this.checkInterval);
			this.checkInterval = null;
		}

		if (this.config.performanceMonitoring) {
			performanceMonitor.stopMonitoring();
		}

		logger.info('Stopped monitoring service', { service: 'MonitoringService' });
	}

	private async performHealthCheck() {
		try {
			// Check if Tauri is available before attempting invoke
			if (typeof invoke === 'undefined') {
				logger.debug('Tauri not available, skipping health check');
				return;
			}

			// Get current system health
			const healthData = await invoke<SystemHealthData>('get_system_health');

			// Check for alerts
			this.checkSystemHealth(healthData);
			this.checkPerformanceMetrics();

			// Update last health check
			this.lastHealthCheck = healthData;

		} catch (error) {
			logger.error('Health check failed', { service: 'MonitoringService', action: 'health_check' }, error);
			this.createAlert({
				type: 'warning',
				title: 'Health Check Failed',
				message: 'Unable to perform system health monitoring. Some alerts may be unavailable.',
				source: 'monitoring-service'
			});
		}
	}

	private checkSystemHealth(healthData: any) {
		const { cpu_usage, total_memory, used_memory, temperatures } = healthData;

		// CPU usage alert
		if (cpu_usage > this.config.cpuThreshold) {
			this.createAlert({
				type: 'warning',
				title: 'High CPU Usage',
				message: `CPU usage is at ${cpu_usage.toFixed(1)}%. This may impact system performance.`,
				source: 'cpu-monitor',
				autoResolve: true
			});
		}

		// Memory usage alert
		const memoryUsagePercent = (used_memory / total_memory) * 100;
		if (memoryUsagePercent > this.config.memoryThreshold) {
			this.createAlert({
				type: 'warning',
				title: 'High Memory Usage',
				message: `Memory usage is at ${memoryUsagePercent.toFixed(1)}%. Consider freeing up memory.`,
				source: 'memory-monitor',
				autoResolve: true
			});
		}

		// Temperature alerts (using primary CPU temperature - lm-sensors preferred, thermal zone fallback)
		// Modern CPUs (especially laptops) safely operate up to 90-95°C under load
		// Only alert at 100°C+ which is the critical thermal throttling threshold
		if (temperatures.cpu >= 100) {
			// Critical: At or above thermal throttling threshold
			this.createAlert({
				type: 'critical',
				title: 'Critical CPU Temperature',
				message: `CPU temperature is ${temperatures.cpu.toFixed(1)}°C. System is at thermal limit and may throttle performance. Check cooling system immediately.`,
				source: 'temperature-monitor',
				autoResolve: true
			});
		} else if (temperatures.cpu >= 95) {
			// Warning: Approaching thermal limit
			this.createAlert({
				type: 'warning',
				title: 'High CPU Temperature',
				message: `CPU temperature is ${temperatures.cpu.toFixed(1)}°C. Approaching thermal limit. Monitor closely and ensure adequate cooling.`,
				source: 'temperature-monitor',
				autoResolve: true
			});
		}

		// GPU temperature alert (NVIDIA GPUs typically throttle at 83°C, critical at 90°C+)
		if (temperatures.gpu) {
			if (temperatures.gpu >= 90) {
				this.createAlert({
					type: 'critical',
					title: 'Critical GPU Temperature',
					message: `GPU temperature is ${temperatures.gpu.toFixed(1)}°C. At or above thermal limit. Performance may be throttled.`,
					source: 'temperature-monitor',
					autoResolve: true
				});
			} else if (temperatures.gpu >= 83) {
				this.createAlert({
					type: 'warning',
					title: 'High GPU Temperature',
					message: `GPU temperature is ${temperatures.gpu.toFixed(1)}°C. Approaching thermal throttling threshold.`,
					source: 'temperature-monitor',
					autoResolve: true
				});
			}
		}
	}

	private checkPerformanceMetrics() {
		const warnings = performanceMonitor.checkPerformanceHealth();

		warnings.forEach(warning => {
			this.createAlert({
				type: 'info',
				title: 'Performance Issue Detected',
				message: warning,
				source: 'performance-monitor',
				autoResolve: false
			});
		});
	}

	private createAlert(alertData: Omit<SystemAlert, 'id' | 'timestamp' | 'acknowledged'>) {
		// Check if similar alert already exists and is not resolved
		const existingAlert = this.alerts.find(alert =>
			alert.title === alertData.title &&
			alert.source === alertData.source &&
			!alert.acknowledged &&
			!alert.resolvedAt
		);

		if (existingAlert) {
			// Update timestamp of existing alert
			existingAlert.timestamp = Date.now();
			return existingAlert.id;
		}

		// Create new alert
		const alert: SystemAlert = {
			id: `alert_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
			timestamp: Date.now(),
			acknowledged: false,
			...alertData
		};

		this.alerts.push(alert);

		// Send notification if alerts are enabled
		if (this.config.alertsEnabled) {
			this.sendAlertNotification(alert);
		}

		logger.warn('System alert created', {
			service: 'MonitoringService',
			alertId: alert.id,
			alertType: alert.type,
			alertTitle: alert.title
		});

		return alert.id;
	}

	private sendAlertNotification(alert: SystemAlert) {
		const notificationType = alert.type === 'critical' ? 'error' :
		                        alert.type === 'warning' ? 'warning' : 'info';

		notificationStore[notificationType](alert.title, alert.message);
	}

	// Public API methods
	getAlerts(): SystemAlert[] {
		return [...this.alerts];
	}

	getActiveAlerts(): SystemAlert[] {
		return this.alerts.filter(alert => !alert.acknowledged && !alert.resolvedAt);
	}

	acknowledgeAlert(alertId: string) {
		const alert = this.alerts.find(a => a.id === alertId);
		if (alert) {
			alert.acknowledged = true;
			logger.info('Alert acknowledged', { service: 'MonitoringService', alertId });
		}
	}

	resolveAlert(alertId: string) {
		const alert = this.alerts.find(a => a.id === alertId);
		if (alert) {
			alert.resolvedAt = Date.now();
			logger.info('Alert resolved', { service: 'MonitoringService', alertId });
		}
	}

	clearResolvedAlerts() {
		this.alerts = this.alerts.filter(alert => !alert.resolvedAt);
	}

	updateConfig(newConfig: Partial<MonitoringConfig>) {
		this.config = { ...this.config, ...newConfig };
		logger.info('Monitoring configuration updated', { service: 'MonitoringService', config: newConfig });
	}

	getConfig(): MonitoringConfig {
		return { ...this.config };
	}

	getHealthStatus() {
		return {
			isMonitoring: this.isMonitoring,
			lastHealthCheck: this.lastHealthCheck,
			activeAlerts: this.getActiveAlerts().length,
			totalAlerts: this.alerts.length,
			performanceMetrics: performanceMonitor.getLatestMetrics()
		};
	}
}

// Singleton instance
export const monitoringService = new MonitoringService();

// Auto-start monitoring only when Tauri is available (not during SSR)
if (typeof window !== 'undefined' && !import.meta.env.SSR) {
	// Check if Tauri is available by checking for the invoke function
	if (typeof invoke !== 'undefined') {
		monitoringService.startMonitoring();
	}
}
