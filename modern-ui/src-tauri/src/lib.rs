pub mod commands;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_system_hardware,
            refresh_system_hardware,
            get_live_telemetry,
            get_zram_status,
            apply_zram_config,
            check_system_updates,
            get_update_channel,
            clean_storage,
            get_storage_details,
            get_system_controls,
            set_system_control,
            run_privileged_command,
            prepare_offline_update,
            prepare_system_upgrade,
            get_offline_update_status,
            reboot_to_offline_update,
            reboot_to_system_upgrade,
            cancel_offline_update
        ])
        .setup(|_app| {
            log::info!("BlazeHouse Linux Kontrol Merkezi başlatıldı.");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
