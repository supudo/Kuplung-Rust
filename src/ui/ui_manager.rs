use egui::{Modifiers, Ui};
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
  dark_mode: bool,
  show_backend: bool,
  panel_backend: panel_backend::PanelBackend
}

impl UIManager {
  pub fn new() -> Self {
    info!("[Kuplung] [UI] Initializing UI...");
    let this = Self {
      dark_mode: false,
      show_backend: false,
      panel_backend: panel_backend::PanelBackend::default()
    };
    info!("[Kuplung] [UI] UI initialized.");
    this
  }

  pub fn render(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      let mut cmd = Command::Nothing;

      // egui backend panel
      self.panel_backend.update(ctx, frame);
      cmd = self.panel_backend_show(ctx, frame);
      self.panel_backend.end_of_frame(ctx);

      egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
          // main menu
          self.show_main_menu(ui);
        });
      });

      self.run_cmd(ctx, cmd);
    });
  }

  fn run_cmd(&mut self, ctx: &egui::Context, cmd: Command) {
    match cmd {
      Command::Nothing => {}
      Command::ResetEverything => {
        ctx.memory_mut(|mem| *mem = Default::default());
      }
    }
  }

  fn show_main_menu(&mut self, ui: &mut Ui) {
    // shortcuts
    let shortcut_quit = egui::KeyboardShortcut::new(Modifiers::NONE, egui::Key::Escape);
    let shortcut_new = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::N);
    let shortcut_backend = egui::KeyboardShortcut::new(Modifiers::SHIFT | Modifiers::CTRL | Modifiers::ALT, egui::Key::B);

    if ui.input_mut(|i| i.consume_shortcut(&shortcut_quit)) { std::process::exit(0); }
    if ui.input_mut(|i| i.consume_shortcut(&shortcut_new)) { todo!() }
    if ui.input_mut(|i| i.consume_shortcut(&shortcut_backend)) { self.toggle_backend(ui); }

    // main menu
    egui::menu::bar(ui, |ui| {
      ui.menu_button("File", |ui| {
        if ui.add(egui::Button::new("New").shortcut_text(ui.ctx().format_shortcut(&shortcut_new))).on_hover_text("New scene").clicked() {
          todo!()
        }
        if ui.button("Open").on_hover_text("Open existing scene").clicked() {
        }
        if ui.button("Open Recent").on_hover_text("Open recent scene").clicked() {
        }
        if ui.button("Save...").on_hover_text("Save scene to a file").clicked() {
        }
        ui.separator();
        if ui.add(egui::Button::new("Quit").shortcut_text(ui.ctx().format_shortcut(&shortcut_quit)), ).clicked() {
          ui.close_menu();
          std::process::exit(0);
        }
      });
      ui.separator();
      ui.menu_button("Help", |ui| {
        if ui.button("Metrics").on_hover_text("Show scene stats").clicked() {
        }
        if ui.add(egui::Button::new("Backend").shortcut_text(ui.ctx().format_shortcut(&shortcut_backend))).on_hover_text("View egui backend").clicked() { self.toggle_backend(ui); }
        if ui.button("About Kuplung").clicked() {
        }
      });
      ui.separator();
      self.show_theme(ui);
    });
  }

  fn panel_backend_show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Command {
    let mut cmd = Command::Nothing;
    egui::SidePanel::left("backend_panel")
      .resizable(false)
      .show_animated(ctx, self.show_backend, |ui| {
        ui.vertical_centered(|ui| {
          ui.heading("💻 Backend");
        });
        ui.separator();
        self.panel_contents_backend(ui, frame, &mut cmd);
      });
    cmd
  }

  fn panel_contents_backend(&mut self, ui: &mut Ui, frame: &mut eframe::Frame, cmd: &mut Command) {
    self.panel_backend.ui(ui, frame);
    ui.separator();
    ui.horizontal(|ui| {
      if ui.button("Reset egui").on_hover_text("Forget scroll, positions, sizes etc").clicked() {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
        ui.close_menu();
      }
      if ui.button("Reset everything").clicked() {
        *cmd = Command::ResetEverything;
        ui.close_menu();
      }
    });
  }

  fn show_theme(&mut self, ui: &mut Ui) {
    #![allow(clippy::collapsible_else_if)]
    if self.dark_mode {
      if ui.button("☀").on_hover_text("Switch to light mode").clicked() {
        self.dark_mode = false;
        ui.ctx().set_visuals(egui::Visuals::light());
      }
    }
    else {
      if ui.button("🌙").on_hover_text("Switch to dark mode").clicked() {
        self.dark_mode = true;
        ui.ctx().set_visuals(egui::Visuals::dark());
      }
    }
  }

  fn toggle_backend(&mut self, ui: &mut Ui) {
    self.show_backend = !self.show_backend;
    ui.close_menu();
  }
}