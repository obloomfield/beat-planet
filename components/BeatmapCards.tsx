import { Beatmap } from "@/lib/beatmap";

export default function BeatmapCards({ beatmaps }: { beatmaps: Beatmap[] }) {
  return (
    <div className="animate-in flex-1 flex flex-col gap-20 opacity-0 max-w-4xl px-3">
      <main className="flex-1 flex flex-col gap-6">
        {beatmaps.map((beatmap) => (
          <div
            key={beatmap.id}
            className="flex flex-row gap-4 rounded-xl border border-green-100 px-4 py-2 my-4"
          >
            <div className="flex flex-col gap-2">
              <h3 className="font-bold text-xl">{beatmap.title}</h3>
              <p className="text-sm">{beatmap.artist}</p>
              <p className="text-sm">{beatmap.author}</p>
            </div>
          </div>
        ))}
      </main>
    </div>
  );
}
