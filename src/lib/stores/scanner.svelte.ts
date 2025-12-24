// Scanner store using Svelte 5 runes

// Import only types used within this store
import type {
	ScanItem,
	ScanResults,
} from '$lib/generated/types';

// State
let scanResults = $state<ScanResults | null>(null);
let isScanning = $state(false);
let scanProgress = $state(0);
let selectedItemIds = $state<Set<string>>(new Set());

// Computed values
const selectedItems = $derived(() => {
	if (!scanResults) return [];
	const findSelected = (items: ScanItem[]): ScanItem[] => {
		const result: ScanItem[] = [];
		for (const item of items) {
			if (selectedItemIds.has(item.id)) {
				result.push(item);
			}
			if (item.children) {
				result.push(...findSelected(item.children));
			}
		}
		return result;
	};
	return findSelected(scanResults.items);
});

const selectedSize = $derived(() => {
	return selectedItems().reduce((sum, item) => sum + item.size, 0);
});

const itemsByCategory = $derived(() => {
	if (!scanResults) return {};
	const categories: Record<string, ScanItem[]> = {};
	const categorize = (items: ScanItem[]) => {
		for (const item of items) {
			if (!categories[item.category]) {
				categories[item.category] = [];
			}
			categories[item.category].push(item);
			if (item.children) {
				categorize(item.children);
			}
		}
	};
	categorize(scanResults.items);
	return categories;
});

export const scanner = {
	get results() {
		return scanResults;
	},
	get isScanning() {
		return isScanning;
	},
	get progress() {
		return scanProgress;
	},
	get selectedIds() {
		return selectedItemIds;
	},
	get selectedItems() {
		return selectedItems();
	},
	get selectedSize() {
		return selectedSize();
	},
	get itemsByCategory() {
		return itemsByCategory();
	},

	setResults(results: ScanResults | null) {
		scanResults = results;
	},

	setScanning(value: boolean) {
		isScanning = value;
	},

	setProgress(value: number) {
		scanProgress = Math.min(100, Math.max(0, value));
	},

	toggleSelection(id: string) {
		const newSet = new Set(selectedItemIds);
		if (newSet.has(id)) {
			newSet.delete(id);
		} else {
			newSet.add(id);
		}
		selectedItemIds = newSet;
	},

	selectAll() {
		if (!scanResults) return;
		const newSet = new Set<string>();
		const addAll = (items: ScanItem[]) => {
			for (const item of items) {
				newSet.add(item.id);
				if (item.children) {
					addAll(item.children);
				}
			}
		};
		addAll(scanResults.items);
		selectedItemIds = newSet;
	},

	clearSelection() {
		selectedItemIds = new Set();
	},

	reset() {
		scanResults = null;
		isScanning = false;
		scanProgress = 0;
		selectedItemIds = new Set();
	}
};
