use std::path::Path;

struct Brightness(u16);

impl Brightness {
    fn builder(path: &str) -> BrightnessBuilder {
        BrightnessBuilder::new(path)
    }
}

struct BrightnessBuilder<'a> {
    path: &'a Path,
}

impl BrightnessBuilder {
    pub fn new(path: &str) -> Self {
        Self {
            path: Path::new(path)
        }
    }

    pub fn build(&self) -> Brightness {

    }

    fn 

}


#[cfg(test)]
mod tests {
    use super::*;

}
