import AuthButton from "@/components/AuthButton";
import CreateMapForm from "@/components/CreateMapForm";
import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarTrigger,
} from "@/components/ui/menubar";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { FaGlobe, FaMap } from "react-icons/fa";
import { FaFileCirclePlus, FaPeopleGroup } from "react-icons/fa6";

export default function NavbarLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <section className="flex-1 w-full flex flex-col gap-20 items-center">
      <nav className="w-full flex justify-center border-b border-b-foreground/10">
        <div className="w-full max-w-4xl flex justify-between items-center p-3 text-sm">
          <div className="flex flex-row gap-2 items-center group">
            {/* group-hover:text-[#f6f9df] */}
            <div className="flex flex-row gap-2 logo items-center transition duration-300">
              {/* group-hover:drop-shadow-[0_5px_10px_rgba(255,255,0,1)] */}
              <FaGlobe className="inline-block text-center text-xl drop-shadow-[0_5px_10px_rgba(100,255,50,1)]" />
              <span className="font-bold text-xl font-mono select-none">
                beat-world
              </span>
            </div>

            <Sheet>
              <CreateMapForm />
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
          </div>
          <AuthButton />
        </div>
      </nav>

      <div className="animate-in flex-1 flex flex-col gap-20 opacity-0 max-w-4xl px-3">
        <main className="flex-1 flex flex-col gap-6">{children}</main>
      </div>
    </section>
  );
}
