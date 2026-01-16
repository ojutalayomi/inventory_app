pub fn send_notification(title: &str, body: &str) {
    if let Err(error) = send_notification_inner(title, body) {
        eprintln!("Notification error: {}", error);
    }
}

#[cfg(target_os = "macos")]
fn send_notification_inner(title: &str, body: &str) -> Result<(), String> {
    let _ = mac_notification_sys::set_application("com.ojutalayomi.inventory-app");
    let mut notification = mac_notification_sys::Notification::new();
    notification.default_sound();
    mac_notification_sys::send_notification(title, None, body, Some(&notification))
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg(target_os = "windows")]
fn send_notification_inner(title: &str, body: &str) -> Result<(), String> {
    use winrt_notification::{Sound, Toast};

    Toast::new("Inventory Manager")
        .title(title)
        .text1(body)
        .sound(Some(Sound::Default))
        .show()
        .map_err(|error| error.to_string())
}

#[cfg(target_os = "linux")]
fn send_notification_inner(title: &str, body: &str) -> Result<(), String> {
    notify_rust::Notification::new()
        .summary(title)
        .body(body)
        .show()
        .map(|_handle| ())
        .map_err(|error| error.to_string())
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn send_notification_inner(_title: &str, _body: &str) -> Result<(), String> {
    Ok(())
}
