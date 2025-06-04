import { useParams } from '@solidjs/router';
import { AiOutlineLeft, AiOutlineRight } from 'solid-icons/ai';
import { FaSolidAnglesLeft, FaSolidAnglesRight } from 'solid-icons/fa';
import { HiSolidArrowLeft, HiSolidCog } from 'solid-icons/hi';
import { Accessor, Component, Setter, Show } from 'solid-js';

import Button from '../InputComponents/Button';
import type { GetImagePayload } from '../stores/currentAlbum';
import { AlbumViewerParams } from '../pages/album/AlbumViewer';

export type ViewType = 'singleImage' | 'allImages';

type AlbumViewerControlsProps = {
	albumId: string;
	lastPage: number;
	updateImage: (payload: GetImagePayload) => Promise<void>;
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
	updateImage,
}) => {
	const params = useParams<AlbumViewerParams>();

	return (
		<div class="flex py-5 relative">
			<a
				class="h-[34px] py-1.5 pl-4 z-10"
				href={`/libraries/${params.libraryId}/${albumId}`}
			>
				<HiSolidArrowLeft size={22} />
			</a>
			<div class="flex-1 flex justify-center absolute w-full py-1.5">
				<span>
					{' '}
					<FaSolidAnglesLeft
						size={22}
						onClick={() => {
							updateImage({ albumId, imageId: 1 });
						}}
					/>{' '}
				</span>
				<span class="px-4">
					<button
						onClick={() => {
							const imageId = Number.parseInt(params.imageId);
							if (imageId > 1) {
								updateImage({ albumId, imageId: imageId - 1 });
							}
						}}
					>
						<AiOutlineLeft size={22} />
					</button>
				</span>
				<p class="my-auto">
					{params.imageId} of {lastPage}
				</p>
				<span class="px-4">
					{' '}
					<AiOutlineRight
						size={22}
						onClick={() => {
							const imageId = Number.parseInt(params.imageId);
							if (imageId < lastPage) {
								updateImage({ albumId, imageId: imageId + 1 });
							}
						}}
					/>{' '}
				</span>
				<span>
					{' '}
					<FaSolidAnglesRight
						size={22}
						onClick={() => {
							updateImage({ albumId, imageId: lastPage });
						}}
					/>{' '}
				</span>
			</div>
			<div class="flex absolute right-4 gap-4 py-1.5">
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
