mod memory;
pub use memory::Memory;
pub use memory::BuildUnready;
pub use memory::BuildReady;
pub use memory::MemoryBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::prelude::*;

    #[test]
    fn test_new() -> io::Result<()> {
        let mem_path = "/proc/meminfo";
        if let Ok(mut mem) = Memory::builder(&mem_path)?.discover() {
            mem.build();
        }
        assert!(true);
        Ok(())
    }
}
