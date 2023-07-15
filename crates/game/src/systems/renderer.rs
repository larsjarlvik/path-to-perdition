use crate::{components, resources};
use bevy_ecs::system::{Query, Res};
use std::iter;

/** Renders all models using the PBR pipeline for each active camera */
pub(crate) fn render_model(
    ctx: Res<resources::RenderContext>,
    surface: Res<resources::Surface>,
    pbr: Option<Res<resources::Pbr>>,
    model_query: Query<&components::Model>,
    camera_query: Query<&components::Camera>,
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
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        let size = surface.window.inner_size();

        if let Some(pbr) = &pbr {
            for camera in camera_query.iter() {
                let view_proj = camera.build_view_projection_matrix(size.width as f32 / size.height as f32);
                ctx.queue.write_buffer(
                    &pbr.uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[resources::pbr::Uniforms {
                        view_proj: view_proj.into(),
                    }]),
                );

                for model in model_query.iter() {
                    render_pass.set_pipeline(&pbr.render_pipeline);
                    render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                    render_pass.set_bind_group(0, &pbr.uniform_bind_group, &[]);
                    render_pass.draw_indexed(0..model.num_indices, 0, 0..1);
                }
            }
        }
    }

    ctx.queue.submit(iter::once(encoder.finish()));
    frame.present();

    surface.window.request_redraw();
}
