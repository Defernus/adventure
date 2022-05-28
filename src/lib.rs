use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub mod app_state;
pub mod camera;
pub mod sun;
#[cfg(test)]
pub mod tests;
pub mod texture;
pub mod utils;
pub mod vec;
pub mod vertex;
pub mod world;

fn handle_window_input(
    event: &WindowEvent,
    state: &mut app_state::AppState,
    control_flow: &mut ControlFlow,
) {
    if state.window_input(event) {
        return;
    }

    match event {
        WindowEvent::CloseRequested
        | WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
            ..
        } => *control_flow = ControlFlow::Exit,
        WindowEvent::Resized(physical_size) => {
            state.resize(*physical_size);
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            state.resize(**new_inner_size);
        }
        _ => {}
    }
}

fn handle_redraw(state: &mut app_state::AppState, control_flow: &mut ControlFlow, window: &Window) {
    state.update(window);
    match state.render() {
        Ok(_) => {}
        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
            state.resize(state.get_size())
        }
        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
    }
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = app_state::AppState::new(&window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            handle_window_input(event, &mut state, control_flow);
        }
        Event::DeviceEvent { ref event, .. } => {
            state.device_input(event);
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            handle_redraw(&mut state, control_flow, &window);
        }
        Event::RedrawEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
