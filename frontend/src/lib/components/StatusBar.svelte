<script>
	/** @type {{ entryCount: number, selectedCount: number, currentPath: string }} */
	let { entryCount, selectedCount, currentPath } = $props();
</script>

<div class="status-bar">
	<span class="status-path" title={currentPath}>{currentPath}</span>
	<span class="status-info">
		{entryCount} oge
		{#if selectedCount > 0}
			&middot; {selectedCount} secili
		{/if}
	</span>
</div>

<style>
	.status-bar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 4px 12px;
		background: #f0f2f5;
		border-top: 1px solid #d1d5db;
		font-size: 12px;
		color: #6b7280;
		min-height: 28px;
		gap: 16px;
	}

	.status-path {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		flex: 1;
	}

	.status-info {
		white-space: nowrap;
		flex-shrink: 0;
	}
</style>
