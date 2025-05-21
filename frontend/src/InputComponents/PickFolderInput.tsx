import { FieldProps, Field } from 'solid-form-handler';
import { Component, For, JSX, Resource, Show, splitProps } from 'solid-js';

type PickFolderInputProps = JSX.InputHTMLAttributes<HTMLInputElement> &
	FieldProps & {
		label: string;
		items: Resource<string[]>;
		setValue: (newValue: string) => void;
	};

const PickFolderInput: Component<PickFolderInputProps> = (props) => {
	const [local, rest] = splitProps(props, [
		'classList',
		'label',
		'name',
		'formHandler',
		'type',
		'items',
		'setValue',
	]);

	return (
		<Field
			{...props}
			mode="input"
			render={(field) => (
				<div class="relative text-left pb-[10px]">
					<label class="peer block text-sm font-medium text-gray-700 dark:text-gray-300">
						{local.label}
						<input
							name={local.name}
							{...rest}
							{...field.props}
							type={local.type}
							class="mt-1block w-full rounded-md scroll-mt-24 scroll-mb-32 dark:bg-stone-800 shadow-xs focus:ring-indigo-500 focus:border-indigo-500 border-gray-300"
						/>
					</label>
					<Show when={field.helpers.error}>
						<div class="h-6 mt-1">
							<p class="text-sm text-red-600">{field.helpers.errorMessage}</p>
						</div>
					</Show>
					<div class="peer-in-focus:hidden w-full absolute bg-stone-800 border-b border-x border-gray-300 rounded-lg-md">
						<Show
							when={
								local.formHandler?.getFieldValue(local.name).slice(-1) === '/'
							}
						>
							<For each={local.items()}>
								{(item) => (
									<p class="border-t" onClick={() => props.setValue(item)}>
										{item}
									</p>
								)}
							</For>
						</Show>
					</div>
				</div>
			)}
		/>
	);
};

export default PickFolderInput;
