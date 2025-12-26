/**
 * File type detection and categorization utilities for Linux
 * Provides functions to identify file types, categorize them, and assign colors
 */

/**
 * File type categories for Linux systems
 */
export type FileTypeCategory =
	| 'image'
	| 'video'
	| 'audio'
	| 'document'
	| 'archive'
	| 'code'
	| 'executable'
	| 'config'
	| 'database'
	| 'font'
	| 'other';

/**
 * File type category information
 */
export interface FileTypeInfo {
	category: FileTypeCategory;
	name: string;
	description: string;
}

/**
 * Common file extensions mapped to categories
 * Based on Linux file type associations and MIME types
 */
const FILE_EXTENSION_MAP: Record<string, FileTypeCategory> = {
	// Images
	jpg: 'image',
	jpeg: 'image',
	png: 'image',
	gif: 'image',
	svg: 'image',
	webp: 'image',
	bmp: 'image',
	ico: 'image',
	tiff: 'image',
	tif: 'image',
	heic: 'image',
	heif: 'image',
	avif: 'image',
	raw: 'image',
	cr2: 'image',
	nef: 'image',
	orf: 'image',
	sr2: 'image',

	// Videos
	mp4: 'video',
	avi: 'video',
	mkv: 'video',
	webm: 'video',
	mov: 'video',
	wmv: 'video',
	flv: 'video',
	m4v: 'video',
	mpg: 'video',
	mpeg: 'video',
	'3gp': 'video',
	ogv: 'video',

	// Audio
	mp3: 'audio',
	flac: 'audio',
	wav: 'audio',
	aac: 'audio',
	ogg: 'audio',
	oga: 'audio',
	m4a: 'audio',
	wma: 'audio',
	opus: 'audio',

	// Documents
	pdf: 'document',
	doc: 'document',
	docx: 'document',
	odt: 'document',
	rtf: 'document',
	txt: 'document',
	md: 'document',
	tex: 'document',
	xls: 'document',
	xlsx: 'document',
	ods: 'document',
	ppt: 'document',
	pptx: 'document',
	odp: 'document',
	csv: 'document',

	// Archives
	zip: 'archive',
	tar: 'archive',
	gz: 'archive',
	bz2: 'archive',
	xz: 'archive',
	'7z': 'archive',
	rar: 'archive',
	deb: 'archive',
	rpm: 'archive',
	tgz: 'archive',
	tbz2: 'archive',
	txz: 'archive',

	// Code
	js: 'code',
	ts: 'code',
	jsx: 'code',
	tsx: 'code',
	py: 'code',
	java: 'code',
	c: 'code',
	cpp: 'code',
	cc: 'code',
	cxx: 'code',
	h: 'code',
	hpp: 'code',
	cs: 'code',
	go: 'code',
	rs: 'code',
	rb: 'code',
	php: 'code',
	swift: 'code',
	kt: 'code',
	scala: 'code',
	bash: 'code',
	zsh: 'code',
	fish: 'code',
	json: 'code',
	xml: 'code',
	html: 'code',
	css: 'code',
	scss: 'code',
	sass: 'code',
	less: 'code',
	sql: 'code',
	yaml: 'code',
	yml: 'code',
	toml: 'code',
	makefile: 'code',
	cmake: 'code',
	dockerfile: 'code',

	// Executables
	exe: 'executable',
	bin: 'executable',
	appimage: 'executable',
	run: 'executable',
	sh: 'executable',

	// Config files
	conf: 'config',
	config: 'config',
	cfg: 'config',
	ini: 'config',
	properties: 'config',
	env: 'config',
	gitconfig: 'config',
	gitignore: 'config',
	editorconfig: 'config',
	prettierrc: 'config',
	eslintrc: 'config',

	// Databases
	db: 'database',
	sqlite: 'database',
	sqlite3: 'database',
	mdb: 'database',
	accdb: 'database',

	// Fonts
	ttf: 'font',
	otf: 'font',
	woff: 'font',
	woff2: 'font',
	eot: 'font'
};

/**
 * Category information mapping
 */
