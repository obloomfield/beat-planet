import { z } from "zod";

export const beatmapFormSchema = z.object({
  title: z.string().min(1).max(100),
  artist: z.string(),
  difficulty: z.number().min(1).max(10),
  bpm: z.number().min(1).max(999),
  song_file:
    typeof window === "undefined"
      ? z.any()
      : z
          .instanceof(FileList)
          .refine(
            (files) => files?.length === 1,
            "Please submit one audio file."
          )
          .refine(
            (files) => files?.[0]?.type === "audio/mpeg",
            "Invalid file type"
          ),
});
