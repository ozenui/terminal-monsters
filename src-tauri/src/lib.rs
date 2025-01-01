pub mod shared;
pub mod utils;

use crate::utils::commands::{get_dex, get_party};

use tauri::{window::Color, Manager};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();

            main_window
                .set_background_color(Some(Color(0, 0, 0, 0)))
                .expect("Unable to set background color to window.");

            #[cfg(target_os = "macos")]
            apply_vibrancy(
                &main_window,
                NSVisualEffectMaterial::HudWindow,
                Some(NSVisualEffectState::Active),
                None,
            )
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_dex, get_party])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
}
