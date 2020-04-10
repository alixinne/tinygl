use std::path::Path;

pub trait WrappedItem {
    fn write(&self, dest: &Path) -> Result<(), crate::Error>;
    fn write_root_include(&self, wr: &mut dyn std::io::Write) -> Result<(), crate::Error>;
}
