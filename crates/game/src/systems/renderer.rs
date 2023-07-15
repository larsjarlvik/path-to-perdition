use crate::{components, resources};
use bevy_ecs::system::{Query, Res};
use std::iter;

/** Renders a model using the PBR pipeline */
pub(crate) fn render_model(
    ctx: Res<resources::RenderContext>,
    surface: Res<resources::Surface>,
    pbr: Option<Res<resources::Pbr>>,
    query: Query<&components::Model>,
) {
    let frame = surface
        .surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        if let Some(pbr) = &pbr {
            for model in query.iter() {
                render_pass.set_pipeline(&pbr.render_pipeline);
                render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
                render_pass.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..model.num_indices, 0, 0..1);
            }
        }
    }

    ctx.queue.submit(iter::once(encoder.finish()));
    frame.present();

    surface.window.request_redraw();
}
