use macroquad::math::{Mat3};

pub mod ray;
pub mod camera;
pub mod objects;
pub mod components;
pub mod scene;
pub mod gizmo;

use crate::scene::Scene;

pub const world_axes:Mat3 = Mat3::IDENTITY;

#[no_mangle]
static mut SCENE: Option<Scene> = None;

pub fn set_scene(s: Scene<'static>) {
    unsafe {SCENE = Some(s);}
}
    
pub fn get_scene() -> &'static mut Scene<'static> {
   unsafe{SCENE.as_mut().unwrap_or_else(|| panic!())}
}



