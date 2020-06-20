use structopt::StructOpt;

mod gen_bindings;

#[derive(StructOpt)]
pub enum Opts {
    /// Generate bindings for OpenGL functions
    GenBindings,
}

#[paw::main]
fn main(opts: Opts) {
    match opts {
        Opts::GenBindings => gen_bindings::gen_bindings(),
    }
}
