<script>
	import { getImageUrl } from '../api.js';

	/** @type {{ entry: import('../types').FileEntry }} */
	let { entry } = $props();

	let imageError = $state(false);
	let imageLoading = $state(true);

	let src = $derived(getImageUrl(entry.path));

	$effect(() => {
		// Reset states when entry changes
		entry.path;
		imageError = false;
		imageLoading = true;
	});
</script>

<div class="image-preview">
	{#if imageLoading && !imageError}
		<div class="image-loading">
			<div class="spinner"></div>
			<span>Resim yukleniyor...</span>
		</div>
	{/if}

	{#if imageError}
		<div class="image-error">
			<svg width="48" height="48" viewBox="0 0 24 24" fill="#d1d5db">
				<path fill-rule="evenodd" d="M4 3a2 2 0 00-2 2v14a2 2 0 002 2h16a2 2 0 002-2V5a2 2 0 00-2-2H4zm16 16H4l4-8 3 6 2-4 3 6z" clip-rule="evenodd"/>
			</svg>
			<span>Resim yuklenemedi</span>
		</div>
	{:else}
		<img
			{src}
			alt={entry.name}
			class="preview-image"
			class:hidden={imageLoading}
			onload={() => { imageLoading = false; }}
			onerror={() => { imageError = true; imageLoading = false; }}
		/>
	{/if}
</div>

<style>
	.image-preview {
		display: flex;
		align-items: center;
		justify-content: center;
		flex: 1;
		overflow: auto;
		padding: 16px;
		background: #f9fafb;
	}

	.preview-image {
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12);
	}

	.preview-image.hidden {
		display: none;
	}

	.image-loading, .image-error {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		color: #9ca3af;
		font-size: 13px;
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
</style>
