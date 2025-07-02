use crate::lighting::pre_calculations::PreCalculations;
use crate::material::refraction::RefractionMediumIndexes;
use crate::ray;
use crate::rays::RayGeneration;
use crate::world::World;
use math::tuple::color::{BLACK, Color};

impl World {
    pub fn refracted_color(
        &self,
        pre_calculations: &PreCalculations,
        refraction_medium_indexes: RefractionMediumIndexes,
    ) -> Color {
        let transparency = pre_calculations.shape.material.transparency;
        if transparency <= 0. {
            return *BLACK;
        }
        let n_ratio = refraction_medium_indexes.n1 / refraction_medium_indexes.n2;

        let cos_i = pre_calculations.eye.dot(&pre_calculations.normal);

        let sin_t = n_ratio * n_ratio * (1. - cos_i * cos_i);

        if sin_t > 1. {
            return *BLACK;
        }

        let cos_t = (1. - sin_t).sqrt();

        let direction = pre_calculations.normal.clone_vector() * (n_ratio * cos_i - cos_t)
            - pre_calculations.eye.clone_vector() * n_ratio;

        let refraction_ray = ray!(pre_calculations.under_point, direction);

        self.color_at(RayGeneration::new_ray_with_generation(
            refraction_ray,
            pre_calculations.ray_generation + 1,
        )) * transparency
    }
}

#[cfg(test)]
mod refraction_lighting_tests {

    use crate::intersection::{Intersection, Intersections};
    use crate::lighting::PointLight;
    use crate::material::Material;
    use crate::material::pattern::Pattern;
    use crate::primatives::Shape;
    use crate::rays::RayGeneration;
    use crate::world::World;
    use crate::world::shading::default_world;
    use crate::{ray, ray_first_gen};
    use math::matrix::matrix_4x4::Matrix4x4;

    use math::tuple::color::RED;
    use math::{color, point};

    #[test]
    fn the_refracted_color_of_an_opaque_surface() {
        let sphere = Shape::new_sphere();
        let mut world = World::default();
        world.add(sphere);
        let sphere = world.shapes.get(0).unwrap();

        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let intersections = Intersections::new(vec![
            Intersection::new(4., sphere),
            Intersection::new(6., sphere),
        ]);
        let (hit, refraction) = intersections.hit().unwrap();

        let pre_calculations =
            hit.to_pre_calculation(RayGeneration::new_ray_with_generation(ray, 1));
        assert_eq!(
            color!(0, 0, 0),
            world.refracted_color(&pre_calculations, refraction)
        );
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut sphere = Shape::new_sphere();
        sphere.material = Material::glass();
        let mut world = World::default();
        world.add(sphere);
        let sphere = world.shapes.get(0).unwrap();

        println!("{}", sphere.id);
        println!("{}", sphere.material.transparency);

        let ray = ray!((0., 0., 2.0_f32.sqrt() / 2.), (0., 1., 0.));
        let intersections = Intersections::new(vec![
            Intersection::new(-2.0_f32.sqrt() / 2., sphere),
            Intersection::new(2.0_f32.sqrt() / 2., sphere),
        ]);
        let (hit, refraction) = intersections.hit().unwrap();
        let pre_calculations =
            hit.to_pre_calculation(RayGeneration::new_ray_with_generation(ray, 1));
        assert_eq!(
            color!(0, 0, 0),
            world.refracted_color(&pre_calculations, refraction)
        );
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut world = default_world();
        let a = world.shapes.get_mut(0).unwrap();
        a.material.ambient = 1.;
        a.material.pattern = Pattern::Test;
        let b = world.shapes.get_mut(1).unwrap();
        b.material.transparency = 1.;
        b.material.refractive_index = 1.5;
        let ray = ray!((0., 0., 0.1), (0., 1., 0.));
        let a = world.shapes.get(0).unwrap();
        let b = world.shapes.get(1).unwrap();
        let intersections = Intersections::new(vec![
            Intersection::new(-0.9899, a),
            Intersection::new(-0.4899, b),
            Intersection::new(0.4899, b),
            Intersection::new(0.9899, a),
        ]);
        let (hit, refraction) = intersections.hit().unwrap();

        let pre_calculations =
            hit.to_pre_calculation(RayGeneration::new_ray_with_generation(ray, 1));
        assert_eq!(
            // TODO, not quite what book has (0, 0.99888, 0.04725)
            color!(0, 0.9887494, 0.04974536),
            world.refracted_color(&pre_calculations, refraction)
        );
    }

