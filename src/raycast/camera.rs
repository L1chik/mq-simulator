use macroquad::prelude::*;
use macroquad::math::{Vec3, Mat3, Vec4};

use crate::ray::Ray;

pub struct MainCamera {
    pub position: Vec3,
    pub local_axes: Mat3
}

impl Default for MainCamera {
    fn default() -> MainCamera{
        MainCamera{
            position: vec3(0., 0., -10.),
            local_axes: crate::world_axes,
        }
    }
}

impl MainCamera {
    pub fn get_view(&self) -> Camera3D{
        Camera3D{
            position: self.position,
            up: self.local_axes.y_axis,
            target: self.position + self.local_axes.z_axis,
            ..Default::default()
        }
    }

    pub fn screen_point_to_ray(&self, point: Vec3) -> Ray {
        let mut point4 = vec4(point.x / (screen_width() * 0.5) - 1., point.y / -(screen_height() * 0.5) + 1., 0., 1.);
        let cam = self.get_view();
        point4 = Mat4::perspective_rh_gl(cam.fovy, cam.aspect.unwrap_or(screen_width() / screen_height()), 0.01, 10000.0).inverse().mul_vec4(point4);

        let mat_view = Mat4::look_at_rh(cam.position, cam.target, cam.up).inverse();
        let mut ray_pos = vec4(0., 0., 0., 1.);
        let mut ray_dir = point4 - ray_pos;
        ray_pos = mat_view.mul_vec4(ray_pos);
        ray_dir = mat_view.mul_vec4(ray_dir);

        Ray{
            origin: ray_pos.truncate(),
            direction: (ray_dir / ray_dir.w - ray_pos).truncate().normalize(),
        }
    }

    pub fn screen_point_to_world_point(&self, point: Vec4) -> Vec4 {
        let world_point = self.get_view().matrix().inverse().mul_vec4(point);
        world_point / world_point.w
    }

    pub fn world_point_to_screen(&self, point: Vec3) -> Vec4 {
        let screen_point = self.get_view().matrix().mul_vec4(point.extend(1.));
        screen_point / screen_point.w
    }
}
