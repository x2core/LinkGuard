import { useSniffer } from "@/hooks/useSniffer";
import { BandwidthChart } from "@/components/dashboard/BandwidthChart";
import { PacketLog } from "@/components/dashboard/PacketLog";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Play, Square, Trash2, Activity } from "lucide-react";

export default function Home() {
  const {
    interfaces,
    selectedInterface,
    setSelectedInterface,
    isSniffing,
    toggleSniffing,
    packets,
    metricsHistory,
    clearPackets,
  } = useSniffer();

  return (
    <div className="bg-background text-foreground flex h-screen flex-1 flex-col gap-6 overflow-hidden p-6">
      <header className="flex shrink-0 items-center justify-between">
        <div className="flex items-center gap-2">
          <div className="bg-primary/20 rounded-lg p-2">
            <Activity className="text-primary h-6 w-6" />
          </div>
          <div>
            <h1 className="text-2xl font-bold tracking-tight">Network Sniffer</h1>
            <p className="text-muted-foreground text-sm">
              Real-time packet inspection and bandwidth monitoring
            </p>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <Select
            value={selectedInterface || undefined}
            onValueChange={setSelectedInterface}
            disabled={isSniffing || interfaces.length === 0}
          >
            <SelectTrigger className="w-[200px]">
              <SelectValue placeholder="Select Interface" />
            </SelectTrigger>
            <SelectContent>
              {interfaces.map((iface) => (
                <SelectItem key={iface} value={iface}>
                  {iface}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          <Button
            onClick={toggleSniffing}
            variant={isSniffing ? "destructive" : "default"}
            className="w-32"
            disabled={!selectedInterface}
          >
            {isSniffing ? (
              <>
                <Square className="mr-2 h-4 w-4" />
                Stop
              </>
            ) : (
              <>
                <Play className="mr-2 h-4 w-4" />
                Start
              </>
            )}
          </Button>

          <Button variant="outline" size="icon" onClick={clearPackets} title="Clear Log">
            <Trash2 className="h-4 w-4" />
          </Button>
        </div>
      </header>

      <main className="flex flex-1 flex-col gap-6 overflow-hidden">
        <div className="shrink-0">
          <BandwidthChart data={metricsHistory} />
        </div>
        <div className="min-h-0 flex-1">
          <PacketLog packets={packets} />
        </div>
      </main>

      {/* Global styles for custom scrollbar within this scope */}
      <style>{`
        .custom-scrollbar::-webkit-scrollbar {
          width: 8px;
          height: 8px;
        }
        .custom-scrollbar::-webkit-scrollbar-track {
          background: transparent;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb {
          background: hsl(var(--muted-foreground) / 0.3);
          border-radius: 4px;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover {
          background: hsl(var(--muted-foreground) / 0.5);
        }
      `}</style>
    </div>
  );
}
