// Theme store using Svelte 5 runes
// This file uses .svelte.ts extension for runes support outside components

export type Theme = 'light' | 'dark' | 'system';

let currentTheme = $state<Theme>('system');

export const theme = {
	get value() {
		return currentTheme;
	},
	set(value: Theme) {
		currentTheme = value;
		applyTheme(value);
		if (typeof localStorage !== 'undefined') {
			localStorage.setItem('theme', value);
		}
	},
	toggle() {
		const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
		this.set(newTheme);
	}
};

export function initTheme() {
	if (typeof window === 'undefined') return;

	const stored = localStorage.getItem('theme') as Theme | null;
	const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

	if (stored) {
		currentTheme = stored;
	} else {
		currentTheme = prefersDark ? 'dark' : 'light';
	}

	applyTheme(currentTheme);

	// Listen for system theme changes
	window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (_e) => {
		if (currentTheme === 'system') {
			applyTheme('system');
		}
	});
}

function applyTheme(value: Theme) {
	if (typeof document === 'undefined') return;

	const isDark =
		value === 'dark' ||
		(value === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);

	document.documentElement.classList.toggle('dark', isDark);
}
