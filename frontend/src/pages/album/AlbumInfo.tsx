import { useParams } from '@solidjs/router';
import { AiFillHeart } from 'solid-icons/ai';
import { Component, createResource, Show } from 'solid-js';

import Loading from '../../components/Loading';
import Tag from '../../components/Tag';
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
            <div class="px-8 pb-4 md:pb-0 md:w-[45%] lg:w-[40%] lg:relative">
              <a
                class="flex flex-col justify-center h-full"
                href={`/a/${albumId}/p/1`}
              >
                <img src={cover()} alt="cover" />
              </a>
            </div>
            <div class="flex-1 flex flex-col mx-8">
              <div class="flex-1 pb-4">
                <p class="text-3xl">{currentAlbumStore.album.name}</p>
                <div class="flex-1 flex flex-col sm:pt-2 lg:pt-8 sm:gap-2 lg:gap-8 sm:text-md lg:text-lg">
                  <p>#{currentAlbumStore.album.id}</p>
                  <div class="pb-1 flex flex-row gap-1">
                    <p>Artist:</p>
                    <Tag
                      href={`/a`}
                      tagParams={{
                        page: 1,
                        artistId: currentAlbumStore.album.artist?.id ?? '',
                      }}
                      text={currentAlbumStore.album.artist?.name ?? ''}
                    />
                  </div>
                  <p class="pb-1">Pages: {currentAlbumStore.album.pages}</p>
                </div>
              </div>
              <div class="pb-20">
                <Button
                  text={
                    <div class="flex gap-1">
                      <AiFillHeart class="h-6" />
                      <p>Favorite</p>
                    </div>
                  }
                  variant="red"
                  type="button"
                />
              </div>
            </div>
          </div>
        </div>
      )}
    </Show>
  );
};

export default AlbumInfo;
