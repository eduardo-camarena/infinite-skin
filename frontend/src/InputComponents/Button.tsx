import {
	Accessor,
	Match,
	Show,
	Switch,
	type Component,
	type JSX,
} from 'solid-js';

import { classNames } from '../utils/classNames';
import Loading from '../components/Loading';

type Variant = 'blue' | 'red' | 'gray';

type ButtonProps = Omit<
	JSX.ButtonHTMLAttributes<HTMLButtonElement>,
	'disabled'
> & {
	text: string | JSX.Element;
	padding?: string;
	loading?: Accessor<boolean>;
	isDisabled?: Accessor<boolean>;
	variant?: Variant;
	rounded?: 'left' | 'right' | 'top' | 'bottom' | 'full' | 'md' | 'none';
};

const Button: Component<ButtonProps> = ({
	text,
	loading,
	isDisabled,
	variant = 'blue',
	rounded = 'md',
	padding = 'px-6 py-2',
	...extraProps
}) => {
	const roundedClass = (() => {
		switch (rounded) {
			case 'left':
				return 'rounded-l-md';
			case 'right':
				return 'rounded-r-md';
			case 'top':
				return 'rounded-t-md';
			case 'bottom':
				return 'rounded-b-md';
			case 'md':
				return 'rounded-md';
			case 'full':
				return 'rounded-full';
			case 'none':
				return '';
		}
	})();

	const getVariantClass = (variant: Variant) => {
		switch (variant) {
			case 'blue':
				return 'shadow-xs text-white bg-indigo-600 hover:bg-indigo-700';
			case 'red':
				return 'shadow-xs text-white bg-red-600 hover:bg-red-500';
			case 'gray':
				return 'shadow-xs text-gray-700 bg-gray-200';
		}
	};

	return (
		<Switch>
			<Match when={typeof isDisabled !== 'undefined' && isDisabled()}>
				<button
					disabled={true}
					class={classNames(
						padding,
						roundedClass,
						getVariantClass('gray'),
						'inline-flex justify-center border border-transparent font-medium focus:outline-hidden focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500',
					)}
					{...extraProps}
				>
					{text}
				</button>
			</Match>
			<Match when={typeof isDisabled === 'undefined' || isDisabled() === false}>
				<button
					class={classNames(
						padding,
						roundedClass,
						getVariantClass(variant),
						'inline-flex justify-center border border-transparent font-medium focus:outline-hidden focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500',
					)}
					{...extraProps}
				>
					<Show
						when={typeof loading === 'undefined' || loading() === false}
						fallback={<Loading margin="m-auto" />}
					>
						{text}
					</Show>
				</button>
			</Match>
		</Switch>
	);
};

export default Button;
