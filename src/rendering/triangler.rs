use std::ffi::CString;
use std::io::Read;
use eframe::egui_glow;
use eframe::glow::HasContext;
use egui_glow::glow;

use log::{info, warn};
use crate::rendering::gl_utils;

#[rustfmt::skip]
pub static VERTEX_DATA: [f32; 15] = [
  -0.5, -0.5,  1.0,  0.0,  0.0,
  0.0,  0.5,  0.0,  1.0,  0.0,
  0.5, -0.5,  0.0,  0.0,  1.0,
];

pub struct Triangler {
  program: glow::Program,
  vertex_array: glow::VertexArray,
  //vertex_buffer: glow::Buffer
}

#[allow(unsafe_code)]
impl Triangler {
  pub fn new(gl: &glow::Context) -> Option<Self> {
    use glow::HasContext as _;
    let shader_version = egui_glow::ShaderVersion::get(gl);
    unsafe {
      let program = gl.create_program().expect("[Kuplung] [Triangler] Cannot create program");

      if !shader_version.is_new_shader_interface() {
        warn!("[Kuplung] [Triangler] Custom 3D painting hasn't been ported to {:?}", shader_version);
        return None;
      }

      let shader_vertex = gl_utils::create_shader(&program, &gl, shader_version, glow::VERTEX_SHADER, "assets/shaders/triangle.vert");
      let shader_fragment = gl_utils::create_shader(&program, &gl, shader_version, glow::FRAGMENT_SHADER, "assets/shaders/triangle.frag");

      gl.link_program(program);
      assert!(gl.get_program_link_status(program), "{}", gl.get_program_info_log(program));

      gl.detach_shader(program, shader_vertex);
      gl.delete_shader(shader_vertex);
      gl.detach_shader(program, shader_fragment);
      gl.delete_shader(shader_fragment);

      let vertex_array = gl.create_vertex_array().expect("[Kuplung] [Triangler] Cannot create vertex array");

      Some(Self {
        program,
        vertex_array: vertex_array,
        //vertex_buffer: vertex_buffer
      })
    }
  }

  pub fn destroy(&self, gl: &glow::Context) {
    use glow::HasContext as _;
    unsafe {
      gl.delete_program(self.program);
      gl.delete_vertex_array(self.vertex_array);
      //gl.delete_buffer(self.vertex_buffer);
    }
  }

  pub fn paint(&self, gl: &glow::Context, angle: f32) {
    use glow::HasContext as _;
    unsafe {
      gl.use_program(Some(self.program));
      gl.uniform_1_f32(gl.get_uniform_location(self.program, "u_angle").as_ref(), angle);
      gl.bind_vertex_array(Some(self.vertex_array));
      gl.draw_arrays(glow::TRIANGLES, 0, 3);
    }
  }
}