use std::sync::Arc;

use eframe::egui_glow;
use egui::mutex::Mutex;
use egui_glow::glow;

use log::info;
use crate::rendering::triangler::Triangler;

pub struct RenderingManager {
  show_triangler: bool,
  triangler: Arc<Mutex<Triangler>>,
  angle: f32,
}

impl RenderingManager {
  pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
    info!("[Kuplung] New RenderingManager...");

    let gl = cc.gl.as_ref()?;
    let this = Self {
      show_triangler: false,
      triangler: Arc::new(Mutex::new(Triangler::new(gl)?)),
      angle: 0.0,
    };

    info!("[Kuplung] New RenderingManager finished.");
    Some(this)
  }

  fn paint_triangler(&mut self, ui: &mut egui::Ui) {
    let (rect, response) = ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());
    self.angle += response.drag_motion().x * 0.01;
    let angle = self.angle;
    let rotating_triangle = self.triangler.clone();
    let cb = egui_glow::CallbackFn::new(move |_info, painter| {
      rotating_triangle.lock().paint(painter.gl(), angle);
    });
    let callback = egui::PaintCallback {
      rect,
      callback: Arc::new(cb),
    };
    ui.painter().add(callback);
  }
}

impl eframe::App for RenderingManager {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::ScrollArea::both()
        .auto_shrink(false)
        .show(ui, |ui| {
          ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("The triangler is being painted using ");
            ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
            ui.label(" (OpenGL).");
          });
          ui.label("It's not a very impressive demo, but it shows you can embed 3D inside of egui.");

          egui::Frame::canvas(ui.style()).show(ui, |ui| {
            self.paint_triangler(ui);
          });
          ui.label("Drag to rotate!");
        });
    });
  }

  fn on_exit(&mut self, gl: Option<&glow::Context>) {
    if let Some(gl) = gl {
      self.triangler.lock().destroy(gl);
    }
  }
}