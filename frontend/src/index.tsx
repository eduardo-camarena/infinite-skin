import { Router, Route } from '@solidjs/router';
import { ParentComponent } from 'solid-js';
import { render } from 'solid-js/web';

import './index.css';

import Login from './components/Login';
import Menu from './components/Menu';
import RequireAuth from './components/RequireAuth';
import Settings from './components/Settings';
import AlbumInfo from './pages/album/AlbumInfo';
import Albums from './pages/album/Albums';
import AlbumViewer from './pages/album/AlbumViewer';
import WelcomeScreen from './pages/WelcomeScreen/WelcomeScreen';
const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?',
  );
}

const App: ParentComponent = ({ children }) => {
  return <Menu>{children}</Menu>;
};

render(
  () => (
    <Router root={App}>
      <Route path="/login" component={Login} />
      <Route path="/" component={RequireAuth}>
        <Route path="/settings" component={Settings} />
        <Route path="/" component={WelcomeScreen} />
        <Route path="/a">
          <Route path="/" component={Albums} />
          <Route path="/:albumId">
            <Route path="/" component={AlbumInfo} />
            <Route path="/p/:imageId" component={AlbumViewer} />
          </Route>
        </Route>
      </Route>
    </Router>
  ),
  root!,
);
