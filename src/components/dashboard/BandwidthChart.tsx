import { useMemo } from "react";
import { Area, AreaChart, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { BandwidthMetrics } from "@/hooks/useSniffer";

interface BandwidthChartProps {
  data: BandwidthMetrics[];
}

const formatBytes = (bytes: number) => {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
};

export function BandwidthChart({ data }: BandwidthChartProps) {
  const chartData = useMemo(() => {
    return data.map((d) => ({
      time: new Date(d.timestamp).toLocaleTimeString([], { hour12: false }),
      upload: d.upload_bytes_per_sec,
      download: d.download_bytes_per_sec,
    }));
  }, [data]);

  return (
    <Card className="col-span-4">
      <CardHeader>
        <CardTitle>Network Bandwidth</CardTitle>
      </CardHeader>
      <CardContent className="pl-2">
        <div className="h-[200px] w-full">
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={chartData} margin={{ top: 5, right: 20, bottom: 5, left: 0 }}>
              <defs>
                <linearGradient id="colorDownload" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#10b981" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
                </linearGradient>
                <linearGradient id="colorUpload" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="#3b82f6" stopOpacity={0} />
                </linearGradient>
              </defs>
              <XAxis
                dataKey="time"
                stroke="#888888"
                fontSize={12}
                tickLine={false}
                axisLine={false}
              />
              <YAxis
                stroke="#888888"
                fontSize={12}
                tickLine={false}
                axisLine={false}
                tickFormatter={(value) => formatBytes(value)}
                width={80}
              />
              <Tooltip
                contentStyle={{
                  backgroundColor: "#1f2937",
                  border: "none",
                  borderRadius: "8px",
                  color: "#f3f4f6",
                }}
                itemStyle={{ color: "#f3f4f6" }}
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                formatter={(value: any) => [`${formatBytes(Number(value))}/s`, ""]}
              />
              <Area
                type="monotone"
                dataKey="download"
                name="Download"
                stroke="#10b981"
                fillOpacity={1}
                fill="url(#colorDownload)"
                isAnimationActive={false}
              />
              <Area
                type="monotone"
                dataKey="upload"
                name="Upload"
                stroke="#3b82f6"
                fillOpacity={1}
                fill="url(#colorUpload)"
                isAnimationActive={false}
              />
            </AreaChart>
          </ResponsiveContainer>
        </div>
      </CardContent>
    </Card>
  );
}
