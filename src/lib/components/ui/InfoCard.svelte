<script lang="ts">
	interface Props {
		title: string;
		icon?: string;
		subtitle?: string;
		children: any;
		class?: string;
		headerClass?: string;
		contentClass?: string;
	}

	let {
		title,
		icon,
		subtitle,
		children,
		class: className = '',
		headerClass = '',
		contentClass = ''
	}: Props = $props();
</script>

<div class="card {className}">
	{#if title || icon}
		<div class="flex items-center justify-between mb-4 {headerClass}">
			<div class="flex items-center gap-3">
				{#if icon}
					<div class="w-4 h-4 flex items-center justify-center">
						<span class="text-xl">{icon}</span>
					</div>
				{/if}
				<div>
					<h3 class="font-semibold">{title}</h3>
					{#if subtitle}
						<p class="text-sm text-[var(--color-text-secondary)]">{subtitle}</p>
					{/if}
				</div>
			</div>
		</div>
	{/if}

	<div class={contentClass}>
		{@render children()}
	</div>
</div>
