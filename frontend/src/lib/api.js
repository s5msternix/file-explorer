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
