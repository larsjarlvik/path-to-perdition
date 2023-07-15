use bevy_ecs::world;
use std::iter;

use crate::resources;

pub(crate) struct App {
    pub world: world::World,
}

impl App {
    pub fn new(instance: wgpu::Instance) -> Self {
        let mut world: world::World = world::World::new();

        world.insert_resource(resources::State {
            instance,
            adapter: None,
            surface_state: None,
            render_state: None,
        });

        Self { world }
    }

    pub fn render(&self) {
        if let Some(state) = self.world.get_resource::<resources::State>() {
            if let Some(ref surface_state) = state.surface_state {
                if let Some(ref rs) = state.render_state {
                    let frame = surface_state
                        .surface
                        .get_current_texture()
                        .expect("Failed to acquire next swap chain texture");
                    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder = rs.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                    {
                        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: None,
                        });
                    }

                    rs.queue.submit(iter::once(encoder.finish()));
                    frame.present();
                }
            }
        }
    }
}
