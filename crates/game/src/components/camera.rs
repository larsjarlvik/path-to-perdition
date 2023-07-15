use bevy_ecs::prelude::*;
use cgmath::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

/** 3D Camera */
#[derive(Component)]
pub(crate) struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    /** Creates a new camera looking at zero */
    pub fn new(eye: Point3<f32>) -> Self {
        Self {
            eye,
            target: Point3::new(0.0, 0.0, 0.0),
            up: cgmath::Vector3::unit_y(),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    /** Returns a view projection matrix based on current values */
    pub fn build_view_projection_matrix(&self, aspect: f32) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}
