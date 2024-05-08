"use client";

import { useEffect, useRef } from "react";
import Loader from "../ui/loader";
import { useEngine } from "./context/WASM";

export default function WASMTest() {
  const engine = useEngine();

  if (!engine.wasm) {
    return <Loader />;
  }

  const parent = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!engine.wasm) {
      return;
    }

    const el = parent.current;
    engine.wasm.main_web();
  }, [parent]);

  return (
    <div className="bg-slate-500">
      <h1>(client side)</h1>
      <div ref={parent}></div>
      {/* <p>3 + 1 = {engine.wasm.}</p> */}
    </div>
  );
}
