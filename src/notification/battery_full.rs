use super::Notification;

pub fn battery_full_notification() {
    Notification::new()
        .summary("Battery Full")
        .body("Remove the charger")
        .icon(format!("{}/assets/battery_full.png", env!("CARGO_MANIFEST_DIR")).as_str())
        .show()
        .unwrap();
}

