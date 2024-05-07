import { cn } from "@/lib/utils";
import "@/styles/globals.css";
import { GeistSans } from "geist/font/sans";
import { Inter as FontSans } from "next/font/google";
import { Toaster } from "react-hot-toast";

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
          {children}
        </main>
      </body>
    </html>
  );
}
