import { HiSolidCog } from 'solid-icons/hi';
import type { ParentComponent } from 'solid-js';

const Menu: ParentComponent = ({ children }) => {
	return (
		<div class="w-full h-full flex flex-col">
			<div class="flex justify-between dark:bg-neutral-900 bg-indigo-600">
				<a class="px-10 py-2 text-lg" href="/">
					infinite-skin
				</a>
				<a class="px-6 py-2" href="/settings">
					<HiSolidCog size={24} />
				</a>
			</div>
			<main class="flex-1">{children}</main>
		</div>
	);
};

export default Menu;
