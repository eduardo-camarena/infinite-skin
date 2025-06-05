import { Component } from 'solid-js';

import { setCurrentAlbumStore } from '../stores/currentAlbum';

const albumsScrollHeights = [48, 72, 96, 120, 144, 168, 192, 216, 240] as const;
type AlbumNameScrollHeight = (typeof albumsScrollHeights)[number];
export type TailwindHeight = `group-hover:h-[${AlbumNameScrollHeight}px]`;

type AlbumThumbnailProps = {
	albumId: number;
	libraryId: number;
	albumName: string;
};

const { VITE_HOST: HOST } = import.meta.env;

const AlbumThumbnail: Component<AlbumThumbnailProps> = ({
	albumId,
	libraryId,
	albumName,
}) => {
	return (
		<a
			href={`/libraries/${libraryId}/albums/${albumId}`}
			class="w-[200px] h-[300px] m-auto relative"
			onClick={() => setCurrentAlbumStore('images', [])}
		>
			<div class="w-full flex flex-col justify-center h-full overflow-hidden">
				<img
					src={`${HOST}/libraries/${libraryId}/albums/${albumId}/images/1`}
					alt={albumName}
					loading="lazy"
					class="w-[200px] h-[300px]"
				/>
			</div>
			<div
				id={`${albumId}`}
				class="absolute py-1 h-12 bottom-0 w-full text-center bg-stone-900/40 font-semibold overflow-hidden group-hover:h-auto"
			>
				<p>{albumId}</p>
				<p>{albumName}</p>
			</div>
		</a>
	);
};

export default AlbumThumbnail;
