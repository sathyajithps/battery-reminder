use battery::units::Ratio;
use battery::{Manager, State};
use spin_sleep::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    loop {
        if let Ok(manager) = Manager::new() {
            if let Ok(mut batteries) = manager.batteries() {
                if let Some(Ok(battery)) = batteries.next() {
                    if battery.state_of_charge() <= Ratio::from(0.30) {
                        if battery.state() == State::Discharging {
                            println!("Charge dummy");
                        }
                        if battery.state() == State::Charging {
                            println!("We good");
                        }
                    }
                    if battery.state_of_charge() > Ratio::from(0.30)
                        && battery.state_of_charge() < Ratio::from(0.90)
                    {
                        println!("Wee boomin");
                    }
                    if battery.state_of_charge() >= Ratio::from(0.90) {
                        if battery.state() == State::Discharging {
                            println!("We good");
                        }
                        if battery.state() == State::Charging {
                            println!("Disconnect the charger");
                        }
                    }
                }
            }
        }
        println!("{:?}", SystemTime::now());
        sleep(Duration::from_secs(180));
    }
}
