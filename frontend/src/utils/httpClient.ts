import axios, { AxiosError } from 'axios';
import { authStore } from '../stores/authStore';

const { VITE_HOST: HOST } = import.meta.env;

export const httpClient = axios.create({
  baseURL: HOST,
  headers: {
    'Content-Type': 'application/json',
  },
});

httpClient.interceptors.request.use((config) => {
  config.withCredentials = Boolean(authStore.token);
  config.headers.Authorization = authStore.token
    ? `Bearer ${authStore.token}`
    : '';

  return config;
});

httpClient.interceptors.response.use(
  (response) => response,
  (error) => {
    throw (error as AxiosError).response?.data;
  }
);
