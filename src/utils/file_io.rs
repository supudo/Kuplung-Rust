use std::error::Error;

pub fn read_shadertoy_shader(stoy: &str) -> Result<String, Box<dyn Error>> {
  let shader_source: String = std::fs::read_to_string(format!("assets/shaders/shadertoy/{}", stoy))?;
  Ok(shader_source)
}