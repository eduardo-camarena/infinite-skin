import { Link, useNavigate, useParams } from '@solidjs/router';
import {
  Component,
  createEffect,
  createResource,
  createSignal,
  For,
  onCleanup,
  Show,
} from 'solid-js';
import { HiSolidArrowLeft, HiSolidCog } from 'solid-icons/hi';

import Loading from '../../components/Loading';
import { albumStore, fetchImage } from '../../stores/album';
import { albumsStore } from '../../stores/albums';

type AlbumViewerParams = {
  albumId: string;
  imageId: string;
};

type ViewType = 'singleImage' | 'allImages';

const AlbumViewer: Component = () => {
  const navigate = useNavigate();
  const params = useParams<AlbumViewerParams>();
  const [viewType, setViewType] = createSignal<ViewType>('singleImage');

  const [image, { mutate }] = createResource(() => ({ ...params }), fetchImage);

  const keydown = async (event: KeyboardEvent) => {
    if (viewType() === 'singleImage') {
      if (event.key === 'ArrowLeft' && Number.parseInt(params.imageId) > 1) {
        navigate(`/a/${params.albumId}/p/${Number.parseInt(params.imageId) - 1}`);
        mutate(await fetchImage({ ...params }));
      } else if (event.key === 'ArrowRight') {
        navigate(Number.parseInt(params.imageId) < (albumStore.album?.pages ?? 0) ? `/a/${params.albumId}/p/${Number.parseInt(params.imageId) + 1}` : `/a/${params.albumId}`);
        mutate(await fetchImage({ ...params }));
      }
    }
  };

  createEffect(() => {
    window.addEventListener('keydown', keydown, true);

    onCleanup(() => {
      window.removeEventListener('keydown', keydown, true);
    });
  });

  return (
    <div class="flex flex-col justify-center">
      <div class="flex py-5 px-4">
        <Link href={`/a/${params.albumId}`}>
          <HiSolidArrowLeft class="h-4" />
        </Link>
        <div class="flex-1 text-center">
          {params.imageId} of {albumStore.album?.pages}
        </div>
        <HiSolidCog class="h-4" />
      </div>
      <div class="flex-1 mx-[10%]">
        {viewType() === 'singleImage' ? (
          <Show when={image()} fallback={<Loading margin="pt-[calc(50%-1rem)]" />}>
            <img src={image()} class="w-min h-auto" alt="logo" />
          </Show>
        ) : (
          <For each={albumStore.images}>
            {(image) => (
              <Show when={image.data}>
                <img src={image.data} class="w-min h-auto" alt="logo" />
              </Show>
            )}
          </For>
        )}
      </div>
      <div class="flex py-5 px-4">
        <Link href={`/a/${params.albumId}`}>
          <HiSolidArrowLeft class="h-4" />
        </Link>
        <div class="flex-1 text-center">
          {params.imageId} of {albumStore.album?.pages}
        </div>
        <HiSolidCog class="h-4" />
      </div>
    </div>
  );
};

export default AlbumViewer;
