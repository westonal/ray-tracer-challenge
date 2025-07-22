use crate::obj;
use crate::test_scenes::TestScene;
use math::tuple::color::{RED, WHITE};
use math::{degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{plane, scene};
use std::default::Default;

pub struct Pawn {}

impl TestScene for Pawn {
    fn name() -> &'static str {
        "chess_pawn"
    }

    fn build_world() -> World {
        let pawn = scene!(
            matrix: matrix4x4!(rotation_y(degrees!(-60)));
            +obj!(
                path: "objs/chess/pawn.obj";
                material: Material::glass();
            );
        );

        let world_scene = scene!(
            +plane!(pattern: Pattern::Checker(
                                *WHITE,
                                *RED,
                                Transform::new(matrix4x4!(scale_all(2.6) translation(0.5, 0., 0.5)))
                             );
            );
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
            ViewMatrix::new_look_at(point!(4, 6, 8), point!(0, 1.4, 0), vector!(0, 1, 0)).into(),
        );
        camera
    }
}
