import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// https://v2.tauri.app/start/frontend/sveltekit/
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
	plugins: [sveltekit()],
	clearScreen: false,
	server: {
    		port: 5174,        // Changed from 5173 to avoid port conflicts
    		strictPort: false, // Allow fallback to other ports if 5174 is busy
		host: host ?? false,
		hmr: host ? {
			protocol: 'ws',
			host,
			port: 5174
		} : false,
		watch: {
			ignored: ['**/src-tauri/**']
		}
	},
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		target: process.env.TAURI_ENV_PLATFORM === 'windows' ? 'chrome105' : 'safari14',
		minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
		sourcemap: !!process.env.TAURI_ENV_DEBUG,
		// Performance optimizations
		chunkSizeWarningLimit: 1000,
		rollupOptions: {
			output: {
				manualChunks: {
					// Separate vendor chunks for better caching
					vendor: ['svelte'],
					router: ['$app/navigation', '$app/stores']
				}
			}
		},
		// Enable CSS code splitting
		cssCodeSplit: true,
		// Asset inlining threshold (small assets will be inlined)
		assetsInlineLimit: 4096
	},
	// Optimize dependencies
	optimizeDeps: {
		include: ['svelte', '@tauri-apps/api']
	},
	// December 2025 optimizations and modern features
	esbuild: {
		// Enable tree shaking optimizations
		treeShaking: true,
		// Use ES2024 features for cutting-edge performance
		target: 'es2024'
	},
	// Modern CSS optimizations
	css: {
		devSourcemap: true
	},
	// Enable modern JavaScript features
	define: {
		// Enable modern browser features
		global: 'globalThis'
	}
});
