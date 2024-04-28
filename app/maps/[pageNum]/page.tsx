"use server";

import BeatmapCards from "@/components/BeatmapCards";
import PageSelector from "@/components/PageSelector";
import { getBeatmaps, getNumBeatmaps } from "@/lib/api";
import { Beatmap } from "@/lib/beatmap";
import { ENTRIES_PER_PAGE } from "@/lib/consts";
import { redirect } from "next/navigation";

export default async function Page({
  params,
}: {
  params: { pageNum: string };
}) {
  const page: number = parseInt(params.pageNum);
  const beatmapCount = await getNumBeatmaps();
  const maxPageCount = beatmapCount
    ? Math.ceil(beatmapCount / ENTRIES_PER_PAGE)
    : 0;
  if (isNaN(page) || page < 1 || (maxPageCount && page > maxPageCount)) {
    return redirect("1");
  }

  const beatmaps: Beatmap[] = await getBeatmaps(page - 1);
  return (
    <div className="">
      <h2 className="font-bold text-4xl mb-4 text-center">All Beatmaps</h2>
      {beatmaps && <BeatmapCards beatmaps={beatmaps} />}
      <PageSelector curPage={page} maxPageCnt={maxPageCount} />
    </div>
  );
}
