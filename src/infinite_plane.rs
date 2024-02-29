use crate::color::Color;
use crate::material::Material;
use crate::surface::{RayIntersectionResult, Surface};
use crate::ray::Ray;
use crate::vector::Vector;

pub struct InfinitePlane {
    position: Vector,
    normal: Vector,
    material: Material
}

impl InfinitePlane {
    pub fn from(position: Vector, normal: Vector, material: Material) -> Option<InfinitePlane> {
        match normal.normalized() {
            Some(normal) => Some(InfinitePlane { position, normal, material }),
            _ => None
        }
    }
}

impl Surface for InfinitePlane {
    fn intersect_with_ray(&self, ray: Ray) -> RayIntersectionResult {
        let distance: Vector = self.position - ray.starting_point;
        let t: f64 = distance.dot(self.normal) / ray.direction.dot(self.normal);

        if t < f64::EPSILON {
            RayIntersectionResult::NoIntersection
        } else {
            RayIntersectionResult::Intersected { surface_material: self.material, surface_normal: self.normal, t }
        }
    }
}
