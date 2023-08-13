import { createStore } from 'solid-js/store';

const { VITE_HOST: HOST } = import.meta.env;

export type Album = {
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

export const [currentAlbumStore, setCurrentAlbumStore] = createStore<AlbumStore>({
  album: null,
  images: [],
});

export const getAlbum = async (albumId: string): Promise<void> => {
  const album = await fetch(`${HOST}/albums/${albumId}`).then((res) =>
    res.json()
  );

  setCurrentAlbumStore('album', album);
};

export const fetchImage = async (payload: {
  albumId: string;
  imageId: string;
}): Promise<string> => {
  const { albumId, imageId } = payload;
  const image = currentAlbumStore.images.find((image) => image.id === imageId);
  if (image === undefined) {
    const bytes = await fetch(
      `${HOST}/albums/${albumId}/images/${imageId}`
    ).then(async (res) => new Blob([await res.arrayBuffer()]));

    const image = URL.createObjectURL(bytes);

    setCurrentAlbumStore('images', (images) => [
      ...images,
      {
        id: imageId,
        data: image.toString(),
      },
    ]);

    return image;
  }
  return image.data;
};

export default Album;
