import { Component } from 'solid-js';

import { setCurrentAlbumStore } from '../stores/currentAlbum';
import { classNames } from '../utils/classNames';

const albumsScrollHeights = [48, 72, 96, 120, 144, 168, 192, 216, 240] as const;
type AlbumNameScrollHeight = (typeof albumsScrollHeights)[number];
export type TailwindHeight = `group-hover:h-[${AlbumNameScrollHeight}px]`;

type AlbumThumbnailProps = {
  albumId: number;
  albumName: string;
  groupHoverScrollHeight: TailwindHeight;
};

const { VITE_HOST: HOST } = import.meta.env;

export const getScrollHeight = (rows: number): TailwindHeight => {
  switch (rows) {
    default:
      return 'group-hover:h-[48px]';
    case 2:
      return 'group-hover:h-[72px]';
    case 3:
      return 'group-hover:h-[96px]';
    case 4:
      return 'group-hover:h-[120px]';
    case 5:
      return 'group-hover:h-[144px]';
    case 6:
      return 'group-hover:h-[168px]';
    case 7:
      return 'group-hover:h-[192px]';
    case 8:
      return 'group-hover:h-[216px]';
    case 9:
      return 'group-hover:h-[240px]';
  }
};

const AlbumThumbnail: Component<AlbumThumbnailProps> = ({
  albumId,
  albumName,
  groupHoverScrollHeight,
}) => {
  console.log(
    albumId,
    groupHoverScrollHeight,
    albumName.length,
    Math.ceil((albumName.length + 1) / 25),
  );
  return (
    <a
      href={`/a/${albumId}`}
      class="w-[200px] h-[300px] m-auto relative group"
      onClick={() => setCurrentAlbumStore('images', [])}
    >
      <div class="flex flex-col justify-center h-full overflow-hidden">
        <img
          src={`${HOST}/albums/${albumId}/images/1`}
          loading="lazy"
          alt={albumName}
        />
      </div>
      <div
        id={`${albumId}`}
        class={classNames(
          'transition-all ease-in-out duration-300 absolute h-12 bottom-0 w-full text-center bg-stone-900/40 font-semibold overflow-hidden',
          groupHoverScrollHeight,
        )}
      >
        <p>{albumId}</p>
        <p>{albumName}</p>
      </div>
    </a>
  );
};

export default AlbumThumbnail;
