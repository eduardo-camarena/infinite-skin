import { createStore } from 'solid-js/store';

type Albums = Array<{
  id: number;
  name: string;
}>;

export const [albumsStore, setAlbumsStore] = createStore<Albums>([]);

export const getAlbums = async (page: number): Promise<void> => {
  const albums = await fetch(`http://localhost:8001/albums/pages/${page}`).then(
    (res) => res.json()
  );

  setAlbumsStore(albums);
};
