use std::sync::Arc;

use eframe::egui_glow;
use egui::mutex::Mutex;
use egui::Ui;
use egui_glow::glow;

use log::info;
use crate::fractals::julia::Julia;
use crate::fractals::mandelbrot::Mandelbrot;
use crate::settings::configuration;

pub struct FractalsManager {
  pub show_fractals: bool,
  show_mandelbrot: bool,
  show_julia: bool,
  fractal_mandelbrot: Arc<Mutex<Mandelbrot>>,
  fractal_julia: Arc<Mutex<Julia>>,
  option_mandelbrot_iterations: i32,
  option_julia_iterations: i32,
}

impl FractalsManager {
  pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
    info!("[Kuplung] New FractalsManager...");

    let gl = cc.gl.as_ref()?;
    let this = Self {
      show_fractals: false,
      show_mandelbrot: true,
      show_julia: false,
      fractal_mandelbrot: Arc::new(Mutex::new(Mandelbrot::new(gl)?)),
      fractal_julia: Arc::new(Mutex::new(Julia::new(gl)?)),
      option_mandelbrot_iterations: 100,
      option_julia_iterations: 256,
    };

    info!("[Kuplung] New FractalsManager finished.");
    Some(this)
  }

  fn paint_mandelbrot(&mut self, ui: &mut Ui) {
    ui.label("Mandelbrot fractal");
    ui.horizontal(|ui| {
      ui.label("Iterations:");
      ui.add(egui::DragValue::new(&mut self.option_mandelbrot_iterations).speed(1.0));
    });
    ui.separator();
    let iterations = self.option_mandelbrot_iterations;
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
      let window_width: f32 = ui.available_size().x;
      let window_height: f32 = ui.available_size().y;
      let (rect, _) = ui.allocate_exact_size(egui::Vec2::from([window_width, window_height]), egui::Sense::drag());
      let fractal_mandelbrot = self.fractal_mandelbrot.clone();
      let cb = egui_glow::CallbackFn::new(move |_, painter| {
        fractal_mandelbrot.lock().paint(painter.gl(), window_width, window_height, iterations);
      });
      let callback = egui::PaintCallback {
        rect,
        callback: Arc::new(cb),
      };
      ui.painter().add(callback);
    });
  }

  fn paint_julia(&mut self, ui: &mut Ui) {
    ui.label("Julia fractal");
    ui.horizontal(|ui| {
      ui.label("Iterations:");
      ui.add(egui::DragValue::new(&mut self.option_julia_iterations).speed(1.0));
    });
    ui.separator();
    let iterations = self.option_julia_iterations;
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
      let window_width: f32 = ui.available_size().x;
      let window_height: f32 = ui.available_size().y;
      let (rect, _) = ui.allocate_exact_size(egui::Vec2::from([window_width, window_height]), egui::Sense::drag());
      let fractal_julia = self.fractal_julia.clone();
      let cb = egui_glow::CallbackFn::new(move |_, painter| {
        fractal_julia.lock().paint(painter.gl(), window_width, window_height, iterations);
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
      .min_size([configuration::WINDOW_WIDTH_FRACTALS, configuration::WINDOW_HEIGHT_FRACTALS])
      .default_size([configuration::WINDOW_WIDTH_FRACTALS, configuration::WINDOW_HEIGHT_FRACTALS])
      .show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
          if ui.button("Close").clicked() {
            ui.close_menu();
            self.show_fractals = false;
          }
          ui.menu_button("Fractals", |ui| {
            if ui.button("Mandelbrot").clicked() {
              ui.close_menu();
              self.show_julia = false;
              self.show_mandelbrot = true;
            }
            if ui.button("Julia").clicked() {
              ui.close_menu();
              self.show_mandelbrot = false;
              self.show_julia = true;
            }
          });
        });
        ui.separator();
        if self.show_mandelbrot { self.paint_mandelbrot(ui); }
        if self.show_julia { self.paint_julia(ui); }
        if !self.show_mandelbrot && !self.show_julia { ui.label("Select fractal from the menu."); }
      });
  }

  fn on_exit(&mut self, gl: Option<&glow::Context>) {
    if let Some(gl) = gl {
      self.fractal_mandelbrot.lock().destroy(gl);
      self.fractal_julia.lock().destroy(gl);
    }
  }
}