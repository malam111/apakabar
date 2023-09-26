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
        let batteries = Battery::builder()
                            .name("Lautan Api".to_string())
                            .build();
        let status = format!("{}", batteries);
        let _xsetroot = Command::new("xsetroot")
                        .args(["-name", &status])
                        .spawn()
                        .expect("failed to display status");
    }

}
