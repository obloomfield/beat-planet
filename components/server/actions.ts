"use server";

import { createClient } from "@/utils/supabase/server";

export async function createBeatmapFormAction(data: FormData) {
  const song = data.get("song_file") as File;
  const title = data.get("title") as string;

  const supabase = createClient();

  const {
    data: { user },
  } = await supabase.auth.getUser();

  const { data: upload_data, error } = await supabase.storage
    .from("map_songs")
    .upload(`${user?.id}/${title}`, song, {
      upsert: true,
    });

  console.log(upload_data);

  if (error) {
    throw error;
  }

  return true;
}
