use math::color;
use math::tuple::color::Color;
use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;

pub struct PointLight {
    position: Point,
    color: Color,
}

impl PointLight {
    pub fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }
}

#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn light(&self, light: &PointLight, point: Point, eye: Normal, normal: Normal) -> Color {
        // Combine surface and light color
        let effective_color = self.color * light.color;

        // find direction to light source
        let light_v = (light.position - point).normalize();

        // find ambient contribution
        let ambient = effective_color * self.ambient;

        let mut result = ambient;

        //
        let light_dot_normal = light_v.dot(normal.clone_vector());
        if light_dot_normal >= 0.0 {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            result = result + diffuse;

            let reflect_v = -light_v.reflect(normal.clone_vector());
            let reflect_dot_eye = reflect_v.dot(eye.normalize().clone_vector());
            if reflect_dot_eye > 0.0 {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                let specular = light.color * self.specular * factor;
                result = result + specular;
            }
        }

        // set to solid color
        color!(result.red(), result.green(), result.blue())
    }
}

impl Material {
    pub fn default() -> Self {
        Self {
            color: color!(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod lighting_tests {
    use super::*;
    use math::tuple::vector::Vector;
    use math::{color, point, vector};

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let point = Point::origin();
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, -10), color!(1., 1., 1.));
        let material = Material::default();
        assert_eq!(
            color!(1.9, 1.9, 1.9),
            material.light(&light, point, eye, normal)
        );
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_at_45_degrees() {
        let point = Point::origin();
        let eye = vector!(0, 2.0_f32.sqrt() / 2., -2.0_f32.sqrt() / 2.).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, -10), color!(1., 1., 1.));
        let material = Material::default();
        assert_eq!(
            color!(1.0, 1.0, 1.0),
            material.light(&light, point, eye, normal)
        );
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_at_45_degrees() {
        let point = Point::origin();
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 10, -10), color!(1., 1., 1.));
        let material = Material::default();
        assert_eq!(
            color!(0.7363961, 0.7363961, 0.7363961),
            material.light(&light, point, eye, normal)
        );
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_refection_vector() {
        let point = Point::origin();
        let eye = vector!(0, -2.0_f32.sqrt() / 2., -2.0_f32.sqrt() / 2.).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 10, -10), color!(1., 1., 1.));
        let material = Material::default();
        assert_eq!(
            color!(1.636396, 1.636396, 1.636396),
            material.light(&light, point, eye, normal)
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let position = Point::origin();
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, 10), color!(1., 1., 1.));
        let material = Material::default();
        assert_eq!(
            color!(0.1, 0.1, 0.1),
            material.light(&light, position, eye, normal)
        );
    }
}
