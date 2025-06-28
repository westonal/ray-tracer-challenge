pub mod pattern;

use crate::material::pattern::Pattern;

#[derive(Debug, PartialEq)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
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
            shadow_boost: 0.,
        }
    }
}
