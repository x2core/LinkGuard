mod plugins;
pub mod sniffer;

use tauri::Manager;
use sniffer::SnifferState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn update_tray_menu(app: tauri::AppHandle, show_text: String, quit_text: String) -> Result<(), String> {
    plugins::system_tray::update_tray_menu(&app, &show_text, &quit_text)
}

#[tauri::command]
fn get_interfaces() -> Vec<String> {
    sniffer::get_mock_interfaces()
}

#[tauri::command]
async fn start_sniffing(app: tauri::AppHandle, state: tauri::State<'_, SnifferState>, interface: String) -> Result<(), String> {
    sniffer::start_mock_sniffing(app, state, interface).await
}

#[tauri::command]
async fn stop_sniffing(state: tauri::State<'_, SnifferState>) -> Result<(), String> {
    sniffer::stop_mock_sniffing(state).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .manage(SnifferState::default())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // When attempting to start a second instance, focus the existing main window
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
                let _ = window.unminimize();
                let _ = window.show();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(plugins::system_tray::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            update_tray_menu,
            get_interfaces,
            start_sniffing,
            stop_sniffing
        ]);

    // Only enable updater in release mode
    #[cfg(not(debug_assertions))]
    let builder = builder.plugin(tauri_plugin_updater::Builder::new().build());

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
