import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: '404.html',
			precompress: false,
			strict: true
		}),
		paths: {
			base: '/Pulito'
		},
		alias: {
			$lib: './src/lib'
		},
		prerender: {
			entries: ['*'],
			handleHttpError: ({ path, referrer, message }) => {
				// Ignore errors for paths that don't include base - SvelteKit will handle these via routing
				if (message.includes('does not begin with `base`')) {
					return;
				}
				throw new Error(message);
			}
		}
	},
	compilerOptions: {
		runes: true
	}
};

export default config;
