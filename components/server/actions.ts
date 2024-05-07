"use server";

import { createClient } from "@/utils/supabase/server";

export async function createBeatmapFormAction(data: FormData) {
  const song = data.get("song_file") as File;
  const title = data.get("title") as string;
  const artist = data.get("artist") as string;
  const difficulty = Number(data.get("difficulty"));
  const bpm = Number(data.get("bpm"));

  const supabase = createClient();

  const {
    data: { user },
  } = await supabase.auth.getUser();

  const { data: upload_data, error: upload_song_error } = await supabase.storage
    .from("map_songs")
    .upload(`${user?.id}/${title}`, song, {
      upsert: true,
    });

  if (upload_song_error) {
    throw upload_song_error;
  }

  const { data: beatmap_data, error: beatmap_error } = await supabase
    .from("beatmaps")
    .insert({
      title,
      artist,
      difficulty,
      bpm,
      offset: 0,
      song_ref: upload_data.path,
      events: [],
      author_id: user?.id,
    });

  console.log(beatmap_data);

  if (beatmap_error) {
    throw beatmap_error;
  }

  return true;
}
