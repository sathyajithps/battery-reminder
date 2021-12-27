use super::Notification;

pub fn battery_low_notification() {
    Notification::new()
        .summary("Battery Low")
        .body("Connect the charger")
        .icon(format!("{}/assets/battery_low.png", env!("CARGO_MANIFEST_DIR")).as_str())
        .show()
        .unwrap();
}

