use crate::{resources, vertex};
use bevy_ecs::prelude::*;
use wgpu::util::DeviceExt;

#[derive(Component)]
pub(crate) struct Model {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Model {
    pub fn new(ctx: &resources::RenderContext) -> Self {
        let vertex_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("model_vertex_buffer"),
            contents: bytemuck::cast_slice(vertex::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(vertex::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: vertex::INDICES.len() as u32,
        }
    }
}
