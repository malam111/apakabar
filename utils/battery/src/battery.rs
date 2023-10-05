use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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



impl From<&str> for PsStatus {
    fn from(value: &str) -> Self {
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
    ps_name: Option<String>,
    ps_status: Option<PsStatus>,
    ps_capacity: Option<u8>,
}

impl Battery {
    pub fn builder(path: &str) -> Result<BatteryBuilder, io::Error> {
        Ok(BatteryBuilder::new(path)?)
    }

    fn from_str(content: &str, attr: Option<&str>) -> Self {
        let mut map = HashMap::new();
        let items = content.split('\n')
            .filter(|x| x.len() > 0 && x.contains('='))
            .map(|item| {
                let item = item.split('=').collect::<Vec<&str>>();
                (item[0], item[1])
            }).filter(|(key, _)| {
                if let Some(attr) = attr {
                    //println!("{:?}{:?}", attr, key);
                    attr.contains(key)
                } else {
                    true
                }
            });
        for (key, value) in items {
            map.insert(key, value);
        }
        let ps_status: PsStatus = map.get(
            BatteryAttr::AttrStatus.value().as_str()
        ).unwrap().clone().into();

        let ps_name = map.get(
            BatteryAttr::AttrName.value().as_str()
        ).unwrap().clone().to_string();

        let ps_capacity: u8 = map.get(
            BatteryAttr::AttrCapacity.value().as_str()
        ).unwrap().parse().unwrap();

        Battery {
            ps_name: Some(ps_name),
            ps_status: Some(ps_status),
            ps_capacity: Some(ps_capacity),
        }
    }
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{} {}", self.ps_name.as_ref().unwrap(), self.ps_capacity.as_ref().unwrap(), self.ps_status.as_ref().unwrap())
    }
}

#[derive(Debug)]
struct Uevent {
    path: PathBuf,
    reader: io::BufReader<File>,

}

pub enum BatteryAttr {
    AttrName,
    AttrStatus,
    AttrCapacity,
}

impl BatteryAttr {
    fn value(&self) -> String {
        let prefix = "POWER_SUPPLY_";
        return format!("{}{}", prefix, match self {
            Self::AttrName => "NAME",
            Self::AttrStatus => "STATUS",
            Self::AttrCapacity => "CAPACITY",
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Attributes {
    pub attr_name: bool,
    pub attr_status: bool,
    pub attr_capacity: bool,
}

impl Into<String> for Attributes {
    fn into(self) -> String {
        let is_name = if self.attr_name { 
            BatteryAttr::AttrName.value() 
        } else { "".to_string() };
        let is_status = if self.attr_status { 
            BatteryAttr::AttrStatus.value() 
        } else { "".to_string() };
        let is_capacity = if self.attr_capacity { 
            BatteryAttr::AttrCapacity.value() 
        } else { "".to_string() };
        format!("{};{};{}", is_name, is_status, is_capacity)
    }
}


#[derive(Debug)]
pub struct BatteryBuilder<'a> {
    psu_dir:    Box<&'a Path>,
    uevents:    Vec<Uevent>,
    attr:       Attributes,
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
            uevents:    Vec::new(),
            attr:       Attributes::default(),
        })
    }

    // manually provide battery path
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

    // manually provide batteries path
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

    pub fn attributes(mut self, attr: Attributes) -> io::Result<Self> {
        self.attr = attr;
        Ok(self)
    }

    // auto discover every battery under the provided root dir
    // default: /sys/class/power_supply/[BAT[0-9]*/uevent
    // Err if uevent doesn't exist
    pub fn discover(mut self) -> io::Result<Self> {
        let paths = (*self.psu_dir).read_dir().expect("Cannot read power supply directory")
            .filter(|node| {
                if let Ok(some_node) = node {
                    return some_node.file_type().unwrap().is_symlink() && 
                        some_node.file_name().to_str().unwrap().contains("BAT");
                } 
                return false
            }).map(|node| {
                node.unwrap().path().join("uevent").to_path_buf()
            });
        let mut uevents = Vec::new();
        for path in paths {
            if !path.is_file() {
                return Err(io::Error::new(
                        io::ErrorKind::NotFound, 
                        format!("{} does not contain information", 
                            path.parent().unwrap().file_name().unwrap().to_str().unwrap()
                        ))
                );
            }
            let file = File::open(&path)?;
            let reader = io::BufReader::new(file);
            uevents.push(Uevent {
                path,
                reader,
            });
        }
        self.uevents = uevents;
        Ok(self)

    }

    pub fn build(&mut self) -> Vec<Battery> {
        //self.discover();
        let mut buffer = String::new();
        let mut batteries = Vec::<Battery>::new();
        
        for uevent in self.uevents.iter_mut().rev() {
            uevent.reader.seek(io::SeekFrom::Start(0));
            uevent.reader.read_to_string(&mut buffer);
            let batt = Battery::from_str(&buffer, Some(Into::<String>::into(self.attr).as_str()));
            //print!("{} ", batt); 
            batteries.push(batt);
            buffer.clear();
        }
        //print!("\n");
        batteries

    }

}
