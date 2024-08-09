use std::error::Error;
use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::ops::Deref;
use env_logger::Env;
use gl::types::GLfloat;
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Icon, Window};

use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::{ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentContext, PossiblyCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};

use glutin_winit::{DisplayBuilder, GlWindow};
use winit::dpi::{LogicalPosition, Position};

use crate::settings::configuration;
use crate::rendering::triangle;

use log::{info, logger, warn};

pub mod gl {
  #![allow(clippy::all)]
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
  pub use Gles2 as Gl;
}

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
    .with_window_icon(Some(icon));

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
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {

    let env = Env::default()
      .filter_or("MY_LOG_LEVEL", "trace")
      .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    info!("[Kuplung] Initializing...");

    let (mut window, gl_config) = match self.display_builder.clone().build(event_loop, self.template.clone(), gl_config_picker) {
      Ok(ok) => ok,
      Err(e) => {
        log::error!("[Kuplung] Error building the display! {}", e.to_string());
        self.exit_state = Err(e);
        event_loop.exit();
        return;
      },
    };

    log::info!("[Kuplung] Picked a config with {} samples", gl_config.num_samples());

    let raw_window_handle = window
      .as_ref()
      .and_then(|window| window.window_handle().ok())
      .map(|handle| handle.as_raw());

    // XXX The display could be obtained from any object created by it, so we can
    // query it from the config.
    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new()
      .with_profile(glutin::context::GlProfile::Core)
      .with_context_api(glutin::context::ContextApi::OpenGl(Some(
        glutin::context::Version::new(configuration::OPENGL_VERSION_MAJOR, configuration::OPENGL_VERSION_MINOR),
      )))
      .with_debug(configuration::GL_DEBUG)
      .build(raw_window_handle);

    let gl_context = self.gl_context.take().unwrap_or_else(|| unsafe {
      gl_display.create_context(&gl_config, &context_attributes).unwrap()
    });

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

    // Make it current.
    let gl_context = gl_context.make_current(&gl_surface).unwrap();

    // The context needs to be current for the Renderer to set up shaders and
    // buffers. It also performs function loading, which needs a current context on
    // WGL.
    self.renderer.get_or_insert_with(|| Renderer::new(&gl_display));

    if let Err(res) = gl_surface.set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap())) {
      log::error!("[Kuplung] Error setting vsync: {res:?}");
    }

    assert!(self.state.replace(AppState { gl_context, gl_surface, window }).is_none());
  }

  fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {
    match event {
      WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
        // Some platforms like EGL require resizing GL surface to update the size
        // Notable platforms here are Wayland and macOS, other don't require it
        // and the function is no-op, but it's wise to resize it for portability
        // reasons.
        if let Some(AppState { gl_context, gl_surface, window: _ }) = self.state.as_ref() {
          gl_surface.resize(
            gl_context,
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
          );
          let renderer = self.renderer.as_ref().unwrap();
          renderer.resize(size.width as i32, size.height as i32);
        }
      },
      WindowEvent::CloseRequested
      | WindowEvent::KeyboardInput {
        event: KeyEvent { logical_key: Key::Named(NamedKey::Escape), .. },
        ..
      } => event_loop.exit(),
      _ => (),
    }
  }

  fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
    if let Some(AppState { gl_context, gl_surface, window }) = self.state.as_ref() {
      let renderer = self.renderer.as_ref().unwrap();
      renderer.draw();
      window.request_redraw();
      gl_surface.swap_buffers(gl_context).unwrap();
    }
  }
}

struct App {
  template: ConfigTemplateBuilder,
  display_builder: DisplayBuilder,
  exit_state: Result<(), Box<dyn Error>>,
  gl_context: Option<NotCurrentContext>,
  renderer: Option<Renderer>,
  // NOTE: `AppState` carries the `Window`, thus it should be dropped after everything else.
  state: Option<AppState>,
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
    }
  }
}

struct AppState {
  gl_context: PossiblyCurrentContext,
  gl_surface: Surface<WindowSurface>,
  // NOTE: Window should be dropped after all resources created using its
  // raw-window-handle.
  window: Window,
}

