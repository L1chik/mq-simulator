use macroquad::
{
    prelude::*,
    models::{Mesh, Vertex}
};


pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

impl Transform {
    fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, Quat::IDENTITY, self.position)
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vec3::ZERO, 
            rotation: Vec3::ZERO, 
            scale: Vec3::ONE,
        }
    }
}

pub struct GameObject{
    transform: Transform,
    mesh: Option<Mesh>,
}

impl GameObject {
    pub fn new(transform: Option<Transform>, mesh: Option<Mesh>) -> GameObject {
        GameObject{
            transform: if let Some(x) = transform {x} else {Default::default()},
            mesh: mesh,
        }
    }
    //TODO REFACTOR!
    pub fn render(&self) {
        let transformed_mesh: Mesh;
        match &self.mesh {
            Some(x) => transformed_mesh = Mesh {
                vertices: x.vertices
                    .iter()
                    .map(|v| 
                        Vertex {
                            position: Mat4::from_scale_rotation_translation(self.transform.scale, Quat::IDENTITY, self.transform.position)
                                .mul_vec4(v.position.extend(1.))
                                .truncate(), 
                            uv: v.uv, 
                            color: v.color,
                        })
                    .collect(),
                indices: x.indices.clone(),
                texture: x.texture,
                },
            _ => return
        }
        draw_mesh(&transformed_mesh);
    }
}

pub trait Render {
    fn render(&self);
}


pub struct Primitivies {
    mesh: Mesh,
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
