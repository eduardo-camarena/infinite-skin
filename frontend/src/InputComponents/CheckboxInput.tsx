import { FieldProps, Field } from 'solid-form-handler';
import { Component, JSX, Show, splitProps } from 'solid-js';

type TextInputProps = JSX.InputHTMLAttributes<HTMLInputElement> &
	FieldProps & {
		label?: string;
	};

const CheckboxInput: Component<TextInputProps> = (props) => {
	const [local, rest] = splitProps(props, [
		'classList',
		'label',
		'formHandler',
	]);

	return (
		<Field
			{...props}
			mode="input"
			render={(field) => (
				<div class="mt-2 text-left pb-[10px] flex items-center gap-2">
					<div class="flex flex-row gap-2">
						<input
							{...rest}
							{...field.props}
							type="checkbox"
							class="block w-5 h-5 rounded-md scroll-mt-24 scroll-mb-32 dark:bg-stone-800 shadow-xs focus:ring-indigo-500 focus:border-indigo-500 border-gray-300"
						/>
						<Show when={field.helpers.error}>
							<div class="h-6 mt-1">
								<p class="text-sm text-red-600">{field.helpers.errorMessage}</p>
							</div>
						</Show>
					</div>
					{local.label && (
						<label
							class="block text-sm font-medium text-gray-700 dark:text-gray-300"
							for={field.props.id}
						>
							{local.label}
						</label>
					)}
				</div>
			)}
		/>
	);
};

export default CheckboxInput;
