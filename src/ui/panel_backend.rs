/// How often we repaint the demo app by default
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode {
  Reactive,
  Continuous,
}

impl Default for RunMode {
  fn default() -> Self {
    Self::Reactive
  }
}

#[derive(Default)]
pub struct PanelBackend {
  pub open: bool,
  run_mode: RunMode,
  egui_windows: EguiWindows,
}

impl PanelBackend {
  pub fn update(&mut self, ctx: &egui::Context, frame: &eframe::Frame) {
    match self.run_mode {
      RunMode::Continuous => {
        ctx.request_repaint();
      }
      RunMode::Reactive => {
      }
    }
  }

  pub fn end_of_frame(&mut self, ctx: &egui::Context) {
    self.egui_windows.windows(ctx);
  }

  pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    integration_ui(ui, frame);
    ui.separator();
    self.run_mode_ui(ui);
    ui.separator();
    ui.label("egui windows:");
    self.egui_windows.checkboxes(ui);

    #[cfg(debug_assertions)]
    if ui.ctx().style().debug.debug_on_hover_with_all_modifiers {
      ui.separator();
      ui.label("Press down all modifiers and hover a widget to see a callstack for it");
    }

    #[cfg(target_arch = "wasm32")]
    {
      ui.separator();
      let mut screen_reader = ui.ctx().options(|o| o.screen_reader);
      ui.checkbox(&mut screen_reader, "🔈 Screen reader").on_hover_text("Experimental feature: checking this will turn on the screen reader on supported platforms");
      ui.ctx().options_mut(|o| o.screen_reader = screen_reader);
    }

    if cfg!(debug_assertions) && cfg!(target_arch = "wasm32") {
      ui.separator();
      #[allow(clippy::manual_assert)]
      if ui.button("panic!()").clicked() {
        panic!("intentional panic!");
      }
    }

    if !cfg!(target_arch = "wasm32") {
      ui.separator();
      if ui.button("Quit").clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
      }
    }
  }

  fn run_mode_ui(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      let run_mode = &mut self.run_mode;
      ui.label("Mode:");
      ui.radio_value(run_mode, RunMode::Reactive, "Reactive")
        .on_hover_text("Repaint when there are animations or input (e.g. mouse movement)");
      ui.radio_value(run_mode, RunMode::Continuous, "Continuous")
        .on_hover_text("Repaint everything each frame");
    });

    if self.run_mode == RunMode::Continuous {
      ui.label(format!(
        "Repainting the UI each frame.",
      ));
    } else {
      ui.label("Only running UI code when there are animations or input.");

      // Add a test for `request_repaint_after`, but only in debug
      // builds to keep the noise down in the official demo.
      if cfg!(debug_assertions) {
        ui.collapsing("More…", |ui| {
          ui.horizontal(|ui| {
            ui.label("Frame number:");
            ui.monospace(ui.ctx().frame_nr().to_string());
          });
          if ui
            .button("Wait 2s, then request repaint after another 3s")
            .clicked()
          {
            log::info!("Waiting 2s before requesting repaint…");
            let ctx = ui.ctx().clone();
            call_after_delay(std::time::Duration::from_secs(2), move || {
              log::info!("Request a repaint in 3s…");
              ctx.request_repaint_after(std::time::Duration::from_secs(3));
            });
          }
        });
      }
    }
  }
}

