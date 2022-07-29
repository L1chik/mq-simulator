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
    pub fn cast<'a, 'b>(&'a self) -> Option<&'b mut GameObject>{
        let mut scene = get_scene();
        let mut min_distance = 10000.;
        let mut result : Option<&mut GameObject> = None;
        for obj in scene.objects.iter_mut() {
            match self.raycast_hit(&obj) {
                Some(x) =>  {
                    if self.origin.distance(x) <= min_distance {
                        min_distance = self.origin.distance(x);
                        result = Some(obj);
                    }
                },
                _ => ()
            }
        }  
        result
    }
    pub fn raycast_hit(&self, obj: &GameObject) -> Option<Vec3> {
        //TODO replace get_transformed_mesh to model_matrix
        let mut mesh: Mesh = Mesh{vertices: vec![], indices: vec![], texture: None};
        let mut result = vec3(0., 0., 0.); 
        let mut min_distance = 10000.;
        let mut hit = false;
        match &obj.get_transformed_mesh() {
            Some(x) => {mesh.vertices = x.vertices.clone(); mesh.indices = x.indices.clone(); mesh.texture = x.texture.clone();},
            None => return None
        }
        for i in 0..mesh.indices.len() / 3 {
            let a = mesh.vertices[mesh.indices[3*i + 0] as usize].position;
            let b = mesh.vertices[mesh.indices[3*i + 1] as usize].position;
            let c = mesh.vertices[mesh.indices[3*i + 2] as usize].position;
            match cross_plane(a, triangle_normal(a, b, c), self) {
                Some(x) => {
                    if inside_triangle(a, b, c, x) {
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
    
    pub fn raycast_hit_to_line(&self, pos: Vec3, dir: Vec3) -> Vec3 {
        (self.direction * pos - dir * self.origin) / (self.direction - dir)
    }

    pub fn raycast_hit_to_plane(&self, pos: Vec3, n: Vec3) -> Vec3 {
        self.origin + self.direction * (n.dot(pos - self.origin) / n.dot(self.direction))
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

fn triangle_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let ab = b - a;
    let ac = c - a;

    ab.cross(ac).normalize()
}

fn inside_triangle(a: Vec3, b: Vec3, c: Vec3, o: Vec3) -> bool {
    let oa = a - o;
    let ob = b - o;
    let oc = c - o;

    let u = oa.cross(ob);
    let v = ob.cross(oc);
    let w = oc.cross(oa);

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
