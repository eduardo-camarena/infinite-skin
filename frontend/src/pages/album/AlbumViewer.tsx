import { useNavigate, useParams } from '@solidjs/router';
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

import AlbumViewerControls, {
  ViewType,
} from '../../components/AlbumViewerControls';
import Loading from '../../components/Loading';
import { currentAlbumStore, fetchImage, getAlbum } from '../../stores/currentAlbum';

const { VITE_HOST: HOST } = import.meta.env;

type AlbumViewerParams = {
  albumId: string;
  imageId: string;
};

const AlbumViewer: Component = () => {
  const navigate = useNavigate();
  const params = useParams<AlbumViewerParams>();
  const [viewType, setViewType] = createSignal<ViewType>('singleImage');

  if (currentAlbumStore.album === null) {
    createResource(params.albumId, getAlbum);
  }
  const [image, { mutate }] = createResource(() => ({ ...params }), fetchImage);

  const keydown = async (event: KeyboardEvent): Promise<void> => {
    if (viewType() === 'singleImage') {
      if (event.key === 'ArrowLeft' && Number.parseInt(params.imageId) > 1) {
        navigate(
          `/a/${params.albumId}/p/${Number.parseInt(params.imageId) - 1}`
        );
        mutate(await fetchImage({ ...params }));
      } else if (event.key === 'ArrowRight') {
        navigate(
          Number.parseInt(params.imageId) < (currentAlbumStore.album?.pages ?? 0)
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
      when={currentAlbumStore.album}
      fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
    >
      {currentAlbumStore.album && (
        <div class="flex flex-col justify-center">
          <AlbumViewerControls
            albumId={params.albumId}
            currentPage={Number.parseInt(params.imageId)}
            lastPage={currentAlbumStore.album.pages}
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
                <For each={Array.from(Array(currentAlbumStore.album.pages).keys())}>
                  {(idx) => (
                    <img
                      src={`${HOST}/albums/${params.albumId}/images/${idx + 1}`}
                      loading="lazy"
                      class="w-min h-auto"
                      alt="logo"
                    />
                  )}
                </For>
              </Match>
            </Switch>
          </div>
          <AlbumViewerControls
            albumId={params.albumId}
            currentPage={Number.parseInt(params.imageId)}
            lastPage={currentAlbumStore.album.pages}
          />
        </div>
      )}
    </Show>
  );
};

export default AlbumViewer;
