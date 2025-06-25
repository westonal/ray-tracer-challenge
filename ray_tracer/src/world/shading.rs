use crate::intersection::Intersect;
use crate::lighting::pre_calculations::PreCalculations;
use crate::rays::Ray;
use crate::world::World;
use math::tuple::color::Color;

impl World {
    pub fn shade(&self, pre_calculations: PreCalculations) -> Color {
        pre_calculations.sphere.material.light(
            &self.light.as_ref().unwrap(),
            pre_calculations.point,
            pre_calculations.eye,
            pre_calculations.normal,
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

#[cfg(test)]
mod world_shading_tests {
    use crate::intersection::Intersection;
    use crate::lighting::{Material, PointLight};
    use crate::primatives::sphere::Sphere;
    use crate::rays::Ray;
    use crate::world::World;
    use math::matrix::matrix_4x4::Matrix4x4;

    use math::{color, point, vector};

    fn default_world() -> World {
        let mut world = World::new();
        world.light = Some(PointLight::new(point!(-10, 10, -10), color!(1.0, 1.0, 1.0)));
        let mut sphere = Sphere::new();
        let mut material = Material::default();
        material.color = color!(0.8, 1., 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        sphere.material = material;
        world.add(sphere);
        world.add(Sphere::new_transformed(Matrix4x4::scale(0.5, 0.5, 0.5)));
        world
    }

    #[test]
    fn shade_an_intersection() {
        let world = default_world();
        let ray = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let first = world.objects.get(0).unwrap();
        let intersection = Intersection::new(4., first);
        let pre_calculations = intersection.to_pre_calculation(ray);
        let c = world.shade(pre_calculations);
        assert_eq!(color!(0.38066125, 0.4758265, 0.28549594), c);
    }

    #[test]
    fn shade_an_intersection_from_inside() {
        let mut world = default_world();
        world.light = Some(PointLight::new(point!(0, 0.25, 0), color!(1., 1., 1.)));
        let ray = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
        let second = world.objects.get(1).unwrap();
        let intersection = Intersection::new(0.5, second);
        let pre_calculations = intersection.to_pre_calculation(ray);
        let c = world.shade(pre_calculations);
        assert_eq!(color!(0.9049845, 0.9049845, 0.9049845), c);
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
        assert_eq!(color!(0.38066125, 0.4758265, 0.28549594), c);
    }
}
