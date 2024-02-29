use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::camera::Camera;
use crate::color::Color;
use crate::directional_light::DirectionalLight;
use crate::infinite_plane::InfinitePlane;
use crate::material::Material;
use crate::multisurface::MultiSurface;
use crate::surface::{RayIntersectionResult, Surface};
use crate::ray::Ray;
use crate::screen::Screen;
use crate::sphere::Sphere;
use crate::vector::Vector;

mod color;
mod surface;
mod ray;
mod vector;
mod vector_macro;
mod screen;
mod camera;
mod basis;
mod sphere;
mod multisurface;
mod infinite_plane;
mod light_source;
mod directional_light;
mod light_ray;
mod material;

fn main() {
    let mut screen: Screen = Screen::from(1920, 1080, Color::BLACK);
    let camera: Camera = Camera::from(Vector::ZERO, Vector::from(0f64, 1f64, 0f64), Vector::from(0f64, 0f64, 1f64), 8f64, Vector::from(16f64, 9f64, 0f64)).unwrap();

    let sphere: Sphere = Sphere::from(Vector::from(0f64, -5f64, 30f64), 10f64, Material::from(Color::from(0f64, 1f64, 0f64, 1f64), 1f64, 1f64, 10f64, 1f64)).unwrap();
    let sphere3: Sphere = Sphere::from(Vector::from(-16f64, 0f64, 30f64), 6f64, Material::from(Color::from(1f64, 0.25f64, 0.125f64, 1f64), 1f64, 0.4f64, 2f64, 0.1f64)).unwrap();
    let sphere2: Sphere = Sphere::from(Vector::from(0f64, 10f64, 30f64), 5f64, Material::from(Color::from(0.35f64, 0.15f64, 0.8f64, 1f64), 1f64, 1f64, 100f64, 1f64)).unwrap();
    let infinite_plane: InfinitePlane = InfinitePlane::from(Vector::from(0f64, -4f64, 0f64), Vector::from(0f64, 1f64, 0f64), Material::from(Color::from(0.3f64, 0.3f64, 0.3f64, 1f64), 1f64, 1f64, 10f64, 1f64)).unwrap();
    let world = MultiSurface::from(vec![&sphere2, &sphere, &sphere3, &infinite_plane]);

    let light = DirectionalLight::from(Vector::from(-1f64, -1f64, 0f64), Color::from(1f64, 1f64, 1f64, 1f64)).unwrap();

    camera.print_to_screen(&world, &light, &mut screen);

    let mut file = File::create("raytraced.pbm").unwrap();
    file.write(screen.to_pbm().as_ref()).unwrap();
}
