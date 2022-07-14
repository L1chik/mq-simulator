use macroquad::prelude::*;
use macroquad::math::{Vec3, Mat3, Vec4};

use crate::{
    ray::Ray,
    world_axes,
    components::{Input, Physics, Render, Draw,},
    };

pub struct MainCamera {
    pub position: Vec3,
    pub local_axes: Mat3,
    last_mouse_pos: Vec2,
    is_panning: bool,
    is_looking: bool,
    cast: bool,
    zoomming: f32,
    look_speed: f32,
}

impl Default for MainCamera {
    fn default() -> MainCamera{
        MainCamera{
            position: vec3(0., 0., -10.),
            local_axes: world_axes,
            last_mouse_pos: vec2(0., 0.),
            is_panning: false,
            is_looking: false,
            cast: false,
            zoomming: 0.,
            look_speed: 0.2,
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

impl Input for MainCamera {
    fn input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.cast = true;
        }

        if is_mouse_button_pressed(MouseButton::Middle) {
            self.last_mouse_pos = mouse_position().into();
            self.is_panning = !self.is_looking;
        }
        
        if is_mouse_button_released(MouseButton::Middle) {
            self.is_panning = false; 
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.last_mouse_pos = mouse_position().into();
            self.is_looking = !self.is_panning;
        }

        if is_mouse_button_released(MouseButton::Right) {
            self.is_looking = false;
        }

        self.zoomming = mouse_wheel().1;
    } 
}

impl Physics for MainCamera {
    fn update(&mut self) {
        let delta = get_frame_time();
        if self.is_panning {
            let mouse_pos: Vec2 = mouse_position().into();
            let delta_mouse_pos = mouse_pos - self.last_mouse_pos;
            self.last_mouse_pos = mouse_pos;
            let mouse_x = delta_mouse_pos.extend(0.);
            let transition = world_axes.inverse().mul_mat3(&self.local_axes);
            let new_x = transition.mul_vec3(mouse_x);
            self.position += new_x * delta;
        }

        if self.is_looking {
            let mouse_pos: Vec2 = mouse_position().into();
            let delta_mouse_pos = self.last_mouse_pos - mouse_pos;
            self.last_mouse_pos = mouse_pos;
            let angle = delta_mouse_pos * self.look_speed * delta;
            let rotate_mat = Mat3::from_axis_angle(self.local_axes.x_axis, angle.y * -1.);
		let rotate_mat2 = Mat3::from_axis_angle(world_axes.y_axis, angle.x);
	    	let sum_rotate = rotate_mat.mul_mat3(&rotate_mat2);
            //let rot = Quat::from_rotation_ypr(angle.x * -1., angle.y, 0.);    
            self.local_axes.z_axis = sum_rotate.mul_vec3(self.local_axes.z_axis).normalize();
            self.local_axes.x_axis = world_axes.y_axis.cross(self.local_axes.z_axis).normalize();
            self.local_axes.y_axis = self.local_axes.z_axis.cross(self.local_axes.x_axis).normalize();
            //main_camera.local = sum_rotate.mul_mat3(&main_camera.local); 
            //main_camera.local.y_axis = rotate_mat.mul_vec3(main_camera.local.y_axis).normalize();
            //main_camera.local.z_axis = rotate_mat.mul_vec3(main_camera.local.z_axis).normalize();
            //main_camera.local.x_axis = rotate_mat2.mul_vec3(main_camera.local.x_axis).normalize();
            //main_camera.local.z_axis = rotate_mat2.mul_vec3(main_camera.local.z_axis).normalize();
        }
        
        self.position += self.local_axes.z_axis * self.zoomming;
    }
}

impl Render for MainCamera {
    fn render(&self){
        clear_background(LIGHTGRAY);
        let cam = self.get_view();
        set_camera(&cam);
        draw_grid(50, 10., BLACK, GRAY);
    }
}
	
impl Draw for MainCamera {
    fn draw(&self) {
        set_default_camera();
    }
}
