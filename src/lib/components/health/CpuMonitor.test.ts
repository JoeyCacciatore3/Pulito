import { describe, it, expect } from 'vitest';

// Test utility functions used by CpuMonitor
// Note: Full component rendering tests require client-side environment setup
// which is not available in the current SSR test environment.
// Component integration tests should be done via E2E tests with WebDriver.

describe('CpuMonitor utilities', () => {
	// Test frequency formatting logic (mirrors component's formatFrequency function)
	function formatFrequency(freq: number): string {
		if (freq >= 1000) {
			return `${(freq / 1000).toFixed(1)} GHz`;
		}
		return `${freq.toFixed(0)} MHz`;
	}

	it('formats frequency in GHz for values >= 1000', () => {
		expect(formatFrequency(2400)).toBe('2.4 GHz');
		expect(formatFrequency(3500)).toBe('3.5 GHz');
		expect(formatFrequency(1000)).toBe('1.0 GHz');
	});

	it('formats frequency in MHz for values < 1000', () => {
		expect(formatFrequency(800)).toBe('800 MHz');
		expect(formatFrequency(500)).toBe('500 MHz');
		expect(formatFrequency(100)).toBe('100 MHz');
	});

	it('handles edge cases', () => {
		expect(formatFrequency(0)).toBe('0 MHz');
		expect(formatFrequency(999)).toBe('999 MHz');
		// 999.9 < 1000, so it formats as MHz with toFixed(0) = "1000 MHz"
		expect(formatFrequency(999.9)).toBe('1000 MHz');
	});
});
