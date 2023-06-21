import { A } from '@solidjs/router';
import { Component } from 'solid-js';

const WelcomeScreen: Component = () => {
  return (
    <div class="flex flex-col content-center items-center h-full">
      <A class="w-full flex-1 flex border-b" href="/">
        <p class="self-center m-auto">TV shows</p>
      </A>
      <A class="w-full flex-1 flex border-b" href="/">
        <p class="self-center m-auto">Movies</p>
      </A>
      <A class="w-full flex-1 flex" href="/a">
        <p class="self-center m-auto">Albums</p>
      </A>
    </div>
  );
};

export default WelcomeScreen;
