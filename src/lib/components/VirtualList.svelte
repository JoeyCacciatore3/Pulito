<script lang="ts">
	import { onMount } from 'svelte';
	import { calculateVirtualRange } from '$lib/utils/virtual-scroll';

	interface Props<T> {
		items: T[];
		itemHeight?: number;
		containerHeight?: number;
		overscan?: number;
		children: import('svelte').Snippet<[item: T, index: number]>;
	}

	let {
		items = [],
		itemHeight = 60,
		containerHeight = 400,
		overscan = 5,
		children
	}: Props<any> = $props();

	let containerRef = $state<HTMLDivElement | null>(null);
	let scrollTop = $state(0);
	let actualContainerHeight = $state(containerHeight);

	// Calculate visible range
	let virtualRange = $derived(() => {
		if (!containerRef || items.length === 0) {
			return {
				startIndex: 0,
				endIndex: 0,
				offsetY: 0,
				totalHeight: 0,
				visibleItems: 0
			};
		}

		return calculateVirtualRange({
			itemHeight,
			containerHeight: actualContainerHeight,
			scrollTop,
			totalItems: items.length,
			overscan
		});
	});

	// Get visible items
	let visibleItems = $derived(() => {
		const range = virtualRange();
		return items.slice(range.startIndex, range.endIndex + 1).map((item, idx) => ({
			item,
			index: range.startIndex + idx
		}));
	});

	// Handle scroll
	function handleScroll(event: Event) {
		const target = event.target as HTMLDivElement;
		scrollTop = target.scrollTop;
	}

	// Update container height on resize
	function updateHeight() {
		if (containerRef) {
			actualContainerHeight = containerRef.clientHeight;
		}
	}

	onMount(() => {
		updateHeight();
		const resizeObserver = new ResizeObserver(updateHeight);
		if (containerRef) {
			resizeObserver.observe(containerRef);
		}
		return () => {
			if (containerRef) {
				resizeObserver.unobserve(containerRef);
			}
			resizeObserver.disconnect();
		};
	});
</script>

<div
	bind:this={containerRef}
	class="virtual-list-container overflow-y-auto"
	onscroll={handleScroll}
	style="height: {containerHeight}px;"
	role="list"
	aria-label="Virtual scrolling list"
>
	<div
		class="virtual-list-content"
		style="height: {virtualRange().totalHeight}px; position: relative;"
	>
		<div
			class="virtual-list-items"
			style="transform: translateY({virtualRange().offsetY}px);"
		>
			{#each visibleItems() as { item, index }}
				<div
					class="virtual-list-item"
					style="height: {itemHeight}px;"
					role="listitem"
					data-index={index}
				>
					{@render children(item, index)}
				</div>
			{/each}
		</div>
	</div>
</div>

<style>
	.virtual-list-container {
		position: relative;
		overflow-anchor: none;
	}

	.virtual-list-content {
		position: relative;
		width: 100%;
	}

	.virtual-list-items {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		will-change: transform;
	}

	.virtual-list-item {
		position: relative;
		width: 100%;
	}
</style>
