use crate::obj;
use crate::test_scenes::{AnimationFrame, SceneTiming, TestScene};
use math::tuple::color::{RED, WHITE};
use math::{color, degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{plane, scene};
use std::default::Default;
use std::time::Duration;

pub struct TeapotAnimated;

impl TestScene for TeapotAnimated {
    fn name(&self) -> &'static str {
        "utah_teapot_animated"
    }

    fn animation(&self) -> Option<SceneTiming> {
        Some(SceneTiming{
            duration: Duration::from_secs(10),
            fps: 25.0,
        })
    }

    fn build_world_at_time(&self, frame: &AnimationFrame) -> World {
        let teapot = scene!(
            matrix: matrix4x4!(rotation_y(degrees!(-60))
                               rotation_y(degrees!(360.0 * frame.progress))
            );
            +obj!(path: "objs/teapot.obj";);
        );

        let world_scene = scene!(
            +plane!(pattern: Pattern::Checker(*WHITE, *RED, Transform::identity()););
            +teapot;
        );

        let mut world = World::default();
        world.add(world_scene);
        world.add_light(PointLight::new(
            point!(2, 20, 10),
            color!(1, 0.5, 0.5) * 0.5,
        ));
        world.add_light(PointLight::new(
            point!(-2, 20, -10),
            color!(0.5, 1, 0.5) * 0.5,
        ));
        world.add_light(PointLight::new(
            point!(-10, 20, -2),
            color!(0.5, 0.5, 1) * 0.5,
        ));
        world
    }

    fn build_camera(&self, size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(35));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(8, 6, 4), point!(0, 0.8, 0), vector!(0, 1, 0)).into(),
        );
        camera
    }
}
