import Playback from "@/components/client/Playback";
import { getMapByID } from "@/lib/api";
import { redirect } from "next/navigation";

export default async function Page({ params }: { params: { mapID: string } }) {
  const map = await getMapByID(Number(params.mapID));

  if (!map) {
    return redirect("/");
  }

  return (
    <div>
      <section className="info flex flex-col space-y-1 ">
        <h1 className="text-4xl font-bold pb-2">Map Info</h1>
        <p>
          <b>title:</b> <span className="font-mono">{map.title}</span>
        </p>
        {map.artist && (
          <p>
            <b>artist:</b> <span className="font-mono">{map.artist}</span>
          </p>
        )}
        <p>
          <b>author:</b>{" "}
          <span className="font-mono">{map.profiles?.email}</span>
        </p>
        <p>
          <b>difficulty:</b>{" "}
          <span className="font-mono">{map.difficulty}/5</span>
        </p>
        <p>
          <b>BPM:</b> <span className="font-mono">{map.bpm}</span>
        </p>
      </section>
      <Playback songPath={`${map.profiles.id}/${map.title}`} />
    </div>
  );
}
