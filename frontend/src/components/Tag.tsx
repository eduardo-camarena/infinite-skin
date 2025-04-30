import { useNavigate } from '@solidjs/router';
import { Component, JSXElement } from 'solid-js';

import { generateUrlWithQueryParams } from '../utils/queryParams';

type TagProps = {
	href: string;
	tagParams: Record<string, string | number>;
	text: string | JSXElement;
};

const Tag: Component<TagProps> = ({ href, tagParams, text }) => {
	const navigate = useNavigate();
	return (
		<div
			onClick={() => {
				navigate(generateUrlWithQueryParams(href, tagParams));
			}}
			class="px-2 dark:bg-neutral-700/40 rounded-md"
		>
			<p>{text}</p>
		</div>
	);
};

export default Tag;
