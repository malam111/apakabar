use std::fmt;
use std::io;
use std::path::Path;

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
        if value.eq_ignore_ascii_case(Self::Charging.value()) { return Self::Charging }
        else if value.eq_ignore_ascii_case(Self::Discharging.value()) { return Self::Discharging }
        else if value.eq_ignore_ascii_case(Self::NotCharging.value()) { return Self::NotCharging }
        else if value.eq_ignore_ascii_case(Self::Full.value()) { return Self::Full }
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
    pub fn builder(path: &str) -> Result<BatteryBuilder, io::Error> {
        Ok(BatteryBuilder::new(path)?)
    }
    
}


impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{} {}", self.ps_name, self.ps_capacity, self.ps_status)
    }
}

pub struct BatteryBuilder<'a> {
    psu_dir: Box<&'a Path>,
    paths: Vec<Box<&'a Path>>,
    battery: Vec<Battery>,
}

impl<'a> BatteryBuilder<'a> {

    // provide root dir of batteries
    pub fn new(psu_dir: &'a str) -> Result<Self, io::Error> {
        let psu_dir = Box::new(Path::new(psu_dir));
        if !psu_dir.is_dir() {
            return Err(io::Error::from(io::ErrorKind::NotFound))
        }
        Ok(Self {
            psu_dir,
            paths: Vec::new(),
            battery: Vec::new(),
        })
    }

    // provide battery path
    pub fn path(mut self, path: &'a str) -> Self {
        let path = Box::new(Path::new(path));
        self.paths.push(path);
        self
    }

    pub fn paths(mut self, paths: Vec<&'a str>) -> Self {
        paths.iter().map(|&path| {
            &self.paths.push(Box::new(Path::new(path)));
            path
        });
        self
    }

    pub fn build(&mut self) -> Battery {
        let needs = [
            "POWER_SUPPLY_NAME",                                                   
            "POWER_SUPPLY_STATUS", 
            "POWER_SUPPLY_CAPACITY"
        ];

        for entry in (*self.psu_dir).read_dir()
                        .expect("Cannot read power supply directory") {
             
        }
        let it = Parser::parse_battery((*self.paths[0]).to_str().unwrap(), |key: &str| {
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
