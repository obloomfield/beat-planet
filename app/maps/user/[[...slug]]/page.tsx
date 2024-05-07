"use server";

import BeatmapCards from "@/components/client/BeatmapCards";
import PageSelector from "@/components/client/PageSelector";
import { getNumUserBeatmaps, getUserBeatmaps } from "@/lib/api";
import { Beatmap } from "@/lib/beatmap";
import { ENTRIES_PER_PAGE } from "@/lib/consts";
import { createClient } from "@/utils/supabase/server";
import { redirect } from "next/navigation";
import toast from "react-hot-toast";

export default async function Page({ params }: { params: { slug: string[] } }) {
  if (
    !params ||
    !params.slug ||
    params.slug.length < 1 ||
    params.slug.length > 2
  ) {
    return redirect("/");
  }

  const user_id = params.slug[0];

  if (params.slug.length === 1) {
    return redirect(`/maps/user/${user_id}/1`);
  }

  const supabase = createClient();
  const { data: user } = await supabase.auth.getUser();

  const { data: query_user, error: query_user_error } =
    await supabase.auth.admin.getUserById(user_id);

  if (query_user_error || !query_user || !query_user.user) {
    console.log(query_user_error);
    console.log(query_user);
    console.log(query_user?.user);

    return redirect("/");
  }

  const own_user = user?.user?.id === user_id;

  const page: number = parseInt(params.slug[1]);
  const beatmapCount = await getNumUserBeatmaps(user_id);
  const maxPageCount = beatmapCount
    ? Math.ceil(beatmapCount / ENTRIES_PER_PAGE)
    : 0;
  if (isNaN(page) || page < 1 || (maxPageCount && page > maxPageCount)) {
    return redirect("1");
  }

  const beatmaps: Beatmap[] = await getUserBeatmaps(user_id, page - 1);
  // console.log(beatmaps);

  return (
    <div className="">
      <h2 className="font-bold text-2xl mb-4 text-center">
        {own_user ? "Your Beatmaps" : `${query_user.user.email}'s Beatmaps`}
      </h2>
      {beatmaps && <BeatmapCards beatmaps={beatmaps} />}
      <PageSelector curPage={page} maxPageCnt={maxPageCount} />
    </div>
  );
}
