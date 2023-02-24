import { useParams } from '@solidjs/router';
import {
  Component,
  createEffect,
  createResource,
  For,
  Suspense,
} from 'solid-js';
import { createStore } from 'solid-js/store';

type ImagesStore = { images: { id: string; data: string }[] };

const [imagesStore, setImagesStore] = createStore<ImagesStore>({
  images: [],
});

type AlbumImage = { albumId: string; imageId: string };

const Image: Component = () => {
  const params = useParams<AlbumImage>();
  const [image] = createResource(
    () => `${params.albumId}/${params.imageId}`,
    async (id) => {
      const bytes = await fetch(`http://localhost:8000/images/${id}.jpg`).then(
        async (res) => new Blob([await res.arrayBuffer()])
      );

      const image = URL.createObjectURL(bytes);

      return image;
    }
  );

  createEffect(() => {
    const imageVal = image();
    if (imageVal) {
      setImagesStore('images', (images) => [
        ...images,
        {
          id: `${params.albumId}/${params.imageId}`,
          data: imageVal.toString(),
        },
      ]);
    }
  }, [image()]);

  return (
    <Suspense fallback={<p>Loading...</p>}>
      <For each={imagesStore.images}>
        {(image) => <img src={image.data} class="h-44" alt="logo" />}
      </For>
    </Suspense>
  );
};

export default Image;
