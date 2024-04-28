import { createClient } from "@/utils/supabase/server";
import { Beatmap } from "./beatmap";
import { ENTRIES_PER_PAGE } from "./consts";

export async function getBeatmaps(pageNum?: number) {
  const supabase = createClient();
  let beatmaps;
  if (pageNum === undefined || isNaN(pageNum) || pageNum < 0) {
    beatmaps = await (await supabase.from("beatmaps").select()).data;
  } else {
    beatmaps = await (
      await supabase
        .from("beatmaps")
        .select(
          `id,
          auth.users: (
            email
          )`
        )
        .range(ENTRIES_PER_PAGE * pageNum, ENTRIES_PER_PAGE * (pageNum + 1))
    ).data;
  }
  console.log(beatmaps);
  return beatmaps as Beatmap[];
}

export async function getNumBeatmaps() {
  const supabase = createClient();
  const { count } = await supabase
    .from("beatmaps")
    .select("*", { count: "exact" });
  return count;
}
