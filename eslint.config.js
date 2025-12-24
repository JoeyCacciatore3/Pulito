import js from '@eslint/js';
import ts from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import svelte from 'eslint-plugin-svelte';
import svelteParser from 'svelte-eslint-parser';

export default [
	js.configs.recommended,
	{
		files: ['**/*.ts'],
		plugins: {
			'@typescript-eslint': ts
		},
		languageOptions: {
			parser: tsParser,
			parserOptions: {
				ecmaVersion: 2022,
				sourceType: 'module',
				project: './tsconfig.json'
			}
		},
		rules: {
			...ts.configs.recommended.rules,
			'@typescript-eslint/no-unused-vars': ['error', {
				argsIgnorePattern: '^_',
				varsIgnorePattern: '^_',
				caughtErrorsIgnorePattern: '^_'
			}],
			'no-unused-vars': ['error', {
				argsIgnorePattern: '^_',
				varsIgnorePattern: '^_',
				caughtErrorsIgnorePattern: '^_'
			}],
			'@typescript-eslint/no-explicit-any': 'warn',
			'@typescript-eslint/prefer-nullish-coalescing': 'error',
			'@typescript-eslint/prefer-optional-chain': 'error',
			'prefer-const': 'error',
			'no-var': 'error',
			'no-undef': 'off' // Let TypeScript handle this
		}
	},
	{
		files: ['**/*.js'],
		rules: {
			'prefer-const': 'error',
			'no-var': 'error',
			'no-undef': 'off'
		}
	},
	{
		files: ['**/*.svelte'],
		plugins: {
			svelte
		},
		languageOptions: {
			parser: svelteParser,
			parserOptions: {
				parser: tsParser,
				extraFileExtensions: ['.svelte']
			}
		},
		rules: {
			...svelte.configs.recommended.rules,
			'svelte/no-at-html-tags': 'off',
			'svelte/valid-compile': 'off',
			'no-undef': 'off', // Svelte handles globals
			'no-unused-vars': ['error', {
				argsIgnorePattern: '^_',
				varsIgnorePattern: '^_',
				caughtErrorsIgnorePattern: '^_',
				ignoreRestSiblings: true
			}]
		}
	},
	{
		files: ['src/test/**'],
		languageOptions: {
			globals: {
				window: 'readonly',
				console: 'readonly',
				global: 'readonly'
			}
		}
	},
	{
		ignores: [
			'build/',
			'node_modules/',
			'src-tauri/',
			'.svelte-kit/',
			'vitest.config.ts',
			'html/',
			'coverage/'
		]
	}
];
