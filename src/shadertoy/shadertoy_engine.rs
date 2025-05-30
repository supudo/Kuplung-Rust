#![allow(non_snake_case)]

use eframe::egui_glow;
use eframe::glow::HasContext;
use egui_glow::glow;
use log::error;
use crate::{do_log, utils};
use crate::settings::kuplung_logger;
use crate::rendering::gl_utils;

#[rustfmt::skip]
pub static SHADERTOY_VERTICES:[f32; 18] = [
  -1.0, -1.0, 0.0,
   1.0, -1.0, 0.0,
   1.0,  1.0, 0.0,
   1.0,  1.0, 0.0,
  -1.0,  1.0, 0.0,
  -1.0, -1.0, 0.0
];

pub struct ShaderToyEngine {
  pub iChannel0_Image: String,
  pub iChannel1_Image: String,
  pub iChannel2_Image: String,
  pub iChannel3_Image: String,
  pub iChannel0_CubeImage: String,
  pub iChannel1_CubeImage: String,
  pub iChannel2_CubeImage: String,
  pub iChannel3_CubeImage: String,
  pub textureWidth: i32,
  pub textureHeight: i32,
  shaderProgram: glow::Program,
  glVAO: glow::VertexArray,
  vboVertices: glow::Buffer,
  tFBO: glow::Framebuffer,
  tRBO: glow::Renderbuffer,
  vs_InFBO: glow::UniformLocation,
  vs_ScreenResolution: glow::UniformLocation,
  iChannelResolution0: [f32; 2],
  iChannelResolution1: [f32; 2],
  iChannelResolution2: [f32; 2],
  iChannelResolution3: [f32; 2],
  iResolution: glow::UniformLocation,
  iGlobalTime: glow::UniformLocation,
  iTimeDelta: glow::UniformLocation,
  iFrame: glow::UniformLocation,
  iFrameRate: glow::UniformLocation,
  /*iChannelTime: [glow::UniformLocation; 4],
  iChannelResolution: [glow::UniformLocation; 4],*/
  iMouse: glow::UniformLocation,
  iDate: glow::UniformLocation,
}

#[allow(unsafe_code)]
impl ShaderToyEngine {
  pub fn new(gl: &glow::Context) -> Option<Self> {
    Self::compile_shadertoy(gl, "".to_string())
  }

  pub fn compile_shadertoy(gl: &glow::Context, stoy: String) -> Option<Self> {
    use glow::HasContext as _;
    unsafe {
      let shaderProgram = gl.create_program().expect("[Kuplung] [ShaderToy-Engine] Cannot create program!");

      let shader_vertex = gl_utils::create_shader(&shaderProgram, &gl, glow::VERTEX_SHADER, "assets/shaders/shadertoy/shadertoy.vert");

      let shader_fragment = gl_utils::create_shader_from_string(&shaderProgram, &gl, glow::FRAGMENT_SHADER, Self::get_stoy(stoy).as_ref());

      gl.link_program(shaderProgram);
      if !gl.get_program_link_status(shaderProgram) {
        error!("[Kuplung] [ShaderToy-Engine] Program cannot be linked! {}", gl.get_program_info_log(shaderProgram));
        panic!("[Kuplung] [ShaderToy-Engine] Program cannot be linked! {}", gl.get_program_info_log(shaderProgram));
      }

      gl.detach_shader(shaderProgram, shader_vertex);
      gl.delete_shader(shader_vertex);
      gl.detach_shader(shaderProgram, shader_fragment);
      gl.delete_shader(shader_fragment);

      let vs_InFBO = gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "vs_inFBO");
      let vs_ScreenResolution = gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "vs_screenResolution");

      let iResolution = gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iResolution");
      let iGlobalTime = gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iGlobalTime");
      let iTimeDelta = gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iTimeDelta");
      let iFrameRate = gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iFrameRate");
      let iFrame = gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iFrame");
      /*let iChannelTime: [glow::UniformLocation; 4] = [
        gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iChannelTime[0]"),
        gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iChannelTime[1]"),
        gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iChannelTime[2]"),
        gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iChannelTime[3]"),
      ];
      let iChannelResolution: [glow::UniformLocation; 4] = [
        gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iChannelResolution[0]"),
        gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iChannelResolution[1]"),
        gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iChannelResolution[2]"),
        gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iChannelResolution[3]"),
      ];*/
      let iMouse = gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iMouse");
      let iDate = gl_utils::get_uniform_no_warning(&shaderProgram, &gl, "iDate");

      let glVAO = gl.create_vertex_array().expect("[Kuplung] [ShaderToy-Engine] Cannot create VAO!");
      gl.bind_vertex_array(Some(glVAO));

