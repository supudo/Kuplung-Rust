use std::{
  env, fs, io,
  fs::File,
  path::{Path, PathBuf}
};
use std::process::Command;
use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};

const ASSETS_DIR: &'static str = "assets";

fn main()-> io::Result<()> {
  let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());
  println!("cargo:rerun-if-changed=build.rs");
  let mut file = File::create(dest.join("gl_bindings.rs")).unwrap();
  Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, [])
    .write_bindings(StructGenerator, &mut file)
    .unwrap();

  // copy the assets folder
  let assets_folder = env::var("OUT_DIR").unwrap();
  let assets_folder = format!("{}/{}", assets_folder, ASSETS_DIR);
  println!("[Kuplung-BUILD] Copying assets to output folder = {}", assets_folder);
  let assets_ref = PathBuf::from(assets_folder);
  if assets_ref.exists() {
    fs::remove_dir_all(&assets_ref).unwrap();
  }
  fs::create_dir(&assets_ref).unwrap();
  copy_dir(ASSETS_DIR, &assets_ref);

  setup_internals();
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

/// set platform-specific resources
#[cfg(target_os = "windows")]
fn setup_internals() {
  let mut res = winres::WindowsResource::new();
  res.set_icon("assets/Kuplung.ico");
  res.set("InternalName", "Kuplung.exe");
  res.compile()?;
}

#[cfg(target_os = "macos")]
fn setup_internals() {
  println!("[Kuplung-BUILD] Creating OSX app container...");

  let container_folder: &str = "release/macos/Kuplung-Rust.app";

  fs::create_dir_all(format!("{}/Contents/MacOS", container_folder))
      .expect("[Kuplung-BUILD] Cannot create osx app container folders!");

  fs::create_dir_all(format!("{}/Contents/Resources", container_folder))
      .expect("[Kuplung-BUILD] Cannot create osx app container folders!");

  fs::copy("target/debug/Kuplung-Rust", format!("{}/Contents/MacOS/Kuplung-Rust", container_folder))
      .expect("[Kuplung-BUILD] Cannot copy binary to app container!");

  fs::copy("assets/Info.plist", format!("{}/Contents/Info.plist", container_folder))
      .expect("[Kuplung-BUILD] Cannot copy Info.plist to app container!");

  println!("[Kuplung-BUILD] Copying assets to the app container folder...");
  let assets_ref = PathBuf::from(format!("{}/Contents/Resources", container_folder));
  if assets_ref.exists() {
    fs::remove_dir_all(&assets_ref).unwrap();
  }
  fs::create_dir(&assets_ref).unwrap();
  copy_dir(ASSETS_DIR, &assets_ref);

  Command::new("hdiutil")
      .arg("create")
      .arg("release/Kuplung-Rust.dmg")
      .arg("-volname")
      .arg("Kuplung-Rust")
      .arg("-srcfolder")
      .arg("release/macos")
      .arg("-ov");
}

