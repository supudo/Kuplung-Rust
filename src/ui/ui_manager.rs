use std::ops::Deref;
use imgui::ConfigFlags;
use imgui_winit_support::WinitPlatform;
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

    //let mut winit_platform = WinitPlatform::init(&mut self.imgui_context);
    //winit_platform.attach_window(self.imgui_context.io_mut(), window, imgui_winit_support::HiDpiMode::Rounded);
  }

  pub fn render_ui(&mut self) {
    //let ui = self.imgui_context.frame();
    //ui.show_demo_window(&mut true);
  }
}