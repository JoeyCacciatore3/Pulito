import { describe, it, expect } from 'vitest';
import {
	getFileExtension,
	getFileTypeCategory,
	getFileTypeFromPath,
	getFileTypeInfo,
	getFileTypeColor,
	getAllFileTypeCategories,
	getFileTypeStatistics
} from './file-types';

describe('file-types', () => {
	describe('getFileExtension', () => {
		it('should extract extension from simple path', () => {
			expect(getFileExtension('file.txt')).toBe('txt');
			expect(getFileExtension('image.jpg')).toBe('jpg');
		});

		it('should handle paths with directories', () => {
			expect(getFileExtension('/path/to/file.pdf')).toBe('pdf');
			expect(getFileExtension('home/user/document.docx')).toBe('docx');
		});

		it('should handle case insensitivity', () => {
			expect(getFileExtension('FILE.PNG')).toBe('png');
			expect(getFileExtension('Image.JPG')).toBe('jpg');
		});

		it('should handle special cases like tar.gz', () => {
			expect(getFileExtension('archive.tar.gz')).toBe('tar.gz');
			expect(getFileExtension('archive.tar.bz2')).toBe('tar.bz2');
		});

		it('should return empty string for files without extension', () => {
			expect(getFileExtension('file')).toBe('');
			expect(getFileExtension('/path/to/file')).toBe('');
		});

		it('should return empty string for empty path', () => {
			expect(getFileExtension('')).toBe('');
		});
	});

	describe('getFileTypeCategory', () => {
		it('should categorize image files', () => {
			expect(getFileTypeCategory('jpg')).toBe('image');
			expect(getFileTypeCategory('png')).toBe('image');
			expect(getFileTypeCategory('gif')).toBe('image');
		});

		it('should categorize video files', () => {
			expect(getFileTypeCategory('mp4')).toBe('video');
			expect(getFileTypeCategory('avi')).toBe('video');
			expect(getFileTypeCategory('mkv')).toBe('video');
		});

		it('should categorize document files', () => {
			expect(getFileTypeCategory('pdf')).toBe('document');
			expect(getFileTypeCategory('docx')).toBe('document');
			expect(getFileTypeCategory('txt')).toBe('document');
		});

		it('should categorize code files', () => {
			expect(getFileTypeCategory('js')).toBe('code');
			expect(getFileTypeCategory('ts')).toBe('code');
			expect(getFileTypeCategory('py')).toBe('code');
		});

		it('should return other for unknown extensions', () => {
			expect(getFileTypeCategory('unknown')).toBe('other');
			expect(getFileTypeCategory('xyz')).toBe('other');
		});

		it('should handle empty extension', () => {
			expect(getFileTypeCategory('')).toBe('other');
		});

		it('should handle extension with leading dot', () => {
			expect(getFileTypeCategory('.jpg')).toBe('image');
			expect(getFileTypeCategory('.png')).toBe('image');
		});
	});

	describe('getFileTypeFromPath', () => {
		it('should get category from full path', () => {
			expect(getFileTypeFromPath('/home/user/image.jpg')).toBe('image');
			expect(getFileTypeFromPath('document.pdf')).toBe('document');
			expect(getFileTypeFromPath('script.js')).toBe('code');
		});

		it('should handle paths without extension', () => {
			expect(getFileTypeFromPath('/path/to/file')).toBe('other');
		});
	});

	describe('getFileTypeInfo', () => {
		it('should return correct info for each category', () => {
			const imageInfo = getFileTypeInfo('image');
			expect(imageInfo.category).toBe('image');
			expect(imageInfo.name).toBe('Images');
			expect(imageInfo.description).toContain('Image files');

			const codeInfo = getFileTypeInfo('code');
			expect(codeInfo.category).toBe('code');
			expect(codeInfo.name).toBe('Code');
		});

		it('should return other for invalid category', () => {
			const info = getFileTypeInfo('invalid' as any);
			expect(info.category).toBe('other');
		});
	});

	describe('getFileTypeColor', () => {
		it('should return different colors for different categories', () => {
			const imageColor = getFileTypeColor('image', false);
			const videoColor = getFileTypeColor('video', false);
			expect(imageColor).not.toBe(videoColor);
		});

		it('should return theme-aware colors', () => {
			const darkColor = getFileTypeColor('image', true);
			const lightColor = getFileTypeColor('image', false);
			expect(darkColor).not.toBe(lightColor);
		});

		it('should return valid RGB colors', () => {
			const color = getFileTypeColor('image', false);
			expect(color).toMatch(/^rgb\(\d+, \d+, \d+\)$/);
		});
	});

	describe('getAllFileTypeCategories', () => {
		it('should return all categories', () => {
			const categories = getAllFileTypeCategories();
			expect(categories.length).toBeGreaterThan(0);
			expect(categories).toContain('image');
			expect(categories).toContain('video');
			expect(categories).toContain('other');
		});
	});

	describe('getFileTypeStatistics', () => {
		it('should calculate statistics correctly', () => {
			const nodes = [
				{ path: '/file1.jpg', size: 1000, isDirectory: false },
				{ path: '/file2.png', size: 2000, isDirectory: false },
				{ path: '/file3.pdf', size: 3000, isDirectory: false },
				{ path: '/dir1', size: 0, isDirectory: true }
			];

			const stats = getFileTypeStatistics(nodes);
			const imageStats = stats.get('image');
			expect(imageStats?.count).toBe(2);
			expect(imageStats?.totalSize).toBe(3000);

			const docStats = stats.get('document');
			expect(docStats?.count).toBe(1);
			expect(docStats?.totalSize).toBe(3000);
		});

		it('should skip directories', () => {
			const nodes = [
				{ path: '/dir1', size: 1000, isDirectory: true },
				{ path: '/file.txt', size: 500, isDirectory: false }
			];

			const stats = getFileTypeStatistics(nodes);
			const docStats = stats.get('document');
			expect(docStats?.count).toBe(1);
			expect(docStats?.totalSize).toBe(500);
		});

		it('should handle empty nodes', () => {
			const stats = getFileTypeStatistics([]);
			const imageStats = stats.get('image');
			expect(imageStats?.count).toBe(0);
			expect(imageStats?.totalSize).toBe(0);
		});
	});
});
