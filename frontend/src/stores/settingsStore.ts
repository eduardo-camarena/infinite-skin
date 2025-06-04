import { createStore } from 'solid-js/store';

import { httpClient } from '../utils/httpClient';

type OptionsStore = {
	loading: 'idle' | 'pending';
};

export const [settingsStore, setSettingsStore] = createStore<OptionsStore>({
	loading: 'idle',
});

export const scan = async (
	libraryIds?: [number, ...number[]],
): Promise<void> => {
	setSettingsStore(() => ({ loading: 'pending' }));

	await httpClient.post('/libraries/scan', { params: { libraryIds } });

	setSettingsStore(() => ({ loading: 'idle' }));
};