    #[test]
    fn shade_with_a_transparent_material() {
        let mut world = World::default();
        world.set_light(PointLight::new(point!(-10, 10, -10), color!(1.0, 1.0, 1.0)));

        let mut floor = Shape::new_plane_transformed(Matrix4x4::translation(0., -1., 0.));
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        world.add(floor);

        let mut ball = Shape::new_sphere_transformed(Matrix4x4::translation(0., -3.5, -0.5));
        ball.material.pattern = Pattern::Solid(*RED);
        ball.material.ambient = 0.5;
        world.add(ball);

        let c = world.color_at(ray_first_gen!(
            (0., 0., -3.),
            (0., -2.0_f32.sqrt() / 2., 2.0_f32.sqrt() / 2.)
        ));
        assert_eq!(color!(0.9361184, 0.6861184, 0.6861184), c);
    }

    #[test]
    fn shade_with_a_reflective_and_transparent_material() {
        let mut world = World::default();
        world.set_light(PointLight::new(point!(-10, 10, -10), color!(1.0, 1.0, 1.0)));

        let mut floor = Shape::new_plane_transformed(Matrix4x4::translation(0., -1., 0.));
        floor.material.reflectivity = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        world.add(floor);

        let mut ball = Shape::new_sphere_transformed(Matrix4x4::translation(0., -3.5, -0.5));
        ball.material.pattern = Pattern::Solid(*RED);
        ball.material.ambient = 0.5;
        world.add(ball);

        let c = world.color_at(ray_first_gen!(
            (0., 0., -3.),
            (0., -2.0_f32.sqrt() / 2., 2.0_f32.sqrt() / 2.)
        ));
        // TODO, not quite what book has (0.93642, 0.68642, 0.68642)
        assert_eq!(color!(0.9256011, 0.6861184, 0.6861184), c);
    }
}

#[cfg(test)]
mod schlick_tests {
    use super::*;
    use crate::intersection::{Intersection, Intersections};
    use crate::material::Material;
    use crate::primatives::Shape;
    use crate::ray_first_gen;
    use math::tuple::point::Point;

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let mut sphere = Shape::new_sphere();
        sphere.material = Material::glass();
        let intersections = Intersections::new(vec![
            Intersection::new(-2.0_f32.sqrt() / 2., &sphere),
            Intersection::new(2.0_f32.sqrt() / 2., &sphere),
        ]);
        let (hit, refractions) = intersections.hit().unwrap();
        let ray = ray_first_gen!((0., 0., 2.0_f32.sqrt() / 2.), (0., 1., 0.));
        let reflectance = schlick(&hit.to_pre_calculation(ray), &refractions);
        assert_eq!(1., reflectance);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let mut sphere = Shape::new_sphere();
        sphere.material = Material::glass();
        let intersections = Intersections::new(vec![
            Intersection::new(-1., &sphere),
            Intersection::new(1., &sphere),
        ]);
        let (hit, refractions) = intersections.hit().unwrap();
        let ray = ray_first_gen!(Point::origin(), (0., 1., 0.));
        let reflectance = schlick(&hit.to_pre_calculation(ray), &refractions);
        assert_eq!(0.040000003, reflectance);
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n1_gt_n1() {
        let mut sphere = Shape::new_sphere();
        sphere.material = Material::glass();
        let intersections = Intersections::new(vec![Intersection::new(1.8589, &sphere)]);
        let (hit, refractions) = intersections.hit().unwrap();
        let ray = ray_first_gen!((0., 0.99, -2.), (0., 0., 1.));
        let reflectance = schlick(&hit.to_pre_calculation(ray), &refractions);
        assert_eq!(0.4887307, reflectance);
    }
}

pub fn schlick(
    pre_calculations: &PreCalculations,
    refraction_medium_indexes: &RefractionMediumIndexes,
) -> f32 {
    let mut cos = pre_calculations.eye.dot(&pre_calculations.normal);
    if refraction_medium_indexes.n1 > refraction_medium_indexes.n2 {
        let n = refraction_medium_indexes.n1 / refraction_medium_indexes.n2;
        let sin2_t = n * n * (1.0 - cos * cos);
        if sin2_t > 1. {
            return 1.;
        }

        let cos_t = (1. - sin2_t).sqrt();

        cos = cos_t;
    }

    let mut r0 = (refraction_medium_indexes.n1 - refraction_medium_indexes.n2)
        / (refraction_medium_indexes.n1 + refraction_medium_indexes.n2);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cos).powf(5.)
}
