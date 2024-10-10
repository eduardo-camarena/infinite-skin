import { useNavigate } from '@solidjs/router';
import { ParentComponent, createEffect } from 'solid-js';

import { getTokenInfo, getUser } from '../stores/authStore';

const RequireAuth: ParentComponent = ({ children }) => {
  const navigate = useNavigate();

  createEffect(() => {
    const persistedToken = localStorage.getItem('token');
    if (!persistedToken) {
      navigate('/login');
    } else {
      getTokenInfo();
      getUser().catch(() => {
        navigate('/login');
      });
    }
  });

  return <>{children}</>;
};

export default RequireAuth;
