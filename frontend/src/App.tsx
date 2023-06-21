import { Route, Routes } from '@solidjs/router';
import type { Component } from 'solid-js';

import Menu from './components/Menu';
import AlbumInfo from './pages/album/AlbumInfo';
import Albums from './pages/album/Albums';
import AlbumViewer from './pages/album/AlbumViewer';
import WelcomeScreen from './pages/WelcomeScreen/WelcomeScreen';

const App: Component = () => {
  return (
    <Menu>
      <Routes>
        <Route path="/" component={WelcomeScreen} />
        <Route path="/a">
          <Route path="/" component={Albums} />
          <Route path="/:albumId">
            <Route path="/" component={AlbumInfo} />
            <Route path="/p/:imageId" component={AlbumViewer} />
          </Route>
        </Route>
      </Routes>
    </Menu>
  );
};

export default App;
