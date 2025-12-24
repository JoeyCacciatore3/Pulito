import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, waitFor } from '@testing-library/svelte';
import SystemHealthMonitor from './SystemHealthMonitor.svelte';

// Mock the Tauri invoke function
vi.mock('$lib/utils/tauri', () => ({
	invoke: vi.fn(),
	formatBytes: (bytes: number) => `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`
}));

// Mock the logger
vi.mock('$lib/utils/logger', () => ({
	logger: {
		debug: vi.fn(),
		error: vi.fn()
	}
}));

// Mock the notification store
vi.mock('$lib/stores/notifications.svelte', () => ({
	notificationStore: {
		warning: vi.fn()
	}
}));

// Mock svelte's onMount to prevent lifecycle errors in tests
vi.mock('svelte', async () => {
	const actual = await vi.importActual('svelte');
	return {
		...actual,
		onMount: vi.fn(() => {
			// Return a cleanup function
			return () => {};
		})
	};
});

import { invoke } from '$lib/utils/tauri';

describe('SystemHealthMonitor', () => {
	const mockHealthData = {
		cpu_usage: 45.5,
		cpu_cores: 8,
		cpu_frequency: 2500,
		core_usages: [20, 30, 50, 40, 25, 35, 45, 55],
		total_memory: 16 * 1024 * 1024 * 1024,
		used_memory: 8 * 1024 * 1024 * 1024,
		available_memory: 8 * 1024 * 1024 * 1024,
		gpu_info: null,
		network_up: 1024 * 1024, // 1MB/s
		network_down: 2 * 1024 * 1024, // 2MB/s
		network_interfaces: [],
		active_connections: [],
		temperatures: {
			cpu: 65,
			cpu_sensors: 67,
			system: 55,
			gpu: null
		},
		disk_read_bytes: 1024 * 1024,
		disk_write_bytes: 512 * 1024,
		disk_read_ops: 100,
		disk_write_ops: 50,
		battery_info: null,
		top_processes: [
			{
				pid: 1234,
				name: 'firefox',
				cpu_usage: 15.5,
				memory_usage: 500 * 1024 * 1024,
				status: 'running',
				user_id: 1000
			}
		],
		load_average: {
			one_minute: 1.5,
			five_minutes: 1.2,
			fifteen_minutes: 1.0
		},
		swap_total: 4 * 1024 * 1024 * 1024,
		swap_used: 1 * 1024 * 1024 * 1024,
		timestamp: Date.now()
	};

	beforeEach(() => {
		vi.clearAllMocks();
		vi.mocked(invoke).mockResolvedValue(mockHealthData);
	});

	it('renders loading state initially', () => {
		const { getByText } = render(SystemHealthMonitor);
		expect(getByText('Loading system health data...')).toBeInTheDocument();
	});

	it('renders health data after loading', async () => {
		const { getByText } = render(SystemHealthMonitor);

		await waitFor(() => {
			expect(getByText('System Health Monitor')).toBeInTheDocument();
		});

		// Check that CPU monitor is rendered
		expect(getByText('CPU')).toBeInTheDocument();
		expect(getByText('45.5%')).toBeInTheDocument();
	});

	it('calls get_system_health on mount', async () => {
		render(SystemHealthMonitor);

		await waitFor(() => {
			expect(invoke).toHaveBeenCalledWith('get_system_health');
		});
	});

	it('shows monitoring status', async () => {
		const { getByText } = render(SystemHealthMonitor);

		await waitFor(() => {
			expect(getByText(/Live • Updates:/)).toBeInTheDocument();
		});
	});

	it('renders all monitor components', async () => {
		const { getByText } = render(SystemHealthMonitor);

		await waitFor(() => {
			expect(getByText('CPU')).toBeInTheDocument();
			expect(getByText('Memory (RAM)')).toBeInTheDocument();
			expect(getByText('Network')).toBeInTheDocument();
			expect(getByText('Temperatures')).toBeInTheDocument();
			expect(getByText('Top Processes')).toBeInTheDocument();
		});
	});

	it('handles monitoring pause/resume', async () => {
		const { getByText } = render(SystemHealthMonitor);

		await waitFor(() => {
			expect(getByText(/▶️ Resume/)).toBeInTheDocument();
		});
	});

	it('shows performance history section', async () => {
		const { getByText } = render(SystemHealthMonitor);

		await waitFor(() => {
			expect(getByText('Performance History')).toBeInTheDocument();
			expect(getByText('Historical performance charts coming soon')).toBeInTheDocument();
		});
	});
});
