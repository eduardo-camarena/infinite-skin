import { createStore, produce } from 'solid-js/store';

const { VITE_HOST: HOST } = import.meta.env;

export type Album = {
  id: number;
  name: string;
  full_name: string;
  pages: number;
  artist_id: number;
};

type Image = { id: number; data: string };

type AlbumStore = {
  album: Album | null;
  images: Image[];
};

export const [currentAlbumStore, setCurrentAlbumStore] =
  createStore<AlbumStore>({
    album: null,
    images: [],
  });

export const getAlbum = async (albumId: string): Promise<void> => {
  if (
    !currentAlbumStore.album ||
    currentAlbumStore.album.id !== Number.parseInt(albumId)
  ) {
    const album = await fetch(`${HOST}/albums/${albumId}`).then((res) =>
      res.json()
    );

    setCurrentAlbumStore({ album: album, images: [] });
  }
};

export type GetImagePayload = {
  albumId: string;
  imageId: number;
};

export const getImage = async (payload: GetImagePayload): Promise<string> => {
  const { albumId, imageId } = payload;
  const image = currentAlbumStore.images.find((image) => image.id === imageId);
  if (image === undefined) {
    const bytes = await fetch(
      `${HOST}/albums/${albumId}/images/${imageId}`
    ).then(async (res) => new Blob([await res.arrayBuffer()]));

    const image = URL.createObjectURL(bytes);

    setCurrentAlbumStore(
      'images',
      produce((images) => {
        images.push({
          id: imageId,
          data: image.toString(),
        });
        images.sort((a, b) => a.id - b.id);
      })
    );

    return image;
  }
  return image.data;
};

export default Album;
