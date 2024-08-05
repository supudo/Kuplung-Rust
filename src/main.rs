mod kuplung;
mod settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  kuplung::app::main(winit::event_loop::EventLoop::new().unwrap())
}