fn integration_ui(ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
  ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 0.0;
    ui.label("egui running inside ");
    ui.hyperlink_to(
      "eframe",
      "https://github.com/emilk/egui/tree/master/crates/eframe",
    );
    ui.label(".");
  });

  #[cfg(feature = "glow")]
  if _frame.gl().is_some() {
    ui.horizontal(|ui| {
      ui.label("Renderer:");
      ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
    });
  }

  #[cfg(not(target_arch = "wasm32"))]
  {
    ui.horizontal(|ui| {
      {
        let mut fullscreen = ui.input(|i| i.viewport().fullscreen.unwrap_or(false));
        if ui
          .checkbox(&mut fullscreen, "🗖 Fullscreen (F11)")
          .on_hover_text("Fullscreen the window")
          .changed()
        {
          ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen));
        }
      }

      let mut size = None;
      egui::ComboBox::from_id_source("viewport-size-combo")
        .selected_text("Resize to…")
        .show_ui(ui, |ui| {
          ui.selectable_value(
            &mut size,
            Some(egui::vec2(375.0, 667.0)),
            "📱 iPhone SE 2nd Gen",
          );
          ui.selectable_value(&mut size, Some(egui::vec2(393.0, 852.0)), "📱 iPhone 15");
          ui.selectable_value(
            &mut size,
            Some(egui::vec2(1280.0, 720.0)),
            "🖥 Desktop 720p",
          );
          ui.selectable_value(
            &mut size,
            Some(egui::vec2(1920.0, 1080.0)),
            "🖥 Desktop 1080p",
          );
        });

      if let Some(size) = size {
        ui.ctx()
          .send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
        ui.ctx()
          .send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        ui.close_menu();
      }
    });
  }
}

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct EguiWindows {
  // egui stuff:
  settings: bool,
  inspection: bool,
  memory: bool,
  output_events: bool,

  #[cfg_attr(feature = "serde", serde(skip))]
  output_event_history: std::collections::VecDeque<egui::output::OutputEvent>,
}

impl Default for EguiWindows {
  fn default() -> Self {
    Self::none()
  }
}

impl EguiWindows {
  fn none() -> Self {
    Self {
      settings: false,
      inspection: false,
      memory: false,
      output_events: false,
      output_event_history: Default::default(),
    }
  }

  fn checkboxes(&mut self, ui: &mut egui::Ui) {
    let Self {
      settings,
      inspection,
      memory,
      output_events,
      output_event_history: _,
    } = self;

    ui.checkbox(settings, "🔧 Settings");
    ui.checkbox(inspection, "🔍 Inspection");
    ui.checkbox(memory, "📝 Memory");
    ui.checkbox(output_events, "📤 Output Events");
  }

  fn windows(&mut self, ctx: &egui::Context) {
    let Self {
      settings,
      inspection,
      memory,
      output_events,
      output_event_history,
    } = self;

    ctx.output(|o| {
      for event in &o.events {
        output_event_history.push_back(event.clone());
      }
    });
    while output_event_history.len() > 1000 {
      output_event_history.pop_front();
    }

    egui::Window::new("🔧 Settings")
      .open(settings)
      .vscroll(true)
      .show(ctx, |ui| {
        ctx.settings_ui(ui);
      });

    egui::Window::new("🔍 Inspection")
      .open(inspection)
      .vscroll(true)
      .show(ctx, |ui| {
        ctx.inspection_ui(ui);
      });

    egui::Window::new("📝 Memory")
      .open(memory)
      .resizable(false)
      .show(ctx, |ui| {
        ctx.memory_ui(ui);
      });

    egui::Window::new("📤 Output Events")
      .open(output_events)
      .resizable(true)
      .default_width(520.0)
      .show(ctx, |ui| {
        ui.label(
          "Recent output events from egui. \
            These are emitted when you interact with widgets, or move focus between them with TAB. \
            They can be hooked up to a screen reader on supported platforms.",
        );

        ui.separator();

        egui::ScrollArea::vertical()
          .stick_to_bottom(true)
          .show(ui, |ui| {
            for event in output_event_history {
              ui.label(format!("{event:?}"));
            }
          });
      });
  }
}

// ----------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
fn call_after_delay(delay: std::time::Duration, f: impl FnOnce() + Send + 'static) {
  std::thread::Builder::new()
    .name("call_after_delay".to_owned())
    .spawn(move || {
      std::thread::sleep(delay);
      f();
    })
    .unwrap();
}