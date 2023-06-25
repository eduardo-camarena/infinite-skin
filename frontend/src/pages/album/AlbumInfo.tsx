import { Link, useParams } from '@solidjs/router';
import { Component, createResource, Show } from 'solid-js';

import Loading from '../../components/Loading';
import Button from '../../InputComponents/Button';
import { albumStore, fetchImage, getAlbum } from '../../stores/album';

const AlbumInfo: Component = () => {
  const { albumId } = useParams<{ albumId: string }>();

  createResource(albumId, getAlbum);
  createResource({ albumId, imageId: '1' }, fetchImage);

  return (
    <Show
      when={albumStore.album && albumStore.images.length}
      fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
    >
      {albumStore.album && albumStore.images.length && (
        <div class="pt-10 px-[15%]">
          <div class="flex flex-row py-12 px-20 bg-neutral-900">
            <div class="px-8">
              <Link href={`/a/${albumId}/p/1`}>
                <img src={albumStore.images[0].data} class="w-min h-[488px]" />
              </Link>
            </div>
            <div class="flex-1 flex flex-col mx-8">
              <div class="flex-1 pb-4">
                <p class="text-3xl">{albumStore.album.name}</p>
                <p class="py-5">#{albumStore.album.id}</p>
                <p class="pb-1">Artist: {albumStore.album.full_name}</p>
                <p class="pb-1">Pages: {albumStore.album.pages}</p>
              </div>
              <div class="pb-20">
                <Button text="Favorite" variant="red" type="button" />
              </div>
            </div>
          </div>
        </div>
      )}
    </Show>
  );
};

export default AlbumInfo;
