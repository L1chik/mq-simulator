use macroquad::prelude::*;
use crate::{
    set_scene,
    world_axes,
    objects::GameObject,
    camera::MainCamera,
    components::{Input, Physics, Render, Draw,},
};


pub struct Scene<'a> {
    pub objects: Vec<GameObject>,
    pub selected: Option<&'a mut GameObject>,
    pub camera: MainCamera,
    casting: bool,
    transformation: bool,
    last_mouse_pos: Vec2,
}


impl<'a> Scene<'a> {
    pub fn new() {
        let scene = Scene{
            objects: vec![],
            selected: None,
            camera: Default::default(),
            casting: false,
            transformation: false,
            last_mouse_pos: vec2(0., 0.),
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

    fn draw_gizmo(&self) {
        match &self.selected {
            Some(x) => {
                let center = self.camera.world_point_to_screen(x.transform.position);
                let rc = vec2((center.x + 1.) * screen_width() / 2., (1. - center.y) * screen_height() / 2.);
                let up = self.camera.world_point_to_screen(x.transform.position + world_axes.y_axis);
                let ru = vec2((up.x + 1.) * screen_width() / 2., (1. - up.y) * screen_height() / 2.);
                let vec_up = (ru - rc).normalize();
                let ru = rc + vec_up * 100.;
                let right = self.camera.world_point_to_screen(x.transform.position + world_axes.x_axis);
                let rr = vec2((right.x + 1.) * screen_width() / 2., (1. - right.y) * screen_height() / 2.);
                let vec_right = (rr - rc).normalize();
                let rr = rc + vec_right * 100.;
                let forward = self.camera.world_point_to_screen(x.transform.position + world_axes.z_axis);
                let rf = vec2((forward.x + 1.) * screen_width() / 2., (1. - forward.y) * screen_height() / 2.);
                let vec_forward = (rf - rc).normalize();
                let rf = rc + vec_forward * 100.;
                

                draw_text(&*format!("{:?}, {:?}", rc, ru), 100., 100., 32., BLACK);
                draw_line(rc.x, rc.y, ru.x, ru.y, 3., GREEN);
                draw_triangle(vec2(ru.x - 20., ru.y), vec2(ru.x + 20., ru.y), vec2(ru.x, ru.y - 50.), GREEN);
                draw_line(rc.x, rc.y, rr.x, rr.y, 3., RED);
                draw_triangle(vec2(rr.x, rr.y - 20.), vec2(rr.x, rr.y + 20.), vec2(rr.x + 50., rr.y), RED);
                draw_line(rc.x, rc.y, rf.x, rf.y, 3., BLUE);
                draw_triangle(vec2(rf.x - 20., rf.y), vec2(rf.x + 20., rf.y), vec2(rf.x, rf.y + 50.), BLUE);
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
            let pos: Vec2 = mouse_position().into();
            if pos.x >= (screen_width() / 2.) - 20. && pos.x <= (screen_width() / 2.) + 20. && pos.y >= (screen_height() / 2.) - 50. && pos.y <= (screen_height() / 2.) + 50. && self.selected.is_some(){
                self.transformation = true;
                self.last_mouse_pos = pos;
            } else {
                self.casting = true;
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.transformation = false;
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

        if self.transformation {
            let mouse_pos: Vec2 = mouse_position().into();
            let delta_mouse_pos = self.last_mouse_pos - mouse_pos;
            self.last_mouse_pos = mouse_pos;
            let y = delta_mouse_pos.y * 2. * delta;
            match self.selected.as_mut() {
                Some(x) => x.transform.translate(vec3(0., y, 0.)),
                _ => ()
            }
        }

        if self.casting {
            let mouse_pos: Vec2 = mouse_position().into();
            let ray = self.camera.screen_point_to_ray(mouse_pos.extend(1.));
            //TODO return the object
            //self.selected = ray.cast();
            ray.cast();
            self.casting = false;
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
        self.draw_gizmo();
    }
}
