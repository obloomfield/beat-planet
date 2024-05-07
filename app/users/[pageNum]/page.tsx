"use server";

import PageSelector from "@/components/client/PageSelector";
import UserCards from "@/components/client/UserCards";
import { getNumUsers, getUsers } from "@/lib/api";
import { ENTRIES_PER_PAGE } from "@/lib/consts";
import { redirect } from "next/navigation";

export default async function Page({
  params,
}: {
  params: { pageNum: string };
}) {
  const page: number = parseInt(params.pageNum);
  const beatmapCount = await getNumUsers();
  const maxPageCount = beatmapCount
    ? Math.ceil(beatmapCount / ENTRIES_PER_PAGE)
    : 0;
  if (isNaN(page) || page < 1 || (maxPageCount && page > maxPageCount)) {
    return redirect("1");
  }

  const users: Profile[] = await getUsers(page - 1);
  // console.log(beatmaps);
  return (
    <div className="">
      <h2 className="font-bold text-4xl mb-4 text-center">All Users</h2>
      {users && <UserCards users={users} />}
      <PageSelector curPage={page} maxPageCnt={maxPageCount} />
    </div>
  );
}
