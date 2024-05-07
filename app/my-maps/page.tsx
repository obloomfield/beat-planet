import { createClient } from "@/utils/supabase/server";
import { redirect } from "next/navigation";

export default async function Page() {
  const supabase = createClient();
  const { data: user } = await supabase.auth.getUser();

  if (!user || !user.user) {
    return redirect("/");
  }

  return redirect(`/maps/user/${user.user.id}/1`);
}
