import { describe, it, expect } from 'vitest';
import { formatBytes, formatRelativeTime, getRiskLevelInfo } from './tauri';

describe('tauri utilities', () => {
	describe('formatBytes', () => {
		it('formats bytes correctly', () => {
			expect(formatBytes(0)).toBe('0 B');
			expect(formatBytes(1024)).toBe('1 KB');
			expect(formatBytes(1024 * 1024)).toBe('1 MB');
			expect(formatBytes(1024 * 1024 * 1024)).toBe('1 GB');
		});

		it('handles decimals correctly', () => {
			expect(formatBytes(1536)).toBe('1.5 KB');
			expect(formatBytes(1024 * 1.5)).toBe('1.5 KB');
		});
	});

	describe('formatRelativeTime', () => {
		it('formats recent times correctly', () => {
			const now = new Date();
			const recent = new Date(now.getTime() - 300000); // 5 minutes ago
			expect(formatRelativeTime(recent.toISOString())).toContain('ago');
		});

		it('formats old dates correctly', () => {
			const old = new Date('2020-01-01');
			expect(formatRelativeTime(old.toISOString())).toMatch(/\d{1,2}\/\d{1,2}\/\d{4}/);
		});
	});

	describe('getRiskLevelInfo', () => {
		it('returns correct risk info for each level', () => {
			const level0 = getRiskLevelInfo(0);
			expect(level0.label).toBe('Safe');
			expect(level0.level).toBe(0);

			const level5 = getRiskLevelInfo(5);
			expect(level5.label).toBe('Critical');
			expect(level5.level).toBe(5);
		});

		it('handles out of bounds levels', () => {
			const level10 = getRiskLevelInfo(10);
			expect(level10.level).toBe(5); // Should default to highest level
		});
	});
});
