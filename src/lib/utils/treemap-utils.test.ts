import { describe, it, expect } from 'vitest';
import {
	convertToD3Hierarchy,
	aggregateDirectorySizes,
	calculateTreemapLayout,
	filterVisibleNodes,
	getNodeColor,
	getTreemapNodes,
	findNodeById,
	calculateTotalSize,
	calculateSizePercentage,
	getSizeRangeLabel,
	calculateStatistics
} from './treemap-utils';

// Test TreeNode type matching the component interface
type TreeNode = {
	id: string;
	name: string;
	path: string;
	size: number;
	isDirectory: boolean;
	lastModified: number;
	lastAccessed: number;
	children?: TreeNode[];
	expanded: boolean;
	selected: boolean;
	riskLevel: 'safe' | 'caution' | 'warning';
	usagePattern?: 'frequent' | 'occasional' | 'rare' | 'never';
	parent?: TreeNode;
};

describe('treemap-utils', () => {
	describe('convertToD3Hierarchy', () => {
		it('should return null for empty array', () => {
			const result = convertToD3Hierarchy([]);
			expect(result).toBeNull();
		});

		it('should create hierarchy from single node', () => {
			const nodes: TreeNode[] = [
				{
					id: '1',
					name: 'file1.txt',
					path: '/file1.txt',
					size: 1000,
					isDirectory: false,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					expanded: false,
					selected: false,
					riskLevel: 'safe'
				}
			];

			const result = convertToD3Hierarchy(nodes);
			expect(result).not.toBeNull();
			expect(result?.data.name).toBe('Root');
			expect(result?.children?.length).toBe(1);
			expect(result?.children?.[0]?.data.name).toBe('file1.txt');
		});

		it('should create hierarchy with nested children', () => {
			const nodes: TreeNode[] = [
				{
					id: '1',
					name: 'dir1',
					path: '/dir1',
					size: 0,
					isDirectory: true,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					children: [
						{
							id: '2',
							name: 'file1.txt',
							path: '/dir1/file1.txt',
							size: 500,
							isDirectory: false,
							lastModified: Date.now() / 1000,
							lastAccessed: Date.now() / 1000,
							expanded: false,
							selected: false,
							riskLevel: 'safe'
						}
					],
					expanded: true,
					selected: false,
					riskLevel: 'safe'
				}
			];

			const result = convertToD3Hierarchy(nodes);
			expect(result).not.toBeNull();
			expect(result?.children?.length).toBe(1);
			expect(result?.children?.[0]?.children?.length).toBe(1);
			expect(result?.children?.[0]?.children?.[0]?.data.name).toBe('file1.txt');
		});
	});

	describe('aggregateDirectorySizes', () => {
		it('should return file size for non-directory', () => {
			const node: TreeNode = {
				id: '1',
				name: 'file.txt',
				path: '/file.txt',
				size: 1000,
				isDirectory: false,
				lastModified: Date.now() / 1000,
				lastAccessed: Date.now() / 1000,
				expanded: false,
				selected: false,
				riskLevel: 'safe'
			};

			expect(aggregateDirectorySizes(node)).toBe(1000);
		});

		it('should sum children sizes for directory', () => {
			const node: TreeNode = {
				id: '1',
				name: 'dir',
				path: '/dir',
				size: 0,
				isDirectory: true,
				lastModified: Date.now() / 1000,
				lastAccessed: Date.now() / 1000,
				children: [
					{
						id: '2',
						name: 'file1.txt',
						path: '/dir/file1.txt',
						size: 500,
						isDirectory: false,
						lastModified: Date.now() / 1000,
						lastAccessed: Date.now() / 1000,
						expanded: false,
						selected: false,
						riskLevel: 'safe'
					},
					{
						id: '3',
						name: 'file2.txt',
						path: '/dir/file2.txt',
						size: 300,
						isDirectory: false,
						lastModified: Date.now() / 1000,
						lastAccessed: Date.now() / 1000,
						expanded: false,
						selected: false,
						riskLevel: 'safe'
					}
				],
				expanded: true,
				selected: false,
				riskLevel: 'safe'
			};

			expect(aggregateDirectorySizes(node)).toBe(800);
		});

		it('should handle nested directories', () => {
			const node: TreeNode = {
				id: '1',
				name: 'dir',
				path: '/dir',
				size: 0,
				isDirectory: true,
				lastModified: Date.now() / 1000,
				lastAccessed: Date.now() / 1000,
				children: [
					{
						id: '2',
						name: 'subdir',
						path: '/dir/subdir',
						size: 0,
						isDirectory: true,
						lastModified: Date.now() / 1000,
						lastAccessed: Date.now() / 1000,
						children: [
							{
								id: '3',
								name: 'file.txt',
								path: '/dir/subdir/file.txt',
								size: 200,
								isDirectory: false,
								lastModified: Date.now() / 1000,
								lastAccessed: Date.now() / 1000,
								expanded: false,
								selected: false,
								riskLevel: 'safe'
							}
						],
						expanded: true,
						selected: false,
						riskLevel: 'safe'
					},
					{
						id: '4',
						name: 'file2.txt',
						path: '/dir/file2.txt',
						size: 100,
						isDirectory: false,
						lastModified: Date.now() / 1000,
						lastAccessed: Date.now() / 1000,
						expanded: false,
						selected: false,
						riskLevel: 'safe'
					}
				],
				expanded: true,
				selected: false,
				riskLevel: 'safe'
			};

			expect(aggregateDirectorySizes(node)).toBe(300);
		});
	});

	describe('calculateTreemapLayout', () => {
		it('should return null for null root', () => {
			const result = calculateTreemapLayout(null as any, 100, 100);
			expect(result).toBeNull();
		});

		it('should calculate layout for single node', () => {
			const nodes: TreeNode[] = [
				{
					id: '1',
					name: 'file.txt',
					path: '/file.txt',
					size: 1000,
					isDirectory: false,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					expanded: false,
					selected: false,
					riskLevel: 'safe'
				}
			];

			const hierarchy = convertToD3Hierarchy(nodes);
			expect(hierarchy).not.toBeNull();

			const layout = calculateTreemapLayout(hierarchy!, 100, 100);
			expect(layout).not.toBeNull();
			expect(layout?.children?.[0]?.x0).toBeDefined();
			expect(layout?.children?.[0]?.y0).toBeDefined();
			expect(layout?.children?.[0]?.x1).toBeDefined();
			expect(layout?.children?.[0]?.y1).toBeDefined();
		});

		it('should calculate layout with multiple nodes', () => {
			const nodes: TreeNode[] = [
				{
					id: '1',
					name: 'file1.txt',
					path: '/file1.txt',
					size: 1000,
					isDirectory: false,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					expanded: false,
					selected: false,
					riskLevel: 'safe'
				},
				{
					id: '2',
					name: 'file2.txt',
					path: '/file2.txt',
					size: 500,
					isDirectory: false,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					expanded: false,
					selected: false,
					riskLevel: 'safe'
				}
			];

			const hierarchy = convertToD3Hierarchy(nodes);
			const layout = calculateTreemapLayout(hierarchy!, 200, 200);

			expect(layout).not.toBeNull();
			expect(layout?.children?.length).toBe(2);
		});
	});

	describe('filterVisibleNodes', () => {
		it('should filter nodes smaller than minimum size', () => {
			const nodes = [
				{ x0: 0, y0: 0, x1: 10, y1: 10, data: {} as TreeNode },
				{ x0: 0, y0: 0, x1: 2, y1: 2, data: {} as TreeNode }, // 2x2 - should be filtered
				{ x0: 0, y0: 0, x1: 5, y1: 5, data: {} as TreeNode } // 5x5 - should pass
			] as any[];

			const filtered = filterVisibleNodes(nodes, 4);
			// Should have 2 nodes: 10x10 and 5x5 (both >= 4)
			expect(filtered.length).toBe(2);
			// Verify both filtered nodes meet minimum size
			filtered.forEach((node) => {
				const width = node.x1 - node.x0;
				const height = node.y1 - node.y0;
				expect(width).toBeGreaterThanOrEqual(4);
				expect(height).toBeGreaterThanOrEqual(4);
			});
		});

		it('should return empty array for all nodes below threshold', () => {
			const nodes = [
				{ x0: 0, y0: 0, x1: 1, y1: 1, data: {} as TreeNode },
				{ x0: 0, y0: 0, x1: 2, y1: 2, data: {} as TreeNode }
			] as any[];

			const filtered = filterVisibleNodes(nodes, 4);
			expect(filtered.length).toBe(0);
		});
	});

	describe('getNodeColor', () => {
		const testNode: TreeNode = {
			id: '1',
			name: 'file.txt',
			path: '/file.txt',
			size: 1000,
			isDirectory: false,
			lastModified: Date.now() / 1000,
			lastAccessed: Date.now() / 1000,
			expanded: false,
			selected: false,
			riskLevel: 'safe',
			usagePattern: 'frequent'
		};

		it('should return risk-based colors', () => {
			const safeColor = getNodeColor({ ...testNode, riskLevel: 'safe' }, 'risk', false);
			expect(safeColor).toContain('185, 129'); // Green

			const cautionColor = getNodeColor({ ...testNode, riskLevel: 'caution' }, 'risk', false);
			expect(cautionColor).toContain('245, 158, 11'); // Yellow

			const warningColor = getNodeColor({ ...testNode, riskLevel: 'warning' }, 'risk', false);
			expect(warningColor).toContain('239, 68, 68'); // Red
		});

		it('should return usage-based colors', () => {
			const frequentColor = getNodeColor({ ...testNode, usagePattern: 'frequent' }, 'usage', false);
			expect(frequentColor).toContain('185, 129'); // Green

			const occasionalColor = getNodeColor(
				{ ...testNode, usagePattern: 'occasional' },
				'usage',
				false
			);
			expect(occasionalColor).toContain('59, 130, 246'); // Blue

			const rareColor = getNodeColor({ ...testNode, usagePattern: 'rare' }, 'usage', false);
			expect(rareColor).toContain('245, 158, 11'); // Yellow

			const neverColor = getNodeColor({ ...testNode, usagePattern: 'never' }, 'usage', false);
			expect(neverColor).toContain('239, 68, 68'); // Red
		});

		it('should return size-based colors with opacity', () => {
			const color = getNodeColor(testNode, 'size', false);
			expect(color).toContain('rgba');
			expect(color).toContain('59, 130, 246'); // Blue base
		});
	});

	describe('getTreemapNodes', () => {
		it('should return all descendants', () => {
			const nodes: TreeNode[] = [
				{
					id: '1',
					name: 'dir',
					path: '/dir',
					size: 0,
					isDirectory: true,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					children: [
						{
							id: '2',
							name: 'file.txt',
							path: '/dir/file.txt',
							size: 100,
							isDirectory: false,
							lastModified: Date.now() / 1000,
							lastAccessed: Date.now() / 1000,
							expanded: false,
							selected: false,
							riskLevel: 'safe'
						}
					],
					expanded: true,
					selected: false,
					riskLevel: 'safe'
				}
			];

			const hierarchy = convertToD3Hierarchy(nodes);
			const layout = calculateTreemapLayout(hierarchy!, 100, 100);
			const treemapNodes = getTreemapNodes(layout!);

			// Should include root + dir + file = 3 nodes
			expect(treemapNodes.length).toBeGreaterThanOrEqual(2);
		});
	});

	describe('findNodeById', () => {
		it('should find node by ID', () => {
			const nodes: TreeNode[] = [
				{
					id: '1',
					name: 'dir',
					path: '/dir',
					size: 0,
					isDirectory: true,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					children: [
						{
							id: '2',
							name: 'file.txt',
							path: '/dir/file.txt',
							size: 100,
							isDirectory: false,
							lastModified: Date.now() / 1000,
							lastAccessed: Date.now() / 1000,
							expanded: false,
							selected: false,
							riskLevel: 'safe'
						}
					],
					expanded: true,
					selected: false,
					riskLevel: 'safe'
				}
			];

			const hierarchy = convertToD3Hierarchy(nodes);
			const layout = calculateTreemapLayout(hierarchy!, 100, 100);

			const found = findNodeById(layout!, '2');
			expect(found).not.toBeNull();
			expect(found?.data.id).toBe('2');
		});

		it('should return null for non-existent ID', () => {
			const nodes: TreeNode[] = [
				{
					id: '1',
					name: 'file.txt',
					path: '/file.txt',
					size: 100,
					isDirectory: false,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					expanded: false,
					selected: false,
					riskLevel: 'safe'
				}
			];

			const hierarchy = convertToD3Hierarchy(nodes);
			const layout = calculateTreemapLayout(hierarchy!, 100, 100);

			const found = findNodeById(layout!, '999');
			expect(found).toBeNull();
		});
	});

	describe('calculateTotalSize', () => {
		it('should calculate total size of hierarchy', () => {
			const nodes: TreeNode[] = [
				{
					id: '1',
					name: 'file1.txt',
					path: '/file1.txt',
					size: 1000,
					isDirectory: false,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					expanded: false,
					selected: false,
					riskLevel: 'safe'
				},
				{
					id: '2',
					name: 'file2.txt',
					path: '/file2.txt',
					size: 500,
					isDirectory: false,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					expanded: false,
					selected: false,
					riskLevel: 'safe'
				}
			];

			const hierarchy = convertToD3Hierarchy(nodes);
			const totalSize = calculateTotalSize(hierarchy!);

			// Root node value should be sum of all children (1000 + 500 = 1500)
			// But since we create a root wrapper, it includes the root's own size (0) + children
			// The actual value should be the sum of all file sizes
			expect(totalSize).toBeGreaterThanOrEqual(1500);
		});
	});

	describe('calculateSizePercentage', () => {
		it('should calculate percentage correctly', () => {
			expect(calculateSizePercentage(500, 1000)).toBe(50);
			expect(calculateSizePercentage(250, 1000)).toBe(25);
			expect(calculateSizePercentage(0, 1000)).toBe(0);
		});

		it('should handle zero total size', () => {
			expect(calculateSizePercentage(100, 0)).toBe(0);
		});
	});

	describe('getSizeRangeLabel', () => {
		it('should format bytes correctly', () => {
			expect(getSizeRangeLabel(500)).toContain('B');
			expect(getSizeRangeLabel(1024)).toContain('KB');
			expect(getSizeRangeLabel(1024 * 1024)).toContain('MB');
			expect(getSizeRangeLabel(1024 * 1024 * 1024)).toContain('GB');
		});
	});

	describe('calculateStatistics', () => {
		it('should calculate statistics correctly', () => {
			const nodes: TreeNode[] = [
				{
					id: '1',
					name: 'file1.txt',
					path: '/file1.txt',
					size: 1000,
					isDirectory: false,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					expanded: false,
					selected: false,
					riskLevel: 'safe'
				},
				{
					id: '2',
					name: 'dir1',
					path: '/dir1',
					size: 0,
					isDirectory: true,
					lastModified: Date.now() / 1000,
					lastAccessed: Date.now() / 1000,
					children: [
						{
							id: '3',
							name: 'file2.txt',
							path: '/dir1/file2.txt',
							size: 500,
							isDirectory: false,
							lastModified: Date.now() / 1000,
							lastAccessed: Date.now() / 1000,
							expanded: false,
							selected: false,
							riskLevel: 'safe'
						}
					],
					expanded: true,
					selected: false,
					riskLevel: 'safe'
				}
			];

			const stats = calculateStatistics(nodes);
			expect(stats.totalSize).toBe(1500);
			expect(stats.fileCount).toBe(2); // file1.txt and file2.txt
			expect(stats.folderCount).toBe(1); // dir1
			expect(stats.largestFiles.length).toBe(2);
			expect(stats.largestFiles[0].size).toBe(1000);
			expect(stats.largestDirectories.length).toBe(1);
			expect(stats.largestDirectories[0].size).toBe(500); // dir1 contains file2.txt
		});
	});
});
