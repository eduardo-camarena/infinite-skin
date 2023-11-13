import { Component, createResource, Show } from 'solid-js';

import Loading from './Loading';
import { getImage } from '../stores/currentAlbum';

type SingleImageProps = {
  albumId: string;
  imageId: number;
};

const SingleImage: Component<SingleImageProps> = ({ albumId, imageId }) => {
  const [image] = createResource(() => ({ albumId, imageId }), getImage);

  return (
    <Show
      when={image()}
      fallback={<Loading margin="py-12 ml-[calc(50%-1rem)]" />}
    >
      <img src={image()} class="w-min h-auto" alt={`${imageId}`} />
    </Show>
  );
};

export default SingleImage;
