use std::fmt;

use crate::parser::Parser;

#[derive(Debug)]
enum PsStatus {
    Unknown,
    Charging,
    Discharging,
    NotCharging,
    Full,
}

impl PsStatus {
    fn value(&self) -> &str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::Charging => "CHARGING",
            Self::Discharging => "DISCHARGING",
            Self::NotCharging => "NOT CHARGING",
            Self::Full => "FULL",
        }
    }
}

impl From<String> for PsStatus {
    fn from(value: String) -> Self {
        if value.eq_ignore_ascii_case("CHARGING") { return Self::Charging }
        else if value.eq_ignore_ascii_case("DISCHARGING") { return Self::Discharging }
        else if value.eq_ignore_ascii_case("NOT CHARGING") { return Self::NotCharging }
        else if value.eq_ignore_ascii_case("FULL") { return Self::Full }
        else { return Self::Unknown }
    }
}

impl Default for PsStatus {
    fn default() -> Self {
        Self::Unknown 
    }
}



impl fmt::Display for PsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(Debug, Default)]
pub struct Battery {
    ps_name: String,
    ps_status: PsStatus,
    ps_capacity: u8,
}

impl Battery {
    pub fn builder() -> BatteryBuilder {
        BatteryBuilder::new()
    }

    fn init(self) -> Self {
        let needs = [
            "POWER_SUPPLY_NAME", 
            "POWER_SUPPLY_STATUS", 
            "POWER_SUPPLY_CAPACITY"
        ];
        let it = Parser::parse_battery(|key: &str| {
                needs.contains(&key)
        });
        let ps_name = if self.ps_name.is_empty() { it[0].clone() } else { self.ps_name };
        Self {
            ps_name: ps_name,
            ps_status: it[1].clone().into(),
            ps_capacity: it[2].parse().unwrap(),
            ..Self::default()
        }
    }
}


impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{} {}", self.ps_name, self.ps_capacity, self.ps_status)
    }
}

pub struct BatteryBuilder {
    ps_name: String,
}

impl BatteryBuilder {
    pub fn new() -> Self {
        Self {
            ps_name: String::new(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.ps_name = name;
        self
    }

    pub fn build(self) -> Battery {
        let bat = Battery {
            ps_name: self.ps_name,
            ..Battery::default()
        };
        bat.init() 
    }

}


