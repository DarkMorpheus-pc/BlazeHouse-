// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    {
        // Fix WebKitGTK inverted Y-axis dmabuf rendering on Wayland when launched from terminal
        if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").as_deref() != Ok("1") {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
            if let Ok(exe) = std::env::current_exe() {
                let args: Vec<String> = std::env::args().skip(1).collect();
                let mut cmd = std::process::Command::new(exe);
                cmd.args(args);
                cmd.env("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
                #[cfg(unix)]
                {
                    use std::os::unix::process::CommandExt;
                    let _ = cmd.exec();
                }
            }
        }
    }
    blazehouse_lib::run();
}
