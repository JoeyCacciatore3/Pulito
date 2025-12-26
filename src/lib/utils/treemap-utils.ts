/**
 * Treemap visualization utilities
 * Provides functions for converting TreeNode data to D3 hierarchy format,
 * calculating treemap layouts, and mapping colors based on various criteria.
 */

import * as d3 from 'd3-hierarchy';
import type { TreeNode as GeneratedTreeNode } from '$lib/generated/types';

// Extended TreeNode type with proper riskLevel type
export type TreeNode = Omit<GeneratedTreeNode, 'riskLevel' | 'children' | 'usagePattern'> & {
	riskLevel: 'safe' | 'caution' | 'warning';
	usagePattern?: 'frequent' | 'occasional' | 'rare' | 'never';
	children?: TreeNode[];
};

// Type guard to convert GeneratedTreeNode to TreeNode
function ensureTreeNode(node: GeneratedTreeNode): TreeNode {
	const validUsagePatterns = ['frequent', 'occasional', 'rare', 'never'] as const;
	const usagePattern = node.usagePattern && validUsagePatterns.includes(node.usagePattern as any)
		? (node.usagePattern as 'frequent' | 'occasional' | 'rare' | 'never')
		: undefined;

	const converted: TreeNode = {
		...node,
		riskLevel: (node.riskLevel === 'safe' || node.riskLevel === 'caution' || node.riskLevel === 'warning'
			? node.riskLevel
			: 'safe') as 'safe' | 'caution' | 'warning',
		usagePattern,
		children: node.children ? node.children.map(ensureTreeNode) : undefined
	};
	return converted;
}

// Convert array of GeneratedTreeNode to TreeNode
function convertTreeNodeArray(nodes: GeneratedTreeNode[]): TreeNode[] {
	return nodes.map(ensureTreeNode);
}

/**
 * Convert TreeNode array to D3 hierarchy structure
 * Handles the conversion from our TreeNode format to d3.HierarchyNode format
 * Accepts both GeneratedTreeNode and TreeNode types
 */
export function convertToD3Hierarchy(
	nodes: GeneratedTreeNode[] | TreeNode[]
): d3.HierarchyNode<TreeNode> | null {
	if (!nodes || nodes.length === 0) {
		return null;
	}

	// Convert to internal TreeNode format
	const convertedNodes = nodes.map((node) => {
		if ('riskLevel' in node && typeof node.riskLevel === 'string' && node.riskLevel !== 'safe' && node.riskLevel !== 'caution' && node.riskLevel !== 'warning') {
			return ensureTreeNode(node as GeneratedTreeNode);
		}
		return node as TreeNode;
	});

	// Create a root node that contains all top-level nodes as children
	const rootData: TreeNode = {
		id: 'root',
		name: 'Root',
		path: '/',
		size: 0,
		isDirectory: true,
		lastModified: Date.now() / 1000,
		lastAccessed: Date.now() / 1000,
		children: convertedNodes,
		expanded: true,
		selected: false,
		riskLevel: 'safe',
		usagePattern: undefined
	};

	// Create hierarchy with children accessor
	// Convert children to TreeNode format
	const hierarchy = d3.hierarchy(rootData, (d: TreeNode) => {
		if (d.children && d.children.length > 0) {
			return d.children.map((child) => {
				// If child is already TreeNode format, return as-is
				if (typeof child.riskLevel === 'string' && (child.riskLevel === 'safe' || child.riskLevel === 'caution' || child.riskLevel === 'warning')) {
					return child as TreeNode;
				}
				// Otherwise convert
				return ensureTreeNode(child as GeneratedTreeNode);
			});
		}
		return null;
	});

	return hierarchy;
}

/**
 * Aggregate directory sizes recursively
 * For directories, size is the sum of all children's sizes
 * Accepts both GeneratedTreeNode and TreeNode types
 */
export function aggregateDirectorySizes(node: GeneratedTreeNode | TreeNode): number {
	if (!node.isDirectory || !node.children || node.children.length === 0) {
		return node.size;
	}

	return node.children.reduce((sum, child) => {
		return sum + aggregateDirectorySizes(child);
	}, 0);
}

/**
 * Calculate treemap layout using D3's squarified algorithm
 * Returns the root node with layout coordinates (x0, y0, x1, y1)
 */
