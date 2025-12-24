import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	clearScreen: false,
	server: {
		port: 5174,
		strictPort: false
	},
	build: {
		target: 'es2024',
		minify: 'esbuild',
		sourcemap: false,
		chunkSizeWarningLimit: 1000,
		rollupOptions: {
			output: {
				manualChunks: {
					vendor: ['svelte'],
					router: ['$app/navigation', '$app/stores']
				}
			}
		},
		cssCodeSplit: true,
		assetsInlineLimit: 4096
	},
	esbuild: {
		treeShaking: true,
		target: 'es2024'
	},
	css: {
		devSourcemap: true
	},
	define: {
		global: 'globalThis'
	}
});
