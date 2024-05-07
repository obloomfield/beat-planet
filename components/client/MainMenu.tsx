"use client";

import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarTrigger,
} from "@/components/ui/menubar";
import { Sheet, SheetTrigger } from "@/components/ui/sheet";
import { useState } from "react";
import { FaMap } from "react-icons/fa";
import { FaFileCirclePlus, FaPeopleGroup } from "react-icons/fa6";
import CreateMapForm from "./CreateMapForm";

export default function MainMenu() {
  const [createBeatmapOpen, setCreateBeatmapOpen] = useState(false);

  return (
    <Sheet open={createBeatmapOpen} onOpenChange={setCreateBeatmapOpen}>
      <CreateMapForm setSheetOpen={setCreateBeatmapOpen} />
      <Menubar>
        <MenubarMenu>
          <MenubarTrigger>beatmaps</MenubarTrigger>
          <MenubarContent className="text-white">
            <MenubarItem>
              All Maps
              <MenubarShortcut>
                <FaMap className="inline-block text-center" />
              </MenubarShortcut>
            </MenubarItem>
            <MenubarSeparator />
            <MenubarItem>
              <SheetTrigger>Create New Map</SheetTrigger>
              <MenubarShortcut>
                <FaFileCirclePlus className="inline-block text-center" />
              </MenubarShortcut>
            </MenubarItem>
            <MenubarItem>
              My Maps
              {/* <MenubarShortcut>
                      <FaUser className="inline-block text-center" />
                    </MenubarShortcut> */}
            </MenubarItem>
          </MenubarContent>
        </MenubarMenu>
        <MenubarMenu>
          <MenubarTrigger>users</MenubarTrigger>
          <MenubarContent className="text-white">
            <MenubarItem>
              All Users
              <MenubarShortcut>
                <FaPeopleGroup className="inline-block text-center" />
              </MenubarShortcut>
            </MenubarItem>
          </MenubarContent>
        </MenubarMenu>
      </Menubar>
    </Sheet>
  );
}
