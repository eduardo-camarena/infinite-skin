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
        <div class="pt-6 md:pt-10 px-4 md:px-[15%]">
          <div class="flex flex-col lg:flex-row py-8 md:px-12 lg:px-20 bg-neutral-900">
            <div class="px-8 pb-4 md:pb-0 md:w-[45%] lg:h-[488px] lg:relative">
              <Link
                class="flex flex-col justify-center h-full"
                href={`/a/${albumId}/p/1`}
              >
                <img src={albumStore.images[0].data} class="h-[400px] w-min" />
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
