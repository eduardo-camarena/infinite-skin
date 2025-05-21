import type { Component, JSX } from 'solid-js';

import { classNames } from '../utils/classNames';

type ButtonProps = Omit<
	JSX.ButtonHTMLAttributes<HTMLButtonElement>,
	'disabled'
> & {
	text: string | JSX.Element;
	padding?: string;
	variant?: 'blue' | 'red' | 'gray';
	rounded?: 'left' | 'right' | 'top' | 'bottom' | 'full' | 'md' | 'none';
};

const Button: Component<ButtonProps> = ({
	text,
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

	const variantClass = (() => {
		switch (variant) {
			case 'blue':
				return 'shadow-xs text-white bg-indigo-600 hover:bg-indigo-700';
			case 'red':
				return 'shadow-xs text-white bg-red-600 hover:bg-red-500';
			case 'gray':
				return 'shadow-xs text-gray-700 bg-gray-200';
		}
	})();

	return (
		<button
			disabled={variant === 'gray'}
			class={classNames(
				padding,
				roundedClass,
				variantClass,
				'inline-flex justify-center border border-transparent font-medium focus:outline-hidden focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500',
			)}
			{...extraProps}
		>
			{text}
		</button>
	);
};

export default Button;
