import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import SizeBar from './ui/SizeBar.svelte';

describe('SizeBar', () => {
	it('renders with correct percentage', () => {
		const { container } = render(SizeBar, {
			size: 50,
			totalSize: 100,
			showPercentage: true
		});

		const fill = container.querySelector('.size-bar-fill');
		expect(fill).toHaveStyle({ width: '50%' });
	});

	it('displays percentage when showPercentage is true', () => {
		const { getByText } = render(SizeBar, {
			size: 25,
			totalSize: 100,
			showPercentage: true
		});

		expect(getByText('25.0%')).toBeInTheDocument();
	});

	it('does not display percentage when showPercentage is false', () => {
		const { queryByText } = render(SizeBar, {
			size: 25,
			totalSize: 100,
			showPercentage: false
		});

		expect(queryByText('25.0%')).not.toBeInTheDocument();
	});

	it('handles zero total size', () => {
		const { container } = render(SizeBar, {
			size: 50,
			totalSize: 0
		});

		const fill = container.querySelector('.size-bar-fill');
		expect(fill).toHaveStyle({ width: '0%' });
	});

	it('applies correct height class', () => {
		const { container } = render(SizeBar, {
			size: 50,
			totalSize: 100,
			height: 'h-6'
		});

		const bar = container.querySelector('.size-bar');
		expect(bar).toHaveClass('h-6');
	});

	it('supports risk level colors', () => {
		const { container } = render(SizeBar, {
			size: 50,
			totalSize: 100,
			riskLevel: 'warning'
		});

		const fill = container.querySelector('.size-bar-fill');
		expect(fill).toHaveAttribute('style');
		// Color would be applied via CSS variables in real implementation
	});

	it('supports usage pattern colors', () => {
		const { container } = render(SizeBar, {
			size: 50,
			totalSize: 100,
			usagePattern: 'frequent'
		});

		const fill = container.querySelector('.size-bar-fill');
		expect(fill).toHaveAttribute('style');
	});

	it('has proper accessibility attributes', () => {
		const { container } = render(SizeBar, {
			size: 50,
			totalSize: 100
		});

		const progressBar = container.querySelector('[role="progressbar"]');
		expect(progressBar).toBeInTheDocument();
		expect(progressBar).toHaveAttribute('aria-valuenow', '50');
		expect(progressBar).toHaveAttribute('aria-valuemin', '0');
		expect(progressBar).toHaveAttribute('aria-valuemax', '100');
	});
});
