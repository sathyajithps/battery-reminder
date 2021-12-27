use battery::units::Ratio;
use battery::{Manager, State};
use spin_sleep::sleep;
use std::time::Duration;

mod notification;

use notification::{battery_full, battery_low};
fn main() {
    loop {
        if let Ok(manager) = Manager::new() {
            if let Ok(mut batteries) = manager.batteries() {
                if let Some(Ok(battery)) = batteries.next() {
                    if battery.state_of_charge() <= Ratio::from(0.30)
                        && battery.state() == State::Discharging
                    {
                        battery_low::battery_low_notification();
                    }
                    if battery.state_of_charge() >= Ratio::from(0.90)
                        && battery.state() == State::Charging
                    {
                        battery_full::battery_full_notification();
                    }
                }
            }
        }
        sleep(Duration::from_secs(180));
    }
}
