import { FieldProps, Field } from 'solid-form-handler';
import { Component, JSX, Show, splitProps } from 'solid-js';

type TextInputProps = JSX.InputHTMLAttributes<HTMLInputElement> &
	FieldProps & {
		label?: string;
	};

const TextInput: Component<TextInputProps> = (props) => {
	const [local, rest] = splitProps(props, [
		'classList',
		'label',
		'formHandler',
		'type',
	]);
	return (
		<Field
			{...props}
			mode="input"
			render={(field) => (
				<div class={`text-left pb-[10px]`}>
					{local.label && (
						<label
							class={`block text-sm font-medium text-gray-700 dark:text-gray-300`}
							for={field.props.id}
						>
							{local.label}
						</label>
					)}
					<div class="mt-1">
						<input
							{...rest}
							{...field.props}
							type={local.type}
							class="block w-full rounded-md scroll-mt-24 scroll-mb-32 dark:bg-stone-800 shadow-xs focus:ring-indigo-500 focus:border-indigo-500 border-gray-300"
						/>
						<Show when={field.helpers.error}>
							<div class="h-6 mt-1">
								<p class="text-sm text-red-600">{field.helpers.errorMessage}</p>
							</div>
						</Show>
					</div>
				</div>
			)}
		/>
	);
};

export default TextInput;
