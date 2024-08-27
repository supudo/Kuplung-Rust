use eframe::{glow, HardwareAcceleration, Renderer, Theme};
use eframe::egui_glow::ShaderVersion;
use eframe::epaint::text::FontData;
use egui::ViewportBuilder;
use env_logger::Env;
use crate::fractals::fractals_manager;
use crate::do_log;
use crate::rendering::rendering_manager;
use crate::shadertoy::shadertoy_manager;
use crate::settings::{configuration, kuplung_logger};
use crate::ui::ui_manager;

pub fn main() -> eframe::Result {
  let env = Env::default()
    .filter_or(configuration::KUPLUNG_LOG_LEVEL, configuration::KUPLUNG_LOG_LEVEL_VALUE)
    .write_style_or(configuration::KUPLUNG_LOG_STYLE, configuration::KUPLUNG_LOG_STYLE_VALUE);
  env_logger::init_from_env(env);

  do_log!("[Kuplung] Initializing Kuplung...");

  let icon = include_bytes!(concat!(env!("OUT_DIR"), "/assets/Kuplung.png"));
  let image = image::load_from_memory(icon).expect("[Kuplung] Failed to open icon path!").to_rgba8();
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
  do_log!("[Kuplung] Window initialized.");
  egui_result
}

#[derive(Default)]
pub struct KuplungApp {
  manager_ui: ui_manager::UIManager,
  manager_rendering: Option<rendering_manager::RenderingManager>,
  manager_fractals: Option<fractals_manager::FractalsManager>,
  manager_shadertoy: Option<shadertoy_manager::ShaderToy>,
}

impl KuplungApp {
  fn new(cc: &eframe::CreationContext<'_>) -> Self {
    // set light mode as initial theme
    cc.egui_ctx.set_visuals(egui::Visuals::light());

    // load fonts
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("font_awesome".to_owned(), FontData::from_static(include_bytes!(concat!(env!("OUT_DIR"), "/assets/fonts/fontawesome-webfont.ttf"))));
    fonts.font_data.insert("font_material".to_owned(), FontData::from_static(include_bytes!(concat!(env!("OUT_DIR"), "/assets/fonts/material-icons-regular.ttf"))));
    cc.egui_ctx.set_fonts(fonts);

    // initialize sub-systems
    let manager_ui = ui_manager::UIManager::new();
    let manager_rendering = rendering_manager::RenderingManager::new(cc);
    let manager_fractals = fractals_manager::FractalsManager::new(cc);
    let manager_shadertoy = shadertoy_manager::ShaderToy::new(cc);
    let this = Self {
      manager_ui,
      manager_rendering,
      manager_fractals,
      manager_shadertoy,
    };

    do_log!("[Kuplung] egui initialized.");
    this
  }
}

impl eframe::App for KuplungApp {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    self.manager_ui.render(ctx, frame);
    if self.manager_ui.show_viewer { self.manager_rendering.as_mut().unwrap().update(ctx, frame); }
    if self.manager_ui.show_fractals {
      self.manager_fractals.as_mut().unwrap().update(ctx, frame);
      if !self.manager_fractals.as_mut().unwrap().show_fractals {
        self.manager_ui.show_fractals = false;
      }
    }
    if self.manager_ui.show_shadertoy {
      self.manager_shadertoy.as_mut().unwrap().update(ctx, frame);
      if !self.manager_shadertoy.as_mut().unwrap().show_shadertoy {
        self.manager_ui.show_shadertoy = false;
      }
    }
  }

  fn on_exit(&mut self, gl: Option<&glow::Context>) {
    self.manager_ui.on_exit();
    if let Some(manager_rendering) = &mut self.manager_rendering {
      manager_rendering.on_exit(gl);
    }
    if let Some(manager_fractals) = &mut self.manager_fractals {
      manager_fractals.on_exit(gl);
    }
    if let Some(manager_shadertoy) = &mut self.manager_shadertoy {
      manager_shadertoy.on_exit(gl);
    }
  }
}
