/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	darkMode: 'class',
	theme: {
		extend: {
			colors: {
				primary: {
					50: '#f0f9ff',
					100: '#e0f2fe',
					200: '#bae6fd',
					300: '#7dd3fc',
					400: '#38bdf8',
					500: '#0ea5e9',
					600: '#0284c7',
					700: '#0369a1',
					800: '#075985',
					900: '#0c4a6e',
					950: '#082f49'
				},
				safe: {
					light: '#dcfce7',
					DEFAULT: '#22c55e',
					dark: '#166534'
				},
				caution: {
					light: '#fef9c3',
					DEFAULT: '#eab308',
					dark: '#854d0e'
				},
				danger: {
					light: '#fed7aa',
					DEFAULT: '#f97316',
					dark: '#9a3412'
				},
				critical: {
					light: '#fecaca',
					DEFAULT: '#ef4444',
					dark: '#991b1b'
				}
			},
			fontFamily: {
				sans: [
					'Inter',
					'ui-sans-serif',
					'system-ui',
					'-apple-system',
					'sans-serif'
				],
				mono: ['JetBrains Mono', 'Fira Code', 'monospace']
			},
			animation: {
				'fade-in': 'fadeIn 0.2s ease-out',
				'slide-in': 'slideIn 0.2s ease-out',
				'spin-slow': 'spin 2s linear infinite'
			},
			keyframes: {
				fadeIn: {
					'0%': { opacity: '0' },
					'100%': { opacity: '1' }
				},
				slideIn: {
					'0%': { opacity: '0', transform: 'translateY(-10px)' },
					'100%': { opacity: '1', transform: 'translateY(0)' }
				}
			}
		}
	},
	plugins: []
};
