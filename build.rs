use std::{
  env, fs, io,
  fs::File,
  path::{Path, PathBuf}
};

use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};

const COPY_DIR: &'static str = "assets";

fn main()-> io::Result<()> {
  let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());
  println!("cargo:rerun-if-changed=build.rs");
  let mut file = File::create(dest.join("gl_bindings.rs")).unwrap();
  Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, [])
    .write_bindings(StructGenerator, &mut file)
    .unwrap();

  // copy the assets folder
  let assetsFolder = env::var("OUT_DIR").unwrap();
  let assetsFolder = format!("{}/{}", assetsFolder, COPY_DIR);
  println!("Copying assets to output folder = {}", assetsFolder);
  let assetsRef = PathBuf::from(assetsFolder);

  if assetsRef.exists() {
    fs::remove_dir_all(&assetsRef).unwrap();
  }
  fs::create_dir(&assetsRef).unwrap();
  copy_dir(COPY_DIR, &assetsRef);

  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/Kuplung.ico");
    res.set("InternalName", "Kuplung.exe");
    res.compile()?;
  }
  Ok(())
}

/// A helper function for recursively copying a directory.
fn copy_dir<P, Q>(from: P, to: Q) where P: AsRef<Path>, Q: AsRef<Path>, {
  let to = to.as_ref().to_path_buf();

  for path in fs::read_dir(from).unwrap() {
    let path = path.unwrap().path();
    let to = to.clone().join(path.file_name().unwrap());

    if path.is_file() {
      fs::copy(&path, to).unwrap();
    }
    else if path.is_dir() {
      if !to.exists() {
        fs::create_dir(&to).unwrap();
      }
      copy_dir(&path, to);
    }
    else { /* Skip other content */
    }
  }
}