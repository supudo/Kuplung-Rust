use egui::Context;

pub fn render_dialog_options(ctx: &Context) {
  egui::Window::new("Options")
    .id(egui::Id::new("dialog_options"))
    .resizable(true)
    .enabled(true)
    .show(ctx, |ui| {
      ui.label("Hello World!");
    });
}