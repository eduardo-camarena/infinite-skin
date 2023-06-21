import { Outlet } from '@solidjs/router';
import type { Component } from 'solid-js';
import { createStore } from 'solid-js/store';

type Album = {
  id: number;
  name: string;
  full_name: string;
  pages: number;
  artist_id: number;
};

type Image = { id: string; data: string };

type AlbumStore = {
  album: Album | null;
  images: Image[];
};

export const [albumStore, setAlbumStore] = createStore<AlbumStore>({
  album: null,
  images: [],
});

const Album: Component = () => {
  return <Outlet />;
};

export default Album;
