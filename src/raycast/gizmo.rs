use macroquad::prelude::*;

use crate::{
    world_axes,
    components::{Input, Physics, Render, Draw,},
    objects::{Transform},
    camera::MainCamera,
};

enum TransformationKind{
    TRANSLATION,
    ROTATION,
    SCALE,
}

struct Arrow{
    line: (Vec2, Vec2),
    triangle: (Vec2, Vec2, Vec2),
    color: Color,
    is_active: bool,
    active_color: Color,
}

impl Arrow {
    fn draw(&self) {
        draw_line(self.line.0.x, self.line.0.y, self.line.1.x, self.line.1.y, 3., if self.is_active {self.active_color} else {self.color});
        draw_triangle(self.triangle.0, self.triangle.1, self.triangle.2, if self.is_active {self.active_color} else {self.color});
    }

    fn calc(&mut self, transform: Transform, dir: Vec3, camera: &MainCamera) {
        let center = camera.world_point_to_screen(transform.position);
        let tip = camera.world_point_to_screen(transform.position + (Quat::from_rotation_ypr(transform.rotation.x, transform.rotation.y, transform.rotation.z).mul_vec3(dir)) * (camera.position.distance(transform.position) * 5. / 30.));
        let end = camera.world_point_to_screen(transform.position + (Quat::from_rotation_ypr(transform.rotation.x, transform.rotation.y, transform.rotation.z).mul_vec3(dir * 1.3)) * (camera.position.distance(transform.position) * 5. / 30.));
        let n = camera.local_axes.z_axis.cross(Quat::from_rotation_ypr(transform.rotation.x, transform.rotation.y, transform.rotation.z).mul_vec3(dir)).normalize();
        let left = camera.world_point_to_screen(transform.position + (Quat::from_rotation_ypr(transform.rotation.x, transform.rotation.y, transform.rotation.z).mul_vec3(dir) + n * 0.1) * (camera.position.distance(transform.position) * 5. / 30.));
        let right = camera.world_point_to_screen(transform.position + (Quat::from_rotation_ypr(transform.rotation.x, transform.rotation.y, transform.rotation.z).mul_vec3(dir) - n * 0.1) * (camera.position.distance(transform.position) * 5. / 30.));
        //let v = (tip - center).normalize();
        //let y = -v.x / v.y;
        //let n = vec2(1., y).normalize();
        self.line = (center, tip);
        self.triangle = (end, left, right);
    }

    fn input(&self) -> bool {
        let o: Vec2 = mouse_position().into();
        //inside triangle
        let u = (self.triangle.0.x - o.x) * (self.triangle.1.y - self.triangle.0.y) - (self.triangle.1.x - self.triangle.0.x) * (self.triangle.0.y - o.y);
        let v = (self.triangle.1.x - o.x) * (self.triangle.2.y - self.triangle.1.y) - (self.triangle.2.x - self.triangle.1.x) * (self.triangle.1.y - o.y);
        let w = (self.triangle.2.x - o.x) * (self.triangle.0.y - self.triangle.2.y) - (self.triangle.0.x - self.triangle.2.x) * (self.triangle.2.y - o.y);
        //on line
        //TODO make right condition
        let l = self.line.1 - self.line.0;
        let t_vec = (o - self.line.0) / l;

        println!("{:?}", t_vec);

        u >= 0. && v >= 0. && w >= 0. || u <= 0. && v <= 0. && w <= 0. || t_vec.x == t_vec.y
    }
}

pub struct Gizmo {
    kind: TransformationKind,
    is_local: bool,
    up_arrow: Arrow,
    right_arrow: Arrow,
    forward_arrow: Arrow,
}

impl Default for Gizmo {
    fn default() -> Gizmo {
        Gizmo{
            kind: TransformationKind::TRANSLATION,
            is_local: false,
            up_arrow: Arrow{line: (vec2(0., 0.), vec2(0., 0.)), triangle: (vec2(0., 0.), vec2(0., 0.), vec2(0., 0.)), color: GREEN, is_active: false, active_color: YELLOW,},
            right_arrow: Arrow{line: (vec2(0., 0.), vec2(0., 0.)), triangle: (vec2(0., 0.), vec2(0., 0.), vec2(0., 0.)), color: RED, is_active: false, active_color: YELLOW,},
            forward_arrow: Arrow{line: (vec2(0., 0.), vec2(0., 0.)), triangle: (vec2(0., 0.), vec2(0., 0.), vec2(0., 0.)), color: BLUE, is_active: false, active_color: YELLOW,},
        }
    }
}

impl Gizmo {
    pub fn calc(&mut self, transform: Transform, is_local: bool, camera: &MainCamera) {
        self.up_arrow.calc(transform, world_axes.y_axis, camera);
        self.right_arrow.calc(transform, world_axes.x_axis, camera);
        self.forward_arrow.calc(transform, world_axes.z_axis, camera);
    }

    pub fn input(&mut self) -> Option<Vec3> {
        if self.right_arrow.input() {
            self.right_arrow.is_active = true;
            Some(world_axes.x_axis)
        } else if self.forward_arrow.input() {
            self.forward_arrow.is_active = true;
            Some(world_axes.z_axis)
        } else if self.up_arrow.input() {
            self.up_arrow.is_active = true;
            Some(world_axes.y_axis)
        } else {
            None
        }
    }

    pub fn deactive(&mut self) {
        self.right_arrow.is_active = false;
        self.forward_arrow.is_active = false;
        self.up_arrow.is_active = false;
    }
}


impl Draw for Gizmo {
    fn draw(&self) {
        self.right_arrow.draw();
        self.forward_arrow.draw();
        self.up_arrow.draw();
    }
}



