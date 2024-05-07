"use client";

import { beatmapFormSchema } from "@/lib/validation";
import { zodResolver } from "@hookform/resolvers/zod";
import { useTransition } from "react";
import { useForm } from "react-hook-form";
import { toast } from "react-hot-toast";
import { z } from "zod";
import { createBeatmapFormAction } from "../server/actions";
import { Button } from "../ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "../ui/form";
import { Input } from "../ui/input";
import Loader from "../ui/loader";
import {
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "../ui/sheet";

export default function CreateMapForm({
  setSheetOpen,
}: {
  setSheetOpen: (open: boolean) => void;
}) {
  const [pending, submitBeatmap] = useTransition();

  const submitAction = async (data: FormData) => {
    createBeatmapFormAction(data)
      .then(() => {
        toast.success("Beatmap created successfully!");
      })
      .catch((error) => {
        console.error(error);
        toast.error("Server error. Check console.");
      })
      .finally(() => {
        setSheetOpen(false);
      });
  };

  const form = useForm<z.infer<typeof beatmapFormSchema>>({
    mode: "all",
    resolver: zodResolver(beatmapFormSchema),
    defaultValues: {
      title: "",
      artist: "",
      difficulty: 1,
      bpm: 120,
    },
  });

  const fileRef = form.register("song_file");

  return (
    <SheetContent>
      <Form {...form}>
        <form
          action={(formData) =>
            submitBeatmap(() => {
              submitAction(formData);
            })
          }
          className="space-y-8"
        >
          <SheetHeader>
            <SheetTitle>Create New Map</SheetTitle>
            <SheetDescription></SheetDescription>
          </SheetHeader>
          <FormField
            control={form.control}
            name="title"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Beatmap Title</FormLabel>
                <FormControl>
                  <Input placeholder="Song Title" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="artist"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Artist</FormLabel>
                <FormControl>
                  <Input placeholder="Artist" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="difficulty"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Difficulty</FormLabel>
                <FormControl>
                  <Input
                    placeholder="Difficulty"
                    type="number"
                    min={1}
                    max={5}
                    {...field}
                    onChange={(event) => field.onChange(+event.target.value)}
                  />
                </FormControl>
                <FormDescription>
                  Please choose a difficulty rating from 1-5
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="bpm"
            render={({ field }) => (
              <FormItem>
                <FormLabel>BPM</FormLabel>
                <FormControl>
                  <Input
                    placeholder="BPM"
                    type="number"
                    min={1}
                    max={999}
                    {...field}
                    onChange={(event) => field.onChange(+event.target.value)}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="song_file"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Song File</FormLabel>
                <FormControl>
                  <Input type="file" {...fileRef} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          {pending ? (
            <Loader />
          ) : (
            <Button
              type="submit"
              disabled={!form.formState.isDirty || !form.formState.isValid}
            >
              Submit
            </Button>
          )}
        </form>
      </Form>
    </SheetContent>
  );
}
