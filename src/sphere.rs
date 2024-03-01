
use crate::material::Material;
use crate::surface::{RayIntersectionResult, Surface};
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {
    material: Material,
    center: Vector,
    radius: f64
}

impl Sphere {
    pub fn from(center: Vector, radius: f64, material: Material) -> Option<Sphere> {
        if radius <= 0f64 {
            None
        } else {
            Some(Sphere { center, radius, material })
        }
    }
}

impl Surface for Sphere {
    fn intersect_with_ray(&self, ray: Ray) -> RayIntersectionResult {
        let distance: Vector = self.center - ray.starting_point;
        let determinant: f64 = ray.direction.length_squared() * self.radius.powi(2) - distance.cross(ray.direction).length_squared();

        if determinant.is_sign_negative() {
            return RayIntersectionResult::NoIntersection;
        }

        let t1: f64 = (distance.dot(ray.direction) + determinant.sqrt()) / ray.direction.length_squared();
        let t2: f64 = (distance.dot(ray.direction) - determinant.sqrt()) / ray.direction.length_squared();
        let t: f64 = if t1 < f64::EPSILON || t1 > t2 { t2 } else { t1 };

        if t < f64::EPSILON {
            return RayIntersectionResult::NoIntersection
        }

        let normal: Vector = (ray.point_at(t) - self.center).normalized_or_zero();
        //println!("{}", self.color * (normal.dot(-distance.normalized_or_zero())));

        RayIntersectionResult::Intersected {
            surface_material: self.material,
            surface_normal: normal,
            t
        }
    }
}