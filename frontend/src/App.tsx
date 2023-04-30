import { Route, Routes } from '@solidjs/router';
import type { Component } from 'solid-js';

import AlbumWrapper from './pages/album/[albumId]';
import AlbumInfo from './pages/album/AlbumInfo';
import AlbumViewer from './pages/album/AlbumViewer';
import WelcomeScreen from './pages/WelcomeScreen/WelcomeScreen';

const App: Component = () => {
  return (
    <main class="text-center w-full h-full">
      <Routes>
        <Route path="/" component={WelcomeScreen} />

        <Route path="/images/:albumId" component={AlbumWrapper}>
          <Route path="/" component={AlbumInfo} />
          <Route path="/:imageId" component={AlbumViewer} />
        </Route>
      </Routes>
    </main>
  );
};

export default App;
