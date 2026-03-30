<script>
	/** @type {{ entry: import('../types').FileEntry, selected: boolean, onSelect: (entry: import('../types').FileEntry, event: MouseEvent) => void, onOpen: (entry: import('../types').FileEntry) => void }} */
	let { entry, selected, onSelect, onOpen } = $props();

	/** @param {number} bytes */
	function formatSize(bytes) {
		if (bytes === 0) return '';
		const units = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(1024));
		const size = (bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0);
		return `${size} ${units[i]}`;
	}

	function getIcon(entry) {
		if (entry.is_dir) return 'folder';
		const ext = entry.extension?.toLowerCase();
		if (['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp', 'bmp', 'ico'].includes(ext)) return 'image';
		if (['mp4', 'avi', 'mov', 'mkv', 'webm'].includes(ext)) return 'video';
		if (['mp3', 'wav', 'ogg', 'flac', 'aac'].includes(ext)) return 'audio';
		if (['zip', 'tar', 'gz', 'rar', '7z', 'bz2'].includes(ext)) return 'archive';
		if (['js', 'ts', 'py', 'rs', 'go', 'java', 'c', 'cpp', 'h', 'css', 'html', 'svelte', 'vue', 'jsx', 'tsx'].includes(ext)) return 'code';
		if (['md', 'txt', 'json', 'xml', 'yaml', 'yml', 'toml', 'ini', 'cfg', 'conf', 'log'].includes(ext)) return 'text';
		if (['pdf'].includes(ext)) return 'pdf';
		return 'file';
	}

	let iconType = $derived(getIcon(entry));
</script>

<div
	class="file-item"
	class:selected
	class:is-dir={entry.is_dir}
	onclick={(e) => onSelect(entry, e)}
	ondblclick={() => onOpen(entry)}
	role="row"
	tabindex="0"
	onkeydown={(e) => {
		if (e.key === 'Enter') onOpen(entry);
	}}
>
	<div class="file-icon">
		{#if iconType === 'folder'}
			<svg width="20" height="20" viewBox="0 0 20 20" fill="#f59e0b">
				<path d="M2 4a2 2 0 012-2h3.586a1 1 0 01.707.293l1.414 1.414A1 1 0 0010.414 4H16a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V4z"/>
			</svg>
		{:else if iconType === 'image'}
			<svg width="20" height="20" viewBox="0 0 20 20" fill="#8b5cf6">
				<path fill-rule="evenodd" d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z" clip-rule="evenodd"/>
			</svg>
		{:else if iconType === 'code'}
			<svg width="20" height="20" viewBox="0 0 20 20" fill="#10b981">
				<path fill-rule="evenodd" d="M12.316 3.051a1 1 0 01.633 1.265l-4 12a1 1 0 11-1.898-.632l4-12a1 1 0 011.265-.633zM5.707 6.293a1 1 0 010 1.414L3.414 10l2.293 2.293a1 1 0 11-1.414 1.414l-3-3a1 1 0 010-1.414l3-3a1 1 0 011.414 0zm8.586 0a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 11-1.414-1.414L16.586 10l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd"/>
			</svg>
		{:else if iconType === 'archive'}
			<svg width="20" height="20" viewBox="0 0 20 20" fill="#f97316">
				<path d="M4 3a2 2 0 100 4h12a2 2 0 100-4H4z"/>
				<path fill-rule="evenodd" d="M3 8h14v7a2 2 0 01-2 2H5a2 2 0 01-2-2V8zm5 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" clip-rule="evenodd"/>
			</svg>
		{:else if iconType === 'pdf'}
			<svg width="20" height="20" viewBox="0 0 20 20" fill="#ef4444">
				<path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clip-rule="evenodd"/>
			</svg>
		{:else if iconType === 'text'}
			<svg width="20" height="20" viewBox="0 0 20 20" fill="#6b7280">
				<path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z" clip-rule="evenodd"/>
			</svg>
		{:else}
			<svg width="20" height="20" viewBox="0 0 20 20" fill="#9ca3af">
				<path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clip-rule="evenodd"/>
			</svg>
		{/if}
	</div>

	<div class="file-name">{entry.name}</div>

	<div class="file-size">{entry.is_dir ? '' : formatSize(entry.size)}</div>

	<div class="file-modified">{entry.modified ?? ''}</div>
</div>

<style>
	.file-item {
		display: grid;
		grid-template-columns: 28px 1fr 90px 160px;
		align-items: center;
		padding: 6px 12px;
		gap: 8px;
		cursor: pointer;
		border-bottom: 1px solid #f3f4f6;
		user-select: none;
		transition: background-color 0.1s;
	}

	.file-item:hover {
		background: #f9fafb;
	}

	.file-item.selected {
		background: #dbeafe;
	}

	.file-item.selected:hover {
		background: #bfdbfe;
	}

	.file-item:focus-visible {
		outline: 2px solid #2563eb;
		outline-offset: -2px;
	}

	.file-icon {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.file-name {
		font-size: 13px;
		color: #1f2937;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.is-dir .file-name {
		font-weight: 500;
	}

	.file-size {
		font-size: 12px;
		color: #6b7280;
		text-align: right;
	}

	.file-modified {
		font-size: 12px;
		color: #6b7280;
		text-align: right;
	}
</style>
