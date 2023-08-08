import { Link, useSearchParams } from '@solidjs/router';
import { Component, createResource, For } from 'solid-js';

import Button from '../../InputComponents/Button';
import { setAlbumStore } from '../../stores/album';
import { albumsStore, getAlbums } from '../../stores/albums';

type AlbumsPageSearchParams = {
  page: string;
};

const Albums: Component = () => {
  const [searchParams, setSearchParams] =
    useSearchParams<AlbumsPageSearchParams>();

  if (!searchParams.page) {
    setSearchParams({ page: 1 });
  }

  createResource(() => (searchParams.page ? +searchParams.page : 1), getAlbums);

  const [lastPageNumber] = createResource(
    async () =>
      await fetch('http://localhost:8001/albums/last-page-number').then((res) =>
        res.json()
      )
  );

  return (
    <div class="pt-8 px-4 sm:m-auto sm:w-[600px] lg:w-[1100px]">
      <div class="flex flex-wrap gap-x-2 gap-y-4">
        <For each={albumsStore}>
          {(album) => (
            <Link
              href={`/a/${album.id}`}
              class="w-[200px] h-[300px] m-auto relative"
              onClick={() => setAlbumStore('images', [])}
            >
              <div class="flex flex-col justify-center h-full">
                <img
                  src={`http://localhost:8001/albums/${album.id}/images/1`}
                  loading="lazy"
                  class="w-min"
                  alt={album.name}
                />
              </div>
              <div class="absolute bottom-0 w-full text-center bg-stone-900/40">
                <p>{album.id}</p>
                <p>{album.name}</p>
              </div>
            </Link>
          )}
        </For>
      </div>
      <div class="pt-6 text-center">
        <Button
          text="Siguiente"
          variant="blue"
          onClick={() => {
            const newPage = Number.parseInt(searchParams.page) + 1;
            if (lastPageNumber().last_page_number >= newPage) {
              getAlbums(newPage);
              setSearchParams({ page: newPage });
            }
          }}
        />
      </div>
    </div>
  );
};

export default Albums;
