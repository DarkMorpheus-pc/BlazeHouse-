fn main() {
    let hardware = blazehouse_lib::commands::get_system_hardware_blocking().unwrap();
    let telemetry = blazehouse_lib::commands::get_live_telemetry_blocking().unwrap();
    let zram = blazehouse_lib::commands::get_zram_status_blocking().unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "hardware": hardware, "telemetry": telemetry, "zram": zram,
            "controls": blazehouse_lib::commands::read_controls()
        }))
        .unwrap()
    );
}
