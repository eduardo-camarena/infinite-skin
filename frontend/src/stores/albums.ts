import { createStore } from 'solid-js/store';

import { AlbumsSearchParams } from '../pages/album/Albums';
import { httpClient } from '../utils/httpClient';

export type Album = {
	id: number;
	name: string;
	full_name: string;
	pages: number;
};

type AlbumsStore = {
	albums: Array<Album>;
};

export const [albumsStore, setAlbumsStore] = createStore<AlbumsStore>({
	albums: [],
});

export type GetAlbumsPayload = {
	page: number;
	params?: AlbumsSearchParams;
};

export const getAlbums = async (payload: GetAlbumsPayload): Promise<void> => {
	const { page, params } = payload;
	const { data } = await httpClient.get(`/albums/pages/${page}`, {
		params,
	});

	setAlbumsStore('albums', data.albums);
};
