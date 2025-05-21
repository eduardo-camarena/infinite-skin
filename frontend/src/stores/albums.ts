import { createStore } from 'solid-js/store';

import { AlbumsSearchParams } from '../pages/album/Albums';
import { httpClient } from '../utils/httpClient';

export interface Album {
	id: number;
	name: string;
	full_name: string;
	pages: number;
}

interface AlbumsStore {
	albums: Array<Album>;
}

export const [albumsStore, setAlbumsStore] = createStore<AlbumsStore>({
	albums: [],
});

export interface GetAlbumsPayload {
	page: number;
	params?: AlbumsSearchParams;
}

export const getAlbums = async (payload: GetAlbumsPayload): Promise<void> => {
	const { page, params } = payload;
	const { data } = await httpClient.get(`/albums/pages/${page}`, {
		params,
	});

	setAlbumsStore('albums', data.albums);
};

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

	console.log(data);
	return data.folders;
};
