use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub enum RayIntersectionResult {
    NoIntersection,
    Intersected {
        surface_material: Material,
        surface_normal: Vector,
        t: f64
    }
}

pub trait Surface {
    fn intersect_with_ray(&self, ray: Ray) -> RayIntersectionResult;
}