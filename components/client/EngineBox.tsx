"use client";

import { Beatmap } from "@/lib/beatmap";
import { createClient } from "@/utils/supabase/client";
import { useState } from "react";
import toast from "react-hot-toast";
import { FaPlay } from "react-icons/fa";
import { Button } from "../ui/button";
import Loader from "../ui/loader";
import { useEngine } from "./context/WASM";

import { toEventsString } from "@/lib/beatmap";

export default function EngineBox({ beatmap }: { beatmap?: Beatmap }) {
  const [engineActive, setEngineActive] = useState(false);

  const engine = useEngine();

  if (!engine.wasm) {
    return <Loader />;
  }

  const runBevyApp = async () => {
    if (!beatmap) {
      toast.error("No beatmap provided");
      return;
    }

    const supabase = createClient();

    const { data } = await supabase.storage
      .from("map_songs")
      .getPublicUrl(beatmap.song_ref);

    setEngineActive(true);
    try {
      engine.wasm?.run_onion_engine(
        beatmap.title,
        "0,0,0",
        data.publicUrl,
        document.cookie
      );
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div>
      {!engineActive && (
        <Button
          className="text-xl drop-shadow-[0_5px_10px_rgba(255,255,255,0.5)]"
          onClick={runBevyApp}
        >
          <FaPlay className="pr-1" /> Play Beatmap
        </Button>
      )}
    </div>
  );
}
