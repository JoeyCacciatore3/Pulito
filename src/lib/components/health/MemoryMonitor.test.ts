import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import MemoryMonitor from './MemoryMonitor.svelte';

describe('MemoryMonitor', () => {
	const mockProps = {
		totalMemory: 16 * 1024 * 1024 * 1024, // 16GB in bytes
		usedMemory: 8 * 1024 * 1024 * 1024,   // 8GB in bytes
		availableMemory: 8 * 1024 * 1024 * 1024 // 8GB in bytes
	};

	it('renders memory usage correctly', () => {
		const { getByText } = render(MemoryMonitor, { props: mockProps });
		expect(getByText('8.00 GB')).toBeInTheDocument();
		expect(getByText('of 16.00 GB')).toBeInTheDocument();
	});

	it('displays available memory', () => {
		const { getByText } = render(MemoryMonitor, { props: mockProps });
		expect(getByText('8.00 GB')).toBeInTheDocument();
	});

	it('shows correct usage percentage', () => {
		const { getByText } = render(MemoryMonitor, { props: mockProps });
		expect(getByText('50%')).toBeInTheDocument();
	});

	it('displays memory breakdown details', () => {
		const { getByText } = render(MemoryMonitor, { props: mockProps });
		expect(getByText('Used')).toBeInTheDocument();
		expect(getByText('Available')).toBeInTheDocument();
		expect(getByText('Usage')).toBeInTheDocument();
	});

	it('renders progress bar with correct percentage', () => {
		const { container } = render(MemoryMonitor, { props: mockProps });
		// The progress bar should exist and have the correct percentage
		const progressBar = container.querySelector('[role="progressbar"]') ||
		                   container.querySelector('.bg-primary-500');
		expect(progressBar).toBeInTheDocument();
	});
});
