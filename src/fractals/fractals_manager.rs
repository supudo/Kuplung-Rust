use std::sync::Arc;

use eframe::egui_glow;
use egui::mutex::Mutex;
use egui_glow::glow;

use log::info;
use crate::fractals::mandelbrot::Mandelbrot;

pub struct FractalsManager {
    show_mandelbrot: bool,
    fractal_mandelbrot: Arc<Mutex<Mandelbrot>>,
    angle: f32,
}

impl FractalsManager {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        info!("[Kuplung] New FractalsManager...");

        let gl = cc.gl.as_ref()?;
        let this = Self {
            show_mandelbrot: true,
            fractal_mandelbrot: Arc::new(Mutex::new(Mandelbrot::new(gl)?)),
            angle: 0.0,
        };

        info!("[Kuplung] New FractalsManager finished.");
        Some(this)
    }

    fn paint_mandelbrot(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());
        self.angle += response.drag_motion().x * 0.01;
        let angle = self.angle;
        let rotating_triangle = self.fractal_mandelbrot.clone();
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

impl eframe::App for FractalsManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Mandelbrot");
                    });

                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        if self.show_mandelbrot { self.paint_mandelbrot(ui); }
                    });
                    ui.label("Drag to rotate!");
                });
        });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.fractal_mandelbrot.lock().destroy(gl);
        }
    }
}