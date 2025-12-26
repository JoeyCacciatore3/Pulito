/**
 * Virtual scrolling utilities for efficient rendering of large lists
 * Provides functions to calculate visible range and manage scroll state
 */

export interface VirtualScrollOptions {
	itemHeight: number;
	containerHeight: number;
	scrollTop: number;
	totalItems: number;
	overscan?: number; // Number of items to render outside viewport
}

export interface VirtualScrollResult {
	startIndex: number;
	endIndex: number;
	offsetY: number;
	totalHeight: number;
	visibleItems: number;
}

/**
 * Calculate which items should be rendered based on scroll position
 */
export function calculateVirtualRange(options: VirtualScrollOptions): VirtualScrollResult {
	const {
		itemHeight,
		containerHeight,
		scrollTop,
		totalItems,
		overscan = 5
	} = options;

	// Calculate visible range
	const startIndex = Math.max(0, Math.floor(scrollTop / itemHeight) - overscan);
	const endIndex = Math.min(
		totalItems - 1,
		Math.ceil((scrollTop + containerHeight) / itemHeight) + overscan
	);

	// Calculate offset for positioning
	const offsetY = startIndex * itemHeight;

	// Total height of all items
	const totalHeight = totalItems * itemHeight;

	// Number of visible items
	const visibleItems = endIndex - startIndex + 1;

	return {
		startIndex,
		endIndex,
		offsetY,
		totalHeight,
		visibleItems
	};
}

/**
 * Calculate visible range for variable height items
 * This is a simplified version - for production, consider using a library
 * that handles dynamic heights more efficiently
 */
export function calculateVariableHeightRange(
	itemHeights: number[],
	containerHeight: number,
	scrollTop: number,
	overscan: number = 5
): VirtualScrollResult {
	let cumulativeHeight = 0;
	let startIndex = 0;
	let endIndex = itemHeights.length - 1;

	// Find start index
	for (let i = 0; i < itemHeights.length; i++) {
		if (cumulativeHeight + itemHeights[i] >= scrollTop) {
			startIndex = Math.max(0, i - overscan);
			break;
		}
		cumulativeHeight += itemHeights[i];
	}

	// Find end index
	let visibleHeight = cumulativeHeight;
	for (let i = startIndex; i < itemHeights.length; i++) {
		visibleHeight += itemHeights[i];
		if (visibleHeight >= scrollTop + containerHeight) {
			endIndex = Math.min(itemHeights.length - 1, i + overscan);
			break;
		}
	}

	// Calculate offset
	let offsetY = 0;
	for (let i = 0; i < startIndex; i++) {
		offsetY += itemHeights[i];
	}

	const totalHeight = itemHeights.reduce((sum, height) => sum + height, 0);

	return {
		startIndex,
		endIndex,
		offsetY,
		totalHeight,
		visibleItems: endIndex - startIndex + 1
	};
}

/**
 * Estimate item height for initial calculation
 * Useful when exact heights aren't known yet
 */
export function estimateItemHeight(
	sampleHeights: number[],
	defaultHeight: number = 50
): number {
	if (sampleHeights.length === 0) {
		return defaultHeight;
	}

	const sum = sampleHeights.reduce((a, b) => a + b, 0);
	return Math.round(sum / sampleHeights.length);
}
