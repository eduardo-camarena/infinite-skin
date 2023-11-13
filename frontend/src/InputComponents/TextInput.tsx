import { Component, JSX, Show, splitProps } from 'solid-js';
import { FieldProps, Field } from 'solid-form-handler';

type TextInputProps = JSX.InputHTMLAttributes<HTMLInputElement> &
  FieldProps & {
    label?: string;
  };

const TextInput: Component<TextInputProps> = (props) => {
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
              class="block w-full rounded-md scroll-mt-24 scroll-mb-32 dark:bg-stone-800 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 border-gray-300"
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
// {!error
//                 ? ' shadow-sm focus:ring-indigo-500 focus:border-indigo-500 border-gray-300'
//                 : 'border-red-300 dark:border-red-500 text-red-900 dark:text-red-600 placeholder-red-300 focus:outline-none focus:ring-red-500 focus:border-red-500'
//                 }

export default TextInput;
