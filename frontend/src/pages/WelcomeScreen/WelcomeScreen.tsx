import { Component } from 'solid-js';

const WelcomeScreen: Component = () => {
  return (
    <div class="flex flex-col content-center items-center h-full">
      <a class="w-full flex-1 flex border-b" href="/">
        <p class="self-center m-auto">TV shows</p>
      </a>
      <a class="w-full flex-1 flex border-b" href="/">
        <p class="self-center m-auto">Movies</p>
      </a>
      <a class="w-full flex-1 flex" href="/a">
        <p class="self-center m-auto">Albums</p>
      </a>
    </div>
  );
};

export default WelcomeScreen;
