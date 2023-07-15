use std::iter;
use winit::event_loop::EventLoopWindowTarget;

pub(crate) struct RenderState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub target_format: wgpu::TextureFormat,
}

pub(crate) struct SurfaceState {
    pub window: winit::window::Window,
    pub surface: wgpu::Surface,
}

pub(crate) struct State {
    instance: wgpu::Instance,
    adapter: Option<wgpu::Adapter>,
    pub surface_state: Option<SurfaceState>,
    pub render_state: Option<RenderState>,
}

impl State {
    pub fn new(instance: wgpu::Instance) -> Self {
        Self {
            instance,
            adapter: None,
            surface_state: None,
            render_state: None,
        }
    }
}

impl State {
    pub fn create_surface<T>(&mut self, event_loop: &EventLoopWindowTarget<T>) {
        let window = winit::window::Window::new(event_loop).unwrap();
        log::info!("WGPU: creating surface for native window");
        let surface = unsafe { self.instance.create_surface(&window).unwrap() };
        self.surface_state = Some(SurfaceState { window, surface });
    }

    pub async fn init_render_state(adapter: &wgpu::Adapter, target_format: wgpu::TextureFormat) -> RenderState {
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

        RenderState {
            device,
            queue,
            target_format,
        }
    }

    pub async fn ensure_render_state_for_surface(&mut self) {
        if let Some(surface_state) = &self.surface_state {
            if self.adapter.is_none() {
                log::info!("WGPU: requesting a suitable adapter (compatible with our surface)");
                let adapter = self
                    .instance
                    .request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::default(),
                        force_fallback_adapter: false,
                        compatible_surface: Some(&surface_state.surface),
                    })
                    .await
                    .expect("Failed to find an appropriate adapter");

                self.adapter = Some(adapter);
            }
            let adapter = self.adapter.as_ref().unwrap();

            if self.render_state.is_none() {
                log::info!("WGPU: finding supported swapchain format");

                let surface_caps = surface_state.surface.get_capabilities(adapter);
                let swapchain_format = surface_caps
                    .formats
                    .iter()
                    .copied()
                    .find(|f| f.is_srgb())
                    .unwrap_or(surface_caps.formats[0]);

                let rs = Self::init_render_state(adapter, swapchain_format).await;
                self.render_state = Some(rs);
            }
        }
    }

    pub fn configure_surface_swapchain(&mut self) {
        if let (Some(render_state), Some(surface_state)) = (&self.render_state, &self.surface_state) {
            let swapchain_format = render_state.target_format;
            let size = surface_state.window.inner_size();

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
            surface_state.surface.configure(&render_state.device, &config);
        }
    }

    pub fn queue_redraw(&self) {
        if let Some(surface_state) = &self.surface_state {
            surface_state.window.request_redraw();
        }
    }

    pub fn resume<T>(&mut self, event_loop: &EventLoopWindowTarget<T>) {
        log::info!("Resumed, creating render state...");
        self.create_surface(event_loop);
        pollster::block_on(self.ensure_render_state_for_surface());
        self.configure_surface_swapchain();
        self.queue_redraw();
    }

    pub fn render(&self) {
        if let Some(ref surface_state) = self.surface_state {
            if let Some(ref rs) = self.render_state {
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
