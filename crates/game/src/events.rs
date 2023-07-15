use crate::state::State;
use winit::event_loop::EventLoop;
use winit::{event::*, event_loop::*};

pub async fn run(event_loop: EventLoop<()>) {
    log::info!("Running mainloop...");

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let mut state = State::new(instance);

    event_loop.run(move |event, event_loop, control_flow| {
        log::info!("Received Winit event: {event:?}");

        *control_flow = ControlFlow::Wait;
        match event {
            Event::Resumed => {
                state.resume(event_loop);
            }
            Event::Suspended => {
                log::info!("Suspended, dropping render state...");
                state.render_state = None;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {
                state.configure_surface_swapchain();
                // Winit: doesn't currently implicitly request a redraw
                // for a resize which may be required on some platforms...
                state.queue_redraw();
            }
            Event::RedrawRequested(_) => {
                log::info!("Handling Redraw Request");
                state.render();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event: _, .. } => {
                log::info!("Window event {:#?}", event);
            }
            _ => {}
        }
    });
}
