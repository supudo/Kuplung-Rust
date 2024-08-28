#![allow(non_snake_case)]

use eframe::egui_glow;
use eframe::glow::{HasContext, NativeUniformLocation};
use egui_glow::glow;
use log::error;
use crate::do_log;
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
  iChannelTime: [glow::UniformLocation; 4],
  iChannelResolution: [glow::UniformLocation; 4],
  iMouse: glow::UniformLocation,
  iDate: glow::UniformLocation,
  iChannel0: glow::UniformLocation,
  iChannel1: glow::UniformLocation,
  iChannel2: glow::UniformLocation,
  iChannel3: glow::UniformLocation,
  tFBO: glow::Framebuffer,
  tRBO: glow::Renderbuffer,
}

#[allow(unsafe_code)]
impl ShaderToyEngine {
  pub fn new(gl: &glow::Context) -> Option<Self> {
    use glow::HasContext as _;
    unsafe {
      let shaderProgram = gl.create_program().expect("[Kuplung] [ShaderToy-Engine] Cannot create program!");

      let shader_vertex = gl_utils::create_shader(&shaderProgram, &gl, glow::VERTEX_SHADER, "assets/shaders/shadertoy/shadertoy.vert");

      let stoy = Self::get_stoy("".to_string());
      let shader_fragment = gl_utils::create_shader_from_string(&shaderProgram, &gl, glow::FRAGMENT_SHADER, stoy.as_ref());

      gl.link_program(shaderProgram);
      if !gl.get_program_link_status(shaderProgram) {
        error!("[Kuplung] [ShaderToy-Engine] Program cannot be linked! {}", gl.get_program_info_log(shaderProgram));
        panic!("[Kuplung] [ShaderToy-Engine] Program cannot be linked! {}", gl.get_program_info_log(shaderProgram));
      }

      gl.detach_shader(shaderProgram, shader_vertex);
      gl.delete_shader(shader_vertex);
      gl.detach_shader(shaderProgram, shader_fragment);
      gl.delete_shader(shader_fragment);

      let vs_InFBO = gl.get_uniform_location(shaderProgram, "vs_inFBO")?;
      let vs_ScreenResolution = gl.get_uniform_location(shaderProgram, "vs_screenResolution")?;

      let iResolution = NativeUniformLocation(0);//gl.get_uniform_location(shaderProgram, "iResolution")?;
      let iGlobalTime = NativeUniformLocation(0);//gl.get_uniform_location(shaderProgram, "iGlobalTime")?;
      let iTimeDelta = NativeUniformLocation(0);//gl.get_uniform_location(shaderProgram, "iTimeDelta")?;
      let iFrameRate = NativeUniformLocation(0);//gl.get_uniform_location(shaderProgram, "iFrameRate")?;
      let iFrame = NativeUniformLocation(0);//gl.get_uniform_location(shaderProgram, "iFrame")?;
      let iChannelTime: [glow::UniformLocation; 4] = [
        NativeUniformLocation(0),//gl.get_uniform_location(shaderProgram, "iChannelTime[0]")?,
        NativeUniformLocation(0),//gl.get_uniform_location(shaderProgram, "iChannelTime[1]")?,
        NativeUniformLocation(0),//gl.get_uniform_location(shaderProgram, "iChannelTime[2]")?,
        NativeUniformLocation(0),//gl.get_uniform_location(shaderProgram, "iChannelTime[3]")?,
      ];
      let iChannelResolution: [glow::UniformLocation; 4] = [
        NativeUniformLocation(0),//gl.get_uniform_location(shaderProgram, "iChannelResolution[0]")?,
        NativeUniformLocation(0),//gl.get_uniform_location(shaderProgram, "iChannelResolution[1]")?,
        NativeUniformLocation(0),//gl.get_uniform_location(shaderProgram, "iChannelResolution[2]")?,
        NativeUniformLocation(0),//gl.get_uniform_location(shaderProgram, "iChannelResolution[3]")?,
      ];
      let iMouse = gl.get_uniform_location(shaderProgram, "iMouse")?;
      let iDate = gl.get_uniform_location(shaderProgram, "iDate")?;
      let iChannel0 = gl.get_uniform_location(shaderProgram, "iChannel0")?;
      let iChannel1 = gl.get_uniform_location(shaderProgram, "iChannel1")?;
      let iChannel2 = gl.get_uniform_location(shaderProgram, "iChannel2")?;
      let iChannel3 = gl.get_uniform_location(shaderProgram, "iChannel3")?;

      let tFBO = gl.create_framebuffer().expect("[Kuplung] [ShaderToy-Engine] Cannot create FBO!");
      let tRBO = gl.create_renderbuffer().expect("[Kuplung] [ShaderToy-Engine] Cannot create RBO!");

      let glVAO = gl.create_vertex_array().expect("[Kuplung] [ShaderToy-Engine] Cannot create VAO!");
      gl.bind_vertex_array(Some(glVAO));

      let vboVertices = gl.create_buffer().expect("[Kuplung] [ShaderToy-Engine] Cannot create VBO!");
      gl.bind_buffer(glow::ARRAY_BUFFER, Some(vboVertices));
      gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&SHADERTOY_VERTICES[..]), glow::STATIC_DRAW);

      gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);
      gl.enable_vertex_attrib_array(0);

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
        iChannelTime,
        iChannelResolution,
        iMouse,
        iDate,
        iChannel0,
        iChannel1,
        iChannel2,
        iChannel3,
        tFBO,
        tRBO
      })
    }
  }

  pub fn get_stoy(stoy: String) -> String {
    let mut shaderFragmentSource: String = "#version 410 core\n
\n
out vec4 outFragmentColor;\n
uniform vec3 iResolution;\n
uniform float iGlobalTime;\n
uniform float iTimeDelta;\n
uniform int iFrame;\n
uniform int iFrameRate;\n
uniform float iChannelTime[4];\n
uniform vec3 iChannelResolution[4];\n
uniform vec4 iMouse;\n
uniform vec4 iDate;\n
\n
uniform sampler2D iChannel0;\n
//uniform samplerCube iChannel0;\n
uniform sampler2D iChannel1;\n
//uniform samplerCube iChannel1;\n
uniform sampler2D iChannel2;\n
//uniform samplerCube iChannel2;\n
uniform sampler2D iChannel3;\n
//uniform samplerCube iChannel3;\n
\n
#define texture2D texture\n
//#define textureCube texture\n
\n".to_owned();

    if stoy.is_empty() {
      shaderFragmentSource.push_str("\n
void mainImage(out vec4 fragColor, in vec2 fragCoord) {\n
   vec2 uv = fragCoord.xy / iResolution.xy;\n
   fragColor = vec4(uv, 0.5 + 0.5 * sin(iGlobalTime), 1.0);\n
}\n\n");
    }

    shaderFragmentSource.push_str("\n
void main() {\n
    vec4 color = vec4(0.0, 0.0, 0.0, 1.0);\n
    mainImage(color, gl_FragCoord.xy);\n
    outFragmentColor = color;\n
}\n
\n");
    shaderFragmentSource
  }

  pub fn setup_fbo(&self, gl: &glow::Context, screen_width: f32, screen_height: f32) {
    unsafe {
      gl.bind_renderbuffer(0, Some(self.tRBO));
      gl.renderbuffer_storage(0, glow::DEPTH_COMPONENT, screen_width as i32, screen_height as i32);
      gl.bind_renderbuffer(0, None);
    }
  }

  pub fn paint(&self, gl: &glow::Context, toy: &str, screen_width: f32, screen_height: f32) {
    unsafe {
      gl.use_program(Some(self.shaderProgram));
      gl.bind_vertex_array(Some(self.glVAO));

      gl.uniform_1_f32(gl.get_uniform_location(self.shaderProgram, "u_window_width").as_ref(), screen_width);
      gl.uniform_1_f32(gl.get_uniform_location(self.shaderProgram, "u_window_height").as_ref(), screen_height);

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