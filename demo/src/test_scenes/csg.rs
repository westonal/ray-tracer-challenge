use crate::test_scenes::TestScene;
use math::matrix::matrix_4x4::Matrix4x4ScaleAll;
use math::tuple::color::{BLACK, BLUE, GREEN, RED, WHITE};
use math::{color, degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{csg, cube, gradient_stops, plane, scene, sphere};

pub struct Csg {}

impl TestScene for Csg {
    fn name() -> &'static str {
        "csg"
    }

    fn build_world() -> World {
        let mut world = World::default();
        world.set_light(PointLight::new(point!(40, 40, 20), *WHITE * 0.9));
        let mut floor = plane!();
        floor.material.pattern = Pattern::Checker(
            *WHITE,
            *BLACK,
            Transform::new(matrix4x4!(rotation_y(degrees!(45)) scale_all(2.))),
        );
        floor.material.reflectivity = 0.5;
        world.add(floor);

        world.add(scene!(
            matrix: matrix4x4!(
                translation(0., 4., 0.)
                scale_all(4.)
            );
            +{
                csg!(cube!()) - csg!({
                    let mut sphere = sphere!(matrix: matrix4x4!(scale_all(1.25)));
                    sphere.material.pattern = Pattern::Solid(*RED);
                    sphere
                }) ^ csg!({let mut sphere = sphere!(matrix: matrix4x4!(scale_all(1.45)));
                    sphere.material.pattern = Pattern::Solid(*BLUE);
                    sphere
                })
            };
            // Glass core
            // TODO investigate: cube casts shadow but only when above is uncommented
            //   Possible failure to count correctly in the flat tree
            +{
                csg!({
                    let mut sphere = sphere!(matrix: matrix4x4!(scale_all(1.25)));
                    sphere.material = Material::glass();
                    sphere
                }) ^ csg!(
                    {let mut cube = cube!();
                        cube.material = Material::glass();
                    cube})
            };
        ));
        world
    }

    fn build_camera(size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(30));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(17, 19, 23), point!(1, 2, -3), vector!(0, 1, 0)).into(),
        );
        camera
    }
}
