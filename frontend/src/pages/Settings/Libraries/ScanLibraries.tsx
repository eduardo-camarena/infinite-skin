import { Component, createResource, createSignal, For, Show } from 'solid-js';
import Button from '../../../InputComponents/Button';
import { getLibraries, libraryStore } from '../../../stores/libraries';
import Loading from '../../../components/Loading';
import { HiSolidCheckCircle } from 'solid-icons/hi';
import { scan } from '../../../stores/settingsStore';

const ScanLibraries: Component = () => {
	const [librariesToScan, setLibrariesToScan] = createSignal<number[]>([]);
	createResource(getLibraries);

	return (
		<div class="mx-6 pt-4">
			<div class="flex flex-wrap gap-x-2 gap-y-4 pb-8">
				<Show
					when={libraryStore.libraries}
					fallback={
						<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />
					}
				>
					<For each={libraryStore.libraries}>
						{(library) => (
							<div
								class="w-[300px] h-[200px] relative"
								onClick={() =>
									setLibrariesToScan(
										librariesToScan().includes(library.id)
											? librariesToScan().filter(
													(libraryId) => libraryId !== library.id,
												)
											: [...librariesToScan(), library.id],
									)
								}
							>
								<Show when={librariesToScan().includes(library.id)}>
									<div class="absolute w-full h-full bg-blue-300/50">
										<div class="pt-[34px]">
											<HiSolidCheckCircle
												style={{
													margin: 'auto',
													color: 'blue',
													opacity: 0.2,
												}}
												size="100"
											/>
										</div>
									</div>
								</Show>
								<div class="flex flex-col justify-center h-full overflow-hidden">
									{/* <img */}
									{/* 	src={`${HOST}/albums/${albumId}/images/1`} */}
									{/* 	alt={library.name} */}
									{/* 	loading="lazy" */}
									{/* /> */}
								</div>
								<div
									id={`${library.id}`}
									class="absolute py-1 h-auto bottom-0 w-full text-center bg-stone-900/40 font-semibold overflow-hidden"
								>
									<p>{library.name}</p>
								</div>
							</div>
						)}
					</For>
				</Show>
			</div>
			<Button
				text="Scan"
				type="button"
				variant="blue"
				onClick={() => {
					scan(
						librariesToScan().length
							? (librariesToScan() as [number, ...number[]])
							: undefined,
					);
					setLibrariesToScan([]);
				}}
			/>
		</div>
	);
};

export default ScanLibraries;
