pub const APP_TITLE: &str = "Kuplung";

// window settings
pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

pub const WINDOW_POSITION_X: f64 = 100.0;
pub const WINDOW_POSITION_Y: f64 = 100.0;

// OpenGL settings
pub const OPENGL_VERSION_MAJOR: u8 = 4;
pub const OPENGL_VERSION_MINOR: u8 = 1;

pub const GL_CLEAR_COLOR_R: f32 = 70.0 / 255.0;
pub const GL_CLEAR_COLOR_G: f32 = 70.0 / 255.0;
pub const GL_CLEAR_COLOR_B: f32 = 70.0 / 255.0;
pub const GL_CLEAR_COLOR_A: f32 = 255.0 / 255.0;

pub const GL_DEBUG: bool = cfg!(debug_assertions);
pub const GL_DEPTH_SIZE: u8 = 24;
pub const GL_MULTISAMPLING: u8 = 4;
pub const GL_STENCIL_SIZE: u8 = 4;
pub const GL_DOUBLE_BUFFERING: bool = false;
pub const GL_HARDWARE_ACCELERATED: bool = true;