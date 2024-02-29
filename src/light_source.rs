use crate::color::Color;
use crate::vector::Vector;

pub trait LightSource {
    fn get_light_direction_from(&self, point: Vector) -> Vector;
    fn get_light_color(&self) -> Color;
}