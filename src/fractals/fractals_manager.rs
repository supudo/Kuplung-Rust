// 1. https://aimlesslygoingforward.com/blog/2016/09/27/mandelbrot-using-shaders-rust/
// 2. https://gpfault.net/posts/mandelbrot-webgl.txt.html

use std::sync::Arc;

use eframe::egui_glow;
use egui::mutex::Mutex;
use egui::Ui;
use egui_glow::glow;

extern crate strum_macros;
extern crate strum;

use crate::fractals::julia::Julia;
use crate::fractals::mandelbrot::Mandelbrot;
use crate::do_log;
use crate::settings::{configuration, kuplung_logger};

pub struct FractalsManager {
  pub show_fractals: bool,
  show_mandelbrot: bool,
  show_julia: bool,
  fractal_mandelbrot: Arc<Mutex<Mandelbrot>>,
  fractal_julia: Arc<Mutex<Julia>>,
  zoom_center: nalgebra_glm::Vec2,
  zoom_center_target: nalgebra_glm::Vec2,
  zoom_size: f32,
  zoom_stop: bool,
  zoom_factor: f32,
  zoom_max_iterations: i32
}

impl FractalsManager {
  pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
    do_log!("[Kuplung] New FractalsManager...");

    let gl = cc.gl.as_ref()?;
    let this = Self {
      show_fractals: false,
      show_mandelbrot: true,
      show_julia: false,
      fractal_mandelbrot: Arc::new(Mutex::new(Mandelbrot::new(gl)?)),
      fractal_julia: Arc::new(Mutex::new(Julia::new(gl)?)),
      zoom_center: nalgebra_glm::Vec2::new(0.0, 0.0),
      zoom_center_target: nalgebra_glm::Vec2::new(0.0, 0.0),
      zoom_size: 1.0,
      zoom_stop: true,
      zoom_factor: 1.0,
      zoom_max_iterations: 500
    };

    do_log!("[Kuplung] New FractalsManager finished.");
    Some(this)
  }

  fn paint_mandelbrot(&mut self, ui: &mut Ui) {
    self.fractal_mandelbrot.lock().draw_ui(ui);

    if !self.zoom_stop {
      self.zoom_max_iterations -= 10;
      if self.zoom_max_iterations < 50 { self.zoom_max_iterations = 50 };
      self.zoom_size *= self.zoom_factor;
      self.zoom_center.x += 0.1 * (self.zoom_center_target[0] - self.zoom_center.x);
      self.zoom_center.y += 0.1 * (self.zoom_center_target[1] - self.zoom_center.y);
    }
    else if self.zoom_max_iterations < 500 {
      self.zoom_max_iterations += 10;
    }

    let zoom_center = self.zoom_center;
    let zoom_size = self.zoom_size;
    let zoom_max_iterations = self.zoom_max_iterations;
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
      let window_width: f32 = ui.available_width();
      let window_height: f32 = ui.available_height();
      let (rect, response) = ui.allocate_exact_size(egui::Vec2::from([window_width, window_height]), egui::Sense::click_and_drag());
      if response.clicked() {
        let clicked_pos = response.interact_pointer_pos();
        let clicked_x = clicked_pos.unwrap().x / window_width;
        let clicked_y = clicked_pos.unwrap().y / window_height;
        self.zoom_center_target[0] = zoom_center[0] - zoom_size / 2.0 + clicked_x * zoom_size;
        self.zoom_center_target[1] = zoom_center[1] + zoom_size / 2.0 - clicked_y * zoom_size;
        self.zoom_stop = false;
        if response.secondary_clicked() { self.zoom_factor = 0.99 } else { self.zoom_factor = 1.01 };

        do_log!("{} x {}", clicked_pos.unwrap().x, clicked_pos.unwrap().y);

        let to_screen = egui::emath::RectTransform::from_to(egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.square_proportions()), response.rect);
        let from_screen = to_screen.inverse();
        if let Some(pointer_pos) = response.interact_pointer_pos() {
          let canvas_pos = from_screen * pointer_pos;
          do_log!("{} x {}", canvas_pos.x, canvas_pos.y);
        }
      }
      let fractal_mandelbrot = self.fractal_mandelbrot.clone();
      let cb = egui_glow::CallbackFn::new(move |_, painter| {
        fractal_mandelbrot.lock().paint(painter.gl(), window_width, window_height, zoom_max_iterations, zoom_center, zoom_size);
      });
      let callback = egui::PaintCallback {
        rect,
        callback: Arc::new(cb),
      };
      ui.painter().add(callback);
    });
  }

  fn paint_julia(&mut self, ui: &mut Ui) {
    self.fractal_julia.lock().draw_ui(ui);
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
      let window_width: f32 = ui.available_size().x;
      let window_height: f32 = ui.available_size().y;
      let (rect, _) = ui.allocate_exact_size(egui::Vec2::from([window_width, window_height]), egui::Sense::drag());
      let fractal_julia = self.fractal_julia.clone();
      let cb = egui_glow::CallbackFn::new(move |_, painter| {
        fractal_julia.lock().paint(painter.gl(), window_width, window_height);
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