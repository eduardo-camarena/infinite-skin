import { Link, useParams } from '@solidjs/router';
import { HiSolidArrowLeft, HiSolidCog } from 'solid-icons/hi';
import { Accessor, Component, Setter, Show } from 'solid-js';

import Button from '../InputComponents/Button';

export type ViewType = 'singleImage' | 'allImages';

type AlbumViewerControlsProps = {
  albumId: string;
  lastPage: number;
} & (
  | {
      viewType?: undefined;
      setViewType?: undefined;
    }
  | {
      viewType: Accessor<ViewType>;
      setViewType: Setter<ViewType>;
    }
);

const AlbumViewerControls: Component<AlbumViewerControlsProps> = ({
  albumId,
  viewType,
  lastPage,
  setViewType,
}) => {
  const params = useParams<{ albumId: string; imageId: string }>();
  return (
    <div class="flex py-5 relative">
      <Link class="h-[34px] py-1.5 pl-4 z-10" href={`/a/${albumId}`}>
        <HiSolidArrowLeft size={22} />
      </Link>
      <div class="flex-1 text-center absolute w-full py-1.5">
        {params.imageId} of {lastPage}
      </div>
      <div class="flex absolute right-4 gap-4">
        {viewType && (
          <Button
            text={
              <Show when={viewType() === 'singleImage'} fallback="Single image">
                All images
              </Show>
            }
            variant="blue"
            rounded="full"
            padding="px-4 py-1"
            onClick={() => {
              if (viewType() === 'singleImage') {
                setViewType('allImages');
              } else {
                setViewType('singleImage');
              }
            }}
          />
        )}
        <button
          class="my-auto z-10"
          onClick={() => {
            console.log('hello');
          }}
        >
          <HiSolidCog size={22} />
        </button>
      </div>
    </div>
  );
};

export default AlbumViewerControls;
