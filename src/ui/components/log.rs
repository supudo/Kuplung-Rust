use clipboard::{ClipboardContext, ClipboardProvider};
use egui::{Context, TextEdit};
use log::info;
use crate::settings::configuration;

#[derive(Default)]
pub struct ComponentLog {
  buffer_filter: String,
  buffer_log: String,
}

impl ComponentLog {
  pub fn new() -> Self {
    info!("[Kuplung] [UI] [Component] Initializing Log...");

    let this = Self {
      buffer_filter: "".to_string(),
      buffer_log: "".to_string(),
    };
    info!("[Kuplung] [UI] [Component] Log initialized.");
    this
  }

  pub fn log_info(&mut self, message: &str) {
    if self.buffer_log.is_empty() {
      self.buffer_log = message.to_string();
    }
    else {
      self.buffer_log = format!("{}\n{}", self.buffer_log.to_string(), message);
    }
  }

  pub fn render_component_log(&mut self, ctx: &Context) {
    let screen_rect = ctx.screen_rect();
    let posx: f32 = screen_rect.size().x / 2.0 - configuration::COMPONENT_LOG_WIDTH / 2.0;
    let posy: f32 = screen_rect.size().y - (configuration::COMPONENT_LOG_HEIGHT + 80.0);
    egui::Window::new("Log")
      .id(egui::Id::new("component_log"))
      .resizable(true)
      .enabled(true)
      .default_size([configuration::COMPONENT_LOG_WIDTH, configuration::COMPONENT_LOG_HEIGHT])
      .default_pos([posx, posy])
      .show(ctx, |ui| {
        ui.horizontal(|ui| {
          if ui.button("Clear").on_hover_text("Clear log").clicked() { self.clear_log_text(); }
          if ui.button("Copy").on_hover_text("Copy log to clipboard").clicked() { self.copy_log_text(); }
          ui.separator();
          ui.text_edit_singleline(&mut self.buffer_filter);
          if ui.button("Filter").on_hover_text("Search log").clicked() { self.filter_log(); }
        });
        ui.separator();
        ui.label("Log messages:");
        egui::Frame::default()
          .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
          .rounding(ui.visuals().widgets.noninteractive.rounding)
          .show(ui, |ui| {
            ui.add_sized(ui.available_size(), TextEdit::multiline(&mut self.buffer_log));
          });
      });
  }

  fn copy_log_text(&mut self) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(self.buffer_log.to_string()).unwrap();
  }

  fn clear_log_text(&mut self) { self.buffer_log = "".to_string(); }

  fn filter_log(&mut self) {}
}