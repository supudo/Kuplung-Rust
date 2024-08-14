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
  vertex_buffer: glow::Buffer
}

#[allow(unsafe_code)]
impl Triangler {
  pub fn new(gl: &glow::Context) -> Option<Self> {
    use glow::HasContext as _;
    let shader_version = egui_glow::ShaderVersion::get(gl);
    unsafe {
      let program = gl.create_program().expect("[Kuplung] [Triangler] Cannot create program!");

      if !shader_version.is_new_shader_interface() {
        warn!("[Kuplung] [Triangler]Triangler hasn't been tested on {:?}.", shader_version);
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

      let vertex_buffer = gl.create_buffer().expect("[Kuplung] [Triangler] Cannot create vertex buffer!");
      gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
      gl.buffer_data_u8_slice(glow::ARRAY_BUFFER,  core::slice::from_raw_parts(VERTEX_DATA.as_ptr() as *const u8, VERTEX_DATA.len() * size_of::<f32>()), glow::STATIC_DRAW);

      let vertex_array = gl.create_vertex_array().expect("[Kuplung] [Triangler] Cannot create vertex array!");
      gl.bind_vertex_array(Some(vertex_array));
      gl.enable_vertex_attrib_array(0);
      gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

      /*let pos_attrib = gl.get_attrib_location(program, b"position\0".as_ptr() as *const _);
      let color_attrib = gl.get_attrib_location(program, b"color\0".as_ptr() as *const _);
      gl.vertex_attrib_pointer_f32(pos_attrib as gl::types::GLuint, 2, gl::FLOAT, 0, 5 * std::mem::size_of::<f32>() as gl::types::GLsizei, std::ptr::null());
      gl.vertex_attrib_pointer_f32(color_attrib as gl::types::GLuint, 3, gl::FLOAT, 0, 5 * std::mem::size_of::<f32>() as gl::types::GLsizei, (2 * std::mem::size_of::<f32>()) as *const () as *const _);
      gl.enable_vertex_attrib_array(pos_attrib as gl::types::GLuint);
      gl.enable_vertex_attrib_array(color_attrib as gl::types::GLuint);*/

      Some(Self {
        program,
        vertex_array: vertex_array,
        vertex_buffer: vertex_buffer
      })
    }
  }

  pub fn destroy(&self, gl: &glow::Context) {
    use glow::HasContext as _;
    unsafe {
      gl.delete_program(self.program);
      gl.delete_vertex_array(self.vertex_array);
      gl.delete_buffer(self.vertex_buffer);
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