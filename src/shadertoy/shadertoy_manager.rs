use std::sync::Arc;

use eframe::egui_glow;
use egui::mutex::Mutex;
use egui::Ui;
use egui_glow::glow;

use crate::shadertoy::shadertoy_engine::ShaderToyEngine;
use crate::do_log;
use crate::settings::{configuration, kuplung_logger};

pub struct ShaderToy {
  pub show_shadertoy: bool,
  current_toy: String,
  shader_toy_engine: Arc<Mutex<ShaderToyEngine>>
}

impl ShaderToy {
  pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
    do_log!("[Kuplung] [ShaderToy] Initializing...");

    let gl = cc.gl.as_ref()?;

    let func_main = "void mainImage(out vec4 fragColor, in vec2 fragCoord)\n
{\n
   vec2 uv = fragCoord.xy / iResolution.xy;\n
   fragColor = vec4(uv, 0.5 + 0.5 * sin(iGlobalTime), 1.0);\n
}\n\n".to_owned();

    let this = Self {
      show_shadertoy: false,
      current_toy: "".to_string(),
      shader_toy_engine: Arc::new(Mutex::new(ShaderToyEngine::new(gl, func_main)?))
    };
    do_log!("[Kuplung] [ShaderToy] Initialized.");
    Some(this)
  }

  fn render_toy(&mut self, ui: &mut Ui) {
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
      let window_width: f32 = ui.available_width();
      let window_height: f32 = ui.available_height();
      let (rect, _) = ui.allocate_exact_size(egui::Vec2::from([window_width, window_height]), egui::Sense::click_and_drag());
      let shader_toy_engine = self.shader_toy_engine.clone();
      let current_toy = self.current_toy.to_string();
      let cb = egui_glow::CallbackFn::new(move |_, painter| {
        shader_toy_engine.lock().setup_fbo(painter.gl(), window_width, window_height);
        shader_toy_engine.lock().paint(painter.gl(), &current_toy, window_width, window_height);
      });
      let callback = egui::PaintCallback {
        rect,
        callback: Arc::new(cb),
      };
      ui.painter().add(callback);
    });
  }
}

impl eframe::App for ShaderToy {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    self.show_shadertoy = true;
    egui::Window::new("ShaderToy")
      .id(egui::Id::new("window_shadertoy"))
      .resizable(true)
      .enabled(true)
      .default_pos([80.0, 80.0])
      .min_size([configuration::WINDOW_WIDTH_SHADERTOY, configuration::WINDOW_HEIGHT_SHADERTOY])
      .default_size([configuration::WINDOW_WIDTH_SHADERTOY, configuration::WINDOW_HEIGHT_SHADERTOY])
      .show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
          if ui.button("Close").clicked() {
            ui.close_menu();
            self.show_shadertoy = false;
          }
          ui.menu_button("Toys", |ui| {
            if ui.button("Artificial").clicked() {
              self.current_toy = "4ljGW1".to_string();
              ui.close_menu();
            }
            if ui.button("Combustible Voronoi Layers").clicked() {
              self.current_toy = "4tlSzl".to_string();
              ui.close_menu();
            }
          });
        });
        ui.separator();
        self.render_toy(ui);
      });
  }

  fn on_exit(&mut self, gl: Option<&glow::Context>) {
    if let Some(gl) = gl {
      self.shader_toy_engine.lock().destroy(gl);
    }
  }
}