use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketInfo {
    pub id: usize, // Needed for virtualized list key
    pub timestamp: u64,
    pub src_ip: String,
    pub dest_ip: String,
    pub protocol: String,
    pub src_port: u16,
    pub dest_port: u16,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthMetrics {
    pub timestamp: u64,
    pub upload_bytes_per_sec: usize,
    pub download_bytes_per_sec: usize,
}
