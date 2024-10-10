import axios from 'axios';

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

  config.data = config.data ? changeObjCase(config.data, 'snake') : config.data;
  config.params = config.params
    ? changeObjCase(config.params, 'snake')
    : config.params;

  return config;
});

httpClient.interceptors.response.use((response) => {
  response.data = changeObjCase(response.data, 'camel');

  return response;
});

type NestedRecord = {
  [key: string]: string | number | boolean | NestedRecord | NestedRecord[];
};

function changeObjCase(
  obj: NestedRecord,
  caseType: 'snake' | 'camel',
): NestedRecord {
  const newObj: NestedRecord = {};

  for (const [key, val] of Object.entries(obj)) {
    const newKey = transformKey(key, caseType);
    if (Array.isArray(val)) {
      newObj[newKey] = val.map((v) => changeObjCase(v, caseType));
    } else if (!Array.isArray(val) && val instanceof Object) {
      newObj[newKey] = changeObjCase(val, caseType);
    } else {
      newObj[newKey] = val;
    }
  }

  return newObj;
}

function transformKey(key: string, caseType: 'snake' | 'camel'): string {
  switch (caseType) {
    case 'snake':
      let newKey = '';
      for (const char of key) {
        if (char === char.toUpperCase()) {
          newKey = newKey + '_' + char.toLowerCase();
        } else {
          newKey = newKey + char;
        }
      }

      return newKey;
    case 'camel':
      return key
        .split('_')
        .map((v, i) => (i === 0 ? v : v[0].toLocaleUpperCase() + v.slice(1)))
        .join('');
  }
}
