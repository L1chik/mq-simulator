use macroquad::prelude::*;
use macroquad::math::{Mat3, Vec3};

pub mod ray;
pub mod primitivies;
pub mod camera;
pub mod objects;

pub const world_axes:Mat3 = Mat3::IDENTITY;

