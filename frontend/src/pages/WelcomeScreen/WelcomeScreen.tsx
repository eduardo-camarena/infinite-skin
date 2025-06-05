import {
	Component,
	createResource,
	createSignal,
	For,
	Match,
	Show,
	Switch,
} from 'solid-js';
import { getLibraries, libraryStore } from '../../stores/libraries';
import Loading from '../../components/Loading';
import AlbumThumbnail from '../../components/AlbumThumbnail';
import { HiSolidArrowRight } from 'solid-icons/hi';
import Button from '../../InputComponents/Button';
import { scan } from '../../stores/settingsStore';

const WelcomeScreen: Component = () => {
	const [loading, setLoading] = createSignal(false);
	createResource(getLibraries);

	return (
		<div class="pt-4 ml-8">
			<div class="prose">
				<div class="not-prose isolate">
					<div class="flex flex-col h-full">
						<Show
							when={libraryStore.libraries}
							fallback={<Loading margin="my-auto" />}
						>
							<For each={libraryStore.libraries}>
								{(library) => (
									<div class="pb-4">
										<a
											class="flex gap-2 hover:font-semibold"
											href={`/libraries/${library.id}/albums`}
										>
											<p class="capitalize">{library.name}</p>
											<div class="my-auto">
												<HiSolidArrowRight size="16" />
											</div>
										</a>
										<Switch
											fallback={
												<Loading margin="ml-[calc(50%-1rem)] pt-[110px]" />
											}
										>
											<Match when={library.previewAlbums === null}>
												<p>No content found</p>
												<Button
													text={
														<Show
															when={loading()}
															fallback={
																<Loading size="w-6 h-6" margin="mx-auto" />
															}
														>
															<p>Scan</p>
														</Show>
													}
													variant="blue"
													onClick={() => {
														setLoading(true);
														scan().finally(() => setLoading(false));
													}}
												/>
											</Match>
											<Match
												when={
													library.previewAlbums && library.previewAlbums.length
												}
											>
												<div class="not-prose overflow-auto">
													<div class="relative">
														<div class="relative flex gap-4 w-full overflow-x-auto snap-x py-4">
															<For each={library.previewAlbums}>
																{(album) => (
																	<div class="shrink-0 snap-center">
																		<AlbumThumbnail
																			albumId={album.id}
																			libraryId={library.id}
																			albumName={album.name}
																		/>
																	</div>
																)}
															</For>
														</div>
													</div>
												</div>
											</Match>
										</Switch>
									</div>
								)}
							</For>
						</Show>
					</div>
				</div>
			</div>
		</div>
	);
};

export default WelcomeScreen;
