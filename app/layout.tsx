import MainMenu from "@/components/client/MainMenu";
import AuthButton from "@/components/server/AuthButton";
import { cn } from "@/lib/utils";
import "@/styles/globals.css";
import { GeistSans } from "geist/font/sans";
import { Inter as FontSans } from "next/font/google";
import { Toaster } from "react-hot-toast";
import { FaGlobe } from "react-icons/fa";

// const defaultUrl = process.env.VERCEL_URL
//   ? `https://${process.env.VERCEL_URL}`
//   : "http://localhost:3000";

const fontSans = FontSans({
  subsets: ["latin"],
  variable: "--font-sans",
});

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={GeistSans.className}>
      <body
        className={cn(
          "min-h-screen bg-background font-sans antialiased",
          fontSans.variable
        )}
      >
        <Toaster position="top-center" />
        <main className="min-h-screen flex flex-col items-center text-foreground">
          <section className="flex-1 w-full flex flex-col gap-10 items-center">
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

                  <MainMenu />
                </div>
                <AuthButton />
              </div>
            </nav>

            <div className="animate-in flex-1 flex flex-col gap-20 opacity-0 max-w-4xl px-3">
              <main className="flex-1 flex flex-col gap-6">{children}</main>
            </div>
          </section>
        </main>
      </body>
    </html>
  );
}
