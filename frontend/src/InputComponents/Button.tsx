import type { Component, JSX } from 'solid-js';

import { classNames } from '../utils/classNames';

type ButtonProps = JSX.ButtonHTMLAttributes<HTMLButtonElement> & {
  text: string;
  padding?: string;
  disabled?: boolean;
  variant: 'blue' | 'gray';
  rounded: 'left' | 'right' | 'top' | 'bottom' | 'full' | 'none';
};

const Button: Component<ButtonProps> = ({
  text,
  variant,
  rounded,
  disabled,
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
      case 'full':
        return 'rounded-md';
      case 'none':
        return '';
    }
  })();

  const variantClass = (() => {
    switch (variant) {
      case 'blue':
        return 'shadow-sm text-white bg-indigo-600 hover:bg-indigo-700';
      case 'gray':
        return 'shadow-sm text-gray-700 bg-gray-200';
    }
  })();

  return (
    <button
      disabled={disabled || variant === 'gray'}
      class={classNames(
        padding,
        roundedClass,
        variantClass,
        'inline-flex justify-center border border-transparent font-medium focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500'
      )}
      {...extraProps}
    >
      {text}
    </button>
  );
};

export default Button;
