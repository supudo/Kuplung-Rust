// https://aimlesslygoingforward.com/blog/2016/09/27/mandelbrot-using-shaders-rust/

use eframe::egui_glow;
use eframe::glow::HasContext;
use egui_glow::glow;

use log::warn;
use crate::rendering::gl_utils;
use crate::settings::configuration;

#[rustfmt::skip]
pub static VERTEX_DATA: [f32; 30] = [
    -1.0,  1.0,  1.0,  0.0,  0.0,
     1.0,  1.0,  0.0,  1.0,  0.0,
    -1.0, -1.0,  0.0,  0.0,  1.0,
    -1.0, -1.0,  1.0,  0.0,  0.0,
     1.0,  1.0,  0.0,  1.0,  0.0,
     1.0, -1.0,  0.0,  0.0,  1.0,
];

pub struct Mandelbrot {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    vertex_buffer: glow::Buffer
}

#[allow(unsafe_code)]
impl Mandelbrot {
    pub fn new(gl: &glow::Context) -> Option<Self> {
        use glow::HasContext as _;
        let shader_version = egui_glow::ShaderVersion::get(gl);
        unsafe {
            let program = gl.create_program().expect("[Kuplung] [Mandelbrot] Cannot create program!");

            if !shader_version.is_new_shader_interface() {
                warn!("[Kuplung] [Mandelbrot] Mandelbrot hasn't been tested on {:?}.", shader_version);
                return None;
            }

            let shader_vertex = gl_utils::create_shader(&program, &gl, shader_version, glow::VERTEX_SHADER, "assets/shaders/fractal_mandelbrot.vert");
            let shader_fragment = gl_utils::create_shader(&program, &gl, shader_version, glow::FRAGMENT_SHADER, "assets/shaders/fractal_mandelbrot.frag");

            gl.link_program(program);
            assert!(gl.get_program_link_status(program), "{}", gl.get_program_info_log(program));

            gl.detach_shader(program, shader_vertex);
            gl.delete_shader(shader_vertex);
            gl.detach_shader(program, shader_fragment);
            gl.delete_shader(shader_fragment);

            let vertex_buffer = gl.create_buffer().expect("[Kuplung] [Mandelbrot] Cannot create vertex buffer!");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER,  core::slice::from_raw_parts(VERTEX_DATA.as_ptr() as *const u8, VERTEX_DATA.len() * size_of::<f32>()), glow::STATIC_DRAW);

            let vertex_array = gl.create_vertex_array().expect("[Kuplung] [Mandelbrot] Cannot create vertex array!");
            gl.bind_vertex_array(Some(vertex_array));

            let attrib_position = gl.get_attrib_location(program, "vs_position");
            gl.vertex_attrib_pointer_f32(attrib_position.unwrap(), 2, glow::FLOAT, false, 2, 0);
            gl.enable_vertex_attrib_array(attrib_position.unwrap());

            let attrib_color = gl.get_attrib_location(program, "vs_color");
            gl.vertex_attrib_pointer_f32(attrib_color.unwrap(), 3, glow::FLOAT, false, 3, 0);
            gl.enable_vertex_attrib_array(attrib_color.unwrap());

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
        unsafe {
            gl.use_program(Some(self.program));
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer));

            gl.uniform_1_f32(gl.get_uniform_location(self.program, "vs_angle").as_ref(), angle);
            /*gl.uniform_1_f32(gl.get_uniform_location(self.program, "vs_position").as_ref(), angle);
            gl.uniform_1_f32(gl.get_uniform_location(self.program, "vs_color").as_ref(), angle);*/

            gl.clear_color(configuration::GL_CLEAR_COLOR_R, configuration::GL_CLEAR_COLOR_G, configuration::GL_CLEAR_COLOR_B, configuration::GL_CLEAR_COLOR_A);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}