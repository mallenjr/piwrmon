use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use regex::Regex;
use std::thread;
use dotenv;

struct UpsStatus {
    status: String
}

fn get_ups_status() -> UpsStatus {
    let output = Command::new("apcaccess")
        .output()
        .expect("failed to call apcaccess");

    let data = String::from_utf8(output.stdout)
        .expect("failed to parse apcaccess output");

    let re = Regex::new(r"STATUS\s+:\s([a-zA-Z]+)").expect("failed to parse apcaccess output");
    let cap = re.captures(&data).expect("failed to parse apcaccess output");

    UpsStatus {
        status: String::from(&cap[1])
    }
}

fn ipmi_base() -> String {
    let user = dotenv::var("USERNAME").unwrap();
    let pass = dotenv::var("PASSWORD").unwrap();
    let host = dotenv::var("HOST").unwrap();

    format!("-I lanplus -H {} -U {} -P {}", host, user, pass)
}

fn system_powered() -> bool {
    let base = ipmi_base();
    let output = Command::new("ipmitool")
        .current_dir("/usr/bin")
        .args(format!("{} chassis power status", base).split(" "))
        .output()
        .expect("failed to call ipmitool");

    let data = String::from_utf8(output.stdout)
        .expect("failed to parse ipmitool output");

    match &data[..] {
        "Chassis Power is on\n" => true,
        _ => false
    }
}

fn shutdown() {
    let base = ipmi_base();
    if !system_powered() {
        return;
    }
    println!("SHUTDOWN!!");
    let _output = Command::new("ipmitool")
        .current_dir("/usr/bin")
        .args(format!("{} chassis power soft", base).split(" "))
        .status()
        .expect("failed to call ipmitool");
}

fn boot() {
    let base = ipmi_base();
    if system_powered() {
        return;
    }
    println!("BOOT!!");
    let _output = Command::new("ipmitool")
        .current_dir("/usr/bin")
        .args(format!("{} chassis power on", base).split(" "))
        .status()
        .expect("failed to call ipmitool");
}

fn main() {
    let sleep_time = Duration::from_millis(1000);
    let mut last_online = Duration::new(0, 0);
    let mut on_batt = Duration::new(0, 0);

    loop {
        let start = SystemTime::now();
        let current_time = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        
        let ups_status = get_ups_status();
        
        match &ups_status.status[..] {
            "ONLINE" => {
                last_online = current_time
            },
            "ONBATT" => {
                on_batt = current_time
            },
            _ => {}
        }

        let powered_on = system_powered();

        if last_online >= on_batt + Duration::from_millis(1000 * 60 * 5) && !powered_on {
            boot();
            thread::sleep(sleep_time * 60 * 20);
        }

        else if on_batt >= last_online + Duration::from_millis(1000 * 5) && powered_on {
            shutdown();
            thread::sleep(sleep_time * 60 * 10);
        }

        thread::sleep(sleep_time);
    }
}
