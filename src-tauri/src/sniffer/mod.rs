pub mod dpi;
pub mod models;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

use self::models::{BandwidthMetrics, PacketInfo};

pub struct SnifferState {
    pub is_sniffing: Arc<AtomicBool>,
    pub interface: Mutex<Option<String>>,
}

impl Default for SnifferState {
    fn default() -> Self {
        Self {
            is_sniffing: Arc::new(AtomicBool::new(false)),
            interface: Mutex::new(None),
        }
    }
}

pub fn get_mock_interfaces() -> Vec<String> {
    vec![
        "eth0".to_string(),
        "wlan0".to_string(),
        "lo".to_string(),
        "en0".to_string(),
    ]
}

pub async fn start_mock_sniffing(app: AppHandle, state: tauri::State<'_, SnifferState>, interface: String) -> Result<(), String> {
    let is_running = state.is_sniffing.load(Ordering::SeqCst);
    if is_running {
        return Err("Sniffing is already running".into());
    }

    state.is_sniffing.store(true, Ordering::SeqCst);
    let mut current_interface = state.interface.lock().await;
    *current_interface = Some(interface);

    let is_sniffing = state.is_sniffing.clone();

    tokio::spawn(async move {
        let mut packet_interval = interval(Duration::from_millis(100)); // Batch every 100ms
        let mut metrics_interval = interval(Duration::from_secs(1)); // Metrics every 1s

        let mut packet_id_counter = 0;
        let mut upload_bytes_counter = 0;
        let mut download_bytes_counter = 0;

        let mut packet_batch = Vec::new();

        loop {
            if !is_sniffing.load(Ordering::SeqCst) {
                break;
            }

            tokio::select! {
                _ = packet_interval.tick() => {
                    // Generate a random number of packets per batch (e.g., 50-200)
                    let num_packets = fastrand::usize(50..200);

                    for _ in 0..num_packets {
                        packet_id_counter += 1;
                        let size = fastrand::usize(64..1500); // Typical packet sizes

                        // Randomly assign direction (upload/download for mock)
                        if fastrand::bool() {
                            upload_bytes_counter += size;
                        } else {
                            download_bytes_counter += size;
                        }

                        let timestamp = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64;

                        let protocol = if fastrand::bool() { "TCP".to_string() } else { "UDP".to_string() };

                        packet_batch.push(PacketInfo {
                            id: packet_id_counter,
                            timestamp,
                            src_ip: format!("{}.{}.{}.{}", fastrand::u8(1..=254), fastrand::u8(0..=255), fastrand::u8(0..=255), fastrand::u8(0..=255)),
                            dest_ip: format!("{}.{}.{}.{}", fastrand::u8(1..=254), fastrand::u8(0..=255), fastrand::u8(0..=255), fastrand::u8(0..=255)),
                            protocol,
                            src_port: fastrand::u16(1024..=65535),
                            dest_port: if fastrand::bool() { 80 } else { 443 },
                            size,
                        });
                    }

                    // Emit batch
                    if !packet_batch.is_empty() {
                        let _ = app.emit("packet-batch", &packet_batch);
                        packet_batch.clear();
                    }
                }
                _ = metrics_interval.tick() => {
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;

                    let metrics = BandwidthMetrics {
                        timestamp,
                        upload_bytes_per_sec: upload_bytes_counter,
                        download_bytes_per_sec: download_bytes_counter,
                    };

                    let _ = app.emit("bandwidth-metrics", &metrics);

                    // Reset counters for the next second
                    upload_bytes_counter = 0;
                    download_bytes_counter = 0;
                }
            }
        }
    });

    Ok(())
}

pub async fn stop_mock_sniffing(state: tauri::State<'_, SnifferState>) -> Result<(), String> {
    state.is_sniffing.store(false, Ordering::SeqCst);
    let mut current_interface = state.interface.lock().await;
    *current_interface = None;
    Ok(())
}
