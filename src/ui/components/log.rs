use clipboard::{ClipboardContext, ClipboardProvider};
use egui::{Context, TextEdit};
use crate::do_log;
use crate::settings::{configuration, kuplung_logger};

#[derive(Default)]
pub struct ComponentLog {
  buffer_filter: String,
}

impl ComponentLog {
  pub fn new() -> Self {
    do_log!("[Kuplung] [UI] [Component] Initializing Log...");

    let this = Self {
      buffer_filter: "".to_string(),
    };
    do_log!("[Kuplung] [UI] [Component] Log initialized.");
    this
  }

  pub fn render_component_log(&mut self, ctx: &Context) {
    let screen_rect = ctx.screen_rect();
    let posx: f32 = screen_rect.size().x / 2.0 - configuration::COMPONENT_LOG_WIDTH / 2.0;
    let posy: f32 = screen_rect.size().y - (configuration::COMPONENT_LOG_HEIGHT + 80.0);
    egui::Window::new("Log")
      .id(egui::Id::new("component_log"))
      .resizable(true)
      .enabled(true)
      .default_size([configuration::COMPONENT_LOG_WIDTH, configuration::COMPONENT_LOG_HEIGHT + 10.0])
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
            egui::ScrollArea::vertical()
              .id_source("log_buffer_output")
              .auto_shrink([false; 2])
              .stick_to_bottom(true)
              .enable_scrolling(true)
              .show(ui, |ui| {
                ui.add_sized(ui.available_size(), TextEdit::multiline(&mut kuplung_logger::get_log()).cursor_at_end(true));
              });
          });
      });
  }

  fn copy_log_text(&mut self) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(kuplung_logger::get_log().to_string()).unwrap();
  }

  fn clear_log_text(&mut self) { kuplung_logger::clear_log(); }

  fn filter_log(&mut self) {}
}