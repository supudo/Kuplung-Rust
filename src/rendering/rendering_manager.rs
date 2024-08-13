use log::info;
use crate::settings::configuration;

#[derive(Default)]
pub struct RenderingManager {
}

impl RenderingManager {
  pub fn new() -> Self {
    info!("[Kuplung] New RenderingManager...");
    let this = Self {
    };
    info!("[Kuplung] New RenderingManager finished.");
    this
  }

  /*pub fn resize(&self, width: i32, height: i32) {
    info!("[Kuplung] RenderingManager resize to {}x{}.", width, height);
    unsafe {
      self.triangle.Viewport(0, 0, width, height);
    }
  }

  pub fn draw(&self) {
    self.triangle.draw_with_clear_color(configuration::GL_CLEAR_COLOR_R, configuration::GL_CLEAR_COLOR_G, configuration::GL_CLEAR_COLOR_B, configuration::GL_CLEAR_COLOR_A)
  }

  pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
      .reduce(|accum, config| {
        let transparency_check = config.supports_transparency().unwrap_or(false) & !accum.supports_transparency().unwrap_or(false);
        if transparency_check || config.num_samples() > accum.num_samples() {
          config
        }
        else {
          accum
        }
      })
      .unwrap()
  }*/
}