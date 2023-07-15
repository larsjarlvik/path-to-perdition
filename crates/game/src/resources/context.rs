use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct RenderContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub target_format: wgpu::TextureFormat,
}

impl RenderContext {
    pub async fn new(adapter: &wgpu::Adapter, target_format: wgpu::TextureFormat) -> Self {
        log::info!("Initializing render state");
        log::info!("WGPU: requesting device");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        log::info!("WGPU: loading shader");

        Self {
            device,
            queue,
            target_format,
        }
    }
}
