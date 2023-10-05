use std::io;
use std::io::prelude::*;
use std::fs;
use std::fmt;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

pub struct Memory {
    avail: f64,
    total: f64,
}

impl Memory {
    pub fn builder(path: &str) -> io::Result<MemoryBuilder<BuildUnready>> {
        Ok(MemoryBuilder::<BuildUnready>::new(path)?)
    }

    fn from_str(content: &str) -> Option<Self> {
        let takes = "MemTotal;MemAvailable";
        let items = content.split('\n')
            .filter(|line| line.contains(':') )
            .map(|line| {
                let keyval = line.split(':').collect::<Vec<&str>>();
                (keyval[0], keyval[1])
            }).filter(|(key, _)| takes.contains(key))
            .collect::<Vec<(&str, &str)>>();

        if items.len() < 2 {
            return None
        }
            
        Some(Memory {
            total: items[0].1.trim().replace(" kB", "").parse().unwrap(),
            avail: items[1].1.trim().replace(" kB", "").parse().unwrap(),
        })
    }
}
        
impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (100_f64*(self.avail/self.total)).round())
    }
}

pub struct BuildUnready;
pub struct BuildReady;

pub struct MemoryBuilder<T> {
    path: PathBuf,
    reader: Option<io::BufReader<fs::File>>,
    marker: PhantomData<T>,
}

impl MemoryBuilder<BuildUnready> {
    // provide path to backlight directory
    // e.g /proc/meminfo
    pub fn new(path: &str) -> io::Result<Self> {
        let path = Path::new(path);
        if !path.is_file() {
            return Err(
                io::Error::from(io::ErrorKind::NotFound)
            )
        }

        Ok(Self {
            path: path.to_path_buf(),
            reader: None,
            marker: PhantomData,
        })
    }

    pub fn discover(mut self) -> io::Result<MemoryBuilder<BuildReady>> {
        let file = fs::File::open(&self.path)?;
        let reader = io::BufReader::new(file);
        Ok(MemoryBuilder::<BuildReady> {
            path: self.path,
            reader: Some(reader),
            marker: PhantomData,
        }) 
    }
}

impl MemoryBuilder<BuildReady> {


    pub fn build(&mut self) -> Memory {
        let mut buffer = String::new();
        if let Some(reader) = self.reader.as_mut() {
            reader.seek(io::SeekFrom::Start(0));
            reader.read_to_string(&mut buffer);
        };
        //println!("{:?}", &buffer);

    
        Memory::from_str(&buffer).unwrap()
    }

}
