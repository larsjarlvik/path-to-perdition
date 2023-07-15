use bevy_ecs::system::Resource;
use winit::event_loop::EventLoopWindowTarget;

/** Window and render surface */
#[derive(Resource)]
pub struct Surface {
    pub window: winit::window::Window,
    pub surface: wgpu::Surface,
}

impl Surface {
    pub fn new<T>(instance: &wgpu::Instance, event_loop: &EventLoopWindowTarget<T>) -> Self {
        let window = winit::window::Window::new(event_loop).unwrap();
        window.set_title("Path to Perdition");

        log::info!("WGPU: creating surface for native window");
        let surface = unsafe { instance.create_surface(&window).unwrap() };

        Self { window, surface }
    }
}
