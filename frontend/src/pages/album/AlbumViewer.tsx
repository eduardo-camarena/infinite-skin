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
import SingleImage from '../../components/SingleImage';
import {
  currentAlbumStore,
  getImage,
  getAlbum,
  GetImagePayload,
} from '../../stores/currentAlbum';

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
  const [image, { mutate }] = createResource(
    () => ({
      albumId: params.albumId,
      imageId: Number.parseInt(params.imageId),
    }),
    getImage
  );

  const updateImage = async (payload: GetImagePayload): Promise<void> => {
    navigate(
      `/a/${payload.albumId}/p/${payload.imageId}`
    );
    mutate(
      await getImage({
        albumId: payload.albumId,
        imageId: payload.imageId,
      })
    );
  };

  const keydown = async (event: KeyboardEvent): Promise<void> => {
    if (viewType() === 'singleImage') {
      if (event.key === 'ArrowLeft' && Number.parseInt(params.imageId) > 1) {
        updateImage({ albumId: params.albumId, imageId: Number.parseInt(params.imageId) - 1 });
      } else if (event.key === 'ArrowRight') {
        updateImage({ albumId: params.albumId, imageId: Number.parseInt(params.imageId) + 1 });
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
            lastPage={currentAlbumStore.album.pages}
            updateImage={updateImage}
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
                  <img src={image()} class="w-min h-auto m-auto" alt={params.imageId} />
                </Show>
              </Match>
              <Match when={viewType() === 'allImages'}>
                <For
                  each={Array.from(Array(currentAlbumStore.album.pages).keys())}
                >
                  {(idx) => (
                    <SingleImage albumId={params.albumId} imageId={idx + 1} />
                  )}
                </For>
              </Match>
            </Switch>
          </div>
          <AlbumViewerControls
            albumId={params.albumId}
            lastPage={currentAlbumStore.album.pages}
            updateImage={updateImage}
          />
        </div>
      )}
    </Show>
  );
};

export default AlbumViewer;
