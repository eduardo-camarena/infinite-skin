import { A } from '@solidjs/router';
import type { ParentComponent } from 'solid-js';

const Menu: ParentComponent = ({ children }) => {
  return (
    <div class="w-full h-full flex flex-col">
      <div class="dark:bg-neutral-900 bg-indigo-600 px-4 py-2">
        <A href="/">infinite-skin</A>
      </div>
      <main class="flex-1">{children}</main>
    </div>
  );
};

export default Menu;
