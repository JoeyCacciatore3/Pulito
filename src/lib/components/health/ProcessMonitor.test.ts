import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import ProcessMonitor from './ProcessMonitor.svelte';

describe('ProcessMonitor', () => {
	const mockProcesses = [
		{
			pid: 1234,
			name: 'firefox',
			cpu_usage: 15.5,
			memory_usage: 500 * 1024 * 1024, // 500MB
			status: 'running',
			user_id: 1000
		},
		{
			pid: 5678,
			name: 'chrome',
			cpu_usage: 8.2,
			memory_usage: 800 * 1024 * 1024, // 800MB
			status: 'running',
			user_id: 1000
		}
	];

	const mockProps = {
		topProcesses: mockProcesses
	};

	it('renders process names correctly', () => {
		const { getByText } = render(ProcessMonitor, { props: mockProps });
		expect(getByText('firefox')).toBeInTheDocument();
		expect(getByText('chrome')).toBeInTheDocument();
	});

	it('displays CPU usage for each process', () => {
		const { getByText } = render(ProcessMonitor, { props: mockProps });
		expect(getByText('15.5%')).toBeInTheDocument();
		expect(getByText('8.2%')).toBeInTheDocument();
	});

	it('shows process IDs', () => {
		const { getByText } = render(ProcessMonitor, { props: mockProps });
		expect(getByText('PID: 1234')).toBeInTheDocument();
		expect(getByText('PID: 5678')).toBeInTheDocument();
	});

	it('displays memory usage', () => {
		const { getByText } = render(ProcessMonitor, { props: mockProps });
		expect(getByText('500.00 MB')).toBeInTheDocument();
		expect(getByText('800.00 MB')).toBeInTheDocument();
	});

	it('limits display to top 5 processes', () => {
		const manyProcesses = Array.from({ length: 10 }, (_, i) => ({
			pid: 1000 + i,
			name: `process${i}`,
			cpu_usage: 5 + i,
			memory_usage: 100 * 1024 * 1024,
			status: 'running',
			user_id: 1000
		}));

		const { queryByText } = render(ProcessMonitor, {
			props: { topProcesses: manyProcesses }
		});

		// Should show first 5
		expect(queryByText('process0')).toBeInTheDocument();
		expect(queryByText('process4')).toBeInTheDocument();

		// Should not show beyond 5
		expect(queryByText('process5')).not.toBeInTheDocument();
	});

	it('handles empty process list', () => {
		const { container } = render(ProcessMonitor, {
			props: { topProcesses: [] }
		});
		// Should render without crashing
		expect(container).toBeInTheDocument();
	});
});
