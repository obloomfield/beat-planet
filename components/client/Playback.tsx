"use client";

import { createClient } from "@/utils/supabase/client";
import { useEffect, useState } from "react";
import AudioPlayer from "react-h5-audio-player";
import "react-h5-audio-player/lib/styles.css";
import Loader from "../ui/loader";

export default function Playback({ songPath }: { songPath: string }) {
  const [songRef, setSongRef] = useState<string | null>(null);

  useEffect(() => {
    const supabase = createClient();
    supabase.storage
      .from("map_songs")
      .download(songPath)
      .then(({ data, error }) => {
        if (error) {
          console.error(error);
          return;
        }
        setSongRef(URL.createObjectURL(data));
      });
  }, []);

  if (!songRef) {
    return <Loader className="mt-8 text-center" />;
  }

  return (
    <AudioPlayer
      className="py-0 rounded-xl mt-8"
      src={songRef}
      autoPlay={false}
      showJumpControls={false}
      layout="stacked-reverse"
    />
  );
}
