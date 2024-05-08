"use client";

import { Button } from "../ui/button";
import Loader from "../ui/loader";
import { useEngine } from "./context/WASM";

export default function WASMTest() {
  const engine = useEngine();

  if (!engine.wasm) {
    return <Loader />;
  }

  const runBevyApp = async () => {
    try {
      engine.wasm?.run_onion_engine("hello there");
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div>
      <Button onClick={runBevyApp}>Run Onion Engine</Button>
    </div>
  );
}
