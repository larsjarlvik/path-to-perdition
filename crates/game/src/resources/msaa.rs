use bevy_ecs::system::Resource;

/** Window and render surface */
#[derive(Resource)]
pub struct Msaa {
    pub view: wgpu::TextureView,
    pub sample_count: u32,
}

impl Msaa {
    pub fn new(ctx: &super::RenderContext, sample_count: u32, width: u32, height: u32) -> Self {
        let multisampled_texture_extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
            size: multisampled_texture_extent,
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: ctx.target_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        };

        let view = ctx
            .device
            .create_texture(multisampled_frame_descriptor)
            .create_view(&wgpu::TextureViewDescriptor::default());

        Self { view, sample_count }
    }

    /** Returns the max supported sample count for the device */
    pub fn get_max_sample_count(surface: &super::Surface, target_format: wgpu::TextureFormat) -> u32 {
        let sample_flags = surface.adapter.get_texture_format_features(target_format).flags;
        if sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X16) {
            16
        } else if sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X8) {
            8
        } else if sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X4) {
            4
        } else if sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X2) {
            2
        } else {
            1
        }
    }
}
