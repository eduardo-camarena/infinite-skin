import { Outlet, useNavigate } from '@solidjs/router';
import { Component, createEffect } from 'solid-js';

import { authStore, setAuthStore } from '../stores/authStore';

const RequireAuth: Component = () => {
  const navigate = useNavigate();
  const auth = authStore;

  createEffect(() => {
    const savedUser = localStorage.getItem('user');
    if (!auth.user && !savedUser) {
      navigate('/login');
    } else if (savedUser) {
      const { loggedInAt, ...newUser } = JSON.parse(savedUser);
      if (new Date(loggedInAt).getDate() < new Date().getDate()) {
        navigate('/login');
      }
      setAuthStore('user', newUser);
    }
  }, []);

  return (
    <>
      <Outlet />
    </>
  );
};

export default RequireAuth;
