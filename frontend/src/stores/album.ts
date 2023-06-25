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

export const getAlbum = async (albumId: string): Promise<void> => {
  const album = await fetch(`http://localhost:8001/albums/${albumId}`).then(
    (res) => res.json()
  );

  setAlbumStore('album', album);
};

export const fetchImage = async (payload: {
  albumId: string;
  imageId: string;
}): Promise<string> => {
  const { albumId, imageId } = payload;
  const image = albumStore.images.find((image) => image.id === imageId);
  if (image === undefined) {
    const bytes = await fetch(
      `http://localhost:8001/albums/${albumId}/${imageId}`
    ).then(async (res) => new Blob([await res.arrayBuffer()]));

    const image = URL.createObjectURL(bytes);

    setAlbumStore('images', (images) => [
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
