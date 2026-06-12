import { useRef } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { PacketInfo } from "@/hooks/useSniffer";

interface PacketLogProps {
  packets: PacketInfo[];
}

export function PacketLog({ packets }: PacketLogProps) {
  const parentRef = useRef<HTMLDivElement>(null);

  const rowVirtualizer = useVirtualizer({
    count: packets.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 35, // Estimated row height
    overscan: 10,
  });

  return (
    <Card className="col-span-4 flex h-full flex-col">
      <CardHeader className="pb-2">
        <CardTitle className="flex items-center justify-between">
          <span>Packet Log</span>
          <span className="text-muted-foreground text-sm font-normal">
            {packets.length} packets captured
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent className="relative flex-1 overflow-hidden p-0">
        <div className="bg-muted/50 text-muted-foreground sticky top-0 z-10 grid w-full grid-cols-6 gap-4 border-b p-4 text-xs font-semibold">
          <div>Time</div>
          <div className="col-span-2">Source</div>
          <div className="col-span-2">Destination</div>
          <div>Proto / Size</div>
        </div>
        <div
          ref={parentRef}
          className="custom-scrollbar h-[calc(100vh-420px)] w-full overflow-auto"
        >
          <div
            style={{
              height: `${rowVirtualizer.getTotalSize()}px`,
              width: "100%",
              position: "relative",
            }}
          >
            {rowVirtualizer.getVirtualItems().map((virtualRow) => {
              const packet = packets[virtualRow.index];
              const time = new Date(packet.timestamp).toISOString().split("T")[1].slice(0, -1);

              return (
                <div
                  key={virtualRow.key}
                  style={{
                    position: "absolute",
                    top: 0,
                    left: 0,
                    width: "100%",
                    height: `${virtualRow.size}px`,
                    transform: `translateY(${virtualRow.start}px)`,
                  }}
                  className="border-border/50 hover:bg-muted/30 grid grid-cols-6 items-center gap-4 border-b px-4 text-sm transition-colors"
                >
                  <div className="text-muted-foreground font-mono text-xs">{time}</div>
                  <div className="col-span-2 truncate font-mono text-xs">
                    {packet.src_ip}:{packet.src_port}
                  </div>
                  <div className="col-span-2 truncate font-mono text-xs">
                    {packet.dest_ip}:{packet.dest_port}
                  </div>
                  <div className="flex items-center gap-2">
                    <span
                      className={`rounded px-2 py-0.5 text-[10px] font-bold ${packet.protocol === "TCP" ? "bg-blue-500/20 text-blue-500" : "bg-orange-500/20 text-orange-500"}`}
                    >
                      {packet.protocol}
                    </span>
                    <span className="text-muted-foreground text-xs">{packet.size} B</span>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
