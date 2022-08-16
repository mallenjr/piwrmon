use std::process::Command;

use crate::helpers;

pub struct UpsStatus {
    pub status: String,
    pub charge: f32
}

impl UpsStatus {
    pub fn get() -> UpsStatus {
        let output = Command::new("apcaccess")
            .output()
            .expect("failed to call apcaccess");

        let data = String::from_utf8(output.stdout)
            .expect("failed to parse apcaccess output");

        let status = helpers::get_field_reg(&data, r"STATUS\s+:\s([a-zA-Z]+)");

        let charge = helpers::get_field_reg(&data, r"BCHARGE\s+:\s([0-9.]+)\sPercent")
            .parse::<f32>()
            .expect("failed to parse ups charge");

        UpsStatus {
            status,
            charge,
        }
    }
}