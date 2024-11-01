import {
  HiSolidChevronDoubleLeft,
  HiSolidChevronDoubleRight,
  HiSolidChevronLeft,
  HiSolidChevronRight,
} from 'solid-icons/hi';
import { Accessor, Component, JSXElement } from 'solid-js';

type PageNumberProps = {
  page: number;
  onClick: () => void;
};

const PageNumber: Component<PageNumberProps> = ({ page, onClick }) => {
  return (
    <div class="my-auto">
      <button class="text-xl font-semibold" onClick={onClick}>
        {page}
      </button>
    </div>
  );
};

type PaginationProps = {
  lastPage: number;
  currentPage: Accessor<number>;
  setNewPage: (newPage: number) => void;
  getNewPage: (page: number) => Promise<void>;
};

const Pagination: Component<PaginationProps> = ({
  lastPage,
  currentPage,
  setNewPage,
  getNewPage,
}) => {
  const pageNumbers = ((): JSXElement => {
    if (lastPage < 7) {
      return Array.from(Array(lastPage), (_, page) => (
        <PageNumber
          page={page + 1}
          onClick={() => {
            getNewPage(page);
            setNewPage(page);
          }}
        />
      ));
    }

    return (
      <div class="px-2 flex flex-row gap-3">
        {currentPage() <= 3 === false && (
          <>
            <PageNumber
              page={1}
              onClick={() => {
                getNewPage(1);
                setNewPage(1);
              }}
            />
            {currentPage() <= 4 === false && (
              <div class="my-auto">
                <button class="text-xl font-semibold">...</button>
              </div>
            )}
          </>
        )}
        {(currentPage() <= 2
          ? [1, 2, 3, 4, 5]
          : currentPage() <= lastPage - 3
            ? [
                currentPage() - 2,
                currentPage() - 1,
                currentPage(),
                currentPage() + 1,
                currentPage() + 2,
              ]
            : [16, 17, 18, 19, 20]
        ).map((page) => (
          <PageNumber
            page={page}
            onClick={() => {
              getNewPage(page);
              setNewPage(page);
            }}
          />
        ))}
        {currentPage() > lastPage - 3 === false && (
          <>
            {currentPage() > lastPage - 4 === false && (
              <div class="my-auto">
                <button class="text-xl font-semibold">...</button>
              </div>
            )}
            <PageNumber
              page={lastPage}
              onClick={() => {
                getNewPage(lastPage);
                setNewPage(lastPage);
              }}
            />
          </>
        )}
      </div>
    );
  })();

  return (
    <div class="pt-6 flex justify-center gap-2">
      <button
        onClick={() => {
          getNewPage(0);
          setNewPage(1);
        }}
      >
        <HiSolidChevronDoubleLeft size="22" />
      </button>
      <button
        onClick={() => {
          const newPage = currentPage() - 1;
          if (newPage > 0) {
            getNewPage(newPage);
            setNewPage(newPage);
          }
        }}
      >
        <HiSolidChevronLeft size="22" />
      </button>
      {pageNumbers}
      <button
        onClick={() => {
          const newPage = currentPage() + 1;
          if (newPage <= lastPage) {
            getNewPage(newPage);
            setNewPage(newPage);
          }
        }}
      >
        <HiSolidChevronRight size="22" />
      </button>
      <button
        onClick={() => {
          getNewPage(lastPage - 1);
          setNewPage(lastPage);
        }}
      >
        <HiSolidChevronDoubleRight size="22" />
      </button>
    </div>
  );
};

export default Pagination;
