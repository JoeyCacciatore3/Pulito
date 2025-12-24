// Navigation store using Svelte 5 runes

export type View = 'dashboard' | 'cleanup' | 'analytics' | 'treeview' | 'filesystem-health' | 'trash' | 'settings';

let currentView = $state<View>('dashboard');

// Use $derived to ensure reactivity
export const navigation = {
	get view() {
		// Access the state directly to ensure reactivity tracking
		return currentView;
	},
	set(view: View) {
		console.log('Navigation: Setting view to', view);
		currentView = view;
	}
};
