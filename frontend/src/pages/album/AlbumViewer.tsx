import { Outlet, useNavigate, useParams } from '@solidjs/router';
import {
  Component,
  createEffect,
  createResource,
  createSignal,
  For,
  onCleanup,
} from 'solid-js';

import Loading from '../../components/Loading';

type ViewType = 'singleImage' | 'allImages';

import { albumStore, setAlbumStore } from './[albumId]';

const fetchImage = async (id: string): Promise<string> => {
  console.log(id);
  const image = albumStore.images.find((image) => image.id === id);
  if (image === undefined) {
    const bytes = await fetch(
      `http://localhost:8000/images/aqua/ganyu (${id}).jpg`
    ).then(async (res) => new Blob([await res.arrayBuffer()]));

    const image = URL.createObjectURL(bytes);

    setAlbumStore('images', (images) => [
      ...images,
      {
        id: id,
        data: image.toString(),
      },
    ]);

    return image;
  }
  return image.data;
};

const AlbumViewer: Component = () => {
  const params = useParams<{ albumId: string; imageId: string }>();
  const [viewType, setViewType] = createSignal<ViewType>('singleImage');

  const [next, setNext] = createSignal(+params.albumId);

  const [image, { refetch, mutate }] = createResource(
    params.imageId,
    fetchImage
  );

  const navigate = useNavigate();
  createEffect(() => {
    window.addEventListener('keydown', async (event) => {
      if (viewType() === 'singleImage') {
        if (event.key === 'ArrowLeft') {
          navigate(`/images/${params.albumId}/${+params.imageId - 1}`);
          mutate(await fetchImage((+params.imageId - 1).toString()));
        } else if (event.key === 'ArrowRight') {
          navigate(`/images/${params.albumId}/${+params.imageId + 1}`);
          mutate(await fetchImage((+params.imageId - 1).toString()));
        }
      }
    });

    onCleanup(() => {
      window.removeEventListener('keypress', () => null);
    });
  });

  return (
    <div class="flex flex-col justify-center px-60">
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
