/**
 * Export utilities for file system data
 * Provides functions to export tree data to CSV and JSON formats
 */

// TreeNode interface matching EnhancedTreeView
interface TreeNode {
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
}

export interface ExportOptions {
	includeMetadata?: boolean;
	includePath?: boolean;
	includeDates?: boolean;
	includeRisk?: boolean;
	includeUsage?: boolean;
	selectedOnly?: boolean;
}

/**
 * Export tree nodes to CSV format
 */
export function exportToCSV(
	nodes: TreeNode[],
	options: ExportOptions = {}
): string {
	const {
		includeMetadata = true,
		includePath = true,
		includeDates = true,
		includeRisk = true,
		includeUsage = true,
		selectedOnly = false
	} = options;

	// Flatten tree structure
	const flatNodes: TreeNode[] = [];
	function traverse(node: TreeNode) {
		if (!selectedOnly || node.selected) {
			flatNodes.push(node);
		}
		if (node.children) {
			node.children.forEach(traverse);
		}
	}
	nodes.forEach(traverse);

	// Build CSV headers
	const headers: string[] = ['Name', 'Type', 'Size (bytes)'];
	if (includePath) headers.push('Path');
	if (includeDates) {
		headers.push('Last Modified', 'Last Accessed');
	}
	if (includeRisk) headers.push('Risk Level');
	if (includeUsage) headers.push('Usage Pattern');
	if (includeMetadata) {
		headers.push('ID', 'Has Children');
	}

	// Build CSV rows
	const rows = flatNodes.map((node) => {
		const row: string[] = [
			escapeCSV(node.name),
			node.isDirectory ? 'Directory' : 'File',
			node.size.toString()
		];

		if (includePath) row.push(escapeCSV(node.path));
		if (includeDates) {
			row.push(
				new Date(node.lastModified * 1000).toISOString(),
				new Date(node.lastAccessed * 1000).toISOString()
			);
		}
		if (includeRisk) row.push(node.riskLevel);
		if (includeUsage) row.push(node.usagePattern || '');
		if (includeMetadata) {
			row.push(node.id, (node.children ? (node.children.length > 0).toString() : 'false'));
		}

		return row.join(',');
	});

	// Combine headers and rows
	return [headers.join(','), ...rows].join('\n');
}

/**
 * Export tree nodes to JSON format
 */
export function exportToJSON(
	nodes: TreeNode[],
	options: ExportOptions = {}
): string {
	const { selectedOnly = false } = options;

	// Filter nodes if needed
	let dataToExport: TreeNode[] = nodes;
	if (selectedOnly) {
		const selected: TreeNode[] = [];
		function traverse(node: TreeNode) {
			if (node.selected) {
				selected.push(node);
			}
			if (node.children) {
				node.children.forEach(traverse);
			}
		}
		nodes.forEach(traverse);
		dataToExport = selected;
	}

	return JSON.stringify(dataToExport, null, 2);
}

/**
 * Download exported data as a file
 */
export function downloadExport(
	content: string,
	filename: string,
	mimeType: string = 'text/plain'
): void {
	const blob = new Blob([content], { type: mimeType });
	const url = URL.createObjectURL(blob);
	const link = document.createElement('a');
	link.href = url;
	link.download = filename;
	document.body.appendChild(link);
	link.click();
	document.body.removeChild(link);
	URL.revokeObjectURL(url);
}

/**
 * Escape CSV field values
 */
function escapeCSV(value: string): string {
	if (value.includes(',') || value.includes('"') || value.includes('\n')) {
		return `"${value.replace(/"/g, '""')}"`;
	}
	return value;
}

/**
 * Format bytes for CSV export
 */
export function formatBytesForExport(bytes: number): string {
	const units = ['B', 'KB', 'MB', 'GB', 'TB'];
	let size = bytes;
	let unitIndex = 0;

	while (size >= 1024 && unitIndex < units.length - 1) {
		size /= 1024;
		unitIndex++;
	}

	return `${size.toFixed(2)} ${units[unitIndex]}`;
}
