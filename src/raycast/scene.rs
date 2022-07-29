use macroquad::prelude::*;
use crate::{
    set_scene,
    world_axes,
    objects::GameObject,
    camera::MainCamera,
    components::{Input, Physics, Render, Draw,},
    gizmo::Gizmo,
};


pub struct Scene<'a> {
    pub objects: Vec<GameObject>,
    pub selected: Option<&'a mut GameObject>,
    pub camera: MainCamera,
    casting: bool,
    transformation: Option<Vec3>,
    normal: Vec3,
    last_mouse_pos: Vec2,
    gizmo: Gizmo,
}

impl<'a> Scene<'a> {
    pub fn new() {
        let scene = Scene{
            objects: vec![],
            selected: None,
            camera: Default::default(),
            casting: false,
            transformation: None,
            normal: Vec3::ZERO,
            last_mouse_pos: vec2(0., 0.),
            gizmo: Default::default(),
        };
        set_scene(scene);
    }
    
    pub fn add_object(&mut self, obj: GameObject) {
        self.objects.push(obj);
    }
    
    fn draw_selected(&self, color: Color) {
        match &self.selected {
            Some(x) => {
                let mesh = x.get_transformed_mesh().unwrap();
                for i in 0..mesh.indices.len() / 3 {
                    let a = mesh.vertices[mesh.indices[3*i + 0] as usize].position;
                    let b = mesh.vertices[mesh.indices[3*i + 1] as usize].position;
                    let c = mesh.vertices[mesh.indices[3*i + 2] as usize].position;
                    //let A_dir = A - x.transform.position;
                    //let B_dir = B - x.transform.position;
                    //let C_dir = C - x.transform.position;
                    draw_line_3d(a, b, color);
                    draw_line_3d(b, c, color);
                    draw_line_3d(a, c, color);
                }
            },
            _ => ()
        }
    }

    pub async fn main_loop(&mut self) {
        loop{
            self.input();
            self.update();
            self.render();
            self.draw();
            next_frame().await
        }
    }
}


impl<'a> Input for Scene<'a> {
    fn input(&mut self) {
        self.camera.input();

        if is_mouse_button_pressed(MouseButton::Left) {
            self.transformation = self.gizmo.input();
            if self.transformation.is_some() && self.selected.is_some(){
                self.last_mouse_pos = mouse_position().into();
            } else {
                self.casting = true;
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.transformation = None;
            self.gizmo.deactive();
        }
    }
}

impl<'a> Physics for Scene<'a> {
    fn update(&mut self) {
        let delta = get_frame_time();
        self.camera.update();
        for obj in self.objects.iter_mut() {
            obj.update();
        }

        match (self.transformation, self.selected.as_mut()) {
            (Some(dir), Some(x)) => {
                let mouse_pos: Vec2 = mouse_position().into();
                let a = self.camera.screen_point_to_ray(self.last_mouse_pos.extend(1.)).raycast_hit_to_plane(x.transform.position, dir.cross(self.camera.local_axes.y_axis));
                let b = self.camera.screen_point_to_ray(mouse_pos.extend(1.)).raycast_hit_to_plane(x.transform.position, dir.cross(self.camera.local_axes.y_axis));
                println!("{:?}, {:?}", a, b);
                self.last_mouse_pos = mouse_pos;
                let ray = self.camera.screen_point_to_ray(mouse_pos.extend(1.));
                let offset = b - a;
                x.transform.translate(Quat::from_rotation_ypr(x.transform.rotation.x, x.transform.rotation.y, x.transform.rotation.z).mul_vec3(dir) * Quat::from_rotation_ypr(x.transform.rotation.x, x.transform.rotation.y, x.transform.rotation.z).mul_vec3(dir).dot(offset));
            },
            _ => ()
        }

        if self.casting {
            let mouse_pos: Vec2 = mouse_position().into();
            let ray = self.camera.screen_point_to_ray(mouse_pos.extend(1.));
            //TODO return the object
            self.selected = ray.cast();
            self.casting = false;
        }
        match &self.selected {
            Some(x) => self.gizmo.calc(x.transform, false, &(self.camera)),
            _ => ()
        }
    }
}

impl<'a> Render for Scene<'a> {
    fn render(&self) {
        self.camera.render();
        for obj in self.objects.iter() {
            obj.render();
        }
        self.draw_selected(YELLOW);
    }
}

impl<'a> Draw for Scene<'a> {
    fn draw(&self) {
        self.camera.draw();
        match &self.selected {
            Some(x) => self.gizmo.draw(),
            _ => ()
        }
    }
}
