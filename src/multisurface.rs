

use crate::surface::{RayIntersectionResult, Surface};
use crate::ray::Ray;


pub struct MultiSurface<'a> {
    surfaces: Vec<&'a dyn Surface>
}

impl<'a> MultiSurface<'a> {
    pub fn from(surfaces: Vec<&dyn Surface>) -> MultiSurface {
        MultiSurface { surfaces }
    }
}

impl<'a> Surface for MultiSurface<'a> {
    fn intersect_with_ray(&self, ray: Ray) -> RayIntersectionResult {
        let intersection = self.surfaces.iter()
            .map(|surface| surface.intersect_with_ray(ray))
            .filter_map(|surface| match surface { 
                RayIntersectionResult::Intersected { surface_normal, surface_material, t } if t.is_sign_positive() => Some((surface_normal, surface_material, t)),
                _ => None
            })
            .min_by(|(_, _, t1), (_, _, t2)| t1.partial_cmp(t2).unwrap());
        
        let (normal, material, t) = match intersection { 
            Some(intersection) => intersection,
            None => return RayIntersectionResult::NoIntersection
        };
        
        if t.is_infinite() {
            RayIntersectionResult::NoIntersection
        } else {
            RayIntersectionResult::Intersected { surface_normal: normal, surface_material: material, t }
        }
    }
}