use std::fs::read_to_string;

pub struct Parser;

impl Parser {
    pub fn parse_battery<F>(f: F) -> Vec<String>
    where
        F: Fn(&str) -> bool,
    {
        let contents = read_to_string("/sys/class/power_supply/BAT0/uevent");
        unsafe {
        let keys = contents.as_ref().unwrap().split('\n')
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .filter(|x| x.len() > 0)
                        .map(|line| {
                            line.slice_unchecked(
                                0,
                                line.find('=').unwrap()
                            )
                        });
        let values = contents.as_ref().unwrap().split('\n')
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .filter(|x| x.len() > 0)
                        .map(|line| {
                            line.slice_unchecked(
                                line.find('=').unwrap()+1,
                                line.len()
                            )
                        });

        let temp = keys.zip(values)
            .filter(|(key, value)| {
                f(key)
            })
            .collect::<Vec<(&str, &str)>>();

        let temp = temp.iter()
            .map(|x| {
                x.1.to_string()
            })
            .collect::<Vec<String>>();
        temp
        }
    }
}
