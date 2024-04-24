"use server";

import Counter from "@/components/Counter";
import { createClient } from "@/utils/supabase/server";
export default async function Page() {
  const supabase = createClient();
  const { data: notes } = await supabase.from("notes").select();
  return (
    <div>
      <h1>Notes</h1>
      <pre>{JSON.stringify(notes, null, 2)}</pre>
      <Counter />
    </div>
  );
}
