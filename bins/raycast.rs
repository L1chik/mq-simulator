use macroquad::prelude::*;

use raycast::objects::{GameObject, Primitivies, Transform};
use raycast::scene::Scene;
use raycast::get_scene;

#[macroquad::main("3D")]
async fn main() {
    let cube1 = GameObject::new(Some(Transform::default()), Some(Primitivies::cube(None)));
    let cube2 = GameObject::new(Some(Transform {position: vec3(0., 0., 0.), rotation: vec3(0., 0., 0.), scale: vec3(5., 5., 5.)}), Some(Primitivies::cube(None)));
    Scene::new();
    let scene = get_scene();
    scene.add_object(cube1);
    scene.add_object(cube2);
    scene.main_loop().await;
}
