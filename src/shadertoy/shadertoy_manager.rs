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
  previous_toy: String,
  current_toy: String,
  current_source: String,
  shader_toy_engine: Arc<Mutex<ShaderToyEngine>>
}

impl ShaderToy {
  pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
    do_log!("[Kuplung] [ShaderToy] Initializing...");

    let gl = cc.gl.as_ref()?;
    let this = Self {
      show_shadertoy: false,
      previous_toy: "".to_string(),
      current_toy: "".to_string(),
      current_source: "".into(),
      shader_toy_engine: Arc::new(Mutex::new(ShaderToyEngine::new(gl)?))
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
      let current_toy = self.current_toy.clone();
      let mut recompile = false;
      if self.current_toy.clone() != self.previous_toy { recompile = true; }
      let cb = egui_glow::CallbackFn::new(move |_, painter| {
        if recompile { shader_toy_engine.lock().reload_shadertoy(&current_toy, painter.gl()); }
        shader_toy_engine.lock().setup_fbo(painter.gl(), window_width, window_height);
        shader_toy_engine.lock().paint(painter.gl(), window_width, window_height);
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
            if ui.button("Seascape").clicked() {
              self.current_toy = "Ms2SD1".to_string();
              ui.close_menu();
            }
            if ui.button("Star Nest").clicked() {
              self.current_toy = "XlfGRj".to_string();
              ui.close_menu();
            }
            if ui.button("Sun Surface").clicked() {
              self.current_toy = "XlSSzK".to_string();
              ui.close_menu();
            }
          });
        });
        ui.separator();
        if ui.button("Compile").clicked() {
        }

        let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
        ui.collapsing("Theme", |ui| {
          ui.group(|ui| {
            theme.ui(ui);
            theme.clone().store_in_memory(ui.ctx());
          });
        });

        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
          let mut layout_job = egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "rs");
          layout_job.wrap.max_width = wrap_width;
          ui.fonts(|f| f.layout_job(layout_job))
        };

        let s_code = self.current_source.clone();
        egui::ScrollArea::vertical().show(ui, |ui| {
          ui.add(
            egui::TextEdit::multiline(&mut s_code)
              .font(egui::TextStyle::Monospace) // for cursor height
              .code_editor()
              .desired_rows(10)
              .lock_focus(true)
              .desired_width(f32::INFINITY)
              .layouter(&mut layouter),
          );
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