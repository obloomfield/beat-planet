"use client";

import { NUM_PAGE_BUTTONS } from "@/lib/consts";
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "./ui/pagination";

export default function PageSelector({
  curPage,
  maxPageCnt,
}: {
  curPage: number;
  maxPageCnt: number;
}) {
  return (
    <Pagination>
      <PaginationContent>
        {curPage > 1 && (
          <>
            <PaginationItem>
              <PaginationPrevious href={`${curPage - 1}`} />
            </PaginationItem>
            <PaginationItem>
              <PaginationEllipsis />
            </PaginationItem>
          </>
        )}
        {Array.from({ length: NUM_PAGE_BUTTONS }, (_, i) => {
          if (curPage + i > maxPageCnt) return null;
          return (
            <PaginationItem key={i}>
              <PaginationLink
                className={i == 0 ? "border border-slate-500" : ""}
                href={`${curPage + i}`}
              >
                {curPage + i}
              </PaginationLink>
            </PaginationItem>
          );
        })}
        {curPage + NUM_PAGE_BUTTONS < maxPageCnt && (
          <PaginationItem>
            <PaginationEllipsis />
          </PaginationItem>
        )}
        {curPage > maxPageCnt && (
          <PaginationItem>
            <PaginationNext href={`${curPage + 1}`} />
          </PaginationItem>
        )}
      </PaginationContent>
    </Pagination>
  );
}
