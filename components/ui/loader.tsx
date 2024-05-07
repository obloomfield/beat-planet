import { cn } from "@/lib/utils";
import { BeatLoader } from "react-spinners";

export default function Loader({ className }: { className?: string }) {
  return (
    <BeatLoader
      className={cn("justify-center content-center p-4", className)}
      color={"#fff"}
      size={12}
    />
  );
}
