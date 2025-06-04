import { useParams, useSearchParams } from '@solidjs/router';
import { Component, createResource, createSignal, For, Show } from 'solid-js';

import AlbumThumbnail from '../../components/AlbumThumbnail';
import ButtonGroup from '../../components/ButtonGroup';
import Loading from '../../components/Loading';
import Pagination from '../../components/PaginationComponent';
import { libraryStore, getCurrentLibraryAlbums } from '../../stores/libraries';
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
	const params = useParams<{ libraryId: string }>();
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
		() => {
			const { page, ...rest } = searchParams;
			return {
				params: rest,
				libraryId: Number.parseInt(params.libraryId),
				page: Number.parseInt(page ?? '1'),
			};
		},
		async (opts) => {
			await getCurrentLibraryAlbums(opts);
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
							{libraryStore.currentLibrarylbums?.length}
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
									await getCurrentLibraryAlbums({
										page: 1,
										params: newSearchParams,
										libraryId: Number.parseInt(params.libraryId),
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
					when={
						libraryStore.currentLibrarylbums && lastPageNumber !== undefined
					}
					fallback={
						<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />
					}
				>
					<For each={libraryStore.currentLibrarylbums}>
						{(album) => (
							<AlbumThumbnail
								href={`/libraries/${params.libraryId}/albums/${album.id}`}
								albumId={album.id}
								albumName={album.name}
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
							await getCurrentLibraryAlbums({
								page: newPage,
								params: searchParams,
								libraryId: Number.parseInt(params.libraryId),
							}),
						)
					}
				/>
			</Show>
		</div>
	);
};

export default Albums;
