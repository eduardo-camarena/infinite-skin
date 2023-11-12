import { Outlet, useNavigate } from '@solidjs/router';
import { Component, createEffect } from 'solid-js';

import { getTokenInfo, getUser } from '../stores/authStore';

const RequireAuth: Component = () => {
  const navigate = useNavigate();

  createEffect(() => {
    const persistedToken = localStorage.getItem('token');
    if (!persistedToken) {
      navigate('/login');
    } else {
      getTokenInfo();
      getUser();
    }
  });

  return (
    <>
      <Outlet />
    </>
  );
};

export default RequireAuth;
