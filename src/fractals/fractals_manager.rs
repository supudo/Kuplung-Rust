// 1. https://aimlesslygoingforward.com/blog/2016/09/27/mandelbrot-using-shaders-rust/
// 2. https://gpfault.net/posts/mandelbrot-webgl.txt.html

use std::sync::Arc;

use eframe::egui_glow;
use egui::mutex::Mutex;
use egui::{TextBuffer, Ui};
use egui_glow::glow;

extern crate strum_macros;
extern crate strum;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

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
  option_mandelbrot_iterations: i32,
  option_mandelbrot_blackandwhite: bool,
  option_mandelbrot_colorpalette: i32,
  option_julia_iterations: i32,
  zoom_center: nalgebra_glm::Vec2,
  target_zoom_center: nalgebra_glm::Vec2,
  zoom_size: f32,
  stop_zooming: bool,
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
      option_mandelbrot_iterations: 500,
      option_mandelbrot_blackandwhite: false,
      option_mandelbrot_colorpalette: 0,
      option_julia_iterations: 256,
      zoom_center: nalgebra_glm::Vec2::new(0.0, 0.0),
      target_zoom_center: nalgebra_glm::Vec2::new(0.0, 0.0),
      zoom_size: 1.0,
      stop_zooming: true,
      zoom_factor: 1.0,
      zoom_max_iterations: 500
    };

    do_log!("[Kuplung] New FractalsManager finished.");
    Some(this)
  }

  fn paint_mandelbrot(&mut self, ui: &mut Ui) {
    ui.label("Mandelbrot fractal");
    ui.separator();
    ui.horizontal(|ui| {
      ui.checkbox(&mut self.option_mandelbrot_blackandwhite, "Black and White");
      ui.separator();
      ui.label("Iterations:");
      ui.add(egui::DragValue::new(&mut self.option_mandelbrot_iterations).speed(1.0));
      ui.separator();
      egui::ComboBox::from_label("Color palette")
        .selected_text(format!("{}", get_mcp(usize::try_from(self.option_mandelbrot_colorpalette).unwrap())))
        .show_ui(ui, |ui| {
          ui.selectable_value(&mut self.option_mandelbrot_colorpalette, 0, "Normal");
          ui.selectable_value(&mut self.option_mandelbrot_colorpalette, 1, "Grainy");
        });
    });
    ui.end_row();

    if !self.stop_zooming {
      self.zoom_max_iterations -= 10;
      if self.zoom_max_iterations < 50 { self.zoom_max_iterations = 50 };
      self.zoom_size *= self.zoom_factor;
      self.zoom_center.x += 0.1 * (self.target_zoom_center[0] - self.zoom_center.x);
      self.zoom_center.y += 0.1 * (self.target_zoom_center[1] - self.zoom_center.y);
      do_log!("[0] {} x {} = {} ; {}", self.zoom_center.x, self.zoom_center.y, self.zoom_size, self.zoom_max_iterations);
    }
    else if self.zoom_max_iterations < 500 {
      self.zoom_max_iterations += 10;
      do_log!("[1] {} x {} = {} ; {}", self.zoom_center.x, self.zoom_center.y, self.zoom_size, self.zoom_max_iterations);
    }

    let iterations = self.option_mandelbrot_iterations;
    let black_and_white = self.option_mandelbrot_blackandwhite;
    let color_palette = self.option_mandelbrot_colorpalette;
    let zoom_center = self.zoom_center;
    let zoom_size = self.zoom_size;
    let zoom_max_iterations = self.zoom_max_iterations;
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
      let window_width: f32 = ui.available_width();
      let window_height: f32 = ui.available_height();
      let (rect, response) = ui.allocate_exact_size(egui::Vec2::from([window_width, window_height]), egui::Sense::click_and_drag());
      if response.clicked() {
        let clicked_pos = response.interact_pointer_pos();
        let x_part = clicked_pos.unwrap().x / window_width;
        let y_part = clicked_pos.unwrap().y / window_height;
        self.target_zoom_center[0] = zoom_center[0] - zoom_size / 2.0 + x_part * zoom_size;
        self.target_zoom_center[1] = zoom_center[1] + zoom_size / 2.0 - y_part * zoom_size;
        self.stop_zooming = false;
        if response.secondary_clicked() { self.zoom_factor = 0.99 } else { self.zoom_factor = 1.01 };
      }
      let fractal_mandelbrot = self.fractal_mandelbrot.clone();
      let cb = egui_glow::CallbackFn::new(move |_, painter| {
        if color_palette == 0 {
          fractal_mandelbrot.lock().paint(painter.gl(), window_width, window_height, iterations, black_and_white, color_palette, zoom_center, zoom_size);
        }
        else {
          fractal_mandelbrot.lock().paint(painter.gl(), window_width, window_height, zoom_max_iterations, black_and_white, color_palette, zoom_center, zoom_size);
        }
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
    ui.separator();
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

#[derive(Debug, PartialEq, EnumIter, AsRefStr)]
enum MandelbrotColorPallete {
  Normal = 0,
  Grainy,
}

fn get_mcp(idx: usize) -> String {
  return MandelbrotColorPallete::iter().nth(idx).unwrap().as_ref().as_str().replace('\"', "");
}