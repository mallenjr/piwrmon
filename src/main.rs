use std::time::{ SystemTime, UNIX_EPOCH, Duration };
use std::thread;

mod helpers;
mod ipmi;
mod ups;
use ipmi::IpmiManager;
use ups::{ UpsStatus };

enum PoweredState {
    On,
    Off,
    ShuttingDown,
    Booting,
}

fn main() {
    let mut online_time = Duration::new(0, 0);
    let mut on_batt_time = Duration::new(0, 0);
    let manager = IpmiManager::new();
    let mut state = if manager.system_powered() == true { PoweredState::On } else { PoweredState::Off };

    loop {
        let time = SystemTime::now();
        let timestamp = time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let ups_status = UpsStatus::get();
        let powered_on = manager.system_powered();
        
        match &ups_status.status[..] {
            "ONLINE" => online_time = timestamp,
            "ONBATT" => on_batt_time = timestamp,
            _ => {}
        }

        match state {
            PoweredState::Booting => {
                if powered_on == false { continue; }
            },
            PoweredState::ShuttingDown => {
                if powered_on == true { continue; }
            },
            _ => {}
        }

        state = if manager.system_powered() == true { PoweredState::On } else { PoweredState::Off };

        if online_time >= on_batt_time + Duration::from_millis(1000 * 60 * 5) && !powered_on && ups_status.charge >= 55.0 {
            manager.boot();
            state = PoweredState::Booting;
        } else if on_batt_time >= online_time + Duration::from_millis(1000 * 60 * 2) && powered_on {
            manager.shutdown();
            state = PoweredState::ShuttingDown;
        }

        thread::sleep(Duration::from_millis(1000));
    }
}
