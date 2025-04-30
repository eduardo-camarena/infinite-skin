import { Component, Show } from 'solid-js';

import Loading from './Loading';
import Button from '../InputComponents/Button';
import { authStore } from '../stores/authStore';
import { scan, settingsStore } from '../stores/settingsStore';

const Settings: Component = () => {
	return (
		<div class="pt-8 flex flex-col content-center items-center h-full">
			<Show
				when={authStore.user}
				fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
			>
				<Show when={authStore.user?.role}>
					<Button
						text={
							<Show
								when={settingsStore.loading === 'idle'}
								fallback={
									<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />
								}
							>
								<p>Scan</p>
							</Show>
						}
						variant="blue"
						onClick={async () => {
							await scan();
						}}
					/>
				</Show>
			</Show>
		</div>
	);
};

export default Settings;
