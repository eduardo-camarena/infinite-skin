import { A } from '@solidjs/router';
import { HiSolidCog } from 'solid-icons/hi';
import type { ParentComponent } from 'solid-js';

const Menu: ParentComponent = ({ children }) => {
  return (
    <div class="w-full h-full flex flex-col">
      <div class="flex dark:bg-neutral-900 bg-indigo-600 px-4 py-2">
        <A href="/" class="flex-1">infinite-skin</A>
        <A href="/settings"><HiSolidCog size={20} /></A>
      </div>
      <main class="flex-1">{children}</main>
    </div>
  );
};

export default Menu;
