use glutin::{
  context::{NotCurrentGlContext, PossiblyCurrentContext},
  display::{GetGlDisplay, GlDisplay},
  surface::{GlSurface},
};

pub fn glow_context(context: &PossiblyCurrentContext) -> glow::Context {
  unsafe {
    glow::Context::from_loader_function_cstr(|s| context.display().get_proc_address(s).cast())
  }
}