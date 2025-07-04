mod air;
mod glass;
pub mod pattern;
pub mod refraction;

use crate::material::pattern::Pattern;
use math::tuple::color::Color;

#[derive(Debug, PartialEq)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub shadow_boost: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Pattern::default(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectivity: 0.,
            transparency: 0.,
            refractive_index: 1.,
            shadow_boost: 0.,
        }
    }
}

impl Material {
    /// All shading disabled, pure ambient solid color
    pub fn solid(color: Color) -> Self {
        Self {
            pattern: Pattern::Solid(color),
            ambient: 1.,
            diffuse: 0.,
            specular: 0.,
            shininess: 0.,
            reflectivity: 0.,
            transparency: 0.,
            refractive_index: 1.,
            shadow_boost: 0.,
        }
    }
}

#[cfg(test)]
mod default_material_tests {
    use super::*;

    #[test]
    fn default_transparency() {
        assert_eq!(0., Material::default().transparency)
    }

    #[test]
    fn default_refractive_index() {
        assert_eq!(1., Material::default().refractive_index)
    }
}
