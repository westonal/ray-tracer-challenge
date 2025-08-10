use crate::test_scenes::TestScene;
use math::tuple::color::{BLACK, BLUE, GREEN, RED, WHITE};
use math::{color, degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::{Shape, Triangle};
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{gradient_stops, plane};

pub struct Triangles;

impl TestScene for Triangles {
    fn name(&self) -> &'static str {
        "triangles"
    }

    fn build_world(&self) -> World {
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

        world.add(Shape::new_triangle_transformed(
            matrix4x4!(translation(0., 1., 0.)),
            Triangle::new([point!(0, 0, 0), point!(0, 1, 0), point!(1, 1, 0)]),
        ));

        let mut triangle = Shape::new_triangle_transformed(
            matrix4x4!(
                scale_all(3.5)
                translation(2., 1., -3.)
                rotation_y(degrees!(55))
            ),
            Triangle::new([point!(0, 0, 0), point!(0, 1, 0), point!(1, 1, 0)]),
        );
        triangle.material = Material::glass();
        triangle.material.pattern = Pattern::Solid(color!(0.5, 0., 0.));
        triangle.material.ambient = 0.3;

        world.add(triangle);

        let mut triangle = Shape::new_triangle_transformed(
            matrix4x4!(
                translation(-8., 4., -3.)
                scale_all(4.)
                rotation_y(degrees!(0))
            ),
            Triangle::new([point!(0, 0, 0), point!(0, 1, 0), point!(1, 1, 0)]),
        );
        triangle.material.pattern = Pattern::Gradient(
            gradient_stops!(
                0. => *RED,
                0.5 => *BLUE,
                1. => *GREEN
            ),
            Transform::new(matrix4x4!(
                rotation_y(degrees!(45))
                translation(-1., 0., 0.)
                scale_all(2.)
            )),
        );
        triangle.material.reflectivity = 0.1;
        world.add(triangle);
        world
    }

    fn build_camera(&self, size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(30));
        camera.set_transform(ViewMatrix::new_look_at(
            point!(17, 19, 23),
            point!(1, 2, -3),
            vector!(0, 1, 0),
        ));
        camera
    }
}
