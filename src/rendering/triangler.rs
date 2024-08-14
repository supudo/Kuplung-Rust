use eframe::egui_glow;
use egui_glow::glow;

use log::warn;

pub struct Triangler {
  program: glow::Program,
  vertex_array: glow::VertexArray,
}

#[allow(unsafe_code)]
impl Triangler {
  pub fn new(gl: &glow::Context) -> Option<Self> {
    use glow::HasContext as _;
    let shader_version = egui_glow::ShaderVersion::get(gl);
    unsafe {
      let program = gl.create_program().expect("[Kuplung] [Triangler] Cannot create program");

      if !shader_version.is_new_shader_interface() {
        warn!("[Kuplung] [Triangler] Custom 3D painting hasn't been ported to {:?}", shader_version);
        return None;
      }

      let (vertex_shader_source, fragment_shader_source) = (
        r#"
                    const vec2 verts[3] = vec2[3](
                        vec2(0.0, 1.0),
                        vec2(-1.0, -1.0),
                        vec2(1.0, -1.0)
                    );
                    const vec4 colors[3] = vec4[3](
                        vec4(1.0, 0.0, 0.0, 1.0),
                        vec4(0.0, 1.0, 0.0, 1.0),
                        vec4(0.0, 0.0, 1.0, 1.0)
                    );
                    out vec4 v_color;
                    uniform float u_angle;
                    void main() {
                        v_color = colors[gl_VertexID];
                        gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                        gl_Position.x *= cos(u_angle);
                    }
                "#,
        r#"
                    precision mediump float;
                    in vec4 v_color;
                    out vec4 out_color;
                    void main() {
                        out_color = v_color;
                    }
                "#,
      );

      let shader_sources = [
        (glow::VERTEX_SHADER, vertex_shader_source),
        (glow::FRAGMENT_SHADER, fragment_shader_source),
      ];

      let shaders: Vec<_> = shader_sources
        .iter()
        .map(|(shader_type, shader_source)| {
          let shader = gl.create_shader(*shader_type).expect("[Kuplung] [Triangler] Cannot create shader");
          gl.shader_source(shader, &format!("{}\n{}", shader_version.version_declaration(), shader_source));
          gl.compile_shader(shader);
          assert!(gl.get_shader_compile_status(shader), "[Kuplung] [Triangler] Failed to compile custom_3d_glow {shader_type}: {}", gl.get_shader_info_log(shader));
          gl.attach_shader(program, shader);
          shader
        })
        .collect();

      gl.link_program(program);
      assert!(gl.get_program_link_status(program), "{}", gl.get_program_info_log(program));

      for shader in shaders {
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
      }

      let vertex_array = gl.create_vertex_array().expect("[Kuplung] [Triangler] Cannot create vertex array");

      Some(Self {
        program,
        vertex_array,
      })
    }
  }

  pub fn destroy(&self, gl: &glow::Context) {
    use glow::HasContext as _;
    unsafe {
      gl.delete_program(self.program);
      gl.delete_vertex_array(self.vertex_array);
    }
  }

  pub fn paint(&self, gl: &glow::Context, angle: f32) {
    use glow::HasContext as _;
    unsafe {
      gl.use_program(Some(self.program));
      gl.uniform_1_f32(gl.get_uniform_location(self.program, "u_angle").as_ref(), angle);
      gl.bind_vertex_array(Some(self.vertex_array));
      gl.draw_arrays(glow::TRIANGLES, 0, 3);
    }
  }
}
