import { Accessor, Component, For } from 'solid-js';

import { classNames } from '../utils/classNames';

type ButtonGroupProps = {
	buttonsText: [string, string, ...string[]];
	textSize?: `text-${'sm' | 'md' | 'lg'}`;
	px?: `px-${number}`;
	py?: `py-${number}`;
	currentSelect: Accessor<number>;
	setCurrentSelect: (newVal: number) => void;
};

const ButtonGroup: Component<ButtonGroupProps> = ({
	buttonsText,
	px = 'px-2.5',
	py = 'py-1',
	textSize = 'text-md',
	currentSelect,
	setCurrentSelect,
}) => {
	return (
		<span class="relative z-0 inline-flex shadow-xs rounded-md">
			<button
				type="button"
				onClick={() => {
					setCurrentSelect(0);
				}}
				class={classNames(
					'relative inline-flex items-center rounded-l-md border border-stone-700 bg-white dark:bg-stone-800 text-gray-700 dark:text-gray-400 hover:bg-gray-50',
					px,
					py,
					textSize,
					currentSelect() === 0
						? 'z-10 outline-hidden ring-1 ring-indigo-500 border-indigo-500'
						: '',
				)}
			>
				{buttonsText[0]}
			</button>
			<For each={buttonsText.slice(1, buttonsText.length - 1)}>
				{(text, index) => {
					return (
						<button
							type="button"
							onClick={() => {
								setCurrentSelect(index() + 1);
							}}
							class={classNames(
								'-ml-px relative inline-flex items-center border border-stone-700 bg-white dark:bg-stone-800 text-gray-700 dark:text-gray-400 hover:bg-gray-50',
								px,
								py,
								textSize,
								currentSelect() === index() + 1
									? 'z-10 outline-hidden ring-1 ring-indigo-500 border-indigo-500'
									: '',
							)}
						>
							{text}
						</button>
					);
				}}
			</For>
			<button
				type="button"
				onClick={() => {
					setCurrentSelect(buttonsText.length - 1);
				}}
				class={classNames(
					'-ml-px relative inline-flex items-center rounded-r-md border border-stone-700 bg-white dark:bg-stone-800 text-gray-700 dark:text-gray-400 hover:bg-gray-50',
					px,
					py,
					textSize,
					currentSelect() === buttonsText.length - 1
						? 'z-10 outline-hidden ring-1 ring-indigo-500 border-indigo-500'
						: '',
				)}
			>
				{buttonsText[buttonsText.length - 1]}
			</button>
		</span>
	);
};

export default ButtonGroup;