// Find the config with the maximum number of samples, so our triangle will be smooth.
pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
  configs
    .reduce(|accum, config| {
      let transparency_check = config.supports_transparency().unwrap_or(false)
        & !accum.supports_transparency().unwrap_or(false);

      if transparency_check || config.num_samples() > accum.num_samples() {
        config
      }
      else {
        accum
      }
    })
    .unwrap()
}

pub struct Renderer {
  program: gl::types::GLuint,
  vao: gl::types::GLuint,
  vbo: gl::types::GLuint,
  gl: gl::Gl,
}

impl Renderer {
  pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
    unsafe {
      let gl = gl::Gl::load_with(|symbol| {
        let symbol = CString::new(symbol).unwrap();
        gl_display.get_proc_address(symbol.as_c_str()).cast()
      });

      if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
        info!("[Kuplung] Running on {}", renderer.to_string_lossy());
      }
      if let Some(version) = get_gl_string(&gl, gl::VERSION) {
        info!("[Kuplung] OpenGL Version {}", version.to_string_lossy());
      }

      if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
        info!("[Kuplung] Shaders version on {}", shaders_version.to_string_lossy());
      }

      let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, triangle::VERTEX_SHADER_SOURCE);
      let fragment_shader = create_shader(&gl, gl::FRAGMENT_SHADER, triangle::FRAGMENT_SHADER_SOURCE);

      let program = gl.CreateProgram();

      gl.AttachShader(program, vertex_shader);
      gl.AttachShader(program, fragment_shader);

      gl.LinkProgram(program);

      gl.UseProgram(program);

      gl.DeleteShader(vertex_shader);
      gl.DeleteShader(fragment_shader);

      let mut vao = std::mem::zeroed();
      gl.GenVertexArrays(1, &mut vao);
      gl.BindVertexArray(vao);

      let mut vbo = std::mem::zeroed();
      gl.GenBuffers(1, &mut vbo);
      gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl.BufferData(
        gl::ARRAY_BUFFER,
        (triangle::VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
        triangle::VERTEX_DATA.as_ptr() as *const _,
        gl::STATIC_DRAW,
      );

      let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
      let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
      gl.VertexAttribPointer(
        pos_attrib as gl::types::GLuint,
        2,
        gl::FLOAT,
        0,
        5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        std::ptr::null(),
      );
      gl.VertexAttribPointer(
        color_attrib as gl::types::GLuint,
        3,
        gl::FLOAT,
        0,
        5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        (2 * std::mem::size_of::<f32>()) as *const () as *const _,
      );
      gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
      gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);

      Self { program, vao, vbo, gl }
    }
  }

  pub fn draw(&self) {
    self.draw_with_clear_color(configuration::GL_CLEAR_COLOR_R, configuration::GL_CLEAR_COLOR_G, configuration::GL_CLEAR_COLOR_B, configuration::GL_CLEAR_COLOR_A)
  }

  pub fn draw_with_clear_color(
    &self,
    red: GLfloat,
    green: GLfloat,
    blue: GLfloat,
    alpha: GLfloat,
  ) {
    unsafe {
      self.gl.UseProgram(self.program);

      self.gl.BindVertexArray(self.vao);
      self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

      self.gl.ClearColor(red, green, blue, alpha);
      self.gl.Clear(gl::COLOR_BUFFER_BIT);
      self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
    }
  }

  pub fn resize(&self, width: i32, height: i32) {
    unsafe {
      self.gl.Viewport(0, 0, width, height);
    }
  }
}

impl Deref for Renderer {
  type Target = gl::Gl;

  fn deref(&self) -> &Self::Target {
    &self.gl
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteProgram(self.program);
      self.gl.DeleteBuffers(1, &self.vbo);
      self.gl.DeleteVertexArrays(1, &self.vao);
    }
  }
}

unsafe fn create_shader(gl: &gl::Gl, shader: gl::types::GLenum, source: &[u8]) -> gl::types::GLuint {
  let shader = gl.CreateShader(shader);
  gl.ShaderSource(shader, 1, [source.as_ptr().cast()].as_ptr(), std::ptr::null());
  gl.CompileShader(shader);
  shader
}

fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
  unsafe {
    let s = gl.GetString(variant);
    (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
  }
}

