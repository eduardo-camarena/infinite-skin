import { Component, createResource } from 'solid-js';

import { getImage } from '../stores/currentAlbum';

type SingleImageProps = {
  albumId: string;
  imageId: number;
}

const SingleImage: Component<SingleImageProps> = ({ albumId, imageId }) => {
  const [image] = createResource(() => ({ albumId, imageId }), getImage);

  return (
    <img
      src={image()}
      class="w-min h-auto"
      alt="logo"
    />
  );
};

export default SingleImage;
