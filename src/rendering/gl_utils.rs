use std::ffi::CString;
use std::io::Read;
use eframe::egui_glow::ShaderVersion;
use eframe::glow;
use eframe::glow::HasContext;
use egui::TextBuffer;
use log::info;

pub unsafe fn create_shader(program: &glow::Program, gl: &glow::Context, shader_version: ShaderVersion, shader_type: u32, shader_filepath: &str) -> glow::Shader {
  info!("[Kuplung] Loading shader file {}", shader_filepath);

  let mut shader_file = std::fs::File::open(shader_filepath).expect(format!("[Kuplung] [GLUtils] Cannot find the shader file specified = {}!", stringify!(shader_filepath)).as_str());
  let mut shader_buffer: Vec<u8> = Vec::new();
  shader_file.read_to_end(&mut shader_buffer).unwrap();
  let shader_source = CString::new(shader_buffer).unwrap();

  let shader = gl.create_shader(shader_type).expect("[Kuplung] [GLUtils] Cannot create shader");
  //gl.shader_source(shader, shader_source.to_str().unwrap());
  gl.shader_source(shader, &format!("{}\n{}", shader_version.version_declaration(), shader_source.to_str().unwrap()));
  gl.compile_shader(shader);
  assert!(gl.get_shader_compile_status(shader), "[Kuplung] [GLUtils] Failed to compile shader {shader_type}: {}", gl.get_shader_info_log(shader));
  gl.attach_shader(*program, shader);
  shader
}