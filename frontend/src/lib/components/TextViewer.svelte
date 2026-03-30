<script>
	import { fetchTextContent } from '../api.js';

	/** @type {{ entry: import('../types').FileEntry }} */
	let { entry } = $props();

	let content = $state('');
	let loading = $state(true);
	/** @type {string|null} */
	let error = $state(null);
	let truncated = $state(false);
	let fileSize = $state(0);
	let lineNumbers = $state('');

	$effect(() => {
		loadContent(entry.path);
	});

	/** @param {string} path */
	async function loadContent(path) {
		loading = true;
		error = null;
		content = '';
		try {
			const data = await fetchTextContent(path);
			content = data.content;
			truncated = data.truncated;
			fileSize = data.size;
			const lines = content.split('\n');
			lineNumbers = lines.map((_, i) => i + 1).join('\n');
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			loading = false;
		}
	}

	/** @param {number} bytes */
	function formatSize(bytes) {
		if (bytes === 0) return '0 B';
		const units = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(1024));
		const size = (bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0);
		return `${size} ${units[i]}`;
	}
</script>

<div class="text-viewer">
	{#if loading}
		<div class="text-loading">
			<div class="spinner"></div>
			<span>Dosya yukleniyor...</span>
		</div>
	{:else if error}
		<div class="text-error">
			<svg width="24" height="24" viewBox="0 0 24 24" fill="#ef4444">
				<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
			</svg>
			<span>{error}</span>
		</div>
	{:else}
		{#if truncated}
			<div class="truncation-notice">
				Dosya cok buyuk ({formatSize(fileSize)}). Ilk 512 KB gosteriliyor.
			</div>
		{/if}
		<div class="code-container">
			<pre class="line-numbers">{lineNumbers}</pre>
			<pre class="code-content">{content}</pre>
		</div>
	{/if}
</div>

<style>
	.text-viewer {
		display: flex;
		flex-direction: column;
		flex: 1;
		overflow: hidden;
		background: #1e1e1e;
	}

	.code-container {
		display: flex;
		flex: 1;
		overflow: auto;
		font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', 'Consolas', 'Monaco', monospace;
		font-size: 13px;
		line-height: 1.5;
	}

	.line-numbers {
		position: sticky;
		left: 0;
		padding: 12px 12px 12px 16px;
		margin: 0;
		background: #252526;
		color: #858585;
		text-align: right;
		user-select: none;
		border-right: 1px solid #333;
		min-width: 48px;
		flex-shrink: 0;
	}

	.code-content {
		padding: 12px 16px;
		margin: 0;
		color: #d4d4d4;
		white-space: pre;
		flex: 1;
		tab-size: 4;
	}

	.truncation-notice {
		padding: 6px 16px;
		background: #332b00;
		color: #e2c541;
		font-size: 12px;
		border-bottom: 1px solid #554400;
		flex-shrink: 0;
	}

	.text-loading, .text-error {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		flex: 1;
		color: #9ca3af;
		font-size: 13px;
	}

	.text-error {
		color: #ef4444;
	}

	.spinner {
		width: 24px;
		height: 24px;
		border: 3px solid #333;
		border-top-color: #2563eb;
		border-radius: 50%;
		animation: spin 0.7s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>
