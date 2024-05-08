"use client";

import { WASMContextProvider } from "./context/WASM";

export function Providers({ children }: { children: React.ReactNode }) {
  return <WASMContextProvider>{children}</WASMContextProvider>;
}
