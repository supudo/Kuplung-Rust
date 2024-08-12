use std::time::Instant;
use glutin::context::PossiblyCurrentContext;
use imgui::ConfigFlags;
use imgui_winit_support::WinitPlatform;

use log::info;
use winit::window::Window;
use crate::ui::imgui_renderer::renderers::AutoRenderer;

pub struct UIManager {
  last_frame: Instant,
  pub imgui_context: imgui::Context,
  renderer: Option<AutoRenderer>
}

impl UIManager {
  pub fn new() -> Self {
    info!("[Kuplung] [UI] Initializing ImGui...");
    let this = Self {
      last_frame: Instant::now(),
      imgui_context: imgui::Context::create(),
      renderer: None
    };
    info!("[Kuplung] [UI] ImGui initialized.");
    this
  }

  pub fn configure_context(&mut self, window: &Window, gl_context: &PossiblyCurrentContext) -> WinitPlatform {
    self.imgui_context.set_ini_filename(None);
    self.imgui_context.io_mut().config_flags.insert(ConfigFlags::DOCKING_ENABLE);
    self.imgui_context.io_mut().config_flags.insert(ConfigFlags::VIEWPORTS_ENABLE);

    let mut winit_platform = WinitPlatform::init(&mut self.imgui_context);
    winit_platform.attach_window(self.imgui_context.io_mut(), window, imgui_winit_support::HiDpiMode::Rounded);

    self.imgui_context.fonts().add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
    self.imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    let gl = crate::kuplung::utils::glow_context(&gl_context);
    self.renderer.get_or_insert_with(|| AutoRenderer::initialize(gl, &mut self.imgui_context).expect("[Kuplung] [UI] failed to create renderer!"));

    info!("[Kuplung] [UI] ImGui context initialized.");

    winit_platform
  }

  pub fn render_start(&mut self) {
    let now = Instant::now();
    self.imgui_context.io_mut().update_delta_time(now.duration_since(self.last_frame));
    self.last_frame = now;
  }

  pub fn render_ui(&mut self, window: &Window, winit_platform: &mut WinitPlatform) {
    let ui = self.imgui_context.frame();
    ui.show_demo_window(&mut true);

    winit_platform.prepare_render(ui, &window);
    let draw_data = self.imgui_context.render();

    let i_rend = self.renderer.as_mut().unwrap();
    i_rend.render(draw_data).expect("[Kuplung] [UI] error rendering ImGui!");
  }
}