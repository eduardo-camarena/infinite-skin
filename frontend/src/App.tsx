import { Route, Routes } from '@solidjs/router';
import type { Component } from 'solid-js';

import Image from './pages/album/[imageId]';
import WelcomeScreen from './pages/WelcomeScreen/WelcomeScreen';

const App: Component = () => {
  return (
    <main class="text-center w-full h-full">
      <Routes>
        <Route path="/" component={WelcomeScreen} />
        <Route path="/images/:albumId/:imageId" component={Image} />
      </Routes>
    </main>
  );
};

export default App;
