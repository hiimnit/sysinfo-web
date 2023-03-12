import { useState } from "react";

import CpuInfo from "./CpuInfo";

function App() {
  const [count, setCount] = useState(0);

  return (
    <div>
      <h1 className="text-6xl font-bold">Hello</h1>

      <CpuInfo />
    </div>
  );
}

export default App;
