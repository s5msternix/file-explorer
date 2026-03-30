const BASE_URL = '/api';

/**
 * @param {string} [path]
 * @returns {Promise<import('./types').BrowseResponse>}
 */
export async function browse(path) {
	const url = path ? `${BASE_URL}/browse?path=${encodeURIComponent(path)}` : `${BASE_URL}/browse`;
	const res = await fetch(url);
	if (!res.ok) {
		const err = await res.json();
		throw new Error(err.error || 'Failed to browse');
	}
	return res.json();
}

/**
 * @param {string} path
 * @returns {string}
 */
export function getImageUrl(path) {
	return `${BASE_URL}/file?path=${encodeURIComponent(path)}`;
}

/**
 * @param {string} path
 * @returns {Promise<import('./types').TextContentResponse>}
 */
export async function fetchTextContent(path) {
	const res = await fetch(`${BASE_URL}/text?path=${encodeURIComponent(path)}`);
	if (!res.ok) {
		const err = await res.json();
		throw new Error(err.error || 'Failed to read file');
	}
	return res.json();
}
