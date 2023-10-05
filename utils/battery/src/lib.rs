mod battery;

pub use battery::Battery;
pub use battery::BatteryBuilder;
pub use battery::Attributes;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_list_new() -> io::Result<()> {
        let root_dir = "/sys/class/power_supply/";
        let attrs = battery::Attributes {
            attr_name: true,
            attr_status: true,
            attr_capacity: true,
        };
        if let Ok(mut builder) = battery::BatteryBuilder::new(&root_dir)?.attributes(attrs)?.discover() {
            loop {
                builder.build();
                sleep(Duration::from_secs(1));
            }
        }
        assert_eq!(1, 1);
        Ok(())
        //Err(io::Error::from(io::ErrorKind::Interrupted))
    }

}


