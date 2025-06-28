use crate::intersection::Intersect;
use crate::lighting::PointLight;
use crate::lighting::pre_calculations::PreCalculations;
use crate::material::Material;
use crate::primatives::Shape;
use crate::rays::Ray;
use crate::world::World;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::Color;
use math::{color, point};

impl World {
    pub fn shade(&self, pre_calculations: PreCalculations) -> Color {
        let direct_lights = self.direct_lights(pre_calculations.over_point);
        let light = &self.light.as_ref().unwrap();
        // TODO, multilight support would light each in turn if they were direct.
        let shadow_factor = if direct_lights.is_empty() { 1. } else { 0. };
        pre_calculations.shape.material.light(
            light,
            pre_calculations.over_point,
            pre_calculations.eye,
            pre_calculations.normal,
            shadow_factor,
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersect(ray);
        if let Some(hit) = intersections.hit() {
            let pre_calculations = hit.to_pre_calculation(ray);
            self.shade(pre_calculations)
        } else {
            self.background
        }
    }
}

/// A test world
pub fn default_world() -> World {
    let mut world = World::new();
    world.light = Some(PointLight::new(point!(-10, 10, -10), color!(1.0, 1.0, 1.0)));
    let mut sphere = Shape::new_sphere();
    let mut material = Material::default();
    material.color = color!(0.8, 1., 0.6);
    material.diffuse = 0.7;
    material.specular = 0.2;
    // turn off shadows
    material.shadow_boost = 1.;
    sphere.material = material;
    world.add(sphere);
    let mut material = Material::default();
    material.shadow_boost = 1.;
    let mut sphere = Shape::new_sphere_transformed(Matrix4x4::scale(0.5, 0.5, 0.5));
    sphere.material = material;
    world.add(sphere);
    world
}

#[cfg(test)]
mod world_shading_tests {
    use crate::intersection::Intersection;
    use crate::lighting::PointLight;

    use crate::rays::Ray;

    use crate::world::shading::default_world;
    use math::{color, point, vector};

    #[test]
    fn shade_an_intersection() {
        let world = default_world();
        let ray = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let first = world.shapes.get(0).unwrap();
        let intersection = Intersection::new(4., first);
        let pre_calculations = intersection.to_pre_calculation(ray);
        let c = world.shade(pre_calculations);
        assert_eq!(color!(0.3804233, 0.4755291, 0.28531748), c);
    }

    #[test]
    fn shade_an_intersection_from_inside() {
        let mut world = default_world();
        world.light = Some(PointLight::new(point!(0, 0.25, 0), color!(1., 1., 1.)));
        let ray = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
        let second = world.shapes.get(1).unwrap();
        let intersection = Intersection::new(0.5, second);
        let pre_calculations = intersection.to_pre_calculation(ray);
        let c = world.shade(pre_calculations);
        assert_eq!(color!(0.90168566, 0.90168566, 0.90168566), c);
    }

    #[test]
    fn color_when_ray_misses() {
        let world = default_world();
        let ray = Ray::new(point!(0, 0, -5), vector!(0, 1, 0));
        let c = world.color_at(ray);
        assert_eq!(color!(0., 0., 0., 0.), c);
    }

    #[test]
    fn color_when_ray_misses_alt_background_color() {
        let mut world = default_world();
        world.background = color!(0., 1., 0.);
        let ray = Ray::new(point!(0, 0, -5), vector!(0, 1, 0));
        let c = world.color_at(ray);
        assert_eq!(color!(0., 1., 0., 1.), c);
    }

    #[test]
    fn shade_an_intersection_with_color_at() {
        let world = default_world();
        let ray = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let c = world.color_at(ray);
        assert_eq!(color!(0.3804233, 0.4755291, 0.28531748), c);
    }
}

#[cfg(test)]
mod world_shadow_shading_tests {
    use super::*;
    use crate::intersection::Intersection;
    use math::vector;
    #[test]
    fn shade_when_given_intersection_in_shadow() {
        let mut world = World::new();
        world.light = Some(PointLight::new(point!(0, 0, -10), color!(1., 1., 1.)));
        world.add(Shape::new_sphere());
        world.add(Shape::new_sphere_transformed(Matrix4x4::translation(
            0., 0., 10.,
        )));
        let second = world.shapes.get(1).unwrap();
        let intersection = Intersection::new(4., &second);
        let ray = Ray::new(point!(0, 0, 5), vector!(0, 0, 1));
        let pre_calculations = intersection.to_pre_calculation(ray);
        let color = world.shade(pre_calculations);
        assert_eq!(color!(0.1, 0.1, 0.1), color);
    }
}
