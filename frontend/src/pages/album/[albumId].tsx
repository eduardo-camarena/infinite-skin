import { Outlet } from '@solidjs/router';
import type { Component } from 'solid-js';
import { createStore } from 'solid-js/store';

type Album = { name: string };

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
