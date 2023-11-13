import { useSearchParams } from '@solidjs/router';
import { Component, createResource } from 'solid-js';

import Button from '../InputComponents/Button';

type PaginatorProps = {
  lastPage: number;
  getNewPage: ({}) => Promise<string>;
};

const Paginator: Component<PaginatorProps> = ({ lastPage, getNewPage }) => {
  const [searchParams, setSearchParams] = useSearchParams<{ page: string }>();

  if (!searchParams.page) {
    setSearchParams({ page: 1 });
  }

  createResource(
    () => (searchParams.page ? Number.parseInt(searchParams.page) : 1),
    getNewPage
  );
  return (
    <div class="pt-6 flex justify-center gap-2">
      <Button
        text="Previous"
        variant="blue"
        padding="py-1 px-4"
        onClick={() => {
          const newPage = Number.parseInt(searchParams.page) - 1;
          if (newPage > 0) {
            getNewPage(newPage);
            setSearchParams({ page: newPage });
          }
        }}
      />
      <Button
        text="Next"
        variant="blue"
        padding="py-1 px-4"
        onClick={() => {
          const newPage = Number.parseInt(searchParams.page) + 1;
          if (newPage <= lastPage) {
            getNewPage(newPage);
            setSearchParams({ page: newPage });
          }
        }}
      />
    </div>
  );
};

export default Paginator;
