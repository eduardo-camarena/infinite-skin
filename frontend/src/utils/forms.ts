import { FormHandler } from 'solid-form-handler';

export function onSubmitHandler<T>(
	formHandler: FormHandler<T>,
	onSubmit: (event: Event, formData: T) => Promise<void> | void,
	onFlowFinish?: () => void,
): (event: Event) => Promise<void> {
	return async (event: Event) => {
		event.preventDefault();
		const formValues = formHandler.formData();

		const { isFormInvalid } = await formHandler.validateForm();

		if (isFormInvalid) {
			throw new Error();
		}

		onSubmit(event, formValues);
		formHandler.resetForm();

		if (onFlowFinish) {
			onFlowFinish();
		}
	};
}
