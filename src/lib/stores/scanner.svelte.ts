// Scanner store using Svelte 5 runes

// Import and re-export all types from generated types
import type {
	ScanItem,
	ScanResults,
	FilesystemHealthResults,
	StorageRecoveryResults,
	DuplicateGroup,
	CacheAnalytics,
	CacheContributor,
	CacheGrowthPoint,
	DiskPulseHealth,
	OldFilesSummary,
	CacheEvent,
	CacheItem,
	SystemHealthData,
	GpuInfo,
	Temperatures,
	CleanResult,
	TrashData,
	TrashItem,
	TrashMetadata,
	NetworkInterfaceInfo,
	NetworkConnection,
	ProcessInfo,
	BatteryInfo,
	LoadAverage,
	TreeNode,
} from '$lib/generated/types';

// Re-export types for backward compatibility
export type {
	ScanItem,
	ScanResults,
	FilesystemHealthResults,
	StorageRecoveryResults,
	DuplicateGroup,
	CacheAnalytics,
	CacheContributor,
	CacheGrowthPoint,
	DiskPulseHealth,
	OldFilesSummary,
	CacheEvent,
	CacheItem,
	SystemHealthData,
	GpuInfo,
	Temperatures,
	CleanResult,
	TrashData,
	TrashItem,
	TrashMetadata,
	NetworkInterfaceInfo,
	NetworkConnection,
	ProcessInfo,
	BatteryInfo,
	LoadAverage,
	TreeNode,
};

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
