use bevy_ecs::system::Resource;
use winit::event_loop::EventLoopWindowTarget;

/** Window and render surface */
#[derive(Resource)]
pub struct Surface {
    pub window: winit::window::Window,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
}

impl Surface {
    pub async fn new<T>(instance: &wgpu::Instance, event_loop: &EventLoopWindowTarget<T>) -> Self {
        let window = winit::window::Window::new(event_loop).unwrap();
        window.set_title("Path to Perdition");

        log::info!("WGPU: creating surface for native window");
        let surface = unsafe { instance.create_surface(&window).unwrap() };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        Self { window, surface, adapter }
    }
}
