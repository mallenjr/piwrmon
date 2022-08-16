use std::process::Command;
use dotenv;

pub struct IpmiManager {
    base: String,
}

impl IpmiManager {
    pub fn new() -> IpmiManager {
        let user = dotenv::var("USERNAME").unwrap();
        let pass = dotenv::var("PASSWORD").unwrap();
        let host = dotenv::var("HOST").unwrap();

        IpmiManager {
            base: format!("-I lanplus -H {} -U {} -P {}", host, user, pass)
        }
    }

    pub fn system_powered(&self) -> bool {
        let output = Command::new("ipmitool")
            .current_dir("/usr/bin")
            .args(format!("{} chassis power status", self.base).split(" "))
            .output()
            .expect("failed to call ipmitool");

        let data = String::from_utf8(output.stdout)
            .expect("failed to parse ipmitool output");

        match &data[..] {
            "Chassis Power is on\n" => true,
            _ => false
        }
    }

    pub fn shutdown(&self) {
        if !self.system_powered() {
            return;
        }

        println!("SHUTDOWN!!");
        let _output = Command::new("ipmitool")
            .current_dir("/usr/bin")
            .args(format!("{} chassis power soft", self.base).split(" "))
            .status()
            .expect("failed to call ipmitool");
    }

    pub fn boot(&self) {
        if self.system_powered() {
            return;
        }
        
        println!("BOOT!!");
        let _output = Command::new("ipmitool")
            .current_dir("/usr/bin")
            .args(format!("{} chassis power on", self.base).split(" "))
            .status()
            .expect("failed to call ipmitool");
    }
}