import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import EnhancedTreeView from './EnhancedTreeView.svelte';

// Mock the Tauri API
vi.mock('$lib/utils/tauri', () => ({
	invoke: vi.fn(),
	formatBytes: (bytes: number) => `${bytes} B`
}));

vi.mock('$lib/stores/notifications.svelte', () => ({
	notificationStore: {
		success: vi.fn(),
		error: vi.fn(),
		warning: vi.fn()
	}
}));

vi.mock('$lib/utils/logger', () => ({
	logger: {
		error: vi.fn(),
		warn: vi.fn(),
		info: vi.fn(),
		debug: vi.fn()
	}
}));

vi.mock('$lib/utils/file-types', () => ({
	getFileExtension: vi.fn(() => ''),
	getFileTypeFromPath: vi.fn(() => 'other')
}));

describe('EnhancedTreeView', () => {
	const mockNodes = [
		{
			id: '1',
			name: 'Documents',
			path: '/home/user/Documents',
			size: 1024,
			isDirectory: true,
			lastModified: 1234567890,
			lastAccessed: 1234567890,
			children: [],
			expanded: false,
			selected: false,
			riskLevel: 'safe' as const,
			usagePattern: 'frequent' as const
		},
		{
			id: '2',
			name: 'file.txt',
			path: '/home/user/Documents/file.txt',
			size: 512,
			isDirectory: false,
			lastModified: 1234567890,
			lastAccessed: 1234567890,
			children: [],
			expanded: false,
			selected: false,
			riskLevel: 'safe' as const,
			usagePattern: 'occasional' as const
		}
	];

	it('renders the component with tree view by default', () => {
		render(EnhancedTreeView);
		expect(screen.getByText('Enhanced File Explorer')).toBeInTheDocument();
		expect(screen.getByRole('tab', { name: /tree/i })).toBeInTheDocument();
	});

	it('switches between view modes', async () => {
		render(EnhancedTreeView);

		const listTab = screen.getByRole('tab', { name: /list/i });
		await fireEvent.click(listTab);

		expect(listTab).toHaveAttribute('aria-selected', 'true');
	});

	it('displays statistics correctly', () => {
		render(EnhancedTreeView);
		expect(screen.getByText('Files')).toBeInTheDocument();
		expect(screen.getByText('Folders')).toBeInTheDocument();
		expect(screen.getByText('Total Size')).toBeInTheDocument();
	});

	it('has accessible navigation', () => {
		render(EnhancedTreeView);

		const tabs = screen.getAllByRole('tab');
		expect(tabs).toHaveLength(3);

		tabs.forEach(tab => {
			expect(tab).toHaveAttribute('aria-selected');
			expect(tab).toHaveAttribute('aria-controls');
		});
	});

	it('supports keyboard navigation', () => {
		render(EnhancedTreeView);

		const treeTab = screen.getByRole('tab', { name: /tree/i });
		treeTab.focus();

		fireEvent.keyDown(treeTab, { key: 'ArrowRight' });
		// Should move to next tab (list)
		expect(screen.getByRole('tab', { name: /list/i })).toHaveFocus();
	});
});
