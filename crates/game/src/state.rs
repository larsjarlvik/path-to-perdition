use crate::resources::{self, Surface};
use bevy_ecs::world;
use winit::event_loop::EventLoopWindowTarget;

pub(crate) struct State {
    pub world: world::World,
    pub instance: wgpu::Instance,
}

impl State {
    async fn ensure_render_context_for_surface(&mut self) {
        if let Some(surface) = &self.world.get_resource::<Surface>() {
            if self.world.get_resource::<resources::RenderContext>().is_none() {
                log::info!("WGPU: finding supported swapchain format");

                let caps = surface.surface.get_capabilities(&surface.adapter);
                let swapchain_format = caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(caps.formats[0]);

                let ctx = resources::RenderContext::new(&surface.adapter, swapchain_format).await;
                self.world.insert_resource(ctx);
            }
        }
    }

    /** Setup the surface if we have one and a render context */
    pub fn configure_surface(&mut self) {
        if let Some(ctx) = self.world.get_resource::<resources::RenderContext>() {
            if let Some(surface) = &self.world.get_resource::<Surface>() {
                let swapchain_format = ctx.target_format;
                let size = surface.window.inner_size();

                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: swapchain_format,
                    width: size.width,
                    height: size.height,
                    present_mode: wgpu::PresentMode::AutoNoVsync,
                    alpha_mode: wgpu::CompositeAlphaMode::Auto,
                    view_formats: vec![],
                };

                log::info!("WGPU: Configuring surface swapchain: format = {swapchain_format:?}, size = {size:?}");
                surface.surface.configure(&ctx.device, &config);
            }
        }
    }

    /** Re-creates and configures the surface on resume */
    pub fn resume<T>(&mut self, event_loop: &EventLoopWindowTarget<T>) {
        log::info!("Resumed, creating render state...");

        self.world
            .insert_resource(pollster::block_on(resources::Surface::new(&self.instance, event_loop)));
        pollster::block_on(self.ensure_render_context_for_surface());

        self.configure_surface();
    }

    /** Delete the render context when app is paused */
    pub fn pause(&mut self) {
        log::info!("Paused, dropping render state...");
        self.world.remove_resource::<resources::RenderContext>();
    }
}
