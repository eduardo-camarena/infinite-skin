import { createStore } from 'solid-js/store';

const { VITE_HOST: HOST } = import.meta.env;

type User = {
  id: number;
  username: string;
  role: 'admin';
};

type UserStore = {
  loading: 'idle' | 'pending';
  user: User | null;
};

export const [authStore, setAuthStore] = createStore<UserStore>({ loading: 'idle', user: null });

export const getUsers = async (): Promise<Omit<User, 'role'>[]> => {
  const users = await fetch(`${HOST}/users`).then((res) => res.json())

  return users;
};

export const login = async (payload: { id: number; password?: string }): Promise<User> => {
  const user = await fetch(`${HOST}/users/${payload.id}/login`, { headers: { 'Content-Type': 'application/json' }, method: 'POST', body: JSON.stringify({ password: payload.password }) }).then((res) =>
    res.json());

  setAuthStore('user', user);
  return user;
};
