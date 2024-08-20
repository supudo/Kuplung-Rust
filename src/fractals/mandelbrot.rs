#![allow(non_snake_case)]

use eframe::egui_glow;
use eframe::glow::HasContext;
use egui_glow::glow;
use log::error;
use crate::rendering::gl_utils;
use crate::settings::configuration;

#[rustfmt::skip]
pub static MANDELBROT_VERTICES:[f32; 12] = [
  // Top-left corner
  -1.0,  1.0,
   1.0,  1.0,
  -1.0, -1.0,

  // Bottom-right corner
  -1.0, -1.0,
   1.0,  1.0,
   1.0, -1.0,
];

pub struct Mandelbrot {
  gl_Program: glow::Program,
  gl_VAO: glow::VertexArray,
  vbo_Vertices: glow::Buffer,
}

#[allow(unsafe_code)]
impl Mandelbrot {
  pub fn new(gl: &glow::Context) -> Option<Self> {
    use glow::HasContext as _;
    unsafe {
      let gl_Program = gl.create_program().expect("[Kuplung] [Mandelbrot] Cannot create program!");

      let shader_vertex = gl_utils::create_shader(&gl_Program, &gl, glow::VERTEX_SHADER, "assets/shaders/fractals/mandelbrot.vert");
      let shader_fragment = gl_utils::create_shader(&gl_Program, &gl, glow::FRAGMENT_SHADER, "assets/shaders/fractals/mandelbrot.frag");

      gl.link_program(gl_Program);
      if !gl.get_program_link_status(gl_Program) {
        error!("[Kuplung] [Mandelbrot] Program cannot be linked! {}", gl.get_program_info_log(gl_Program));
        panic!("[Kuplung] [Mandelbrot] Program cannot be linked! {}", gl.get_program_info_log(gl_Program));
      }

      gl.detach_shader(gl_Program, shader_vertex);
      gl.delete_shader(shader_vertex);
      gl.detach_shader(gl_Program, shader_fragment);
      gl.delete_shader(shader_fragment);

      let gl_VAO = gl.create_vertex_array().expect("[Kuplung] [Mandelbrot] Cannot create vertex array!");
      gl.bind_vertex_array(Some(gl_VAO));

      let vbo_Vertices = gl.create_buffer().expect("[Kuplung] [Mandelbrot] Cannot create vertex buffer!");
      gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo_Vertices));
      gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&MANDELBROT_VERTICES[..]), glow::STATIC_DRAW);

      gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, size_of::<f32>() as i32 * 6, 0);
      gl.enable_vertex_attrib_array(0);

      gl.bind_vertex_array(None);

      Some(Self {
        gl_Program,
        gl_VAO,
        vbo_Vertices,
      })
    }
  }

  pub fn destroy(&self, gl: &glow::Context) {
    use glow::HasContext as _;
    unsafe {
      gl.delete_program(self.gl_Program);
      gl.delete_vertex_array(self.gl_VAO);
      gl.delete_buffer(self.vbo_Vertices);
    }
  }

  pub fn paint(&self, gl: &glow::Context, screen_width: f32, screen_height: f32) {
    unsafe {
      gl.clear_color(configuration::GL_CLEAR_COLOR_R, configuration::GL_CLEAR_COLOR_G, configuration::GL_CLEAR_COLOR_B, configuration::GL_CLEAR_COLOR_A);
      gl.clear(glow::COLOR_BUFFER_BIT);

      gl.use_program(Some(self.gl_Program));
      gl.bind_vertex_array(Some(self.gl_VAO));

      gl.uniform_1_f32(gl.get_uniform_location(self.gl_Program, "u_window_width").as_ref(), screen_width);
      gl.uniform_1_f32(gl.get_uniform_location(self.gl_Program, "u_window_height").as_ref(), screen_height);

      gl.draw_arrays(glow::TRIANGLES, 0, 3);

      gl.bind_vertex_array(None);
    }
  }
}