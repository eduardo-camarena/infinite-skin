import { A, useLocation, useNavigate } from '@solidjs/router';
import { HiSolidArrowLeft } from 'solid-icons/hi';
import type { ParentComponent } from 'solid-js';

const Menu: ParentComponent = ({ children }) => {
  const location = useLocation();
  const navigate = useNavigate();
  return (
    <div class="w-full h-full flex flex-col">
      <div class="dark:bg-neutral-900 bg-indigo-600 px-4 py-2">
        <A href="/">infinite-skin</A>
      </div>
      <main class="flex-1">
        {location.pathname !== '/' && (
          <div class="flex pt-6 px-4 lg:hidden">
            <HiSolidArrowLeft
              size={24}
              onClick={() => {
                navigate(-1);
              }}
            />
          </div>
        )}
        {children}
      </main>
    </div>
  );
};

export default Menu;
