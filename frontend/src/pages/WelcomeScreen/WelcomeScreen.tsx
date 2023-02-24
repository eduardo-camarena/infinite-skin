import type { Component } from 'solid-js';

import logo from '../../logo.svg';

const WelcomeScreen: Component = () => {
  return (
    <div class="flex flex-col justify-center content-center w-full pt-40">
      <img
        src={logo}
        class="animate-spin h-44 pointer-events-none"
        alt="logo"
      />
      <p class="pt-8">Hello world.</p>
    </div>
  );
};

export default WelcomeScreen;
