/**
 * Treemap data aggregation utilities
 * Groups small items and optimizes data for visualization
 */

import type { TreeNode } from './treemap-utils';

export interface AggregationOptions {
	threshold: number; // Percentage threshold (0.01 = 1%)
	maxItems: number; // Maximum items per level
	depth: number; // Maximum depth to show
}

export interface AggregatedNode extends TreeNode {
	isAggregated?: boolean;
	aggregatedCount?: number;
}

/**
 * Aggregate small nodes into "Other" category
 */
export function aggregateSmallNodes(
	nodes: TreeNode[],
	totalSize: number,
	options: AggregationOptions = { threshold: 0.01, maxItems: 30, depth: 2 }
): AggregatedNode[] {
	const { threshold, maxItems, depth } = options;
	const thresholdSize = totalSize * threshold;

	const significant: TreeNode[] = [];
	const small: TreeNode[] = [];

	// Separate significant and small nodes
	nodes.forEach(node => {
		if (node.size >= thresholdSize) {
			significant.push(node);
		} else {
			small.push(node);
		}
	});

	// Limit number of significant items if needed
	if (significant.length > maxItems) {
		const topItems = significant
			.sort((a, b) => b.size - a.size)
			.slice(0, maxItems - 1); // Reserve one slot for "Other"

		const remainingSmall = significant.slice(maxItems - 1).concat(small);
		small.push(...remainingSmall);
		significant.splice(maxItems - 1);
		significant.push(...topItems);
	}

	// Create "Other" aggregate if there are small items
	if (small.length > 0) {
		const otherSize = small.reduce((sum, node) => sum + node.size, 0);
		const otherNode: AggregatedNode = {
			id: 'aggregated-other',
			name: 'Other',
			path: 'aggregated',
			size: otherSize,
			isDirectory: true,
			lastModified: 0,
			lastAccessed: 0,
			children: undefined,
			expanded: false,
			selected: false,
			riskLevel: 'safe',
			isAggregated: true,
			aggregatedCount: small.length
		};
		significant.push(otherNode);
	}

	return significant as AggregatedNode[];
}

/**
 * Limit tree depth for visualization
 */
export function limitTreeDepth(nodes: TreeNode[], maxDepth: number): TreeNode[] {
	function traverse(node: TreeNode, currentDepth: number): TreeNode {
		if (currentDepth >= maxDepth) {
			// Create a summary node for deeper levels
			return {
				...node,
				children: undefined,
				size: node.size // Keep original size for correct proportions
			};
		}

		if (node.children && node.children.length > 0) {
			return {
				...node,
				children: node.children.map(child => traverse(child, currentDepth + 1))
			};
		}

		return node;
	}

	return nodes.map(node => traverse(node, 0));
}

/**
 * Pre-process treemap data for optimal visualization
 */
export function preprocessTreemapData(
	nodes: TreeNode[],
	totalSize: number,
	options: Partial<AggregationOptions> = {}
): TreeNode[] {
	const defaultOptions: AggregationOptions = {
		threshold: 0.01, // 1%
		maxItems: 30,
		depth: 3
	};

	const finalOptions = { ...defaultOptions, ...options };

	// Step 1: Limit depth
	let processed = limitTreeDepth(nodes, finalOptions.depth);

	// Step 2: Aggregate small items
	processed = aggregateSmallNodes(processed, totalSize, finalOptions);

	return processed;
}

/**
 * Calculate optimal treemap dimensions based on data
 */
export function calculateOptimalDimensions(
	nodeCount: number,
	containerWidth: number,
	containerHeight: number
): { width: number; height: number } {
	// For many small items, use more height
	const aspectRatio = containerWidth / containerHeight;
	const itemDensity = nodeCount / (containerWidth * containerHeight / 10000); // Items per 10k pixels

	let optimalWidth = containerWidth;
	let optimalHeight = containerHeight;

	// Adjust dimensions based on item density
	if (itemDensity > 1) {
		// Many items - prioritize height for better readability
		optimalHeight = Math.max(containerHeight, containerHeight * 1.5);
	} else if (itemDensity < 0.1) {
		// Few items - use normal dimensions
		optimalHeight = containerHeight;
	}

	return { width: optimalWidth, height: optimalHeight };
}
