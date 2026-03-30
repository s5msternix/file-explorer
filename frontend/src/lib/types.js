/**
 * @typedef {Object} FileEntry
 * @property {string} name
 * @property {string} path
 * @property {boolean} is_dir
 * @property {number} size
 * @property {string|null} modified
 * @property {string|null} extension
 */

/**
 * @typedef {Object} BrowseResponse
 * @property {string} current_path
 * @property {string|null} parent_path
 * @property {FileEntry[]} entries
 */

export {};
