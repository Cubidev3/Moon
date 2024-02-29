use std::f64::consts::PI;
use std::ptr::read;
use crate::basis::Basis;
use crate::color::Color;
use crate::light_ray::LightRay;
use crate::light_source::LightSource;
use crate::surface::{RayIntersectionResult, Surface};
use crate::ray::Ray;
use crate::screen::{Screen, self};
use crate::surface;
use crate::vector::Vector;

pub struct Camera {
    pub basis: Basis,
    pub position: Vector,
    pub focal_length: f64,
    pub sensor_extents: Vector
}

impl Camera {
    pub fn from(position: Vector, up_direction: Vector, look_direction: Vector, focal_length: f64, sensor_size: Vector) -> Option<Camera> {
        let basis = match Basis::from_wv(-look_direction, up_direction) {
            Some(b) => b,
            _ => return None
        };

        let sensor_extents = sensor_size / 2f64;

        Some(Camera {
            basis,
            position,
            focal_length,
            sensor_extents
        })
    }

    pub fn print_to_screen<S: Surface, L: LightSource>(&self, surface: &S, light_source: &L, screen: &mut Screen) {
        let (width, height) = screen.resolution();
        let pixel_extents = Vector::from(self.sensor_extents.x / width as f64, self.sensor_extents.y / height as f64, 0f64);

        for (px, py) in screen.pixel_positions() {
            let pixel_position: Vector = self.pixel_world_position_at(px, py, width, height, pixel_extents);
            let ray: Ray = Ray::from_points(self.position, pixel_position).unwrap();

            screen.paint_at(px, py, self.get_final_color_from_ray(ray, surface, light_source, 3)).expect("should not go wrong");
        }
    }

    pub fn pixel_world_position_at(&self, x: usize, y: usize, width: usize, height: usize, pixel_extents: Vector) -> Vector {
        self.position - self.basis.u * pixel_extents.x * (2f64 * x as f64 - width as f64 + 1f64) - self.basis.v * pixel_extents.y * (2f64 * y as f64 - height as f64 + 1f64) - self.basis.w * self.focal_length
    }

    fn get_final_color_from_ray<S: Surface, L: LightSource>(&self, ray: Ray, surface: &S, light_source: &L, reflections: u8) -> Color {
        match self.get_final_ray(ray, surface, light_source, reflections) {
            Some(ray) => ray.color,
            _ => Color::BLACK
        }
    }

    fn get_final_ray<S: Surface, L: LightSource>(&self, ray: Ray, surface: &S, light_source: &L, reflections: u8) -> Option<LightRay> {
        if reflections == 0 {
            return None;
        }

        let ray_intersection = surface.intersect_with_ray(ray);

        match ray_intersection {
            RayIntersectionResult::Intersected { surface_material, surface_normal, t } => {
                let reflection_point = ray.point_at(t);
                let direction_from_camera = (reflection_point - self.position).normalized_or_zero();
                let reflection_ray = Ray::from(reflection_point, direction_from_camera.reflected(surface_normal)).unwrap();

                let ray_to_light = Ray::from(reflection_point, -light_source.get_light_direction_from(reflection_point)).unwrap();
                let light_multiplier = match surface.intersect_with_ray(ray_to_light) {
                    RayIntersectionResult::NoIntersection => 1f64,
                    _ => 0f64
                };

                let diffuse_multiplier = surface_material.diffuse_coefficient * surface_normal.dot(-light_source.get_light_direction_from(reflection_point)).max(0f64);

                let bisector = (-direction_from_camera + ray_to_light.direction).normalized_or_zero();

                let specular_multiplier = surface_material.specular_coefficient * bisector.dot(surface_normal).max(0f64).powf(surface_material.shininess_coefficient);

                let received_color = light_source.get_light_color() * light_multiplier;
                let reflection_color = match self.get_final_ray(reflection_ray, surface, light_source, reflections - 1) {
                    Some(light_ray) => light_ray.color * surface_material.mirror_reflection_coefficient,
                    _ => Color::ZERO
                };

                LightRay::from(reflection_point, -ray.direction, Color::mix_of(vec![surface_material.color, (received_color + reflection_color) * (diffuse_multiplier + specular_multiplier)]))
            },

            _ => None
        }
    }
    //fn get_materials_from_propagation<S: Surface, L: LightSource>(ray: Ray, surface: &S, light_source: &L, reflections: u8) -> Vec<(>
}

