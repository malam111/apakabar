use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::battery::Battery;

pub struct FrontEnd;

impl FrontEnd {

    pub fn main_loop() {
        loop {
            Self::render();
            thread::sleep(Duration::from_secs(1));
        }

    }

    fn render() {
        let paths = [
            "BAT0",
            "BAT1",
        ];
        let mut status = String::new();
        for path in paths {
            status.push_str(
                format!("{} ", 
                    Battery::builder(
                        format!("/sys/class/power_supply/{}/uevent", &path)
                        .as_str()
                    ).build()
                ).as_str()
            );
             
        }
        let _xsetroot = Command::new("xsetroot")
                        .args(["-name", &status])
                        .spawn()
                        .expect("failed to display status");
    }

}
