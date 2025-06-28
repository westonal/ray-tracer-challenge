pub mod pre_calculations;

use crate::material::Material;
use math::color;
use math::tuple::color::Color;
use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;

pub struct PointLight {
    pub position: Point,
    pub color: Color,
}

impl PointLight {
    pub fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }
}

impl Material {
    pub fn light(
        &self,
        light: &PointLight,
        point: Point,
        eye: Normal,
        normal: Normal,
        shadow_factor: f32,
    ) -> Color {
        let surface_color = self.pattern.color_at(&point);

        // Combine surface and light color
        let effective_color = surface_color * light.color;

        // find direction to light source
        let light_v = (light.position - point).normalize();

        let anti_shadow = (1. + self.shadow_boost - shadow_factor.clamp(0.0, 1.0)).clamp(0.0, 1.0);

        // find ambient contribution
        let ambient = effective_color * self.ambient;

        let mut result = ambient;

        //
        let light_dot_normal = light_v.dot(&normal);
        if light_dot_normal >= 0.0 {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            result = result + diffuse * anti_shadow;

            let reflect_v = -light_v.reflect(normal.clone_vector());
            let reflect_dot_eye = reflect_v.dot(&eye);
            if reflect_dot_eye > 0.0 {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess) * anti_shadow;
                let specular = light.color * self.specular * factor;
                result = result + specular;
            }
        }

        // set to solid color
        color!(result.red(), result.green(), result.blue())
    }
}

#[cfg(test)]
mod lighting_tests {
    use super::*;

    use math::{color, point, vector};

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let point = Point::origin();
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, -10), color!(1, 1, 1));
        let material = Material::default();
        assert_eq!(
            color!(1.9, 1.9, 1.9),
            material.light(&light, point, eye, normal, 0.)
        );
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_at_45_degrees() {
        let point = Point::origin();
        let eye = vector!(0, 2.0_f32.sqrt() / 2., -2.0_f32.sqrt() / 2.).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, -10), color!(1, 1, 1));
        let material = Material::default();
        assert_eq!(
            color!(1.0, 1.0, 1.0),
            material.light(&light, point, eye, normal, 0.)
        );
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_at_45_degrees() {
        let point = Point::origin();
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 10, -10), color!(1, 1, 1));
        let material = Material::default();
        assert_eq!(
            color!(0.7363961, 0.7363961, 0.7363961),
            material.light(&light, point, eye, normal, 0.)
        );
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_refection_vector() {
        let point = Point::origin();
        let eye = vector!(0, -2.0_f32.sqrt() / 2., -2.0_f32.sqrt() / 2.).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 10, -10), color!(1, 1, 1));
        let material = Material::default();
        assert_eq!(
            color!(1.636396, 1.636396, 1.636396),
            material.light(&light, point, eye, normal, 0.)
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let position = Point::origin();
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, 10), color!(1, 1, 1));
        let material = Material::default();
        assert_eq!(
            color!(0.1, 0.1, 0.1),
            material.light(&light, position, eye, normal, 0.)
        );
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_but_in_full_shadow() {
        let point = Point::origin();
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, -10), color!(1, 1, 1));
        let material = Material::default();
        assert_eq!(
            color!(0.1, 0.1, 0.1),
            material.light(&light, point, eye, normal, 1.)
        );
    }
}

#[cfg(test)]
mod non_solid_pattern_tests {
    use super::*;
    use crate::material::pattern::Pattern;
    use math::{point, vector};

    #[test]
    fn lighting_with_stripe_applied() {
        let point = Point::origin();
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, -10), color!(1, 1, 1));
        let mut material = Material::default();
        material.ambient = 1.;
        material.diffuse = 0.;
        material.specular = 0.;
        let a = color!(1, 1, 0);
        let b = color!(0, 0, 1);
        assert_ne!(a, b);
        material.pattern = Pattern::Stripe(a, b);
        assert_eq!(
            a,
            material.light(&light, point!(0.9, 0, 0), eye, normal, 0.)
        );
        assert_eq!(
            b,
            material.light(&light, point!(1.1, 0, 0), eye, normal, 0.)
        );
    }
}
