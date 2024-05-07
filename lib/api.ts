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
        .select(`*, profiles(email)`)
        .range(ENTRIES_PER_PAGE * pageNum, ENTRIES_PER_PAGE * (pageNum + 1) - 1)
    ).data;
  }
  return beatmaps as Beatmap[];
}

export async function getNumBeatmaps() {
  const supabase = createClient();
  const { count } = await supabase
    .from("beatmaps")
    .select("*", { count: "exact" });
  return count;
}

export async function getUserBeatmaps(user_id: string, pageNum?: number) {
  const supabase = createClient();

  let beatmaps;
  if (pageNum === undefined || isNaN(pageNum) || pageNum < 0) {
    beatmaps = await (
      await supabase.from("beatmaps").select().eq("author_id", user_id)
    ).data;
  } else {
    beatmaps = await (
      await supabase
        .from("beatmaps")
        .select(`*, profiles(email)`)
        .eq("author_id", user_id)
        .range(ENTRIES_PER_PAGE * pageNum, ENTRIES_PER_PAGE * (pageNum + 1) - 1)
    ).data;
  }
  return beatmaps as Beatmap[];
}

export async function getNumUserBeatmaps(user_id: string) {
  const supabase = createClient();
  const { count } = await supabase
    .from("beatmaps")
    .select("*", { count: "exact" })
    .eq("author_id", user_id);
  return count;
}

export async function getUsers(pageNum?: number) {
  const supabase = createClient();

  let profiles;
  if (pageNum === undefined || isNaN(pageNum) || pageNum < 0) {
    profiles = await (await supabase.from("profiles").select()).data;
  } else {
    profiles = await (
      await supabase
        .from("profiles")
        .select()
        .range(ENTRIES_PER_PAGE * pageNum, ENTRIES_PER_PAGE * (pageNum + 1) - 1)
    ).data;
  }
  return profiles as Profile[];
}

export async function getNumUsers() {
  const supabase = createClient();
  const { count } = await supabase
    .from("profiles")
    .select("*", { count: "exact" });
  return count;
}
