use macroquad::prelude::*;
use crate::{
    set_scene,
    objects::GameObject,
    camera::MainCamera,
    components::{Input, Physics, Render,},
};


pub struct Scene<'a> {
    pub objects: Vec<GameObject>,
    pub selected: Option<&'a GameObject>,
    pub camera: MainCamera,
}


impl<'a> Scene<'a> {
    pub fn new() {
        let scene = Scene{
            objects: vec![],
            selected: None,
            camera: Default::default(),
        };
        set_scene(scene);
    }
    
    pub fn add_object(&mut self, obj: GameObject) {
        self.objects.push(obj);
    }
    
    fn draw_selected(&self, color: Color) {
        match self.selected {
            Some(x) => {
                let mesh = x.get_transformed_mesh().unwrap();
                for i in 0..mesh.indices.len() / 3 {
                    let A = mesh.vertices[mesh.indices[3*i + 0] as usize].position;
                    let B = mesh.vertices[mesh.indices[3*i + 1] as usize].position;
                    let C = mesh.vertices[mesh.indices[3*i + 2] as usize].position;
                    //let A_dir = A - x.transform.position;
                    //let B_dir = B - x.transform.position;
                    //let C_dir = C - x.transform.position;
                    draw_line_3d(A, B, color);
                    draw_line_3d(B, C, color);
                    draw_line_3d(A, C, color);
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
            next_frame().await
        }
    }
}


impl<'a> Input for Scene<'a> {
    fn input(&mut self) {
        self.camera.input();
    }
}

impl<'a> Physics for Scene<'a> {
    fn update(&mut self) {
        self.camera.update();
        for obj in self.objects.iter_mut() {
            obj.update();
        }
        match self.selected {
            Some(x) => println!("Selected object: {:?}", x.transform.position),
            None => ()
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
