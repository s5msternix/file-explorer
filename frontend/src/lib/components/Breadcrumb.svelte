<script>
	/** @type {{ path: string, onNavigate: (path: string|null) => void }} */
	let { path, onNavigate } = $props();

	let segments = $derived.by(() => {
		if (!path || path === '/') return [];
		const parts = path.split('/').filter(Boolean);
		let accumulated = '';
		return parts.map((part) => {
			accumulated += '/' + part;
			return { name: part, path: accumulated };
		});
	});
</script>

<nav class="breadcrumb">
	<button class="breadcrumb-item root" onclick={() => onNavigate(null)} aria-label="Ana dizin">
		<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
			<path d="M8 1L1 7h2v6h4V9h2v4h4V7h2L8 1z"/>
		</svg>
	</button>
	{#each segments as segment}
		<span class="separator">/</span>
		<button class="breadcrumb-item" onclick={() => onNavigate(segment.path)}>
			{segment.name}
		</button>
	{/each}
</nav>

<style>
	.breadcrumb {
		display: flex;
		align-items: center;
		gap: 2px;
		padding: 8px 12px;
		background: #f0f2f5;
		border-bottom: 1px solid #d1d5db;
		font-size: 13px;
		min-height: 36px;
		overflow-x: auto;
		white-space: nowrap;
	}

	.breadcrumb-item {
		background: none;
		border: none;
		cursor: pointer;
		padding: 2px 6px;
		border-radius: 4px;
		color: #2563eb;
		font-size: 13px;
		font-family: inherit;
		display: inline-flex;
		align-items: center;
	}

	.breadcrumb-item:hover {
		background: #dbeafe;
	}

	.breadcrumb-item.root {
		color: #6b7280;
	}

	.breadcrumb-item.root:hover {
		color: #2563eb;
	}

	.separator {
		color: #9ca3af;
		user-select: none;
	}
</style>
