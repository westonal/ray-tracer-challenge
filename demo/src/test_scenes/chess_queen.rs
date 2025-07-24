use crate::obj;
use crate::test_scenes::TestScene;
use math::tuple::color::{BLACK, WHITE};
use math::{color, degrees, matrix4x4, point, vector};
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

pub struct Queen;

impl TestScene for Queen {
    fn name() -> &'static str {
        "chess_queen"
    }

    fn build_world() -> World {
        let queen = scene!(
            matrix: matrix4x4!(
                translation(0.75, 0., 0.)
                scale_all(0.22)
            );
            +obj!(
                path: "objs/chess/queen.obj";
                material: {
                    let mut glass = Material::glass();
                    glass.reflectivity = 0.9;
                    glass.transparency = 0.7;
                    glass.ambient = 0.2;
                    glass.pattern = Pattern::Solid(color!(0.5, 0.5, 0.5));
                    glass
                };
            );
        );

        let world_scene = scene!(
            +plane!(pattern: Pattern::Checker(
                                *WHITE,
                                *BLACK,
                                Transform::new(matrix4x4!(scale_all(2.6) translation(0.5, 0., 0.5)))
                             );
            );
            +queen;
        );

        let mut world = World::default();
        world.add(world_scene);
        world.add_light(PointLight::new(point!(2, 20, 10), *WHITE));
        world
    }

    fn build_camera(size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(35));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(4, 6, 8), point!(0, 1.8, 0), vector!(0, 1, 0)).into(),
        );
        camera
    }
}
