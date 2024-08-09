use glutin::config::Config;
use glutin::prelude::*;

pub mod gl {
  #![allow(clippy::all)]
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
  pub use Gles2 as Gl;
}

pub struct Renderer {
  program: gl::types::GLuint,
  vao: gl::types::GLuint,
  vbo: gl::types::GLuint,
  gl: gl::Gl,
}

pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
  configs
    .reduce(|accum, config| {
      let transparency_check = config.supports_transparency().unwrap_or(false) & !accum.supports_transparency().unwrap_or(false);
      if transparency_check || config.num_samples() > accum.num_samples() {
        config
      }
      else {
        accum
      }
    })
    .unwrap()
}