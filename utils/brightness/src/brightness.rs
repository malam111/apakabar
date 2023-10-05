use std::io;
use std::io::prelude::*;
use std::fs;
use std::fmt;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

pub struct Brightness(u16);

impl Brightness {
    pub fn builder(path: &str) -> io::Result<BrightnessBuilder<BuildUnready>> {
        Ok(BrightnessBuilder::<BuildUnready>::new(path)?)
    }

    fn from_str(content: &str) -> Option<Self> {
        if let Ok(brightness) = content.trim().parse::<u16>() {
            return Some(Self(brightness))
        }
        None
    }
}
        
impl fmt::Display for Brightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct BuildUnready;
pub struct BuildReady;

pub struct BrightnessBuilder<T> {
    path: PathBuf,
    reader: Option<io::BufReader<fs::File>>,
    max: Option<u16>,
    marker: PhantomData<T>,
}

impl BrightnessBuilder<BuildUnready> {
    // provide path to backlight directory
    // e.g /sys/class/backlight/intel_backlight
    pub fn new(path: &str) -> io::Result<Self> {
        let path = Path::new(path);
        if !path.is_dir() {
            return Err(
                io::Error::from(io::ErrorKind::NotFound)
            )
        }

        Ok(Self {
            path: path.to_path_buf(),
            reader: None,
            max: None,
            marker: PhantomData,
        })
    }

    pub fn discover(mut self) -> io::Result<BrightnessBuilder<BuildReady>> {
        let path = self.path.join("brightness");
        println!("{:?}", path);
        if !path.is_file() {
            return Err(
                io::Error::from(io::ErrorKind::NotFound)
            )
        }
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let max = self.path.join("max_brightness");
        let max = fs::read_to_string(max)?.parse().ok();
        Ok(BrightnessBuilder::<BuildReady> {
            path: self.path,
            reader: Some(reader),
            max: max,
            marker: PhantomData,
        }) 
    }
}

impl BrightnessBuilder<BuildReady> {


    pub fn build(&mut self) -> Brightness {
        let mut buffer = String::new();
        if let Some(reader) = self.reader.as_mut() {
            reader.seek(io::SeekFrom::Start(0));
            reader.read_to_string(&mut buffer);
        };
        //println!("{:?}", &buffer);

    
        Brightness::from_str(&buffer).unwrap()
    }

}
