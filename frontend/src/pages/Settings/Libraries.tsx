import {
	Component,
	createResource,
	createSignal,
	Match,
	Show,
	Switch,
} from 'solid-js';

import Loading from '../../components/Loading';
import Button from '../../InputComponents/Button';
import { authStore } from '../../stores/authStore';
import { scan, settingsStore } from '../../stores/settingsStore';
import TextInput from '../../InputComponents/TextInput';
import CheckboxInput from '../../InputComponents/CheckboxInput';
import { useFormHandler } from 'solid-form-handler';
import { zodSchema } from 'solid-form-handler/zod';
import { onSubmitHandler } from '../../utils/forms';
import { z } from 'zod';
import { createLibrary, getPossibleLibraries } from '../../stores/libraries';
import PickFolderInput from '../../InputComponents/PickFolderInput';

const newLibrarySchema = z.object({
	name: z.string().nonempty(),
	location: z.string().nonempty(),
	isPrivate: z.string().optional(),
});

const CreateLibrary: Component = () => {
	const [loading, setLoading] = createSignal(false);
	const formHandler = useFormHandler(zodSchema(newLibrarySchema));
	formHandler.setFieldValue('location', '/media_folder/');

	const [possibleLibraries, { mutate: mutatePossibleLibraries }] =
		createResource(formHandler.formData().location, getPossibleLibraries);

	const onSubmit = onSubmitHandler(
		formHandler,
		async (_, formValues): Promise<void> => {
			await createLibrary({
				...formValues,
				isPrivate: formValues.isPrivate === 'on',
			});
		},
	);

	return (
		<div class="pt-8 flex flex-col content-center items-center h-full">
			<Show
				when={authStore.user}
				fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
			>
				<Show when={authStore.user?.role}>
					<div class="sm:w-[80%] md:w-[35%] py-4">
						<h1 class="pb-2">Create new Library</h1>
						<form onSubmit={onSubmit}>
							<TextInput label="Name" name="name" formHandler={formHandler} />
							<PickFolderInput
								label="Location"
								name="location"
								items={possibleLibraries}
								setValue={async (newValue) => {
									formHandler.setFieldValue('location', `${newValue}/`);
									mutatePossibleLibraries(await getPossibleLibraries(newValue));
								}}
								formHandler={formHandler}
							/>
							<CheckboxInput
								label="Is private"
								name="isPrivate"
								formHandler={formHandler}
							/>
							<Button
								text="Create"
								type="submit"
								variant="gray"
								isDisabled={formHandler.isFormInvalid}
							/>
						</form>
					</div>
					<Button
						text={
							<Show
								when={settingsStore.loading === 'idle'}
								fallback={
									<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />
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
				</Show>
			</Show>
		</div>
	);
};

export default CreateLibrary;
