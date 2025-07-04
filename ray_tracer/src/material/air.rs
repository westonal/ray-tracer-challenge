use crate::material::Material;
use crate::material::pattern::Pattern;
use math::tuple::color::TRANSPARENT;

impl Material {
    pub fn air() -> Self {
        let mut air = Self::default();
        air.pattern = Pattern::Solid(*TRANSPARENT);
        air.transparency = 1.;
        air.refractive_index = 1.;
        air.ambient = 0.;
        air.diffuse = 0.;
        // Air is usually used as a bubble. The Air needs to be reflective to bounce rays back into
        // the source medium during internal reflection.
        air.reflectivity = 1.;
        air.specular = 0.;
        air.shininess = 0.;
        air
    }
}
