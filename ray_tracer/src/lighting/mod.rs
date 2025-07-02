pub mod pre_calculations;
pub mod refraction_lighting;

use crate::material::Material;
use crate::transform::Transform;
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
        transform: &Transform,
        world_point: Point,
        eye: Normal,
        normal: Normal,
        shadow_factor: f32,
    ) -> Color {
        let surface_color = self.pattern.color_at(world_point, transform);

        // Combine surface and light color
        let effective_color = surface_color * light.color;

        // find direction to light source
        let light_v = (light.position - world_point).normalize();

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

        result.set_alpha(1.);
        result
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
            material.light(&light, &Transform::identity(), point, eye, normal, 0.)
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
            material.light(&light, &Transform::identity(), point, eye, normal, 0.)
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
            material.light(&light, &Transform::identity(), point, eye, normal, 0.)
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
            material.light(&light, &Transform::identity(), point, eye, normal, 0.)
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
            material.light(&light, &Transform::identity(), position, eye, normal, 0.)
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
            material.light(&light, &Transform::identity(), point, eye, normal, 1.)
        );
    }
}

#[cfg(test)]
mod non_solid_pattern_tests {
    use super::*;
    use crate::material::pattern::Pattern;
    use crate::transform::Transform;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::tuple::color::{RED, WHITE};
    use math::{color, degrees, point, vector};

    #[test]
    fn lighting_with_stripe_applied() {
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, -10), color!(1, 1, 1));
        let mut material = Material::solid(*WHITE);
        material.pattern = Pattern::Stripe(*WHITE, *RED, Transform::identity());
        assert_eq!(
            *WHITE,
            material.light(
                &light,
                &Transform::identity(),
                point!(0.9, 0, 0),
                eye,
                normal,
                0.
            )
        );
        assert_eq!(
            *RED,
            material.light(
                &light,
                &Transform::identity(),
                point!(1.1, 0, 0),
                eye,
                normal,
                0.
            )
        );
    }

    #[test]
    fn lighting_with_stripe_applied_rotate_in_pattern_transform() {
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, -10), color!(1, 1, 1));
        let mut material = Material::solid(*WHITE);
        material.pattern = Pattern::Stripe(
            *WHITE,
            *RED,
            Transform::new(Matrix4x4::rotation_z(degrees!(90))),
        );
        assert_eq!(
            *WHITE,
            material.light(
                &light,
                &Transform::identity(),
                point!(0, 0.9, 0),
                eye,
                normal,
                0.
            )
        );
        assert_eq!(
            *RED,
            material.light(
                &light,
                &Transform::identity(),
                point!(0, 1.1, 0),
                eye,
                normal,
                0.
            )
        );
    }

    #[test]
    fn lighting_with_stripe_applied_rotate_in_object_transform() {
        let eye = vector!(0, 0, -1).normalize();
        let normal = vector!(0, 0, -1).normalize();
        let light = PointLight::new(point!(0, 0, -10), color!(1, 1, 1));
        let mut material = Material::solid(*WHITE);
        material.pattern = Pattern::Stripe(*WHITE, *RED, Transform::identity());
        let transform = Transform::new(Matrix4x4::rotation_z(degrees!(90)));
        assert_eq!(
            *WHITE,
            material.light(&light, &transform, point!(0, 0.9, 0), eye, normal, 0.)
        );
        assert_eq!(
            *RED,
            material.light(&light, &transform, point!(0, 1.1, 0), eye, normal, 0.)
        );
    }
}

#[cfg(test)]
mod reflection_lighting_tests {
    use super::*;
    use crate::primatives::Shape;
    use crate::rays::RayGeneration;
    use crate::world::World;
    use crate::{ray, ray_first_gen};
    use math::color;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::tuple::color::{GREEN, RED, WHITE};

    #[test]
    fn reflect_color_off_a_plane() {
        let mut sphere = Shape::new_sphere_transformed(Matrix4x4::translation(0., 10., 10.));
        sphere.material = Material::solid(*RED);
        let mut material = Material::solid(*GREEN);
        material.reflectivity = 0.25;
        let mut plane = Shape::new_plane();
        plane.material = material;
        let mut world = World::default();
        world.set_light(PointLight::new(Point::origin(), *WHITE));
        world.add(sphere);
        world.add(plane);
        // Shoot straight at sphere
        assert_eq!(
            *RED,
            world.color_at(ray_first_gen!((0., 5., 5.), (0., 10., 10.)))
        );
        // Bounce off
        assert_eq!(
            color!(0.25, 1, 0, 1),
            world.color_at(ray_first_gen!((0., 10., -10.), (0., -10., 10.)))
        );
    }

    #[test]
    fn just_before_infinite_recursion_ends() {
        // two parallel reflective planes
        let plane1 = solid_reflective_plane(*GREEN, 0.25);
        let mut plane2 = solid_reflective_plane(*RED, 0.25);
        plane2.transform = Transform::new(Matrix4x4::translation(0., 10., 0.));
        let mut world = World::default();
        world.max_ray_generation = 4;
        world.set_light(PointLight::new(Point::origin(), *WHITE));
        world.add(plane1);
        world.add(plane2);
        // Shoot straight at plane 1

        // first case should have no color from the reflection
        let ray = RayGeneration::new_ray_with_generation(
            ray!((0., 5., 0.), (0., -1., 0.)),
            world.max_ray_generation,
        );
        assert_eq!(color!(0, 1, 0), world.color_at(ray));

        // second case should have one color from the reflection
        let ray = RayGeneration::new_ray_with_generation(
            ray!((0., 5., 0.), (0., -1., 0.)),
            world.max_ray_generation - 1,
        );
        assert_eq!(color!(0.25, 1, 0), world.color_at(ray));

        let ray = ray_first_gen!((0., 5., 0.), (0., -1., 0.));
        assert_eq!(color!(0.265625, 1.0625, 0), world.color_at(ray));
    }

    fn solid_reflective_plane(color: Color, reflectivity: f32) -> Shape {
        let mut material = Material::solid(color);
        material.reflectivity = reflectivity;
        let mut plane = Shape::new_plane();
        plane.material = material;
        plane
    }
}
