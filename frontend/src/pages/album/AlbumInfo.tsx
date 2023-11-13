import { Link, useParams } from '@solidjs/router';
import { Component, createResource, Show } from 'solid-js';

import Loading from '../../components/Loading';
import Button from '../../InputComponents/Button';
import {
  currentAlbumStore,
  getImage,
  getAlbum,
} from '../../stores/currentAlbum';

const AlbumInfo: Component = () => {
  const { albumId } = useParams<{ albumId: string }>();

  createResource(albumId, getAlbum);
  const [cover] = createResource({ albumId, imageId: 1 }, getImage);

  return (
    <Show
      when={currentAlbumStore.album && cover}
      fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
    >
      {currentAlbumStore.album && (
        <div class="pt-6 md:pt-10 px-4 md:px-[15%]">
          <div class="flex flex-col lg:flex-row py-8 md:px-12 lg:px-20 bg-neutral-900">
            <div class="px-8 pb-4 md:pb-0 md:w-[45%] lg:h-[488px] lg:relative">
              <Link
                class="flex flex-col justify-center h-full"
                href={`/a/${albumId}/p/1`}
              >
                <img src={cover()} alt="cover" class="h-[400px] w-min" />
              </Link>
            </div>
            <div class="flex-1 flex flex-col mx-8">
              <div class="flex-1 pb-4">
                <p class="text-3xl">{currentAlbumStore.album.name}</p>
                <p class="py-5">#{currentAlbumStore.album.id}</p>
                <p class="pb-1">Artist: {currentAlbumStore.album.full_name}</p>
                <p class="pb-1">Pages: {currentAlbumStore.album.pages}</p>
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
