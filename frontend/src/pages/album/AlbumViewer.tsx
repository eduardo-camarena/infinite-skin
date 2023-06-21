import { useNavigate, useParams } from '@solidjs/router';
import {
  Component,
  createEffect,
  createResource,
  createSignal,
  For,
  onCleanup,
} from 'solid-js';

import { albumStore, setAlbumStore } from './[albumId]';
import Loading from '../../components/Loading';

type AlbumViewerParams = {
  albumId: string;
  imageId: string;
};

type ViewType = 'singleImage' | 'allImages';

const fetchImage = async (payload: AlbumViewerParams): Promise<string> => {
  const { albumId, imageId } = payload;
  const image = albumStore.images.find((image) => image.id === imageId);
  if (image === undefined) {
    const bytes = await fetch(
      `http://localhost:8001/albums/${albumId}/${imageId}`
    ).then(async (res) => new Blob([await res.arrayBuffer()]));

    const image = URL.createObjectURL(bytes);

    setAlbumStore('images', (images) => [
      ...images,
      {
        id: imageId,
        data: image.toString(),
      },
    ]);

    return image;
  }
  return image.data;
};

const AlbumViewer: Component = () => {
  const params = useParams<AlbumViewerParams>();
  const [viewType, setViewType] = createSignal<ViewType>('singleImage');

  const [image, { mutate }] = createResource(() => ({ ...params }), fetchImage);

  const navigate = useNavigate();
  createEffect(() => {
    window.addEventListener('keydown', async (event) => {
      if (viewType() === 'singleImage') {
        if (event.key === 'ArrowLeft' && +params.imageId > 1) {
          navigate(`/a/${params.albumId}/p/${+params.imageId - 1}`);
          mutate(await fetchImage({ ...params }));
        } else if (event.key === 'ArrowRight') {
          navigate(`/a/${params.albumId}/p/${+params.imageId + 1}`);
          mutate(await fetchImage({ ...params }));
        }
      }
    });

    onCleanup(() => {
      window.removeEventListener('keypress', () => null);
    });
  });

  return (
    <div class="flex flex-col justify-center mx-[10%]">
      {viewType() === 'singleImage' ? (
        <img src={image()!} class="w-min h-auto" alt="logo" />
      ) : (
        <For each={albumStore.images}>
          {(image) => <img src={image.data!} class="w-min h-auto" alt="logo" />}
        </For>
      )}
    </div>
  );
};

export default AlbumViewer;
