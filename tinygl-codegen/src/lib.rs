pub mod compiler;
pub mod types;

pub use tinygl_compiler::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use tinygl_compiler::Result;

pub use compiler::WrappedItem;

pub fn write<'a>(dest: impl AsRef<Path>, items: &[&'a dyn WrappedItem]) -> Result<()> {
    // Write master shaders.rs file
    let output_rs = File::create(dest.as_ref())?;
    let mut wr = BufWriter::new(output_rs);

    for item in items {
        writeln!(wr, "{}", item.generate()?)?;
    }

    Ok(())
}
