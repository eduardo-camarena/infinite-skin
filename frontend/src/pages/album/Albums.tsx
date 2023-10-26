import { Link } from '@solidjs/router';
import { Component, createResource, For, Show } from 'solid-js';

import Loading from '../../components/Loading';
import Paginator from '../../components/PaginationComponent';
import { albumsStore, getAlbums } from '../../stores/albums';
import { setCurrentAlbumStore } from '../../stores/currentAlbum';

const Albums: Component = () => {
  const [lastPageNumber] = createResource<number>(async () => {
    const response = await fetch(
      'http://localhost:8001/albums/last-page-number'
    ).then((res) => res.json());

    return response.last_page_number;
  });

  return (
    <div class="pt-8 px-4 sm:m-auto sm:w-[600px] lg:w-[1100px]">
      <div class="flex flex-wrap gap-x-2 gap-y-4">
        <For each={albumsStore}>
          {(album) => (
            <Link
              href={`/a/${album.id}`}
              class="w-[200px] h-[300px] m-auto relative"
              onClick={() => setCurrentAlbumStore('images', [])}
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
      <Show when={lastPageNumber() !== undefined} fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}>
        <Paginator lastPage={lastPageNumber() ?? 0} getNewPage={getAlbums} />
      </Show>
    </div>
  );
};

export default Albums;
