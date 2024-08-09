use glutin::display::Display;

use crate::rendering::triangle::Renderer;

pub fn initialize(gl_display: Display) -> Renderer {
  Renderer::new(&gl_display)
}