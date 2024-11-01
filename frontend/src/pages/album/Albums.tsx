import { useSearchParams } from '@solidjs/router';
import { Component, createResource, createSignal, For, Show } from 'solid-js';

import AlbumThumbnail, {
  getScrollHeight,
} from '../../components/AlbumThumbnail';
import ButtonGroup from '../../components/ButtonGroup';
import Loading from '../../components/Loading';
import Pagination from '../../components/PaginationComponent';
import { albumsStore, getAlbums } from '../../stores/albums';
import { Artist } from '../../stores/currentAlbum';
import { httpClient } from '../../utils/httpClient';

const orderByColumns = ['id', 'rating', 'name', 'pages'] as const;
type OrderByColumn = (typeof orderByColumns)[number];

export type AlbumsSearchParams = Partial<{
  page: string;
  artistId: string;
  orderByType: 'asc' | 'desc';
  orderByColumn: OrderByColumn;
}>;

const Albums: Component = () => {
  const [currentSelect, setCurrentSelect] = createSignal(0);
  const [searchParams, setSearchParams] = useSearchParams<AlbumsSearchParams>();
  const [currentPage, setCurrentPage] = createSignal(
    Number.parseInt(searchParams.page ?? '1'),
  );

  if (!searchParams.page || Number.parseInt(searchParams.page) <= 0) {
    setSearchParams({ page: 1 });
  }

  const [lastPageNumber] = createResource<number>(async () => {
    const { data } = await httpClient.get('/albums/last-page-number', {
      params: searchParams.artistId
        ? { artistId: searchParams.artistId }
        : undefined,
    });

    return data.lastPageNumber;
  });

  const [, { mutate: mutateAlbums }] = createResource(
    () => ({
      page: Number.parseInt(searchParams.page ?? '1'),
      params: searchParams,
    }),
    async (opts) => {
      await getAlbums(opts);
    },
  );

  const [artist] = createResource(async () => {
    if (!searchParams.artistId) {
      return undefined;
    }

    const { data } = await httpClient.get(`/artists/${searchParams.artistId}`);

    return data as Artist;
  });

  return (
    <div class="pt-8 px-4 sm:m-auto sm:w-[600px] lg:w-[1100px]">
      <Show when={artist()}>
        <div class="pb-6">
          <div class="flex text-2xl font-semibold justify-center">
            <p class="px-3 py-1">Artist</p>
            <p class="bg-stone-900 px-3 py-1 rounded-l-md">{artist()?.name}</p>
            <p class="bg-stone-800 px-3 py-1 rounded-r-md">
              {albumsStore.albums.length}
            </p>
          </div>
          <div class="pt-4 flex text-xl font-semibold justify-center gap-4">
            <ButtonGroup
              buttonsText={
                ['Date', ...orderByColumns.slice(1)] as [
                  string,
                  string,
                  ...string[],
                ]
              }
              currentSelect={currentSelect}
              setCurrentSelect={async (newVal) => {
                const newSearchParams = {
                  ...searchParams,
                  orderByColumn: orderByColumns[newVal],
                  orderByType: orderByColumns[newVal] === 'id' ? 'desc' : 'asc',
                } satisfies AlbumsSearchParams;
                setSearchParams(newSearchParams);
                mutateAlbums(
                  await getAlbums({
                    page: 1,
                    params: newSearchParams,
                  }),
                );
                setCurrentSelect(newVal);
              }}
            />
          </div>
        </div>
      </Show>
      <div class="flex flex-wrap gap-x-2 gap-y-4">
        <Show
          when={albumsStore.albums.length && lastPageNumber !== undefined}
          fallback={
            <Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />
          }
        >
          <For each={albumsStore.albums}>
            {(album) => (
              <AlbumThumbnail
                albumId={album.id}
                albumName={album.name}
                groupHoverScrollHeight={getScrollHeight(
                  Math.ceil(album.name.length / 25),
                )}
              />
            )}
          </For>
        </Show>
      </div>
      <Show
        when={lastPageNumber() !== undefined}
        fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
      >
        <Pagination
          lastPage={lastPageNumber() ?? 0}
          currentPage={currentPage}
          setNewPage={(newPage) => {
            setCurrentPage(newPage);
            setSearchParams({ ...searchParams, page: `${newPage}` });
          }}
          getNewPage={async (newPage) =>
            mutateAlbums(
              await getAlbums({ page: newPage, params: searchParams }),
            )
          }
        />
      </Show>
    </div>
  );
};

export default Albums;
