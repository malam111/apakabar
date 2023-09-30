mod battery;
mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_new() {
        let root_dir = "/sys/class/power_supply/";
        if let Ok(mut builder) = battery::BatteryBuilder::new(&root_dir) {
            loop {
                builder.build();
            }
        }
        assert_eq!(1, 1);
    }
}
