import { Link, useNavigate, useParams } from '@solidjs/router';
import {
  Component,
  createEffect,
  createResource,
  createSignal,
  For,
  Match,
  onCleanup,
  Show,
  Switch,
} from 'solid-js';
import { HiSolidArrowLeft, HiSolidCog } from 'solid-icons/hi';

import Loading from '../../components/Loading';
import { albumStore, fetchImage, getAlbum } from '../../stores/album';
import { albumsStore } from '../../stores/albums';
import Button from '../../InputComponents/Button';
import AlbumViewerControls, { ViewType } from '../../components/AlbumViewerControls';

type AlbumViewerParams = {
  albumId: string;
  imageId: string;
};

const AlbumViewer: Component = () => {
  const navigate = useNavigate();
  const params = useParams<AlbumViewerParams>();
  const [viewType, setViewType] = createSignal<ViewType>('singleImage');

  if (albumStore.album === null) {
    createResource(params.albumId, getAlbum);
  }
  const [image, { mutate }] = createResource(() => ({ ...params }), fetchImage);

  const keydown = async (event: KeyboardEvent) => {
    if (viewType() === 'singleImage') {
      if (event.key === 'ArrowLeft' && Number.parseInt(params.imageId) > 1) {
        navigate(
          `/a/${params.albumId}/p/${Number.parseInt(params.imageId) - 1}`
        );
        mutate(await fetchImage({ ...params }));
      } else if (event.key === 'ArrowRight') {
        navigate(
          Number.parseInt(params.imageId) < (albumStore.album?.pages ?? 0)
            ? `/a/${params.albumId}/p/${Number.parseInt(params.imageId) + 1}`
            : `/a/${params.albumId}`
        );
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
    <Show
      when={albumStore.album}
      fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
    >
      <div class="flex flex-col justify-center">
        <AlbumViewerControls
          albumId={params.albumId}
          currentPage={Number.parseInt(params.imageId)}
          lastPage={albumStore.album!.pages}
          viewType={viewType}
          setViewType={setViewType}
        />
        <div class="flex-1 mx-[10%]">
          <Switch>
            <Match when={viewType() === 'singleImage'}>
              <Show
                when={image()}
                fallback={<Loading margin="pt-[calc(50%-1rem)]" />}
              >
                <img src={image()} class="w-min h-auto m-auto" alt="logo" />
              </Show>
            </Match>
            <Match when={viewType() === 'allImages'}>
              <For each={albumStore.images}>
                {(image) => (
                  <Show when={image.data}>
                    <img src={image.data} class="w-min h-auto" alt="logo" />
                  </Show>
                )}
              </For>
            </Match>
          </Switch>
        </div>
        <AlbumViewerControls
          albumId={params.albumId}
          currentPage={Number.parseInt(params.imageId)}
          lastPage={albumStore.album!.pages}
        />
      </div>
    </Show>
  );
};

export default AlbumViewer;
