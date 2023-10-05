mod brightness;

pub use brightness::Brightness;


#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use std::fs;
    use std::path;
    use std::marker::PhantomData;

    #[test]
    fn test_new() -> io::Result<()> {
        let brig_path = "/sys/class/backlight/intel_backlight";
        if let Ok(mut brig) = brightness::Brightness::builder(&brig_path)?.discover() {
            brig.build();
        }
        assert_eq!(1,1);
        Ok(())
    }

    //#[test]
    //#[ignore = "testing"]
    //fn test_phantom() -> io::Result<()> {
    //    let builder = brightness::BrightnessBuilder::<brightness::BuildReady> {
    //        path: path::Path::new("a").to_path_buf(),
    //        reader: io::BufReader::new(fs::File::open(".cargo-lock")?),
    //        marker: PhantomData,
    //    };

    //    Ok(())
    //}
}
