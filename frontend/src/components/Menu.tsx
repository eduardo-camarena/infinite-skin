import { A } from '@solidjs/router';
import type { ParentComponent } from 'solid-js';

const Menu: ParentComponent = ({ children }) => {
  return (
    <div class="w-full h-full">
      <div class="dark:bg-neutral-900 bg-indigo-600 px-8 py-2">
        <A href="/">infinite-skin</A>
      </div>
      <main class="text-center w-full h-[calc(100%-40px)]">{children}</main>
    </div>
  );
};

export default Menu;
