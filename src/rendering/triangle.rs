use std::ffi::{CStr, CString};
use std::io::Read;
use std::ops::Deref;

use glutin::prelude::*;

use log::info;
use crate::settings::configuration;

#[rustfmt::skip]
pub static VERTEX_DATA: [f32; 15] = [
  -0.5, -0.5,  1.0,  0.0,  0.0,
  0.0,  0.5,  0.0,  1.0,  0.0,
  0.5, -0.5,  0.0,  0.0,  1.0,
];

pub mod gl {
  #![allow(clippy::all)]
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
  pub use Gles2 as Gl;
}

pub struct Triangle {
  program: gl::types::GLuint,
  vao: gl::types::GLuint,
  vbo: gl::types::GLuint,
  gl: gl::Gl,
}

impl Triangle {
  pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
    unsafe {
      let gl = gl::Gl::load_with(|symbol| {
        let symbol = CString::new(symbol).unwrap();
        gl_display.get_proc_address(symbol.as_c_str()).cast()
      });

      if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
        info!("[Kuplung] Running on {}", renderer.to_string_lossy());
      }
      if let Some(version) = get_gl_string(&gl, gl::VERSION) {
        info!("[Kuplung] OpenGL Version {}", version.to_string_lossy());
      }

      if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
        info!("[Kuplung] Shaders version on {}", shaders_version.to_string_lossy());
      }

      let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, "assets/shaders/triangle.vert");
      let fragment_shader = create_shader(&gl, gl::FRAGMENT_SHADER, "assets/shaders/triangle.frag");

      let program = gl.CreateProgram();

      gl.AttachShader(program, vertex_shader);
      gl.AttachShader(program, fragment_shader);

      gl.LinkProgram(program);

      gl.UseProgram(program);

      gl.DeleteShader(vertex_shader);
      gl.DeleteShader(fragment_shader);

      let mut vao = std::mem::zeroed();
      gl.GenVertexArrays(1, &mut vao);
      gl.BindVertexArray(vao);

      let mut vbo = std::mem::zeroed();
      gl.GenBuffers(1, &mut vbo);
      gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl.BufferData(gl::ARRAY_BUFFER, (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, VERTEX_DATA.as_ptr() as *const _, gl::STATIC_DRAW);

      let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
      let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
      gl.VertexAttribPointer(pos_attrib as gl::types::GLuint, 2, gl::FLOAT, 0, 5 * std::mem::size_of::<f32>() as gl::types::GLsizei, std::ptr::null());
      gl.VertexAttribPointer(color_attrib as gl::types::GLuint, 3, gl::FLOAT, 0, 5 * std::mem::size_of::<f32>() as gl::types::GLsizei, (2 * std::mem::size_of::<f32>()) as *const () as *const _);
      gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
      gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);

      Self { program, vao, vbo, gl }
    }
  }

  pub fn draw(&self) {
    self.draw_with_clear_color(configuration::GL_CLEAR_COLOR_R, configuration::GL_CLEAR_COLOR_G, configuration::GL_CLEAR_COLOR_B, configuration::GL_CLEAR_COLOR_A)
  }

  pub fn draw_with_clear_color(
    &self,
    red: ::gl::types::GLfloat,
    green: ::gl::types::GLfloat,
    blue: ::gl::types::GLfloat,
    alpha: ::gl::types::GLfloat,
  ) {
    unsafe {
      self.gl.UseProgram(self.program);

      self.gl.BindVertexArray(self.vao);
      self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

      self.gl.ClearColor(red, green, blue, alpha);
      self.gl.Clear(gl::COLOR_BUFFER_BIT);
      self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
    }
  }

  pub fn resize(&self, width: i32, height: i32) {
    unsafe {
      self.gl.Viewport(0, 0, width, height);
    }
  }
}

impl Deref for Triangle {
  type Target = gl::Gl;
  fn deref(&self) -> &Self::Target {
    &self.gl
  }
}

impl Drop for Triangle {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteProgram(self.program);
      self.gl.DeleteBuffers(1, &self.vbo);
      self.gl.DeleteVertexArrays(1, &self.vao);
    }
  }
}

unsafe fn create_shader(gl: &gl::Gl, shader: gl::types::GLenum, shader_filepath: &str) -> gl::types::GLuint {
  info!("[Kuplung] Loading shader file {}", shader_filepath);

  let mut shader_file = std::fs::File::open(shader_filepath).expect("[Kuplung] Cannot fine the shader file specified!");
  let mut shader_buffer: Vec<u8> = Vec::new();
  shader_file.read_to_end(&mut shader_buffer).unwrap();
  let shader_source = CString::new(shader_buffer).unwrap();

  let shader = gl.CreateShader(shader);
  gl.ShaderSource(shader, 1, [shader_source.as_ptr().cast()].as_ptr(), std::ptr::null());
  gl.CompileShader(shader);
  shader
}

fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
  unsafe {
    let s = gl.GetString(variant);
    (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
  }
}