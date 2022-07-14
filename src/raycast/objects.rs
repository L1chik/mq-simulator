use macroquad::
{
    prelude::*,
    models::{Mesh, Vertex},
};
use crate::{
    components::{Render, Physics},
};



pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, Quat::from_rotation_ypr(self.rotation.x, self.rotation.y, self.rotation.z), self.position)
    }

    pub fn translate(&mut self, dir: Vec3) {
        self.position = Mat4::from_translation(dir).mul_vec4(self.position.extend(1.)).truncate();
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: vec3(-10., 10., 3.), 
            rotation: vec3(0., 0., 45.0_f32.to_radians()),
            scale: vec3(5., 5., 5.),
        }
    }
}

pub struct GameObject{
    pub transform: Transform,
    pub mesh: Option<Mesh>,
}

impl GameObject {
    pub fn new(transform: Option<Transform>, mesh: Option<Mesh>) -> GameObject {
        GameObject{
            transform: if let Some(x) = transform {x} else {Default::default()},
            mesh: mesh,
        }
    }

    pub fn get_transformed_mesh(&self) -> Option<Mesh> {
        let transformed_mesh: Mesh;
        match &self.mesh {
            Some(x) => transformed_mesh = Mesh {
                vertices: x.vertices
                    .iter()
                    .map(|v| 
                        Vertex {
                            position: self.transform
                                .matrix() 
                                .mul_vec4(v.position.extend(1.))
                                .truncate(), 
                            uv: v.uv, 
                            color: v.color,
                        })
                    .collect(),
                indices: x.indices.clone(),
                texture: x.texture,
                },
            _ => return None
        }
        Some(transformed_mesh)
    }
}

impl Render for GameObject {
    //TODO REFACTOR!
    fn render(&self) {
        match self.get_transformed_mesh() {
            Some(x) => draw_mesh(&x),
            _ => ()
        }
    }
}

impl Physics for GameObject {
    fn update(&mut self) {

    }
}

pub struct Primitivies {
}

impl Primitivies {
    pub fn cube(texture: Option<Texture2D>) -> Mesh {
        let color: Color = GRAY;
        Mesh {
            vertices: vec![
                Vertex{position: vec3(0.5, -0.5, -0.5), uv: vec2(0., 0.), color: color},
                Vertex{position: vec3(0.5, 0.5, -0.5),  uv: vec2(0., 1.), color: color},
                Vertex{position: vec3(0.5, 0.5, 0.5),   uv: vec2(1., 1.), color: color},
                Vertex{position: vec3(0.5, -0.5, 0.5),  uv: vec2(1., 0.), color: color},

                Vertex{position: vec3(0.5, 0.5, -0.5),  uv: vec2(0., 0.), color: color},
                Vertex{position: vec3(-0.5, 0.5, -0.5), uv: vec2(0., 1.), color: color},
                Vertex{position: vec3(-0.5, 0.5, 0.5),  uv: vec2(1., 1.), color: color},
                Vertex{position: vec3(0.5, 0.5, 0.5),   uv: vec2(1., 0.), color: color},

                Vertex{position: vec3(-0.5, -0.5, -0.5),uv: vec2(0., 0.), color: color},
                Vertex{position: vec3(-0.5, 0.5, -0.5), uv: vec2(0., 1.), color: color},
                Vertex{position: vec3(0.5, 0.5, -0.5),  uv: vec2(1., 1.), color: color},
                Vertex{position: vec3(0.5, -0.5, -0.5), uv: vec2(1., 0.), color: color},

                Vertex{position: vec3(0.5, -0.5, 0.5),  uv: vec2(0., 0.), color: color},
                Vertex{position: vec3(0.5, 0.5, 0.5),   uv: vec2(0., 1.), color: color},
                Vertex{position: vec3(-0.5, 0.5, 0.5),  uv: vec2(1., 1.), color: color},
                Vertex{position: vec3(-0.5, -0.5, 0.5), uv: vec2(1., 0.), color: color},

                Vertex{position: vec3(-0.5, -0.5, -0.5),uv: vec2(0., 0.), color: color},
                Vertex{position: vec3(0.5, -0.5, -0.5), uv: vec2(0., 1.), color: color},
                Vertex{position: vec3(0.5, -0.5, 0.5),  uv: vec2(1., 1.), color: color},
                Vertex{position: vec3(-0.5, -0.5, 0.5), uv: vec2(1., 0.), color: color},

                Vertex{position: vec3(-0.5, -0.5, 0.5), uv: vec2(0., 0.), color: color},
                Vertex{position: vec3(-0.5, 0.5, 0.5),  uv: vec2(0., 1.), color: color},
                Vertex{position: vec3(-0.5, 0.5, -0.5), uv: vec2(1., 1.), color: color},
                Vertex{position: vec3(-0.5, -0.5, -0.5),uv: vec2(1., 0.), color: color},
            ],
            indices: vec![
                0, 1, 2, 0, 2, 3,
                4, 5, 6, 4, 6, 7,
                8, 9, 10, 8, 10, 11,
                12, 13, 14, 12, 14, 15,
                16, 17, 18, 16, 18, 19,
                20, 21, 22, 20, 22, 23,
            ],
            texture: texture,
        }
    }
}
