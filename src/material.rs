use crate::color::Color;

#[derive(Copy, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub diffuse_coefficient: f64,
    pub specular_coefficient: f64,
    pub shininess_coefficient: f64,
    pub mirror_reflection_coefficient: f64
}

impl Material {
    pub fn from(color: Color, diffuse_coefficient: f64, specular_coefficient: f64, shininess_coefficient: f64, mirror_reflection_coefficient: f64) -> Material {
        Material { 
            color,
            diffuse_coefficient,
            specular_coefficient,
            shininess_coefficient,
            mirror_reflection_coefficient
        }
    }
}