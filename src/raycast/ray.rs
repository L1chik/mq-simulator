use macroquad::prelude::*;
use macroquad::math::{Vec3};
use crate::{
    objects::GameObject,
    get_scene,
};


pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn cast(& self){
        let mut scene = get_scene();
        let mut min_distance = 10000.;
        let mut result : Option<&GameObject> = None;
        for obj in &scene.objects {
            println!("{:?}", obj.transform.position);
            match self.raycast_hit(&obj) {
                Some(x) =>  {
                    println!("HIT!");
                    if self.origin.distance(x) <= min_distance {
                        min_distance = self.origin.distance(x);
                        result = Some(&obj);
                    }
                },
                _ => println!("MISS!")
            }
        }  
        scene.selected = result;
    }
    pub fn raycast_hit(&self, obj: &GameObject) -> Option<Vec3> {
        let mut mesh: Mesh = Mesh{vertices: vec![], indices: vec![], texture: None};
        let mut result = vec3(0., 0., 0.); 
        let mut min_distance = 10000.;
        let mut hit = false;
        match &obj.get_transformed_mesh() {
            Some(x) => {mesh.vertices = x.vertices.clone(); mesh.indices = x.indices.clone(); mesh.texture = x.texture.clone();},
            None => return None
        }
        for i in 0..mesh.indices.len() / 3 {
            let A = mesh.vertices[mesh.indices[3*i + 0] as usize].position;
            let B = mesh.vertices[mesh.indices[3*i + 1] as usize].position;
            let C = mesh.vertices[mesh.indices[3*i + 2] as usize].position;
            match cross_plane(A, triangle_normal(A, B, C), self) {
                Some(x) => {
                    if inside_triangle(A, B, C, x) {
                        if x.distance(self.origin) <= min_distance {
                            min_distance = x.distance(self.origin);
                            result = x;
                            hit = true;
                        }
                    }
                },
                _ => ()
            }
        }
        if hit {
            Some(result)
        }else {
            None
        }
    }
}


fn cross_plane(m: Vec3, n: Vec3, ray: &Ray) -> Option<Vec3> {
    let t = - (ray.origin.dot(n) - n.dot(m)) / (n.dot((*ray).direction));
    if t >= 0. {
        Some((*ray).origin + (*ray).direction * t)
    }else {
        None
    }
}

fn triangle_normal(A: Vec3, B: Vec3, C: Vec3) -> Vec3 {
    let AB = B - A;
    let AC = C - A;

    AB.cross(AC).normalize()
}

fn inside_triangle(A: Vec3, B: Vec3, C: Vec3, O: Vec3) -> bool {
    let OA = A - O;
    let OB = B - O;
    let OC = C - O;

    let u = OA.cross(OB);
    let v = OB.cross(OC);
    let w = OC.cross(OA);

    if u.dot(v) < 0. {
        false
    }
    else if u.dot(w) < 0. {
        false
    }
    else{
        true
    }
}
