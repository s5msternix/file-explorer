<script>
	import FileItem from './FileItem.svelte';

	/** @type {{ entries: import('../types').FileEntry[], selectedPaths: Set<string>, onSelect: (entry: import('../types').FileEntry, event: MouseEvent) => void, onOpen: (entry: import('../types').FileEntry) => void }} */
	let { entries, selectedPaths, onSelect, onOpen } = $props();
</script>

<div class="file-list">
	<div class="file-list-header">
		<div class="header-icon"></div>
		<div class="header-name">Ad</div>
		<div class="header-size">Boyut</div>
		<div class="header-modified">Degistirilme Tarihi</div>
	</div>

	<div class="file-list-body">
		{#if entries.length === 0}
			<div class="empty-state">
				<svg width="48" height="48" viewBox="0 0 48 48" fill="#d1d5db">
					<path d="M8 8a4 4 0 014-4h7.172a4 4 0 012.828 1.172l2.828 2.828A4 4 0 0027.656 9H36a4 4 0 014 4v22a4 4 0 01-4 4H12a4 4 0 01-4-4V8z"/>
				</svg>
				<p>Bu dizin bos</p>
			</div>
		{:else}
			{#each entries as entry (entry.path)}
				<FileItem
					{entry}
					selected={selectedPaths.has(entry.path)}
					{onSelect}
					{onOpen}
				/>
			{/each}
		{/if}
	</div>
</div>

<style>
	.file-list {
		display: flex;
		flex-direction: column;
		flex: 1;
		overflow: hidden;
	}

	.file-list-header {
		display: grid;
		grid-template-columns: 28px 1fr 90px 160px;
		align-items: center;
		padding: 6px 12px;
		gap: 8px;
		background: #f9fafb;
		border-bottom: 2px solid #e5e7eb;
		font-size: 11px;
		font-weight: 600;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		user-select: none;
	}

	.header-icon {
		width: 28px;
	}

	.header-size,
	.header-modified {
		text-align: right;
	}

	.file-list-body {
		flex: 1;
		overflow-y: auto;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		padding: 48px 16px;
		color: #9ca3af;
		font-size: 14px;
	}
</style>
