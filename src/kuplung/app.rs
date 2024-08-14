use eframe::{HardwareAcceleration, Renderer, Theme};
use eframe::egui_glow::ShaderVersion;
use egui::ViewportBuilder;
use log::info;

use crate::rendering::rendering_manager;
use crate::settings::configuration;
use crate::ui::ui_manager;

pub fn main() -> eframe::Result {
  let icon = include_bytes!(concat!(env!("OUT_DIR"), "/assets/Kuplung.png"));
  let image = image::load_from_memory(icon).expect("[Kuplung] Failed to open icon path~").to_rgba8();
  let (icon_width, icon_height) = image.dimensions();

  let egui_options = eframe::NativeOptions {
    depth_buffer: configuration::GL_DEPTH_SIZE,
    stencil_buffer: configuration::GL_STENCIL_SIZE,
    hardware_acceleration: if configuration::GL_HARDWARE_ACCELERATED { HardwareAcceleration::Preferred } else { HardwareAcceleration::Off },
    default_theme: Theme::Light,
    multisampling: configuration::GL_MULTISAMPLING,
    centered: true,
    renderer: Renderer::Glow,
    shader_version: Option::from(ShaderVersion::Gl140),
    viewport: ViewportBuilder::default()
      .with_inner_size([configuration::WINDOW_WIDTH, configuration::WINDOW_HEIGHT])
      .with_resizable(true)
      .with_visible(true)
      .with_icon(egui::IconData {
        rgba: image.into_raw(),
        width: icon_width,
        height: icon_height,
      }),
    ..Default::default()
  };
  let egui_result = eframe::run_native(configuration::APP_TITLE, egui_options, Box::new(|cc| Ok(Box::new(KuplungApp::new(cc)))));
  info!("[Kuplung] Window initialized.");
  egui_result
}

#[derive(Default)]
struct KuplungApp {
  manager_rendering: rendering_manager::RenderingManager,
  manager_ui: ui_manager::UIManager,
}

impl KuplungApp {
  fn new(cc: &eframe::CreationContext<'_>) -> Self {
    // set light mode as initial theme
    cc.egui_ctx.set_visuals(egui::Visuals::light());

    // initialize sub-systems
    let manager_ui = ui_manager::UIManager::new();
    let manager_rendering = rendering_manager::RenderingManager::new();
    let this = Self {
      manager_ui,
      manager_rendering
    };

    info!("[Kuplung] egui initialized.");
    this
  }
}

impl eframe::App for KuplungApp {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    self.manager_ui.render(ctx, frame);
  }
}
