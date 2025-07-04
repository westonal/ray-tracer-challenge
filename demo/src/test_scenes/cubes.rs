use crate::test_scenes::TestScene;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::{BLACK, BLUE, GREEN, RED, WHITE};
use math::{color, degrees, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::gradient_stops;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::Shape;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;

pub struct Cubes {}

impl TestScene for Cubes {
    fn name() -> &'static str {
        "cubes"
    }

    fn build_world() -> World {
        let mut world = World::default();
        world.set_light(PointLight::new(point!(40, 40, 20), *WHITE * 0.9));
        let mut floor = Shape::new_plane();
        floor.material.pattern = Pattern::Checker(
            *WHITE,
            *BLACK,
            Transform::new(Matrix4x4::rotation_y(degrees!(45)).pre_scale_all(2.)),
        );
        floor.material.reflectivity = 0.5;
        world.add(floor);

        world.add(Shape::new_cube_transformed(
            Matrix4x4::identity().pre_translation(0., 1., 0.),
        ));

        let mut cube = Shape::new_cube_transformed(
            Matrix4x4::identity()
                .pre_scale_all(3.5)
                .pre_translation(2., 1., -3.)
                .pre_rotation_y(degrees!(55)),
        );
        cube.material = Material::glass();
        cube.material.pattern = Pattern::Solid(color!(0.5, 0., 0.));
        cube.material.ambient = 0.3;

        world.add(cube);

        let mut cube = Shape::new_cube_transformed(
            Matrix4x4::identity()
                .pre_translation(-8., 4., -3.)
                .pre_scale_all(4.)
                .pre_rotation_y(degrees!(0)),
        );
        cube.material.pattern = Pattern::Gradient(
            gradient_stops!(
                0. => *RED,
                0.5 => *BLUE,
                1. => *GREEN
            ),
            Transform::new(
                Matrix4x4::rotation_y(degrees!(45))
                    .pre_translation(-1., 0., 0.)
                    .pre_scale_all(2.),
            ),
        );
        cube.material.reflectivity = 0.1;
        world.add(cube);
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
