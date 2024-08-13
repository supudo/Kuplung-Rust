use log::info;
use crate::ui::panel_backend;

#[derive(Clone, Copy, Debug)]
#[must_use]
enum Command {
  Nothing,
  ResetEverything,
}

#[derive(Default)]
pub struct UIManager {
  show_backend: bool,
  panel_backend: panel_backend::PanelBackend
}

impl UIManager {
  pub fn new() -> Self {
    info!("[Kuplung] [UI] Initializing UI...");
    let this = Self {
      show_backend: false,
      panel_backend: panel_backend::PanelBackend::default()
    };
    info!("[Kuplung] [UI] UI initialized.");
    this
  }

  pub fn render(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          if ui.button("New").clicked() {
          }
          if ui.button("Open").clicked() {
          }
          if ui.button("Open Recent").clicked() {
          }
          if ui.button("Save...").clicked() {
          }
          ui.separator();
          if ui.button("Quit").clicked() {
            std::process::exit(0);
          }
        });
        ui.separator();
        ui.menu_button("Help", |ui| {
          if ui.button("Metrics").clicked() {
          }
          if ui.button("Backend").clicked() {
            self.show_backend = true;
          }
          if ui.button("About Kuplung").clicked() {
          }
        });
      });

      let mut cmd = Command::Nothing;
      self.panel_backend.update(ctx, frame);
      //cmd = self.backend_panel(ctx, frame);
      self.run_cmd(ctx, cmd);
    });
  }

  /*fn panel_show_backend(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Command {
    let mut cmd = Command::Nothing;
    egui::SidePanel::left("backend_panel")
      .resizable(false)
      .show_animated(ctx, self.show_backend, |ui| {
        ui.vertical_centered(|ui| {
          ui.heading("💻 Backend");
        });

        ui.separator();
        self.backend_panel_contents(ui, frame, &mut cmd);
      });
  }*/

  fn run_cmd(&mut self, ctx: &egui::Context, cmd: Command) {
    match cmd {
      Command::Nothing => {}
      Command::ResetEverything => {
        ctx.memory_mut(|mem| *mem = Default::default());
      }
    }
  }
}