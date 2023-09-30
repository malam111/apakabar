use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;

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

impl From<&str> for Battery {
    fn from(content: &str) -> Self {
        content.split('\n')
            .filter(|x| x.len() > 0)
            .map(|item| {
                let item = item.split('=').collect::<Vec<&str>>();
                (item[0], item[1])
            });
        Battery::default()     
    }
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{} {}", self.ps_name, self.ps_capacity, self.ps_status)
    }
}

#[derive(Debug)]
struct Uevent {
    path: PathBuf,
    reader: io::BufReader<File>,

}

pub struct BatteryBuilder<'a> {
    psu_dir: Box<&'a Path>,
    uevents: Vec<Uevent>,
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
            uevents: Vec::new(),
            battery: Vec::new(),
        })
    }

    // provide battery path
    pub fn path(mut self, path: &'a str) -> io::Result<Self> {
        let path = Path::new(&path).to_path_buf();
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);
        self.uevents.push(
            Uevent {
                path,
                reader,
            }
        );
        Ok(self)
    }

    pub fn paths(mut self, paths: Vec<&'a str>) -> io::Result<Self> {
        paths.iter().map(|&path| {
            let path = Path::new(&path).to_path_buf();
            let file = File::open(&path).unwrap();
            let reader = io::BufReader::new(file);
            &self.uevents.push(
                Uevent {
                    path,
                    reader,
                }
            );
            ()
        });
        Ok(self)
    }

    fn discover(&mut self) -> io::Result<()> {
        let paths = (*self.psu_dir).read_dir().expect("Cannot read power supply directory")
            .filter(|node| 
                if let Ok(some_node) = node {
                    return some_node.file_type().unwrap().is_symlink() && 
                        some_node.file_name().to_str().unwrap().contains("BAT");
                } else {
                    return false
                }
            ).map(|node| {
                node.unwrap().path().join("uevent").to_path_buf()
            });
        let mut uevents = Vec::new();
        for path in paths {
            if !path.is_file() {
                return Err(io::Error::new(io::ErrorKind::NotFound, format!("{} does not contain information", path.parent().unwrap().file_name().unwrap().to_str().unwrap())));
            }
            let file = File::open(&path)?;
            let reader = io::BufReader::new(file);
            uevents.push(Uevent {
                path,
                reader,
            });
        }
        self.uevents = uevents;
        Ok(())

    }

    pub fn build(&mut self) -> Battery {
        let needs = [
            "POWER_SUPPLY_NAME",                                                   
            "POWER_SUPPLY_STATUS", 
            "POWER_SUPPLY_CAPACITY"
        ];
        self.discover();
        let mut buffer = String::new();
        
        for uevent in self.uevents.iter_mut() {
            uevent.reader.read_to_string(&mut buffer);
            println!("{:?}", buffer); 
        }

        //let it = Parser::parse_battery((*self.paths[0]).to_str().unwrap(), |key: &str| {
        //        needs.contains(&key)
        //});
        Battery {
            //ps_name: it[0].clone(),
            //ps_status: it[1].clone().into(),
            //ps_capacity: it[2].parse().unwrap(),
            ..Battery::default()
        }
    }
}
