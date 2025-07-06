use crate::intersection::Intersect;
use crate::lighting::pre_calculations::PreCalculations;
use crate::lighting::refraction_lighting::schlick;
use crate::lighting::surface_hit::SurfaceHit;
use crate::material::refraction::RefractionMediumIndexes;
use crate::ray;
use crate::rays::RayGeneration;
use crate::world::World;
use math::color;
use math::tuple::color::Color;

impl World {
    pub fn shade(&self, pre_calculations: PreCalculations) -> Color {
        self.shade_with_refraction(pre_calculations, RefractionMediumIndexes::new(1.0, 1.0))
    }

    pub fn shade_with_refraction(
        &self,
        pre_calculations: PreCalculations,
        refraction_medium_indexes: RefractionMediumIndexes,
    ) -> Color {
        let mut result = color!(0, 0, 0, 0);
        let direct_lights = self.direct_lights_2(&pre_calculations.surface_hit);
        let material = &pre_calculations.shape.material;
        for light in &self.lights {
            // TODO, multilight support would light each in turn if they were direct.
            let shadow_factor = if direct_lights.is_empty() { 1. } else { 0. };
            result = result
                + material.light(
                    light,
                    &pre_calculations.shape.transform,
                    pre_calculations.surface_hit.point,
                    pre_calculations.eye,
                    pre_calculations.normal,
                    shadow_factor,
                )
        }
        if pre_calculations.ray_generation < self.max_ray_generation {
            let (reflect, refract) = if material.reflectivity > 0. && material.transparency > 0. {
                let reflectance = schlick(&pre_calculations, &refraction_medium_indexes);
                (reflectance, 1. - reflectance)
            } else {
                (1., 1.)
            };

            result = result
                + self.refracted_color(&pre_calculations, refraction_medium_indexes) * refract;
            result = result + self.reflection_color(pre_calculations) * reflect;
        }
        result.clamp_alpha();
        result
    }

    pub(crate) fn reflection_color(&self, pre_calculations: PreCalculations) -> Color {
        let r = pre_calculations.shape.material.reflectivity;
        if r > 0. {
            self.color_at(RayGeneration::new_ray_with_generation(
                ray!(pre_calculations.surface_hit.point, pre_calculations.reflection, epsilon:SurfaceHit::EPSILON),
                pre_calculations.ray_generation + 1,
            )) * r
        } else {
            color!(0, 0, 0, 0)
        }
    }

    pub fn color_at(&self, ray: RayGeneration) -> Color {
        let intersections = self.intersect(*ray);
        if let Some((hit, refractions)) = intersections.hit() {
            let pre_calculations = hit.to_pre_calculation(ray);
            self.shade_with_refraction(pre_calculations, refractions)
        } else {
            self.background
        }
    }
}

#[cfg(test)]
mod world_shading_tests {
    use crate::intersection::Intersection;
    use crate::lighting::PointLight;

    use crate::ray_first_gen;
    use crate::world::World;
    use math::{assert_color, color, point, vector};

    #[test]
    fn shade_an_intersection() {
        let world = World::default_world();
        let ray = ray_first_gen!(point!(0, 0, -5), vector!(0, 0, 1));
        let first = world.shapes.get(0).unwrap();
        let intersection = Intersection::new(4., first);
        let pre_calculations = intersection.to_pre_calculation(ray);
        let c = world.shade(pre_calculations);
        assert_color!(color!(0.3807, 0.4758, 0.2855), c);
    }

    #[test]
    fn shade_an_intersection_from_inside() {
        let mut world = World::default_world();
        world.set_light(PointLight::new(point!(0, 0.25, 0), color!(1, 1, 1)));
        let ray = ray_first_gen!(point!(0, 0, 0), vector!(0, 0, 1));
        let second = world.shapes.get(1).unwrap();
        let intersection = Intersection::new(0.5, second);
        let pre_calculations = intersection.to_pre_calculation(ray);
        let c = world.shade(pre_calculations);
        assert_color!(color!(0.9050, 0.9050, 0.9050), c);
    }

    #[test]
    fn color_when_ray_misses() {
        let world = World::default_world();
        let ray = ray_first_gen!(point!(0, 0, -5), vector!(0, 1, 0));
        let c = world.color_at(ray);
        assert_eq!(color!(0., 0., 0., 0.), c);
    }

    #[test]
    fn color_when_ray_misses_alt_background_color() {
        let mut world = World::default_world();
        world.background = color!(0., 1., 0.);
        let ray = ray_first_gen!(point!(0, 0, -5), vector!(0, 1, 0));
        let c = world.color_at(ray);
        assert_eq!(color!(0., 1., 0., 1.), c);
    }

