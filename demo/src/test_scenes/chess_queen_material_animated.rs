use crate::obj;
use crate::test_scenes::{AnimationFrame, AnimationSpec, TestScene};
use animation::animation_spec;
use math::tuple::color::{BLACK, WHITE};
use math::{color, degrees, matrix4x4, point};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::world::World;
use ray_tracer::{light, look_at};
use ray_tracer::{plane, scene};
use std::default::Default;
use std::time::Duration;

pub struct QueenMaterialAnimation;

impl TestScene for QueenMaterialAnimation {
    fn name(&self) -> &'static str {
        "chess_queen_material_animation"
    }

    fn animation_spec(&self) -> Option<AnimationSpec> {
        Some(animation_spec!(8;seconds @29.97;fps))
    }

    fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
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
                    glass.refractive_index = 1.5;
                    glass
                };
            );
        );

        let world_scene = scene!(
            matrix: matrix4x4!(
              rotation_y(degrees!(360.0 * frame.loop_progress))
            );
            +plane!(pattern: Pattern::Checker(
                                *WHITE,
                                *BLACK,
                                Transform::new(matrix4x4!(scale_all(2.6) translation(0.5, 0., 0.5)))
                             );
            );
            +queen;
        );

        let mut world = World::default();
        world.render_preferences.max_ray_generation = 5;
        world.push(world_scene);
        world.push(light!(point!(2, 20, 10)));
        world
    }

    fn build_camera_for_frame(&self, size: Size, frame: &AnimationFrame) -> Camera {
        let prog = frame.loop_progress;
        let mut camera = Camera::new(size, degrees!(35.0 - cycle(prog) * 20.0));
        camera.set_transform(look_at!(point!(4, 6, 8) => point!(0, 1.8 + cycle(prog) * 1.25, 0)));
        camera
    }
}

fn cycle(input: f32) -> f32 {
    degrees!(input * 180.0).sin_cos().0
}
