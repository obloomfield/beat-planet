import { useMountEffectOnce } from "@/lib/hooks/useMountEffectOnce";
import type { ReactNode } from "react";
import { createContext, useContext, useState } from "react";

const initial: IWASMContext = {};

export const WASMContext = createContext(initial);

export const WASMContextProvider: React.FC<WASMContextProviderProps> = ({
  children,
}) => {
  const [state, setState] = useState<IWASMContext>(initial);

  useMountEffectOnce(() => {
    (async () => {
      const wasm = await import("onion-engine");
      await wasm.default();
      setState({ wasm });
    })();
  });

  return <WASMContext.Provider value={state}>{children}</WASMContext.Provider>;
};

interface IWASMContext {
  wasm?: typeof import("onion-engine");
}

interface WASMContextProviderProps {
  children: ReactNode;
}

export const useEngine = () => useContext(WASMContext);
