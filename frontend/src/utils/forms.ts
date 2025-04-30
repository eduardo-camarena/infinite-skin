import { FormHandler } from 'solid-form-handler';

export async function onSubmitHandler<T, V>(
	event: Event,
	formHandler: FormHandler,
	onSubmit: (payload: T) => Promise<V>,
	payload: T,
): Promise<V | undefined> {
	event.preventDefault();
	try {
		await formHandler.validateForm();
		const res = await onSubmit(payload);
		formHandler.resetForm();

		return res;
	} catch (error) {
		console.error(error);
	}
}
