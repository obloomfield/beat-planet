"use client";

import { cn } from "@/lib/utils";
import Link from "next/link";

export default function UserCards({ users }: { users: Profile[] }) {
  return (
    <div className="animate-in flex-1 flex flex-col gap-20 opacity-0 max-w-4xl px-3">
      <main className="flex-1 flex flex-col">
        {users.map((user) => (
          <Link
            href={`/maps/user/${user.id}/1`}
            key={user.id}
            className={cn(
              "flex flex-row gap-4 rounded-xl inner-border inner-border-green-100 px-4 py-2 my-4",
              "hover:drop-shadow-[0_5px_10px_rgba(255,255,255,0.5)] hover:inner-border-2 hover:inner-border-[#f9fcdc]"
            )}
          >
            <div className="flex flex-col gap-2">
              <h3 className="font-bold text-xl">{user.email}</h3>
            </div>
          </Link>
        ))}
      </main>
    </div>
  );
}