      let vboVertices = gl.create_buffer().expect("[Kuplung] [ShaderToy-Engine] Cannot create VBO!");
      gl.bind_buffer(glow::ARRAY_BUFFER, Some(vboVertices));
      gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&SHADERTOY_VERTICES[..]), glow::STATIC_DRAW);

      gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);
      gl.enable_vertex_attrib_array(0);

      let tFBO = gl.create_framebuffer().expect("[Kuplung] [ShaderToy-Engine] Cannot create FBO!");
      let tRBO = gl.create_renderbuffer().expect("[Kuplung] [ShaderToy-Engine] Cannot create RBO!");

      gl.bind_vertex_array(None);

      let iChannel0_Image = "".to_string();
      let iChannel1_Image = "".to_string();
      let iChannel2_Image = "".to_string();
      let iChannel3_Image = "".to_string();
      let iChannel0_CubeImage = "".to_string();
      let iChannel1_CubeImage = "".to_string();
      let iChannel2_CubeImage = "".to_string();
      let iChannel3_CubeImage = "".to_string();
      let textureWidth = 0;
      let textureHeight = 0;
      let iChannelResolution0 = [0.0, 0.0];
      let iChannelResolution1 = [0.0, 0.0];
      let iChannelResolution2 = [0.0, 0.0];
      let iChannelResolution3 = [0.0, 0.0];

      Some(Self {
        iChannel0_Image,
        iChannel1_Image,
        iChannel2_Image,
        iChannel3_Image,
        iChannel0_CubeImage,
        iChannel1_CubeImage,
        iChannel2_CubeImage,
        iChannel3_CubeImage,
        textureWidth,
        textureHeight,
        shaderProgram,
        glVAO,
        vboVertices,
        tFBO,
        tRBO,
        vs_InFBO,
        vs_ScreenResolution,
        iChannelResolution0,
        iChannelResolution1,
        iChannelResolution2,
        iChannelResolution3,
        iResolution,
        iGlobalTime,
        iTimeDelta,
        iFrame,
        iFrameRate,
        /*iChannelTime,
        iChannelResolution,*/
        iMouse,
        iDate,
      })
    }
  }

  pub fn get_stoy(stoy: String) -> String {
    let mut shaderFragmentSource = String::from(r#"#version 410 core

out vec4 outFragmentColor;
uniform vec3 iResolution;
uniform float iGlobalTime;
uniform float iTimeDelta;
uniform int iFrame;
uniform int iFrameRate;
uniform float iChannelTime[4];
uniform vec3 iChannelResolution[4];
uniform vec4 iMouse;
uniform vec4 iDate;

uniform sampler2D iChannel0;
//uniform samplerCube iChannel0;
uniform sampler2D iChannel1;
//uniform samplerCube iChannel1;
uniform sampler2D iChannel2;
//uniform samplerCube iChannel2;
uniform sampler2D iChannel3;
//uniform samplerCube iChannel3;

#define texture2D texture
#define textureCube texture
"#);
    if stoy.is_empty() {
      shaderFragmentSource.push_str(r#"
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
   vec2 uv = fragCoord.xy / iResolution.xy;
   fragColor = vec4(uv, 0.5 + 0.5 * sin(iGlobalTime), 1.0);
}
"#);
    }
    else {
      let shader_source = utils::file_io::read_shadertoy_shader(format!("{}.stoy", stoy).as_str());
      shaderFragmentSource.push_str(shader_source.unwrap().as_str());
    }
    shaderFragmentSource.push_str(r#"
void glslOptimizerFix() {
  float f = iTimeDelta * iChannelTime[0] * iChannelTime[1] * iChannelTime[2] * iChannelTime[3];
  vec3 v3 = iChannelResolution[0] * iChannelResolution[1] * iChannelResolution[2] * iChannelResolution[3];
  int i = iFrame * iFrameRate;
  vec4 v4 = iMouse * iDate;

  vec2 s1 = texture2D(iChannel0,vec2(0,0)).xy;
  vec2 s2 = texture2D(iChannel1,vec2(0,0)).xy;
  vec2 s3 = texture2D(iChannel2,vec2(0,0)).xy;
  vec2 s4 = texture2D(iChannel3,vec2(0,0)).xy;
}

void main() {
    glslOptimizerFix();
    vec4 color = vec4(0.0, 0.0, 0.0, 1.0);
    mainImage(color, gl_FragCoord.xy);
    outFragmentColor = color;
}
"#);
    shaderFragmentSource
  }

  pub fn reload_shadertoy(&mut self, stoy: &str, gl: &glow::Context) {
    Self::compile_shadertoy(gl, stoy.to_string());
  }

  pub fn setup_fbo(&self, gl: &glow::Context, screen_width: f32, screen_height: f32) {
    unsafe {
      gl.bind_renderbuffer(0, Some(self.tRBO));
      gl.renderbuffer_storage(0, glow::DEPTH_COMPONENT, screen_width as i32, screen_height as i32);
      gl.bind_renderbuffer(0, None);
    }
  }

  pub fn paint(&self, gl: &glow::Context, screen_width: f32, screen_height: f32) {
    unsafe {
      gl.use_program(Some(self.shaderProgram));
      gl.bind_vertex_array(Some(self.glVAO));

      gl.uniform_2_f32(Option::from(&self.vs_ScreenResolution), screen_width, screen_height);
      gl.uniform_3_f32(Option::from(&self.iResolution), screen_width, screen_height, 1.0);
      gl.uniform_1_f32(Option::from(&self.iGlobalTime), 1.0);
      gl.uniform_1_f32(Option::from(&self.iTimeDelta), 1.0);

      gl.draw_arrays(glow::TRIANGLES, 0, 6);

      gl.bind_vertex_array(None);
    }
  }

  pub fn destroy(&self, gl: &glow::Context) {
    do_log!("[Kuplung] [ShaderToy-Engine] DESTROY!");
    use glow::HasContext as _;
    unsafe {
      gl.delete_program(self.shaderProgram);
      gl.delete_vertex_array(self.glVAO);
      gl.delete_buffer(self.vboVertices);
    }
  }
}