import { useEffect, useState } from "react";

type CpuInfo = {
  dt: string;
  coreInfos: CoreInfo[];
};

type CoreInfo = {
  name: string;
  brand: string;
  usage: number;
  frequency: number;
};

const CpuInfo = () => {
  const [cpuInfo, setCpuInfo] = useState<CpuInfo>();

  useEffect(() => {
    const ws = new WebSocket("ws://127.0.0.1:9002");

    ws.addEventListener("message", (event) => {
      const data = JSON.parse(event.data) as CpuInfo;
      setCpuInfo(data);
    });

    return () => ws.close();
  }, []);

  return (
    <>
      {!cpuInfo ? (
        "no info"
      ) : (
        <div className="flex flex-row flex-wrap gap-2 p-1">
          {cpuInfo.coreInfos.map((e) => (
            <PercentageBox pct={e.usage} key={e.name} name={e.name} />
          ))}
        </div>
      )}
    </>
  );
};

const PercentageBox = ({ pct, name }: { pct: number; name: string }) => {
  return (
    <div className="bg-slate-700 rounded-md shadow p-1">
      <div className="h-24 w-24 relative">
        <div
          style={{ maxHeight: `${pct}%` }}
          className="absolute bg-slate-300 h-full w-full rounded bottom-0 transition-all ease-linear shadow-sm"
        />
        <div className="absolute flex h-full w-full items-center justify-center font-bold text-white text-xl">
          <div>{name}</div>
        </div>
      </div>
    </div>
  );
};

export default CpuInfo;
