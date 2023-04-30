import { HiOutlineBeaker } from 'solid-icons/hi';
import { Component } from 'solid-js';

import Button from '../../InputComponents/Button';

const WelcomeScreen: Component = () => {
  return (
    <div class="flex flex-col content-center w-full pt-40">
      <HiOutlineBeaker class="w-10 h-10 !text-red-600 m-auto" />
      <div>
        <Button
          text="Scan"
          variant="blue"
          rounded="full"
          onClick={async () => {
            console.log('hello');
            (
              await fetch('http://localhost:8001/albums/scan', {
                method: 'POST',
              })
            ).status;
          }}
        />
      </div>
    </div>
  );
};

export default WelcomeScreen;
