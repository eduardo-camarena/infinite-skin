import { jwtDecode } from 'jwt-decode';
import { createStore } from 'solid-js/store';
import { httpClient } from '../utils/httpClient';

const { VITE_HOST: HOST } = import.meta.env;

type User = {
  id: number;
  username: string;
  role: 'admin';
};

type DecodedToken = {
  sub: number;
  iat: number;
  exp: number;
};

type UserStore = {
  loading: 'idle' | 'pending';
  user: User | null;
} & ({
  token: null;
  decodedToken: null;
} | {
  token: string;
  decodedToken: DecodedToken;
});

export const [authStore, setAuthStore] = createStore<UserStore>({ loading: 'idle', user: null, token: null, decodedToken: null });

export const getTokenInfo = (): void => {
  const persistedToken = localStorage.getItem('token');

  if (persistedToken === null) {
    return;
  }

  let decodedToken = jwtDecode<DecodedToken>(persistedToken);

  if (new Date().valueOf() > decodedToken.exp * 1000) {
    return;
  }

  setAuthStore(() => ({ token: persistedToken, decodedToken }));
};

export const getUsers = async (): Promise<Omit<User, 'role'>[]> => {
  const { data } = await httpClient.get('/users');

  return data;
};

export const getUser = async (): Promise<void> => {
  const { data } = await httpClient.get('/users/me');
  console.log(data);

  setAuthStore(() => ({ user: data }));
}

export type NewUserPayload = {
  username: string;
  password: string;
  role: 'admin' | 'user';
}

export const newUser = async (payload: NewUserPayload): Promise<User> => {
  const { data } = await httpClient.post(`/users/new`, payload);
  const { token, ...user } = data;

  setAuthStore(() => ({ user, token }));
  localStorage.setItem('token', token);
  return user;
}

export const login = async (payload: { id: number; password?: string }): Promise<User> => {
  const { data } = await httpClient.post(`/users/login`, payload);
  const { token, ...user } = data;

  setAuthStore(() => ({ user, token }));
  localStorage.setItem('token', token);
  return user;
};
