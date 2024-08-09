use std::error::Error;
use std::ffi::CString;
use std::num::NonZeroU32;
use std::time::Instant;
use env_logger::Env;
use raw_window_handle::HasWindowHandle;

use glow::Context;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentContext, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};

use glutin_winit::{DisplayBuilder, GlWindow};

/*use imgui_winit_glow_renderer_viewports::Renderer;
use imgui_winit_support::winit::window::WindowBuilder;*/
use log::info;

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, Position};
use winit::event::{KeyEvent, StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::{Icon, Window};

use crate::rendering::rendering_manager;
use crate::settings::configuration;
use crate::ui::ui_manager;

fn load_icon() -> Icon {
  let data_icon = include_bytes!(concat!(env!("OUT_DIR"), "/assets/Kuplung.png")).as_ref();
  let (icon_rgba, icon_width, icon_height) = {
    let image = image::load_from_memory(data_icon).unwrap().into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };
  Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

pub fn main(event_loop: winit::event_loop::EventLoop<()>) -> Result<(), Box<dyn Error>> {
  let icon = load_icon();

  let window_attributes = Window::default_attributes()
    .with_transparent(false)
    .with_title(configuration::APP_TITLE)
    .with_position(Position::Logical(LogicalPosition::new(configuration::WINDOW_POSITION_X, configuration::WINDOW_POSITION_Y)))
    .with_inner_size(winit::dpi::LogicalSize::new(configuration::WINDOW_WIDTH, configuration::WINDOW_HEIGHT))
    .with_window_icon(Some(icon))
    .with_resizable(true)
    .with_visible(true);

  let template = ConfigTemplateBuilder::new()
    .with_depth_size(configuration::GL_DEPTH_SIZE)
    .with_multisampling(configuration::GL_MULTISAMPLING)
    .with_stencil_size(configuration::GL_STENCIL_SIZE)
    .with_single_buffering(configuration::GL_DOUBLE_BUFFERING)
    .prefer_hardware_accelerated(Some(configuration::GL_HARDWARE_ACCELERATED));
  let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));
  let mut app = App::new(template, display_builder);
  event_loop.run_app(&mut app)?;
  app.exit_state
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let env = Env::default()
      .filter_or(configuration::KUPLUNG_LOG_LEVEL, configuration::KUPLUNG_LOG_LEVEL_VALUE)
      .write_style_or(configuration::KUPLUNG_LOG_STYLE, configuration::KUPLUNG_LOG_STYLE_VALUE);
    env_logger::init_from_env(env);

    info!("[Kuplung] Initializing...");

    let (mut window, gl_config) = match self.display_builder.clone().build(event_loop, self.template.clone(), rendering_manager::RenderingManager::gl_config_picker) {
      Ok(ok) => ok,
      Err(e) => {
        log::error!("[Kuplung] Error building the display! {}", e.to_string());
        self.exit_state = Err(e);
        event_loop.exit();
        return;
      },
    };

    info!("[Kuplung] Picked a config with {} samples", gl_config.num_samples());

    let raw_window_handle = window
      .as_ref()
      .and_then(|window| window.window_handle().ok())
      .map(|handle| handle.as_raw());

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new()
      .with_profile(GlProfile::Core)
      .with_context_api(ContextApi::OpenGl(Some(
        glutin::context::Version::new(configuration::OPENGL_VERSION_MAJOR, configuration::OPENGL_VERSION_MINOR),
      )))
      .with_debug(configuration::GL_DEBUG)
      .build(raw_window_handle);

    let gl_context = self.gl_context.take().unwrap_or_else(|| unsafe {
      gl_display.create_context(&gl_config, &context_attributes).unwrap()
    });

    /*let (window, gl_config) = DisplayBuilder::new()
      .with_window_builder(Some(window_builder))
      .build(&event_loop, self.template.b, |mut configs| {
        configs.next().unwrap()
      })
      .expect("Failed to create main window");*/

    let window = window.take().unwrap_or_else(|| {
      let window_attributes = Window::default_attributes()
        .with_transparent(true)
        .with_title(configuration::APP_TITLE);
      glutin_winit::finalize_window(event_loop, window_attributes, &gl_config).unwrap()
    });

    let attrs = window
      .build_surface_attributes(Default::default())
      .expect("[Kuplung] Failed to build surface attributes");
    let gl_surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

    let gl_context = gl_context.make_current(&gl_surface).unwrap();

    let glow = unsafe {
      Context::from_loader_function(|name| {
        let name = CString::new(name).unwrap();
        gl_context.display().get_proc_address(&name)
      })
    };

    self.renderer.get_or_insert_with(|| rendering_manager::RenderingManager::new(gl_display));

    if let Err(res) = gl_surface.set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap())) {
      log::error!("[Kuplung] Error setting vsync: {res:?}");
    }

    self.ui_manager.get_or_insert_with(|| ui_manager::UIManager::new());
    ui_manager::UIManager::configure_context(self.ui_manager.as_mut().unwrap(), &window);

    /*let mut glow_renderer = Renderer::new(&mut self.ui_manager.as_mut().unwrap().imgui_context, &window, &glow)
      .expect("[Kuplung] Failed to init Renderer!");*/

    assert!(self.state.replace(AppState { gl_context, gl_surface, window }).is_none());
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {
    match event {
      WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
        if let Some(AppState { gl_context, gl_surface, window: _ }) = self.state.as_ref() {
          gl_surface.resize(gl_context, NonZeroU32::new(size.width).unwrap(), NonZeroU32::new(size.height).unwrap());
          let renderer = self.renderer.as_ref().unwrap();
          renderer.resize(size.width as i32, size.height as i32);
        }
      },
      WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
        event: KeyEvent { logical_key: Key::Named(NamedKey::Escape), .. },
        ..
      } => event_loop.exit(),
      _ => (),
    }
  }

  fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
    if let Some(AppState { gl_context, gl_surface, window }) = self.state.as_ref() {
      let uim = self.ui_manager.as_mut().unwrap();
      uim.render_ui();

      let renderer = self.renderer.as_ref().unwrap();
      renderer.draw();

      window.request_redraw();
      gl_surface.swap_buffers(gl_context).unwrap();
    }
  }

  fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
    let now = Instant::now();

    if !self.ui_manager.is_none() {
      let uim = self.ui_manager.as_mut().unwrap();
      uim.imgui_context.io_mut().update_delta_time(now.duration_since(self.last_frame));
      self.last_frame = now;
    }
  }
}

struct App {
  template: ConfigTemplateBuilder,
  display_builder: DisplayBuilder,
  exit_state: Result<(), Box<dyn Error>>,
  gl_context: Option<NotCurrentContext>,
  renderer: Option<rendering_manager::RenderingManager>,
  state: Option<AppState>,
  ui_manager: Option<ui_manager::UIManager>,
  last_frame: Instant
}

impl App {
  fn new(template: ConfigTemplateBuilder, display_builder: DisplayBuilder) -> Self {
    Self {
      template,
      display_builder,
      exit_state: Ok(()),
      gl_context: None,
      state: None,
      renderer: None,
      ui_manager: None,
      last_frame: Instant::now(),
    }
  }
}

struct AppState {
  gl_context: PossiblyCurrentContext,
  gl_surface: Surface<WindowSurface>,
  window: Window,
}
