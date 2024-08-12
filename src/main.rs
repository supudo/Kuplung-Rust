#![cfg_attr(
  all(
    not(debug_assertions),
  ),
  windows_subsystem = "windows"
)]

mod kuplung;
mod settings;
mod rendering;
mod ui;
mod imgui_renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  kuplung::app::main(winit::event_loop::EventLoop::new().unwrap())
}