extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate imgui_winit_support;

use imgui::*;

mod utilities;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

fn main() {
  utilities::ui::GUIRun("Kuplung".to_owned(), CLEAR_COLOR, hello_world);
}

fn hello_world<'a>(ui: &Ui<'a>) -> bool {
  let mut open = true;
  ui.show_demo_window(&mut open);
  open
  // ui.window(im_str!("Test Window"))
  //     .size((300.0, 100.0), ImGuiCond::FirstUseEver)
  //     .build(|| {
  //         ui.text(im_str!("Hello world!"));
  //         ui.text(im_str!("こんにちは世界！"));
  //         ui.text(im_str!("This...is...imgui-rs!"));
  //         ui.separator();
  //         let mouse_pos = ui.imgui().mouse_pos();
  //         ui.text(im_str!(
  //             "Mouse Position: ({:.1},{:.1})",
  //             mouse_pos.0,
  //             mouse_pos.1
  //         ));
  //     });

  // true
}
