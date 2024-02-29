use crate::color::Color;
use crate::light_ray::LightRay;
use crate::light_source::LightSource;
use crate::vector::Vector;

pub struct DirectionalLight {
    direction: Vector,
    color: Color
}

impl DirectionalLight {
    pub fn from(direction: Vector, color: Color) -> Option<DirectionalLight> {
        match direction.normalized() {
            Some(direction) => Some(DirectionalLight { direction, color }),
            None => None
        }
    }
}

impl LightSource for DirectionalLight {
    fn get_light_direction_from(&self, point: Vector) -> Vector {
        self.direction
    }

    fn get_light_color(&self) -> Color {
        self.color
    }
}
