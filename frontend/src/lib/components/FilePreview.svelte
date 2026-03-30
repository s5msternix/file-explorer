<script>
	import ImagePreview from './ImagePreview.svelte';
	import TextViewer from './TextViewer.svelte';

	/** @type {{ entry: import('../types').FileEntry | null, onClose: () => void }} */
	let { entry, onClose } = $props();

	const IMAGE_EXTENSIONS = ['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp', 'bmp', 'ico'];
	const TEXT_EXTENSIONS = [
		'txt', 'md', 'json', 'xml', 'yaml', 'yml', 'toml', 'ini', 'cfg', 'conf', 'log',
		'js', 'ts', 'jsx', 'tsx', 'svelte', 'vue', 'html', 'css', 'scss', 'less',
		'py', 'rs', 'go', 'java', 'c', 'cpp', 'h', 'hpp', 'cs', 'rb', 'php',
		'sh', 'bash', 'zsh', 'fish', 'bat', 'ps1',
		'sql', 'graphql', 'proto',
		'env', 'gitignore', 'editorconfig', 'prettierrc', 'eslintrc',
		'csv', 'tsv', 'lock',
	];

	let previewType = $derived.by(() => {
		if (!entry) return 'none';
		const ext = entry.extension?.toLowerCase();
		const name = entry.name.toLowerCase();
		if (ext && IMAGE_EXTENSIONS.includes(ext)) return 'image';
		if (ext && TEXT_EXTENSIONS.includes(ext)) return 'text';
		if (['makefile', 'dockerfile', 'rakefile', 'gemfile', 'procfile'].includes(name)) return 'text';
		return 'unsupported';
	});

	/** @param {number} bytes */
	function formatSize(bytes) {
		if (bytes === 0) return '0 B';
		const units = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(1024));
		const size = (bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0);
		return `${size} ${units[i]}`;
	}
</script>

{#if entry}
	<div class="preview-panel">
		<div class="preview-header">
			<div class="preview-title">
				<span class="preview-name" title={entry.name}>{entry.name}</span>
				{#if entry.size > 0}
					<span class="preview-size">{formatSize(entry.size)}</span>
				{/if}
			</div>
			<button class="close-btn" onclick={onClose} title="Kapat">
				<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
					<path d="M4.646 4.646a.5.5 0 01.708 0L8 7.293l2.646-2.647a.5.5 0 01.708.708L8.707 8l2.647 2.646a.5.5 0 01-.708.708L8 8.707l-2.646 2.647a.5.5 0 01-.708-.708L7.293 8 4.646 5.354a.5.5 0 010-.708z"/>
				</svg>
			</button>
		</div>

		<div class="preview-content">
			{#if previewType === 'image'}
				<ImagePreview {entry} />
			{:else if previewType === 'text'}
				<TextViewer {entry} />
			{:else}
				<div class="unsupported">
					<svg width="48" height="48" viewBox="0 0 24 24" fill="#d1d5db">
						<path fill-rule="evenodd" d="M4 4a2 2 0 012-2h8.586A2 2 0 0116 2.586L19.414 6A2 2 0 0120 7.414V20a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clip-rule="evenodd"/>
					</svg>
					<span>Bu dosya turu onizleme icin desteklenmiyor</span>
					{#if entry.extension}
						<span class="ext-badge">.{entry.extension}</span>
					{/if}
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.preview-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: #ffffff;
		overflow: hidden;
	}

	.preview-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 8px 12px;
		background: #f0f2f5;
		border-bottom: 1px solid #d1d5db;
		gap: 8px;
		flex-shrink: 0;
	}

	.preview-title {
		display: flex;
		align-items: center;
		gap: 8px;
		overflow: hidden;
	}

	.preview-name {
		font-size: 13px;
		font-weight: 600;
		color: #1f2937;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.preview-size {
		font-size: 11px;
		color: #6b7280;
		white-space: nowrap;
		flex-shrink: 0;
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border: none;
		border-radius: 4px;
		background: none;
		cursor: pointer;
		color: #6b7280;
		flex-shrink: 0;
	}

	.close-btn:hover {
		background: #e5e7eb;
		color: #1f2937;
	}

	.preview-content {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	.unsupported {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		flex: 1;
		color: #9ca3af;
		font-size: 13px;
		padding: 32px;
	}

	.ext-badge {
		display: inline-block;
		padding: 2px 8px;
		background: #f3f4f6;
		border-radius: 4px;
		font-size: 12px;
		font-family: monospace;
		color: #6b7280;
	}
</style>
