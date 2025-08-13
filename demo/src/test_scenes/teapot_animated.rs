use crate::obj;
use crate::test_scenes::{AnimationFrame, AnimationSpec, TestScene};
use animation::animation_spec;
use math::tuple::color::{RED, WHITE};
use math::{degrees, matrix4x4, point};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::world::World;
use ray_tracer::{light, look_at};
use ray_tracer::{plane, scene};
use std::default::Default;
use std::time::Duration;

pub struct TeapotAnimated;

impl TestScene for TeapotAnimated {
    fn name(&self) -> &'static str {
        "utah_teapot_animated"
    }

    fn animation_spec(&self) -> Option<AnimationSpec> {
        Some(animation_spec!(4;seconds @25;fps))
    }

    fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
        let teapot = scene!(
            matrix: matrix4x4!(rotation_y(degrees!(-60))
                               rotation_y(degrees!(360.0 * frame.loop_progress))
            );
            +obj!(path: "objs/teapot.obj";);
        );

        let world_scene = scene!(
            +light!(point!(2, 20, 10));
            +plane!(pattern: Pattern::Checker(*WHITE, *RED, Transform::identity()););
            +teapot;
        );

        let mut world = World::default();
        world.push(world_scene);
        world
    }

    fn build_camera(&self, size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(35));
        camera.set_transform(look_at!(point!(8, 6, 4) => point!(0, 0.8, 0)));
        camera
    }
}
