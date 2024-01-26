import { Component, Show } from 'solid-js';

import Loading from './Loading';
import Button from '../InputComponents/Button';
import { authStore } from '../stores/authStore';
import { httpClient } from '../utils/httpClient';

const Settings: Component = () => {
  return (
    <div class="pt-8 flex flex-col content-center items-center h-full">
      <Show
        when={authStore.user}
        fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
      >
        <Show when={authStore.user?.role}>
          <Button
            text="Scan"
            variant="blue"
            onClick={() => {
              httpClient.post('/scan').then((response) => {
                console.log(response.status);
              });
            }}
          />
        </Show>
      </Show>
    </div>
  );
};

export default Settings;
