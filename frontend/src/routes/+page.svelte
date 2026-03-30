<script>
	import FileExplorer from '$lib/components/FileExplorer.svelte';

	let selectedFiles = $state([]);

	function handleSelectionChange(selected) {
		selectedFiles = selected;
	}
</script>

<div class="app">
	<header class="app-header">
		<h1>Dosya Gezgini</h1>
	</header>
	<main class="app-main">
		<FileExplorer onSelectionChange={handleSelectionChange} />
	</main>
	{#if selectedFiles.length > 0}
		<aside class="selection-panel">
			<h3>Secili Dosyalar ({selectedFiles.length})</h3>
			<ul>
				{#each selectedFiles as file (file.path)}
					<li class:is-dir={file.is_dir}>
						<span class="sel-icon">{file.is_dir ? '📁' : '📄'}</span>
						<span class="sel-name">{file.name}</span>
					</li>
				{/each}
			</ul>
		</aside>
	{/if}
</div>

<style>
	:global(*) {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
	}

	:global(html, body) {
		height: 100%;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
		background: #f3f4f6;
	}

	.app {
		display: grid;
		grid-template-rows: auto 1fr auto;
		height: 100vh;
		max-width: 960px;
		margin: 0 auto;
		padding: 16px;
		gap: 12px;
	}

	.app-header {
		text-align: center;
	}

	.app-header h1 {
		font-size: 20px;
		font-weight: 600;
		color: #1f2937;
	}

	.app-main {
		min-height: 0;
	}

	.selection-panel {
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		padding: 12px 16px;
		max-height: 180px;
		overflow-y: auto;
	}

	.selection-panel h3 {
		font-size: 13px;
		font-weight: 600;
		color: #374151;
		margin-bottom: 8px;
	}

	.selection-panel ul {
		list-style: none;
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.selection-panel li {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 12px;
		color: #4b5563;
		padding: 2px 0;
	}

	.sel-icon {
		font-size: 14px;
	}

	.sel-name {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>