export function calculateTreemapLayout(
	root: d3.HierarchyNode<TreeNode>,
	width: number,
	height: number,
	padding: number = 2
): d3.HierarchyRectangularNode<TreeNode> | null {
	if (!root) {
		return null;
	}

	// Sum values: for files use size, for directories sum children
	root.sum((d: TreeNode) => {
		const node = ensureTreeNode(d as GeneratedTreeNode);
		if (node.isDirectory && node.children && node.children.length > 0) {
			// Directory size is sum of children
			return aggregateDirectorySizes(node);
		}
		return node.size;
	});

	// Sort by descending size for better visual organization
	root.sort((a, b) => {
		const aValue = a.value ?? 0;
		const bValue = b.value ?? 0;
		return bValue - aValue; // Descending order
	});

	// Create treemap layout with squarified tiling
	const treemap = d3
		.treemap<TreeNode>()
		.size([width, height])
		.tile(d3.treemapSquarify)
		.paddingInner(padding)
		.paddingOuter(0)
		.round(true);

	// Apply layout
	treemap(root);

	return root as d3.HierarchyRectangularNode<TreeNode>;
}

/**
 * Filter visible nodes based on minimum size threshold
 * Hides nodes that are too small to be visible or meaningful
 */
export function filterVisibleNodes(
	nodes: d3.HierarchyRectangularNode<TreeNode>[],
	minSize: number = 4
): d3.HierarchyRectangularNode<TreeNode>[] {
	return nodes.filter((node) => {
		if (node.x0 === undefined || node.y0 === undefined || node.x1 === undefined || node.y1 === undefined) {
			return false;
		}
		const width = node.x1 - node.x0;
		const height = node.y1 - node.y0;
		return width >= minSize && height >= minSize;
	});
}

/**
 * Get node color based on selected color mode
 * Integrates with existing color utilities from the codebase
 * Accepts both GeneratedTreeNode and TreeNode types
 */
export function getNodeColor(
	node: GeneratedTreeNode | TreeNode,
	colorMode: 'size' | 'risk' | 'usage' | 'filetype',
	isDark: boolean = false
): string {
	const nodeWithRisk = ensureTreeNode(node as GeneratedTreeNode);
	const colors = {
		dark: {
			primary: 'rgb(59, 130, 246)',
			secondary: 'rgb(16, 185, 129)',
			warning: 'rgb(245, 158, 11)',
			danger: 'rgb(239, 68, 68)',
			textSecondary: 'rgba(255, 255, 255, 0.6)'
		},
		light: {
			primary: 'rgb(59, 130, 246)',
			secondary: 'rgb(16, 185, 129)',
			warning: 'rgb(245, 158, 11)',
			danger: 'rgb(239, 68, 68)',
			textSecondary: 'rgba(0, 0, 0, 0.6)'
		}
	};

	const themeColors = isDark ? colors.dark : colors.light;

	switch (colorMode) {
		case 'risk': {
			// Use risk level colors
			switch (nodeWithRisk.riskLevel) {
				case 'safe':
					return themeColors.secondary; // Green
				case 'caution':
					return themeColors.warning; // Yellow
				case 'warning':
					return themeColors.danger; // Red
				default:
					return themeColors.textSecondary;
			}
		}
		case 'usage': {
			// Use usage pattern colors
			if (!nodeWithRisk.usagePattern) {
				return themeColors.textSecondary;
			}
			switch (nodeWithRisk.usagePattern) {
				case 'frequent':
					return themeColors.secondary; // Green
				case 'occasional':
					return themeColors.primary; // Blue
				case 'rare':
					return themeColors.warning; // Yellow
				case 'never':
					return themeColors.danger; // Red
				default:
					return themeColors.textSecondary;
			}
		}
		case 'filetype': {
			// File type based coloring
			if (nodeWithRisk.isDirectory) {
				return isDark ? 'rgba(156, 163, 175, 0.7)' : 'rgba(107, 114, 128, 0.7)'; // Gray for directories
			}
			// Import synchronously - file-types is a regular module, not circular
			// Use require for synchronous import in this context
			try {
				// eslint-disable-next-line @typescript-eslint/no-require-imports
				const { getFileTypeFromPath, getFileTypeColor } = require('./file-types');
				const fileType = getFileTypeFromPath(nodeWithRisk.path);
				return getFileTypeColor(fileType, isDark);
			} catch {
				// Fallback if import fails
				return themeColors.textSecondary;
			}
		}
		case 'size':
		default: {
			// Size-based gradient: larger files = darker color
			// This is a simplified version - in practice, you'd normalize against total size
			// For now, use a simple opacity-based approach
			const maxSize = 1024 * 1024 * 100; // 100MB as reference
			const normalizedSize = Math.min(nodeWithRisk.size / maxSize, 1);
			const opacity = 0.3 + normalizedSize * 0.7; // 0.3 to 1.0 opacity
			return `rgba(59, 130, 246, ${opacity})`; // Blue with varying opacity
		}
	}
}

