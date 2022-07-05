use macroquad::prelude::*;
use std::fmt;

use macroquad::models::{draw_mesh, Vertex, Mesh};

use macroquad::math::{Vec3, Vec2, Mat4};

use crate::ray::Ray;

#[derive(Debug)]
pub struct transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3
}


pub struct GameObject {
    pub transform: transform,
    pub mesh: Option<Mesh>
}

impl GameObject {
    pub fn draw(&self) {
        match &self.mesh {
            Some(x) => draw_mesh(&x),
            None => (),
        }
    }

    fn scale_matrix(&self) -> Mat3 {
        return Mat3::from_cols_array_2d(&[[self.transform.scale.x, 0., 0.],[0., self.transform.scale.y, 0.],[0., 0., self.transform.scale.z]]);
    }

    pub fn raycast_hit(&self, ray: &Ray) -> Option<Vec3> {
        let mut mesh = Mesh{
            vertices: vec![],
            indices: vec![],
            texture: None
        };
        let mut result = vec3(0., 0., 0.); 
        let mut min_distance = 10000.;
        let mut hit = false;
        match &self.mesh {
            Some(x) => {mesh.vertices = x.vertices.clone(); mesh.indices = x.indices.clone(); mesh.texture = x.texture.clone();},
            None => return None
        }
        println!("{:?}", mesh.indices);
        for i in 0..mesh.indices.len() / 3 {
            println!("{}", i);
            let A = mesh.vertices[mesh.indices[3*i + 0] as usize].position;
            let B = mesh.vertices[mesh.indices[3*i + 1] as usize].position;
            let C = mesh.vertices[mesh.indices[3*i + 2] as usize].position;
            println!("{:?}, {:?}, {:?}", A, B, C);
            match cross_plane(&A, triangle_normal(&A, &B, &C), &ray) {
                Some(x) => {
                    if inside_triangle(A, B, C, &x) {
                        if x.distance(ray.origin) <= min_distance {
                            min_distance = x.distance(ray.origin);
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


fn cross_plane(m: &Vec3, n: Vec3, ray: &Ray) -> Option<Vec3> {
    let t = - (ray.origin.dot(n) - n.dot(*m)) / (n.dot(ray.direction));
    if t >= 0. {
        Some(ray.origin + ray.direction * t)
    }else {
        None
    }
}

fn triangle_normal(A: &Vec3, B: &Vec3, C: &Vec3) -> Vec3 {
    let AB = *B - *A;
    let AC = *C - *A;

    AB.cross(AC).normalize()
}

fn inside_triangle(A: Vec3, B: Vec3, C: Vec3, O: &Vec3) -> bool {
    let OA = A - *O;
    let OB = B - *O;
    let OC = C - *O;

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

impl fmt::Debug for GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {    
        write!(f, "Transform: {}", self.transform.position)
    }
}

impl Default for GameObject {
    fn default() -> GameObject {
        GameObject{
            transform: transform{
                position: vec3(0., 0., 0.),
                rotation: vec3(0., 0., 0.),
                scale: vec3(1., 1., 1.)
            },
            mesh: None
        }
    }
}

pub trait Primitivies {
    fn cube() -> Self;
    fn cube_with_pos(pos: Vec3) -> Self;
}

impl Primitivies for GameObject {
    fn cube() -> GameObject {
        let mut cube = GameObject::default();
        cube.mesh = Some(Mesh{
            vertices: vec![
                Vertex{position: cube.transform.position + cube.scale_matrix().mul_vec3(vec3(-1., 1., -1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: cube.transform.position + cube.scale_matrix().mul_vec3(vec3(-1., 1., 1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: cube.transform.position + cube.scale_matrix().mul_vec3(vec3(1., 1., 1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: cube.transform.position + cube.scale_matrix().mul_vec3(vec3(1., 1., -1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: cube.transform.position + cube.scale_matrix().mul_vec3(vec3(1., -1., -1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: cube.transform.position + cube.scale_matrix().mul_vec3(vec3(-1., -1., -1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: cube.transform.position + cube.scale_matrix().mul_vec3(vec3(-1., -1., 1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: cube.transform.position + cube.scale_matrix().mul_vec3(vec3(1., -1., 1.)), uv: vec2(0., 0.), color: GRAY},
            ],
            indices: vec![
                0, 3, 5,    4, 3, 5, //front
                3, 2, 4,    7, 2, 4, //right
                0, 1, 3,    2, 1, 3, //up
                0, 1, 5,    6, 1, 5, //left
                5, 6, 4,    7, 6, 4, //down
                1, 6, 2,    7, 6, 2, //back
            ],
            texture: None
        });
        cube
    }

    fn cube_with_pos(pos: Vec3) -> GameObject {
        let mut cube = GameObject::default();
        cube.mesh = Some(Mesh{
            vertices: vec![
                Vertex{position: pos + cube.scale_matrix().mul_vec3(vec3(-1., 1., -1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: pos + cube.scale_matrix().mul_vec3(vec3(-1., 1., 1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: pos + cube.scale_matrix().mul_vec3(vec3(1., 1., 1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: pos + cube.scale_matrix().mul_vec3(vec3(1., 1., -1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: pos + cube.scale_matrix().mul_vec3(vec3(1., -1., -1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: pos + cube.scale_matrix().mul_vec3(vec3(-1., -1., -1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: pos + cube.scale_matrix().mul_vec3(vec3(-1., -1., 1.)), uv: vec2(0., 0.), color: GRAY},
                Vertex{position: pos + cube.scale_matrix().mul_vec3(vec3(1., -1., 1.)), uv: vec2(0., 0.), color: GRAY},
            ],
            indices: vec![
                0, 3, 5,    4, 3, 5, //front
                3, 2, 4,    7, 2, 4, //right
                0, 1, 3,    2, 1, 3, //up
                0, 1, 5,    6, 1, 5, //left
                5, 6, 4,    7, 6, 4, //down
                1, 6, 2,    7, 6, 2, //back
            ],
            texture: None
        });
        cube
    }
}
