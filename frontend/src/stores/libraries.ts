import { createStore } from 'solid-js/store';
import { httpClient } from '../utils/httpClient';

import { AlbumsSearchParams } from '../pages/album/Albums';

export interface Album {
	id: number;
	name: string;
	full_name: string;
	pages: number;
}

export interface Library {
	id: number;
	name: string;
}

interface LibraryStore {
	libraries: Array<Library & { previewAlbums: Array<Album> | null }> | null;
	currentLibrarylbums: Array<Album> | null;
}

export const [libraryStore, setLibraryStore] = createStore<LibraryStore>({
	libraries: null,
	currentLibrarylbums: null,
});

export interface NewLibraryPayload {
	name: string;
	location: string;
	isPrivate: boolean;
}

export const createLibrary = async (
	payload: NewLibraryPayload,
): Promise<void> => {
	const { data } = await httpClient.post('/libraries', payload);

	return data;
};

export interface GetPossibleLibrariesResponse {
	folders: string[];
}

export const getPossibleLibraries = async (path: string): Promise<string[]> => {
	const { data } = await httpClient.get('/libraries/possible-folders', {
		params: { path },
	});

	return data.folders;
};

export const getLibraries = async (): Promise<void> => {
	const { data } = await httpClient.get<{ libraries: Library[] }>('/libraries');

	const libraries = await Promise.all(
		data.libraries.map(async (library) => {
			const previewAlbums = await getAlbums({
				page: 1,
				libraryId: library.id,
			});
			return {
				...library,
				previewAlbums: previewAlbums.length ? previewAlbums : null,
			};
		}),
	);

	setLibraryStore('libraries', libraries);
};

export interface GetAlbumsPayload {
	page: number;
	libraryId: number;
	params?: AlbumsSearchParams;
}

const getAlbums = async (payload: GetAlbumsPayload): Promise<Array<Album>> => {
	const { page, libraryId, params } = payload;

	const { data } = await httpClient.get(
		`libraries/${libraryId}/albums/pages/${page}`,
		{
			params,
		},
	);

	return data.albums;
};

export const getCurrentLibraryAlbums = async (
	payload: GetAlbumsPayload,
): Promise<void> => {
	console.log(payload);
	setLibraryStore('currentLibrarylbums', await getAlbums(payload));
};