const CATEGORY_INFO: Record<FileTypeCategory, FileTypeInfo> = {
	image: {
		category: 'image',
		name: 'Images',
		description: 'Image files (JPG, PNG, GIF, etc.)'
	},
	video: {
		category: 'video',
		name: 'Videos',
		description: 'Video files (MP4, AVI, MKV, etc.)'
	},
	audio: {
		category: 'audio',
		name: 'Audio',
		description: 'Audio files (MP3, FLAC, WAV, etc.)'
	},
	document: {
		category: 'document',
		name: 'Documents',
		description: 'Document files (PDF, DOC, TXT, etc.)'
	},
	archive: {
		category: 'archive',
		name: 'Archives',
		description: 'Archive files (ZIP, TAR, GZ, etc.)'
	},
	code: {
		category: 'code',
		name: 'Code',
		description: 'Source code files'
	},
	executable: {
		category: 'executable',
		name: 'Executables',
		description: 'Executable files and installers'
	},
	config: {
		category: 'config',
		name: 'Config',
		description: 'Configuration files'
	},
	database: {
		category: 'database',
		name: 'Databases',
		description: 'Database files'
	},
	font: {
		category: 'font',
		name: 'Fonts',
		description: 'Font files'
	},
	other: {
		category: 'other',
		name: 'Other',
		description: 'Other file types'
	}
};

/**
 * Get file extension from path
 */
export function getFileExtension(path: string): string {
	if (!path) {
		return '';
	}

	const parts = path.split('.');
	if (parts.length < 2) {
		return '';
	}

	// Get last part (extension)
	const ext = parts[parts.length - 1].toLowerCase();

	// Handle special cases (e.g., .tar.gz)
	if (ext === 'gz' || ext === 'bz2' || ext === 'xz') {
		const prevPart = parts[parts.length - 2]?.toLowerCase();
		if (prevPart === 'tar') {
			return 'tar.' + ext;
		}
	}

	return ext;
}

/**
 * Get file type category from extension
 */
export function getFileTypeCategory(extension: string): FileTypeCategory {
	if (!extension) {
		return 'other';
	}

	const ext = extension.toLowerCase().replace(/^\./, ''); // Remove leading dot if present
	return FILE_EXTENSION_MAP[ext] || 'other';
}

/**
 * Get file type category from file path
 */
export function getFileTypeFromPath(path: string): FileTypeCategory {
	const extension = getFileExtension(path);
	return getFileTypeCategory(extension);
}

/**
 * Get file type information
 */
export function getFileTypeInfo(category: FileTypeCategory): FileTypeInfo {
	return CATEGORY_INFO[category] || CATEGORY_INFO.other;
}

/**
 * Get color for file type category (theme-aware)
 */
export function getFileTypeColor(category: FileTypeCategory, isDark: boolean = false): string {
	const colors = {
		dark: {
			image: 'rgb(59, 130, 246)', // Blue
			video: 'rgb(139, 92, 246)', // Purple
			audio: 'rgb(16, 185, 129)', // Green
			document: 'rgb(245, 158, 11)', // Yellow
			archive: 'rgb(251, 146, 60)', // Orange
			code: 'rgb(239, 68, 68)', // Red
			executable: 'rgb(236, 72, 153)', // Pink
			config: 'rgb(6, 182, 212)', // Cyan
			database: 'rgb(168, 85, 247)', // Violet
			font: 'rgb(34, 197, 94)', // Emerald
			other: 'rgb(156, 163, 175)' // Gray
		},
		light: {
			image: 'rgb(37, 99, 235)', // Blue
			video: 'rgb(124, 58, 237)', // Purple
			audio: 'rgb(5, 150, 105)', // Green
			document: 'rgb(217, 119, 6)', // Yellow
			archive: 'rgb(234, 88, 12)', // Orange
			code: 'rgb(220, 38, 38)', // Red
			executable: 'rgb(219, 39, 119)', // Pink
			config: 'rgb(8, 145, 178)', // Cyan
			database: 'rgb(147, 51, 234)', // Violet
			font: 'rgb(22, 163, 74)', // Emerald
			other: 'rgb(107, 114, 128)' // Gray
		}
	};

	return isDark ? colors.dark[category] : colors.light[category];
}

/**
 * Get all file type categories
 */
export function getAllFileTypeCategories(): FileTypeCategory[] {
	return Object.keys(CATEGORY_INFO) as FileTypeCategory[];
}

/**
 * Get file type statistics from nodes
 */
export function getFileTypeStatistics(nodes: Array<{ path: string; size: number; isDirectory: boolean }>): Map<FileTypeCategory, { count: number; totalSize: number }> {
	const stats = new Map<FileTypeCategory, { count: number; totalSize: number }>();

	// Initialize all categories
	getAllFileTypeCategories().forEach((category) => {
		stats.set(category, { count: 0, totalSize: 0 });
	});

	nodes.forEach((node) => {
		if (node.isDirectory) {
			return; // Skip directories
		}

		const category = getFileTypeFromPath(node.path);
		const current = stats.get(category) || { count: 0, totalSize: 0 };
		stats.set(category, {
			count: current.count + 1,
			totalSize: current.totalSize + node.size
		});
	});

	return stats;
}
