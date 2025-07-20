use crate::test_scenes::TestScene;
use math::tuple::color::{BLACK, WHITE};
use math::tuple::point::Point;
use math::{degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{cube, plane, scene, sphere};

pub struct GlassSphereWithAir {}

impl TestScene for GlassSphereWithAir {
    fn name() -> &'static str {
        "glass_sphere_with_air"
    }

    fn build_world() -> World {
        let mut world = World::default();
        world.add_light(PointLight::new(point!(-300, 200, 20), *WHITE));
        let mut plane = plane!(matrix: matrix4x4!(translation(0., -32., 0.)));
        plane.material.pattern =
            Pattern::Checker(*BLACK, *WHITE, Transform::new(matrix4x4!(scale_all(3.))));
        world.add(plane);

        let scene = scene!(
            matrix: matrix4x4!(scale_all(5.));
            bounding_volume: cube!();
            +{
                let mut sphere = sphere!();
                sphere.material = Material::glass();
                sphere
            };
            +scene!(
                matrix: matrix4x4!(scale_all(0.5));
                +{
                    let mut sphere = sphere!();
                    sphere.material = Material::air();
                    sphere
                };
            );
        );

        world.add(scene);
        world
    }

    fn build_camera(size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(20));
        camera.set_transform(*ViewMatrix::new_look_at(
            point!(0, 40, 0),
            Point::origin(),
            vector!(0, 0, 1),
        ));
        camera
    }
}
