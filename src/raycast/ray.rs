use macroquad::math::{Vec3};
use crate::primitivies::GameObject;


pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn cast<'a>(&'a self, scene: &'a Vec<GameObject>) -> Option<&GameObject> {
        let mut result: Option<&GameObject> = None;
        let mut min_distance = 10000.;
        for obj in scene {
            match obj.raycast_hit(&*self) {
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
}
