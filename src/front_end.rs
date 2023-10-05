use std::process::Command;
use std::thread;
use std::time::Duration;
use std::io;

use battery;
use brightness;
use memory;

pub struct FrontEnd;

impl FrontEnd {

    pub fn main_loop() -> io::Result<()> {
        let root_dir = "/sys/class/power_supply/";
        let brig_path = "/sys/class/backlight/intel_backlight";
        let mem_path = "/proc/meminfo";

        let attrs = battery::Attributes {
            attr_name: true,
            attr_status: true,
            attr_capacity: true,
        };
        let mut batt = battery::Battery::builder(&root_dir)?.attributes(attrs)?.discover()?;
        let mut brig = brightness::Brightness::builder(&brig_path)?.discover()?;
        let mut mem = memory::Memory::builder(&mem_path)?.discover()?;
        
        let mut status = String::new();
        loop {
            status.push_str(
                batt.build()
                    .iter()
                    .fold(String::new(), |mut buffer: String, batt| {
                        buffer.push_str(
                            format!(" {}", batt).as_str()
                        );
                        buffer
                    }).as_str()
            );
            status.push_str(
                format!(" {}", brig.build()).as_str()
            );
            status.push_str(
                format!(" {}", mem.build()).as_str()
            );
            Self::render(&status);
            status.clear();
            thread::sleep(Duration::from_secs(1));
        }

    }

    fn render(status: &str) {
        //println!("{}", &status);
        let _xsetroot = Command::new("xsetroot")
                        .args(["-name", &status])
                        .spawn()
                        .expect("failed to display status");
    }

}
