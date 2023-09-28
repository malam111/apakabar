use std::fmt;

use crate::parser::Parser;

#[derive(Debug)]
enum PS_STATUS {
    UNKNOWN,
    CHARGING,
    DISCHARGING,
    NOT_CHARGING,
    FULL,
}

impl PS_STATUS {
    fn value(&self) -> &str {
        match self {
            Self::UNKNOWN => "UNKNOWN",
            Self::CHARGING => "CHARGING",
            Self::DISCHARGING => "DISCHARGING",
            Self::NOT_CHARGING => "NOT CHARGING",
            Self::FULL => "FULL",
        }
    }
}

impl From<String> for PS_STATUS {
    fn from(value: String) -> Self {
        if value.eq_ignore_ascii_case("CHARGING") { return Self::CHARGING }
        else if value.eq_ignore_ascii_case("DISCHARGING") { return Self::DISCHARGING }
        else if value.eq_ignore_ascii_case("NOT CHARGING") { return Self::NOT_CHARGING }
        else if value.eq_ignore_ascii_case("FULL") { return Self::FULL }
        else { return Self::UNKNOWN }
    }
}

impl Default for PS_STATUS {
    fn default() -> Self {
        Self::UNKNOWN 
    }
}



impl fmt::Display for PS_STATUS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(Debug, Default)]
pub struct Battery {
    ps_name: String,
    ps_status: PS_STATUS,
    ps_capacity: u8,
}

impl Battery {
    pub fn builder(path: &str) -> BatteryBuilder {
        BatteryBuilder::new(path )
    }
    
}


impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{} {}", self.ps_name, self.ps_capacity, self.ps_status)
    }
}

pub struct BatteryBuilder<'a> {
    path: &'a str
}

impl<'a> BatteryBuilder<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            path
        }
    }

    pub fn build(mut self) -> Battery {
        let needs = [
            "POWER_SUPPLY_NAME",                                                   
            "POWER_SUPPLY_STATUS", 
            "POWER_SUPPLY_CAPACITY"
        ];
        let it = Parser::parse_battery(self.path, |key: &str| {
                needs.contains(&key)
        });
        Battery {
            ps_name: it[0].clone(),
            ps_status: it[1].clone().into(),
            ps_capacity: it[2].parse().unwrap(),
            //..Self::default()
        }
    }
}
