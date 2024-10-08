#![allow(non_snake_case)]

use eframe::egui_glow;
use eframe::glow::HasContext;
use egui_glow::glow;
use log::error;
use crate::rendering::gl_utils;
use crate::settings::configuration;

#[rustfmt::skip]
pub static TRIANGLER_VERTICES:[f32; 18] = [
   0.0,  1.0, 0.0, 1.0, 0.0, 0.0,   // top right
   1.0, -0.5, 0.0, 0.0, 1.0, 0.0,   // bottom right
  -1.0, -0.5, 0.0, 0.0, 0.0, 1.0,   // bottom left
];

#[rustfmt::skip]
pub static TRIANGLER_INDICES: [i32; 3] = [ 0, 1, 2 ];

pub struct Triangler {
  gl_Program: glow::Program,
  gl_VAO: glow::VertexArray,
  vbo_Vertices: glow::Buffer,
  vbo_Indices: glow::Buffer,
}

#[allow(unsafe_code)]
impl Triangler {
  pub fn new(gl: &glow::Context) -> Option<Self> {
    use glow::HasContext as _;
    unsafe {
      let gl_Program = gl.create_program().expect("[Kuplung] [Triangler] Cannot create program!");

      let shader_vertex = gl_utils::create_shader(&gl_Program, &gl, glow::VERTEX_SHADER, "assets/shaders/viewer/triangle.vert");
      let shader_fragment = gl_utils::create_shader(&gl_Program, &gl, glow::FRAGMENT_SHADER, "assets/shaders/viewer/triangle.frag");

      gl.link_program(gl_Program);
      if !gl.get_program_link_status(gl_Program) {
        error!("[Kuplung] [Triangler] Program cannot be linked! {}", gl.get_program_info_log(gl_Program));
        panic!("[Kuplung] [Triangler] Program cannot be linked! {}", gl.get_program_info_log(gl_Program));
      }

      gl.detach_shader(gl_Program, shader_vertex);
      gl.delete_shader(shader_vertex);
      gl.detach_shader(gl_Program, shader_fragment);
      gl.delete_shader(shader_fragment);

      let gl_VAO = gl.create_vertex_array().expect("[Kuplung] [Triangler] Cannot create vertex array!");
      gl.bind_vertex_array(Some(gl_VAO));

      let vbo_Vertices = gl.create_buffer().expect("[Kuplung] [Triangler] Cannot create vertex buffer!");
      gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo_Vertices));
      gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&TRIANGLER_VERTICES[..]), glow::STATIC_DRAW);

      let vbo_Indices = gl.create_buffer().expect("[Kuplung] [Triangler] Cannot create indices buffer!");
      gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(vbo_Indices));
      gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&TRIANGLER_INDICES[..]), glow::STATIC_DRAW);

      gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, size_of::<f32>() as i32 * 6, 0);
      gl.enable_vertex_attrib_array(0);

      gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, size_of::<f32>() as i32 * 6, size_of::<f32>() as i32 * 3);
      gl.enable_vertex_attrib_array(1);

      gl.bind_vertex_array(None);

      Some(Self {
        gl_Program,
        gl_VAO,
        vbo_Vertices,
        vbo_Indices
      })
    }
  }

  pub fn destroy(&self, gl: &glow::Context) {
    use glow::HasContext as _;
    unsafe {
      gl.delete_program(self.gl_Program);
      gl.delete_vertex_array(self.gl_VAO);
      gl.delete_buffer(self.vbo_Vertices);
      gl.delete_buffer(self.vbo_Indices);
    }
  }

  pub fn paint(&self, gl: &glow::Context, angle: f32) {
    unsafe {
      gl.clear_color(configuration::GL_CLEAR_COLOR_R, configuration::GL_CLEAR_COLOR_G, configuration::GL_CLEAR_COLOR_B, configuration::GL_CLEAR_COLOR_A);
      gl.clear(glow::COLOR_BUFFER_BIT);

      gl.use_program(Some(self.gl_Program));
      gl.bind_vertex_array(Some(self.gl_VAO));
      gl.uniform_1_f32(gl.get_uniform_location(self.gl_Program, "vs_angle").as_ref(), angle);
      gl.draw_elements(glow::TRIANGLES, 3, glow::UNSIGNED_INT, 0);
      gl.bind_vertex_array(None);
    }
  }
}