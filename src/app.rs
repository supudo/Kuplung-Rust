use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::platform::windows::WindowExtWindows;

#[cfg(windows)]
use winit::platform::windows::IconExtWindows;
#[cfg(windows)]
use winit::window::Icon;

extern crate gl;

pub fn kuplung_app() {
    create_window();
}

fn create_window() {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_title(configuration::APP_TITLE)
        .with_resizable(true)
        .with_inner_size(winit::dpi::LogicalSize::new(configuration::WINDOW_WIDTH, configuration::WINDOW_HEIGHT))
        .with_window_icon(Some(Icon::from_resource(configuration::WINDOWS_ICON_RESOURCE_ID, None).expect("Can't load icon!")));
    let window = builder.build(&event_loop).unwrap();

    gl::load_with(|_s| window.hwnd() as *const _);
    gl::Viewport::load_with(|_s| window.hwnd() as *const _);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}