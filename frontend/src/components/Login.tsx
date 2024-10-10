import { useNavigate } from '@solidjs/router';
import { Component, createResource, For, Match, Switch } from 'solid-js';

import CreateUser from './CreateAdmin';
import Loading from './Loading';
import { getUsers, login } from '../stores/authStore';

const Login: Component = () => {
  const navigate = useNavigate();
  const [users] = createResource(getUsers);

  return (
    <Switch
      fallback={<Loading margin="ml-[calc(50%-1rem)] mt-[calc(50%-1rem)]" />}
    >
      <Match when={users()?.length === 0}>
        <CreateUser isAdmin />
      </Match>

      <Match when={users()?.length}>
        <div class="px-12 pt-6">
          <div class="flex flex-wrap justify-center gap-8">
            <For each={users()}>
              {(user) => {
                return (
                  <div
                    class="flex flex-col"
                    onClick={() => {
                      login({ id: user.id }).then(() => {
                        navigate('/');
                      });
                    }}
                  >
                    <p>{user.username}</p>
                  </div>
                );
              }}
            </For>
          </div>
        </div>
      </Match>
    </Switch>
  );
};

export default Login;
