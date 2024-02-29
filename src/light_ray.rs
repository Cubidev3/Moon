use crate::color::Color;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct LightRay {
    pub ray: Ray,
    pub color: Color
}

impl LightRay {
    pub fn from(starting_point: Vector, direction: Vector, color: Color) -> Option<LightRay> {
        match Ray::from(starting_point, direction) {
            Some(ray) => Some(LightRay{ ray, color }),
            None => None
        }
    }

    pub fn from_ray(ray: Ray, color: Color) -> LightRay {
        LightRay { ray, color }
    }
}