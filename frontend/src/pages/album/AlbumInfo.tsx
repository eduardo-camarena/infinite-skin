import { useParams } from '@solidjs/router';
import { Component, createResource, Show } from 'solid-js';

import { albumStore, setAlbumStore } from './[albumId]';
import Loading from '../../components/Loading';

export const getAlbum = async (albumId: string): Promise<void> => {
  const album = await fetch(`http://localhost:8001/albums/${albumId}`).then(
    (res) => res.json()
  );

  setAlbumStore('album', album);
};

const AlbumInfo: Component = () => {
  const { albumId } = useParams<{ albumId: string }>();
  createResource(albumId, getAlbum);

  return (
    <Show
      when={albumStore.album}
      fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
    >
      <p class="text-[40px]">hello</p>
    </Show>
  );
};

export default AlbumInfo;
