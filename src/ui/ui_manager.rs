use imgui::ConfigFlags;
use log::info;
use winit::window::Window;

pub struct UIManager {
  pub imgui_context: imgui::Context
}

impl UIManager {
  pub fn new() -> Self {
    info!("[Kuplung] Initializing ImGui...");
    let this = Self {
      imgui_context: imgui::Context::create()
    };
    info!("[Kuplung] ImGui initialized.");
    this
  }

  pub fn configure_context(&mut self, window: &Window) {
    self.imgui_context.io_mut().config_flags.insert(ConfigFlags::DOCKING_ENABLE);
    self.imgui_context.io_mut().config_flags.insert(ConfigFlags::VIEWPORTS_ENABLE);
    self.imgui_context.set_ini_filename(None);
  }

  pub fn render_ui(&mut self) {
    let ui = self.imgui_context.frame();
    ui.show_demo_window(&mut true);
  }
}