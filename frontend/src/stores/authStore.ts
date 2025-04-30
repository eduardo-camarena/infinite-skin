import { jwtDecode } from 'jwt-decode';
import { createStore } from 'solid-js/store';

import { httpClient } from '../utils/httpClient';

type User = {
	id: number;
	username: string;
	role: 'admin' | 'user';
};

type DecodedToken = {
	sub: number;
	iat: number;
	exp: number;
};

type UserStore = {
	loading: 'idle' | 'pending';
	user: User | null;
} & (
	| {
			token: null;
			decodedToken: null;
	  }
	| {
			token: string;
			decodedToken: DecodedToken;
	  }
);

export const [authStore, setAuthStore] = createStore<UserStore>({
	loading: 'idle',
	user: null,
	token: null,
	decodedToken: null,
});

export const getTokenInfo = (): void => {
	const persistedToken = localStorage.getItem('token');

	if (persistedToken === null) {
		return;
	}

	const decodedToken = jwtDecode<DecodedToken>(persistedToken);

	if (new Date().valueOf() > decodedToken.exp * 1000) {
		return;
	}

	setAuthStore(() => ({ token: persistedToken, decodedToken }));
};

export const getUsers = async (): Promise<Omit<User, 'role'>[]> => {
	const { data } = await httpClient.get<{ users: User[] }>('/users');
	console.log(data);

	return data.users;
};

export const getUser = async (): Promise<void> => {
	try {
		const { data } = await httpClient.get('/users/me');

		setAuthStore(() => ({ user: data }));
	} catch (e) {
		localStorage.removeItem('token');
		setAuthStore(() => ({ token: null, decodedToken: null }));

		throw e;
	}
};

export type NewUserPayload = {
	username: string;
	password: string;
	role: 'admin' | 'user';
};

export const newUser = async (payload: NewUserPayload): Promise<User> => {
	const { data } = await httpClient.post(`/users/new`, payload);
	const { token, ...user } = data;

	setAuthStore(() => ({ user, token }));
	localStorage.setItem('token', token);
	return user;
};

export const login = async (payload: {
	id: number;
	password?: string;
}): Promise<User> => {
	const { data } = await httpClient.post(`/users/login`, payload);
	const { token, ...user } = data;

	setAuthStore(() => ({ user, token }));
	localStorage.setItem('token', token);
	return user;
};