    #[test]
    fn shade_an_intersection_with_color_at() {
        let world = World::default_world();
        let ray = ray_first_gen!(point!(0, 0, -5), vector!(0, 0, 1));
        let c = world.color_at(ray);
        assert_color!(color!(0.3807, 0.4758, 0.2855), c);
    }
}

#[cfg(test)]
mod world_shadow_shading_tests {
    use super::*;
    use crate::intersection::Intersection;
    use crate::lighting::PointLight;
    use crate::primatives::Shape;
    use crate::ray_first_gen;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::{point, vector};

    #[test]
    fn shade_when_given_intersection_in_shadow() {
        let mut world = World::default();
        world.set_light(PointLight::new(point!(0, 0, -10), color!(1, 1, 1)));
        world.add(Shape::new_sphere());
        world.add(Shape::new_sphere_transformed(Matrix4x4::translation(
            0., 0., 10.,
        )));
        let second = world.shapes.get(1).unwrap();
        let intersection = Intersection::new(4., &second);
        let ray = ray_first_gen!(point!(0, 0, 5), vector!(0, 0, 1));
        let pre_calculations = intersection.to_pre_calculation(ray);
        let color = world.shade(pre_calculations);
        assert_eq!(color!(0.1, 0.1, 0.1), color);
    }
}

#[cfg(test)]
mod world_pattern_shading_tests {
    use super::*;
    use crate::intersection::Intersection;
    use crate::lighting::PointLight;
    use crate::material::Material;
    use crate::material::pattern::Pattern;
    use crate::primatives::Shape;
    use crate::ray;
    use crate::rays::Ray;
    use crate::transform::Transform;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::tuple::color::{BLUE, GREEN, RED};
    use math::{degrees, point, vector};

    struct TestScene {
        world: World,
    }

    impl TestScene {
        fn color_ray(&self, ray: Ray) -> Color {
            let first = self.world.shapes.get(0).unwrap();
            let intersection = Intersection::new(4., &first);
            let pre_calculations =
                intersection.to_pre_calculation(RayGeneration::new_first_generation_ray(ray));
            self.world.shade(pre_calculations)
        }
    }

    impl TestScene {
        fn given(stripe_transform: Matrix4x4, plane_transformation: Matrix4x4) -> TestScene {
            let mut world = World::default();
            world.set_light(PointLight::new(point!(0, 0, -10), color!(1, 1, 1)));
            let mut plane = Shape::new_plane_transformed(plane_transformation);
            let mut material = Material::solid(*BLUE);
            material.pattern = Pattern::Stripe(*GREEN, *RED, Transform::new(stripe_transform));
            plane.material = material;
            world.add(plane);
            TestScene { world }
        }
    }

    #[test]
    fn stripes_on_non_rotated_plane() {
        let stripe_transform = Matrix4x4::identity();
        let plane_transformation = Matrix4x4::identity();
        let scene = TestScene::given(stripe_transform, plane_transformation);
        assert_eq!(
            scene.color_ray(ray!(point!(0, 3, 0), vector!(0, -1, 0))),
            *GREEN
        );
        assert_eq!(
            scene.color_ray(ray!(point!(1, 3, 0), vector!(0, -1, 0))),
            *RED
        );
    }

    #[test]
    fn stripes_on_rotated_plane() {
        let stripe_transform = Matrix4x4::identity();
        let plane_transformation = Matrix4x4::rotation_y(degrees!(90));
        let scene = TestScene::given(stripe_transform, plane_transformation);
        assert_eq!(
            scene.color_ray(ray!(point!(0, 3, 0), vector!(0, -1, 0))),
            *GREEN
        );
        assert_eq!(
            scene.color_ray(ray!(point!(0, 3, 1), vector!(0, -1, 0))),
            *RED
        );
    }

    #[test]
    fn stripes_on_rotated_pattern() {
        let stripe_transform = Matrix4x4::rotation_y(degrees!(90));
        let plane_transformation = Matrix4x4::identity();
        let scene = TestScene::given(stripe_transform, plane_transformation);
        assert_eq!(
            scene.color_ray(ray!(point!(0, 3, 0), vector!(0, -1, 0))),
            *GREEN
        );
        assert_eq!(
            scene.color_ray(ray!(point!(0, 3, 1), vector!(0, -1, 0))),
            *RED
        );
    }

    #[test]
    fn stripes_on_rotated_pattern_and_scene() {
        let stripe_transform = Matrix4x4::rotation_y(degrees!(90));
        let plane_transformation = Matrix4x4::rotation_z(degrees!(-90));
        let scene = TestScene::given(stripe_transform, plane_transformation);
        assert_eq!(
            scene.color_ray(ray!(point!(3, 0.1, 0), vector!(-1, 0, 0))),
            *GREEN
        );
        assert_eq!(
            scene.color_ray(ray!(point!(3, 0.1, 1), vector!(-1, 0, 0))),
            *RED
        );
    }
}