/**
 * Get all descendants of a node for treemap rendering
 * Flattens the hierarchy to get all nodes that should be rendered
 */
export function getTreemapNodes(
	root: d3.HierarchyRectangularNode<TreeNode>
): d3.HierarchyRectangularNode<TreeNode>[] {
	if (!root) {
		return [];
	}
	return root.descendants();
}

/**
 * Find node by ID in the hierarchy
 * Useful for zooming and selection
 */
export function findNodeById(
	root: d3.HierarchyRectangularNode<TreeNode>,
	id: string
): d3.HierarchyRectangularNode<TreeNode> | null {
	if (!root) {
		return null;
	}

	// Check if this node matches
	if (root.data.id === id) {
		return root;
	}

	// Search children
	if (root.children) {
		for (const child of root.children) {
			const found = findNodeById(child, id);
			if (found) {
				return found;
			}
		}
	}

	return null;
}

/**
 * Calculate total size of all nodes in hierarchy
 * Used for normalization in size-based coloring
 */
export function calculateTotalSize(root: d3.HierarchyNode<TreeNode>): number {
	if (!root) {
		return 0;
	}

	root.sum((d: TreeNode) => {
		const node = ensureTreeNode(d as GeneratedTreeNode);
		if (node.isDirectory && node.children && node.children.length > 0) {
			return aggregateDirectorySizes(node);
		}
		return node.size;
	});

	return root.value ?? 0;
}

/**
 * Calculate size percentage of a node relative to total size
 */
export function calculateSizePercentage(nodeSize: number, totalSize: number): number {
	if (!totalSize || totalSize === 0) {
		return 0;
	}
	return (nodeSize / totalSize) * 100;
}

/**
 * Get size range label for legend (e.g., "0-1 MB", "1-10 MB")
 */
export function getSizeRangeLabel(size: number): string {
	const mb = size / (1024 * 1024);
	const gb = size / (1024 * 1024 * 1024);

	if (gb >= 1) {
		return `${gb.toFixed(1)} GB`;
	} else if (mb >= 1) {
		return `${mb.toFixed(1)} MB`;
	} else {
		const kb = size / 1024;
		return kb >= 1 ? `${kb.toFixed(1)} KB` : `${size} B`;
	}
}

/**
 * Get size range category for legend grouping
 */
export function getSizeRangeCategory(size: number): string {
	const mb = size / (1024 * 1024);
	const gb = size / (1024 * 1024 * 1024);

	if (gb >= 10) {
		return '10+ GB';
	} else if (gb >= 1) {
		return '1-10 GB';
	} else if (mb >= 100) {
		return '100 MB - 1 GB';
	} else if (mb >= 10) {
		return '10-100 MB';
	} else if (mb >= 1) {
		return '1-10 MB';
	} else {
		return '< 1 MB';
	}
}

/**
 * Calculate statistics for a set of nodes
 */
export interface TreemapStatistics {
	totalSize: number;
	fileCount: number;
	folderCount: number;
	largestFiles: Array<{ name: string; path: string; size: number }>;
	largestDirectories: Array<{ name: string; path: string; size: number }>;
}

export function calculateStatistics(
	nodes: GeneratedTreeNode[] | TreeNode[]
): TreemapStatistics {
	let totalSize = 0;
	let fileCount = 0;
	let folderCount = 0;
	const files: Array<{ name: string; path: string; size: number }> = [];
	const directories: Array<{ name: string; path: string; size: number }> = [];

	function processNode(node: GeneratedTreeNode | TreeNode): number {
		const nodeWithRisk = ensureTreeNode(node as GeneratedTreeNode);
		let nodeSize = nodeWithRisk.size;

		if (nodeWithRisk.isDirectory) {
			folderCount++;
			if (nodeWithRisk.children && nodeWithRisk.children.length > 0) {
				// Calculate directory size from children
				nodeSize = nodeWithRisk.children.reduce((sum, child) => {
					return sum + processNode(child);
				}, 0);
			}
			directories.push({
				name: nodeWithRisk.name,
				path: nodeWithRisk.path,
				size: nodeSize
			});
			// Directory size is already counted in children processing
		} else {
			fileCount++;
			files.push({
				name: nodeWithRisk.name,
				path: nodeWithRisk.path,
				size: nodeSize
			});
			totalSize += nodeSize;
		}

		return nodeSize;
	}

	nodes.forEach(processNode);

	// Sort and get top 5
	files.sort((a, b) => b.size - a.size);
	directories.sort((a, b) => b.size - a.size);

	return {
		totalSize,
		fileCount,
		folderCount,
		largestFiles: files.slice(0, 5),
		largestDirectories: directories.slice(0, 5)
	};
}
