use crate::obj;
use crate::test_scenes::TestScene;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::{RED, WHITE};
use math::{degrees, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::Shape;
use ray_tracer::scene;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use std::default::Default;

pub struct Pawn {}

impl TestScene for Pawn {
    fn name() -> &'static str {
        "chess_pawn"
    }

    fn build_world() -> World {
        let pawn = scene!(
            matrix: Matrix4x4::rotation_y(degrees!(-60));
            +obj!(path: "objs/chess/pawn.obj");
        );

        let world_scene = scene!(
            +{
                let mut plane = Shape::new_plane();
                plane.material.pattern = Pattern::Checker(*WHITE, *RED, Transform::identity());
                plane
            };
            +pawn;
        );

        let mut world = World::default();
        world.add(world_scene);
        world.add_light(PointLight::new(point!(2, 20, 10), *WHITE));
        world
    }

    fn build_camera(size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(35));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(8, 6, 4), point!(0, 0.8, 0), vector!(0, 1, 0)).into(),
        );
        camera
    }
}
