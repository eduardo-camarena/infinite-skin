import { createStore } from 'solid-js/store';

const { VITE_HOST: HOST } = import.meta.env;

type Albums = Array<{
  id: number;
  name: string;
}>;

export const [albumsStore, setAlbumsStore] = createStore<Albums>([]);

export const getAlbums = async (page: number): Promise<void> => {
  const albums = await fetch(`${HOST}/albums/pages/${page}`).then((res) =>
    res.json()
  );

  setAlbumsStore(albums);
};
