import { useNavigate } from '@solidjs/router';
import { Component, createResource, For, Show } from 'solid-js';

import Loading from './Loading';
import { getUsers, login } from '../stores/authStore';

const Login: Component = () => {
  const navigate = useNavigate();
  const [users] = createResource(getUsers);

  return (
    <Show when={users()} fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}>
      <div class="px-12">
        <div class="flex flex-wrap justify-center gap-8">
          <For each={users()}>
            {(user) => (
              <div class="flex flex-col" onClick={() => {
                login({ id: user.id }).then((user) => {
                  localStorage.setItem('user', JSON.stringify({ ...user, loggedInAt: new Date().toISOString() }))
                  navigate('/');
                });
              }}>
                <p>{user.username}</p>
              </div>
            )}
          </For>
        </div>
      </div>
    </Show>
  );
};

export default Login;
