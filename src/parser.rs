use std::fs::read_to_string;

pub struct Parser;

impl Parser {
    fn check_for_battiers() {
        
    }

    pub fn parse_battery<F>(f: F) -> Vec<String>
    where
        F: Fn(&str) -> bool,
    {
        let contents = read_to_string("/sys/class/power_supply/BAT0/uevent");
        let keys = contents.as_ref().unwrap().split('\n')
                        .filter(|x| x.len() > 0)
                        .map(|line| {
                            let split = line.split('=').collect::<Vec<&str>>();
                            (split[0], split[1])

                        })
                        .filter(|(key, _)| f(key))
                        .map(|(_, value)| value.to_string())
                        .collect::<Vec<String>>();

        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test] 
    fn test_parse() {
        let keys = ["POWER_SUPPLY_NAME"];
        let ret = Parser::parse_battery(|s: &str| keys.contains(&s));
    }


}
