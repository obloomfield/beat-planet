import { EngineTest } from "@/components/client/EngineTest";
import WASMTest from "@/components/client/WASMTest";

export default function Page() {
  return (
    <div>
      <h1>Test WASM</h1>
      <WASMTest />
      {/* <EngineTest /> */}
    </div>
  );
}
