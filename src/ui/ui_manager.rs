use egui::{Context, Modifiers, Ui};
use log::info;
use crate::ui::dialogs::options;
use crate::ui::panel_backend;
use crate::ui::components::log::ComponentLog;

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
  panel_backend: panel_backend::PanelBackend,
  show_options: bool,
  show_about: bool,
  show_component_log: bool,
  component_log: ComponentLog
}

impl UIManager {
  pub fn new() -> Self {
    info!("[Kuplung] [UI] Initializing UI...");
    let this = Self {
      dark_mode: false,
      show_backend: false,
      panel_backend: panel_backend::PanelBackend::default(),
      show_options: false,
      show_about: false,
      show_component_log: true,
      component_log: ComponentLog::new()
    };
    info!("[Kuplung] [UI] UI initialized.");
    this
  }

  pub fn render(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |_| {
      let mut cmd = Command::Nothing;

      // main menu
      egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
          // main menu
          self.show_main_menu(ui);
        });
      });

      // egui backend panel
      self.panel_backend.update(ctx);
      cmd = self.panel_backend_show(ctx, frame);
      self.panel_backend.end_of_frame(ctx);

      if self.show_options { self.render_options(ctx); }
      if self.show_component_log { self.render_component_log(ctx); }
      if self.show_about { self.render_about(ctx); }

      self.run_cmd(ctx, cmd);
    });
  }

  pub fn log_info(&mut self, message: &str) {
    self.component_log.log_info(message);
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
    let shortcut_open = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::O);
    let shortcut_save = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::S);
    let shortcut_backend = egui::KeyboardShortcut::new(Modifiers::SHIFT | Modifiers::CTRL | Modifiers::ALT, egui::Key::B);
    let shortcut_about = egui::KeyboardShortcut::new(Modifiers::NONE, egui::Key::F1);

    if ui.input_mut(|i| i.consume_shortcut(&shortcut_quit)) { std::process::exit(0); }
    if ui.input_mut(|i| i.consume_shortcut(&shortcut_new)) { self.toggle_dialog_new(ui); }
    if ui.input_mut(|i| i.consume_shortcut(&shortcut_open)) { self.toggle_dialog_open(ui); }
    if ui.input_mut(|i| i.consume_shortcut(&shortcut_save)) { self.toggle_dialog_save(ui); }
    if ui.input_mut(|i| i.consume_shortcut(&shortcut_backend)) { self.toggle_backend(ui); }
    if ui.input_mut(|i| i.consume_shortcut(&shortcut_about)) { self.toggle_about(ui); }

    // main menu
    egui::menu::bar(ui, |ui| {
      ui.menu_button("File", |ui| {
        if ui.add(egui::Button::new("🗋 New").shortcut_text(ui.ctx().format_shortcut(&shortcut_new))).on_hover_text("New scene").clicked() {
          self.toggle_dialog_new(ui);
        }
        if ui.add(egui::Button::new("🗁 Open").shortcut_text(ui.ctx().format_shortcut(&shortcut_open))).on_hover_text("Open existing scene").clicked() {
          self.toggle_dialog_open(ui);
        }
        if ui.button("🗐 Open Recent").on_hover_text("Open recent scene").clicked() {
        }
        if ui.add(egui::Button::new("🖴 Save").shortcut_text(ui.ctx().format_shortcut(&shortcut_save))).on_hover_text("New Save scene to a file").clicked() {
          self.toggle_dialog_save(ui);
        }
        ui.separator();
        if ui.add(egui::Button::new("🗙 Quit").shortcut_text(ui.ctx().format_shortcut(&shortcut_quit)), ).clicked() { self.exit_kuplung(ui); }
      });
      ui.separator();
      ui.menu_button("Help", |ui| {
        if ui.button("📏 Metrics").on_hover_text("Show scene stats").clicked() {
        }
        if ui.add(egui::Button::new("📺 Backend").shortcut_text(ui.ctx().format_shortcut(&shortcut_backend))).on_hover_text("View egui backend").clicked() { self.toggle_backend(ui); }
        if ui.button("🛠 Options").on_hover_text("Configure Kuplung options").clicked() { self.toggle_options(ui); }
        ui.separator();
        if ui.button("🖹 Log").on_hover_text("Toggle log window").clicked() { self.toggle_component_log(ui); }
        ui.separator();
        if ui.button("About Kuplung").clicked() { self.toggle_about(ui); }
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

  fn toggle_dialog_new(&mut self, ui: &mut Ui) {
    ui.close_menu();
  }

  fn toggle_dialog_open(&mut self, ui: &mut Ui) {
    ui.close_menu();
  }

  fn toggle_dialog_save(&mut self, ui: &mut Ui) {
    ui.close_menu();
  }

  fn toggle_options(&mut self, ui: &mut Ui) {
    ui.close_menu();
    self.show_options = !self.show_options;
  }

  fn toggle_component_log(&mut self, ui: &mut Ui) {
    ui.close_menu();
    self.show_component_log = !self.show_component_log;
  }

  fn toggle_about(&mut self, ui: &mut Ui) {
    ui.close_menu();
    self.show_about = !self.show_about;
  }

  fn render_options(&mut self, ctx: &Context) {
    options::render_dialog_options(ctx);
  }

  fn render_component_log(&mut self, ctx: &Context) {
    self.component_log.render_component_log(ctx);
  }

  fn render_about(&mut self, ctx: &Context) {
    egui::Window::new("About Kuplung")
      .id(egui::Id::new("about_kuplung"))
      .title_bar(false)
      .auto_sized()
      .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
      .max_width(400.0)
      .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
          ui.label("Kuplung 1.0");
          ui.separator();
          ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("By ");
            ui.hyperlink_to(
              "supudo.net",
              "https://supudo.net",
            );
            ui.label(" + ");
            ui.hyperlink_to(
              "github.com/supudo",
              "https://github.com/supudo/Kuplung-Rust",
            );
            ui.label(".");
          });
          ui.label("Whatever license...");
          ui.separator();
          ui.label("Hold mouse wheel to rotate around");
          ui.label("Left Alt + Mouse wheel to increase/decrease the FOV");
          ui.label("Left Shift + Mouse wheel to increase/decrease the FOV");
          if ui.button("Close").clicked() { self.show_about = false; }
        });
      });
  }

  fn exit_kuplung(&mut self, ui: &mut Ui) {
    ui.close_menu();
    std::process::exit(0);
  }
}