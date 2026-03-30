<script>
	import Breadcrumb from './Breadcrumb.svelte';
	import FileList from './FileList.svelte';
	import StatusBar from './StatusBar.svelte';
	import { browse } from '../api.js';

	/**
	 * @type {{
	 *   apiBase?: string,
	 *   onSelectionChange?: (selected: import('../types').FileEntry[]) => void
	 * }}
	 */
	let { apiBase, onSelectionChange } = $props();

	/** @type {import('../types').FileEntry[]} */
	let entries = $state([]);
	let currentPath = $state('/');
	/** @type {string|null} */
	let parentPath = $state(null);
	/** @type {Set<string>} */
	let selectedPaths = $state(new Set());
	let loading = $state(false);
	/** @type {string|null} */
	let error = $state(null);
	/** @type {string|null} */
	let lastSelectedPath = $state(null);

	let selectedEntries = $derived(
		entries.filter((e) => selectedPaths.has(e.path))
	);

	$effect(() => {
		onSelectionChange?.(selectedEntries);
	});

	async function loadDirectory(/** @type {string|null} */ path) {
		loading = true;
		error = null;
		try {
			const data = await browse(path ?? undefined);
			entries = data.entries;
			currentPath = data.current_path;
			parentPath = data.parent_path;
			selectedPaths = new Set();
			lastSelectedPath = null;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			loading = false;
		}
	}

	/** @param {import('../types').FileEntry} entry */
	/** @param {MouseEvent} event */
	function handleSelect(entry, event) {
		const newSelection = new Set(selectedPaths);

		if (event.ctrlKey || event.metaKey) {
			if (newSelection.has(entry.path)) {
				newSelection.delete(entry.path);
			} else {
				newSelection.add(entry.path);
			}
			lastSelectedPath = entry.path;
		} else if (event.shiftKey && lastSelectedPath) {
			const lastIndex = entries.findIndex((e) => e.path === lastSelectedPath);
			const currentIndex = entries.findIndex((e) => e.path === entry.path);
			if (lastIndex !== -1 && currentIndex !== -1) {
				const start = Math.min(lastIndex, currentIndex);
				const end = Math.max(lastIndex, currentIndex);
				for (let i = start; i <= end; i++) {
					newSelection.add(entries[i].path);
				}
			}
		} else {
			newSelection.clear();
			newSelection.add(entry.path);
			lastSelectedPath = entry.path;
		}

		selectedPaths = newSelection;
	}

	/** @param {import('../types').FileEntry} entry */
	function handleOpen(entry) {
		if (entry.is_dir) {
			loadDirectory(entry.path);
		}
	}

	function handleNavigate(/** @type {string|null} */ path) {
		loadDirectory(path);
	}

	function goUp() {
		if (parentPath !== null) {
			loadDirectory(parentPath);
		} else if (currentPath !== '/') {
			loadDirectory(null);
		}
	}

	function refresh() {
		loadDirectory(currentPath === '/' ? null : currentPath);
	}

	loadDirectory(null);
</script>

<div class="file-explorer">
	<div class="toolbar">
		<button
			class="toolbar-btn"
			onclick={goUp}
			disabled={currentPath === '/'}
			title="Ust dizine git"
		>
			<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
				<path d="M8 3.293l-6 6V14h4v-4h4v4h4V9.293l-6-6zM8 1L0 9h3v6h4v-4h2v4h4V9h3L8 1z"/>
			</svg>
		</button>
		<button
			class="toolbar-btn"
			onclick={() => { if (parentPath !== null) loadDirectory(parentPath); else if (currentPath !== '/') loadDirectory(null); }}
			disabled={currentPath === '/' && parentPath === null}
			title="Geri"
		>
			<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
				<path d="M11 2L5 8l6 6V2z"/>
			</svg>
		</button>
		<button class="toolbar-btn" onclick={refresh} title="Yenile">
			<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
				<path d="M13.65 2.35A7.958 7.958 0 008 0C3.58 0 0 3.58 0 8s3.58 8 8 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 018 14 6 6 0 118 2c1.66 0 3.14.69 4.22 1.78L9 7h7V0l-2.35 2.35z"/>
			</svg>
		</button>
	</div>

	<Breadcrumb path={currentPath} onNavigate={handleNavigate} />

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<span>Yukleniyor...</span>
		</div>
	{:else if error}
		<div class="error">
			<svg width="24" height="24" viewBox="0 0 24 24" fill="#ef4444">
				<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
			</svg>
			<p>{error}</p>
			<button class="retry-btn" onclick={refresh}>Tekrar Dene</button>
		</div>
	{:else}
		<FileList
			{entries}
			{selectedPaths}
			onSelect={handleSelect}
			onOpen={handleOpen}
		/>
	{/if}

	<StatusBar
		entryCount={entries.length}
		selectedCount={selectedPaths.size}
		{currentPath}
	/>
</div>

<style>
	.file-explorer {
		display: flex;
		flex-direction: column;
		height: 100%;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		overflow: hidden;
		background: #ffffff;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
	}

	.toolbar {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 6px 8px;
		background: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: 1px solid transparent;
		border-radius: 6px;
		background: none;
		cursor: pointer;
		color: #4b5563;
		transition: all 0.15s;
	}

	.toolbar-btn:hover:not(:disabled) {
		background: #e5e7eb;
		border-color: #d1d5db;
		color: #1f2937;
	}

	.toolbar-btn:disabled {
		opacity: 0.35;
		cursor: not-allowed;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		flex: 1;
		padding: 48px 16px;
		color: #6b7280;
		font-size: 14px;
	}

	.spinner {
		width: 24px;
		height: 24px;
		border: 3px solid #e5e7eb;
		border-top-color: #2563eb;
		border-radius: 50%;
		animation: spin 0.7s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.error {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 8px;
		flex: 1;
		padding: 48px 16px;
		color: #ef4444;
		font-size: 14px;
	}

	.error p {
		margin: 0;
	}

	.retry-btn {
		margin-top: 8px;
		padding: 6px 16px;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
		cursor: pointer;
		color: #374151;
		font-size: 13px;
	}

	.retry-btn:hover {
		background: #f3f4f6;
	}
</style>
