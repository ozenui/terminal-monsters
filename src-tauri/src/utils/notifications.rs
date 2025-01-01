use notify_rust::{set_application, Notification};

pub fn send_system_notification(summary: &str) {
    #[cfg(target_os = "macos")]
    {
        let _ = set_application("com.ozenui.terminal-monsters");
    }

    let mut icon_path = dirs::home_dir().unwrap();
    icon_path.push(".config/terminal-monsters/.assets/logo.png");
    let icon_path_str = icon_path.to_str().unwrap();

    let mut notification = Notification::new();
    notification
        .summary(summary)
        .body("Run `tm` from your terminal to check your party.")
        .icon(icon_path_str)
        .show()
        .unwrap();
}
