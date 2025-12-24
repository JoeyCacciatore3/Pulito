/// <reference types="vitest" />
import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}'],
		environment: 'jsdom',
		setupFiles: ['./src/test/setup.ts'],
		server: {
			deps: {
				inline: ['@testing-library/svelte']
			}
		},
		globals: true,
		testTimeout: 10000, // 10 second timeout for tests
		coverage: {
			provider: 'v8',
			reporter: ['text', 'json', 'html', 'lcov'],
			include: ['src/**/*.{ts,svelte}'],
			exclude: [
				'node_modules/',
				'src-tauri/',
				'build/',
				'src/**/*.test.ts',
				'src/**/*.spec.ts',
				'src/**/*.d.ts',
				'src/routes/**', // Marketing pages excluded from coverage
				'src/lib/components/ui/**' // UI components excluded (simple wrappers)
			],
			// Coverage thresholds - progressive goals
			thresholds: {
				lines: 50, // Start with 50%, increase over time
				functions: 45,
				branches: 40,
				statements: 50
			},
			// Report uncovered lines
			reportOnFailure: true
		},
		// Better test output
		outputFile: {
			json: './test-results/test-results.json'
		},
		// Test reporter configuration
		reporters: ['verbose', 'json', 'html']
	}
});
