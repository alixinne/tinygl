use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

mod struct_generator;

pub fn gen_bindings() {
    let dest = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR missing, make sure this command is run through cargo"),
    );

    let dest = dest.parent().unwrap().join("tinygl/src/gl/desktop/");
    std::fs::create_dir_all(&dest).expect("failed to create bindings directory");

    for ver in [(4, 4), (4, 5), (4, 6)].iter() {
        let path = &Path::new(&dest).join(format!(
            "bindings{major}{minor}.rs",
            major = ver.0,
            minor = ver.1
        ));

        // Build the OpenGL bindings
        let mut file = File::create(&path).unwrap();

        {
            use gl_generator::{Api, Fallbacks, Profile, Registry};
            Registry::new(Api::Gl, *ver, Profile::Core, Fallbacks::All, [])
                .write_bindings(struct_generator::StructGenerator, &mut file)
                .unwrap();
        }

        println!("wrote {}", path.display());
    }
}
