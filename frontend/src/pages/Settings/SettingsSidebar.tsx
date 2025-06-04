import { ParentComponent } from 'solid-js';

const SettingsSidebar: ParentComponent = ({ children }) => {
	return (
		<div class="h-full w-full flex flex-row items-stretch gap-2">
			<div class="h-full border-r border-gray-500">
				<div class="flex flex-col">
					<a
						class="text-xl px-12 py-2 hover:bg-stone-800"
						href="/settings/libraries"
					>
						Libraries
					</a>
				</div>
			</div>
			<div class="flex-1">{children}</div>
		</div>
	);
};

export default SettingsSidebar;
