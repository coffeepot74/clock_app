// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{Manager, Position, PhysicalPosition};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let monitor = window.current_monitor()?.unwrap();
            let monitor_size = monitor.size();

            // ★ monitor_size.width を i32にキャストして使う
            let monitor_width = monitor_size.width as i32;
            let _monitor_height = monitor_size.height as i32;

            // ★ windowサイズ、marginもi32で統一
            let window_width: i32 = 200;
            let _window_height: i32 = 90;
            let margin_from_top: i32 = 50;

            // ★ すべてi32同士で計算
            let new_x = monitor_width - (window_width * 2);
            let new_y = margin_from_top;

            window.set_position(Position::Physical(PhysicalPosition {
                x: new_x,
                y: new_y,
            }))?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
