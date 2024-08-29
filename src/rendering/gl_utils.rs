use std::ffi::CString;
use std::io::Read;
use eframe::glow;
use eframe::glow::HasContext;
use egui::TextBuffer;
use crate::do_log;
use crate::settings::kuplung_logger;

pub unsafe fn create_shader(program: &glow::Program, gl: &glow::Context, shader_type: u32, shader_filepath: &str) -> glow::Shader {
  do_log!("[Kuplung] [GLUtils] Loading shader file {}", shader_filepath);

  let mut shader_file = std::fs::File::open(shader_filepath).expect(format!("[Kuplung] [GLUtils] Cannot find the shader file specified = {}!", stringify!(shader_filepath)).as_str());
  let mut shader_buffer: Vec<u8> = Vec::new();
  shader_file.read_to_end(&mut shader_buffer).unwrap();
  let shader_source = CString::new(shader_buffer).unwrap();

  let shader = gl.create_shader(shader_type).expect("[Kuplung] [GLUtils] Cannot create shader");
  gl.shader_source(shader, shader_source.to_str().unwrap());
  gl.compile_shader(shader);
  if !gl.get_shader_compile_status(shader) {
    do_log!("[Kuplung] [GLUtils] Failed to compile shader {shader_filepath} {shader_type}: {}", gl.get_shader_info_log(shader));
  }
  gl.attach_shader(*program, shader);
  shader
}

pub unsafe fn create_shader_from_string(program: &glow::Program, gl: &glow::Context, shader_type: u32, shader_source: &str) -> glow::Shader {
  do_log!("[Kuplung] [GLUtils] Loading shader source.");

  let shader = gl.create_shader(shader_type).expect("[Kuplung] [GLUtils] Cannot create shader");
  gl.shader_source(shader, shader_source);
  gl.compile_shader(shader);
  if !gl.get_shader_compile_status(shader) {
    do_log!("[Kuplung] [GLUtils] Failed to compile shader from source {shader_type}: {}", gl.get_shader_info_log(shader));
  }
  gl.attach_shader(*program, shader);
  shader
}

pub unsafe fn get_uniform(program: &glow::Program, gl: &glow::Context, var_name: &str) -> glow::UniformLocation {
  let ul = gl.get_uniform_location(*program, var_name);
  if ul.is_none() {
    do_log!("[Kuplung] [GLUtils] Cannot get uniform for {}. {}", var_name.as_str(), gl.get_program_info_log(*program));
  }
  ul.expect(format!("[Kuplung] [GLUtils] Cannot get uniform for {}. {}", var_name.as_str(), gl.get_program_info_log(*program)).as_str())
}

pub unsafe fn get_uniform_no_warning(program: &glow::Program, gl: &glow::Context, uniform_name: &str) -> glow::UniformLocation {
  let ul = gl.get_uniform_location(*program, uniform_name);
  if ul.is_none() {
    do_log!("[Kuplung] [GLUtils] Cannot get uniform for {}. {}", uniform_name.as_str(), gl.get_program_info_log(*program));
  }
  ul.unwrap()
}

pub unsafe fn print_program_uniforms(program: &glow::Program, gl: &glow::Context) {
  let attr_count = gl.get_active_attributes(*program);
  let unif_count = gl.get_active_uniforms(*program);
  do_log!("[Kuplung] [STOY] {} - {}", attr_count, unif_count);
  for i in 0..unif_count {
    let unif = gl.get_active_uniform(*program, i).unwrap();
    do_log!("[Kuplung] [STOY] {} - {} : {}", unif.name, unif.utype, unif.size);
  }
}