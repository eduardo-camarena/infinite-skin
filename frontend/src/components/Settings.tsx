import { Component, Show } from 'solid-js';
import Button from '../InputComponents/Button';
import { authStore } from '../stores/authStore';
import { httpClient } from '../utils/httpClient';
import Loading from './Loading';

const Settings: Component = () => {
  return (
    <div class="flex flex-col content-center items-center h-full">
      <Show when={authStore.user} fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}>
        <Show when={authStore.user?.role}>
          <Button text="Scan" variant="blue" onClick={() => {
            httpClient.post('/albums/scan').then((response) => {
              console.log(response.status);
            });
          }} />
        </Show>
      </Show>
    </div>
  );
};

export default Settings;
