use std::sync::Arc;

use eframe::egui_glow;
use egui::mutex::Mutex;
use egui::Ui;
use egui_glow::glow;

use log::info;
use crate::fractals::mandelbrot::Mandelbrot;
use crate::settings::configuration;

pub struct FractalsManager {
  pub show_fractals: bool,
  show_mandelbrot: bool,
  fractal_mandelbrot: Arc<Mutex<Mandelbrot>>,
}

impl FractalsManager {
  pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
    info!("[Kuplung] New FractalsManager...");

    let gl = cc.gl.as_ref()?;
    let this = Self {
      show_fractals: false,
      show_mandelbrot: true,
      fractal_mandelbrot: Arc::new(Mutex::new(Mandelbrot::new(gl)?)),
    };

    info!("[Kuplung] New FractalsManager finished.");
    Some(this)
  }

  fn paint_mandelbrot(&mut self, ui: &mut Ui) {
    ui.horizontal(|ui| {
      ui.spacing_mut().item_spacing.x = 0.0;
      ui.label("Mandelbrot fractal");
    });
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
      let window_width: f32 = ui.available_size().x;
      let window_height: f32 = ui.available_size().y;
      let (rect, _) = ui.allocate_exact_size(egui::Vec2::from([window_width, window_height]), egui::Sense::drag());
      let rotating_triangle = self.fractal_mandelbrot.clone();
      let cb = egui_glow::CallbackFn::new(move |_, painter| {
        rotating_triangle.lock().paint(painter.gl(), window_width, window_height, 100.0);
      });
      let callback = egui::PaintCallback {
        rect,
        callback: Arc::new(cb),
      };
      ui.painter().add(callback);
    });
  }
}

impl eframe::App for FractalsManager {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    self.show_fractals = true;
    egui::Window::new("Fractals")
      .id(egui::Id::new("window_fractals"))
      .resizable(true)
      .enabled(true)
      .default_pos([80.0, 80.0])
      .min_size([configuration::WINDOW_POSITION_WIDTH_FRACTALS, 200.0])
      .default_size([configuration::WINDOW_POSITION_WIDTH_FRACTALS, configuration::WINDOW_POSITION_HEIGHT_FRACTALS])
      .show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
          if ui.button("Close").clicked() {
            ui.close_menu();
            self.show_fractals = false;
          }
          ui.menu_button("Fractals", |ui| {
            if ui.button("Mandelbrot").clicked() {
              ui.close_menu();
              self.show_mandelbrot = true;
            }
          });
        });
        ui.separator();
        if self.show_mandelbrot { self.paint_mandelbrot(ui); }
        if !self.show_mandelbrot { ui.label("Select fractal from the menu."); }
      });
  }

  fn on_exit(&mut self, gl: Option<&glow::Context>) {
    if let Some(gl) = gl {
      self.fractal_mandelbrot.lock().destroy(gl);
    }
  }
}