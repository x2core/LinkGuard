import { useState, useEffect, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export interface PacketInfo {
  id: number;
  timestamp: number;
  src_ip: string;
  dest_ip: string;
  protocol: string;
  src_port: number;
  dest_port: number;
  size: number;
}

export interface BandwidthMetrics {
  timestamp: number;
  upload_bytes_per_sec: number;
  download_bytes_per_sec: number;
}

const MAX_PACKETS = 50000;

export function useSniffer() {
  const [interfaces, setInterfaces] = useState<string[]>([]);
  const [selectedInterface, setSelectedInterface] = useState<string | null>(null);
  const [isSniffing, setIsSniffing] = useState(false);
  const [packets, setPackets] = useState<PacketInfo[]>([]);
  const [metricsHistory, setMetricsHistory] = useState<BandwidthMetrics[]>([]);

  useEffect(() => {
    // Load interfaces on mount
    invoke<string[]>("get_interfaces").then((ifaces) => {
      setInterfaces(ifaces);
      if (ifaces.length > 0) {
        setSelectedInterface(ifaces[0]);
      }
    });

    const unlistenPackets = listen<PacketInfo[]>("packet-batch", (event) => {
      setPackets((prev) => {
        const newPackets = [...event.payload, ...prev]; // Prepend new packets
        if (newPackets.length > MAX_PACKETS) {
          return newPackets.slice(0, MAX_PACKETS);
        }
        return newPackets;
      });
    });

    const unlistenMetrics = listen<BandwidthMetrics>("bandwidth-metrics", (event) => {
      setMetricsHistory((prev) => {
        const newHistory = [...prev, event.payload];
        if (newHistory.length > 60) {
          // Keep last 60 seconds
          return newHistory.slice(-60);
        }
        return newHistory;
      });
    });

    return () => {
      unlistenPackets.then((f) => f());
      unlistenMetrics.then((f) => f());
    };
  }, []);

  const toggleSniffing = useCallback(async () => {
    if (!selectedInterface) return;

    if (isSniffing) {
      await invoke("stop_sniffing");
      setIsSniffing(false);
    } else {
      await invoke("start_sniffing", { interface: selectedInterface });
      setIsSniffing(true);
    }
  }, [isSniffing, selectedInterface]);

  const clearPackets = useCallback(() => {
    setPackets([]);
    setMetricsHistory([]);
  }, []);

  return {
    interfaces,
    selectedInterface,
    setSelectedInterface,
    isSniffing,
    toggleSniffing,
    packets,
    metricsHistory,
    clearPackets,
  };
}
