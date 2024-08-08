#![windows_subsystem = "windows"]

extern crate sdl2;

use sdl2::Sdl;
use crate::settings::configuration;

mod settings;

fn main() {
  let sdl_context = sdl2::init().expect("Error: SDL could not initialize!");
  let video_system = sdl_context.video().expect("Error: SDL Video could not initialize!");

  let gl_attr = video_system.gl_attr();
  gl_attr.set_depth_size(23);
  gl_attr.set_stencil_size(8);
  gl_attr.set_multisample_samples(4);
  gl_attr.set_multisample_buffers(1);
  gl_attr.set_context_flags().forward_compatible().set();
  gl_attr.set_context_major_version(configuration::OPENGL_VERSION_MAJOR);
  gl_attr.set_context_minor_version(configuration::OPENGL_VERSION_MINOR);
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_flags().debug().set();
  
  let window = video_system.window(configuration::APP_TITLE, configuration::WINDOW_WIDTH, configuration::WINDOW_HEIGHT)
    .position(configuration::WINDOW_POSITION_X, configuration::WINDOW_POSITION_Y)
    .opengl()
    .resizable()
    .allow_highdpi()
    .build()
    .expect("Error: Window could not be created!");

  let gl_context = window.gl_create_context().expect("Error: Unable to create OpenGL context!");
  let gl = gl::load_with(|s| video_system.gl_get_proc_address(s) as *const std::os::raw::c_void);

  window.gl_make_current(&gl_context).expect("Error: Unable to set current context!");
  video_system.gl_set_swap_interval(1).expect("Warning: Unable to set VSync!");
  
  /*let (dw, dh) = window.drawable_size();
  unsafe {
    configuration::DRAWABLE_SIZE_WIDTH = dw;
    configuration::DRAWABLE_SIZE_HEIGHT = dh;
  }*/

  unsafe {
    //gl::Viewport(0, 0, configuration::DRAWABLE_SIZE_WIDTH, configuration::DRAWABLE_SIZE_HEIGHT);
    gl::ClearColor(configuration::GL_CLEAR_COLOR_R, configuration::GL_CLEAR_COLOR_G, configuration::GL_CLEAR_COLOR_B, configuration::GL_CLEAR_COLOR_A);
    gl::Clear(gl::COLOR_BUFFER_BIT);
  }

  let mut event_pump = sdl_context.event_pump().unwrap();
  'main: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit {..} => break 'main,
        _ => {},
      }
    }

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    window.gl_swap_window();
  }
}